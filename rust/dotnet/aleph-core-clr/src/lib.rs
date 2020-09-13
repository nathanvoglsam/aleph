//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub mod raw;

use crate::raw::{
    coreclr_create_delegate_fn, coreclr_initialize_fn, coreclr_shutdown_fn,
    CORECLR_CREATE_DELEGATE_FN_SYMBOL, CORECLR_INITIALIZE_FN_SYMBOL, CORECLR_SHUTDOWN_FN_SYMBOL,
};
use aleph_target::Platform;
use libloading::{Error, Symbol};
use std::collections::HashMap;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_int, c_uint, c_void};
use std::path::Path;
use std::sync::Mutex;

///
/// This represents the set of errors that can occur when loading the coreclr dynamic library
///
#[derive(Debug)]
pub enum LibraryLoadError {
    ///
    /// An error occurred when trying to open a shared object/dynamically linked library
    ///
    LoadError(libloading::Error),

    ///
    /// An IO error occurred
    ///
    IOError(std::io::Error),
}

impl From<libloading::Error> for LibraryLoadError {
    fn from(error: Error) -> Self {
        LibraryLoadError::LoadError(error)
    }
}

impl From<std::io::Error> for LibraryLoadError {
    fn from(error: std::io::Error) -> Self {
        LibraryLoadError::IOError(error)
    }
}

///
/// A wrapper around dynamically loading the CoreCLR library
///
pub struct Library {
    library: libloading::Library,
}

impl Library {
    ///
    /// Wrapper around dlopen/LoadLibrary. Will load the library directly by the platform's name for
    /// coreclr
    ///
    pub fn new() -> Result<Library, LibraryLoadError> {
        let library = libloading::Library::new(Self::library_name())?;
        let out = Self { library };
        Ok(out)
    }

    ///
    /// Wrapper around dlopen/LoadLibrary. Will load the library by appending the name of the
    /// library to the given path
    ///
    pub fn new_in_path(path: impl AsRef<Path>) -> Result<Library, LibraryLoadError> {
        let path = path.as_ref();
        let path = path.join(Self::library_name());
        let path = path.canonicalize()?;

        let library = libloading::Library::new(&path)?;
        let out = Self { library };
        Ok(out)
    }

    ///
    /// Returns the name of the coreclr library on the platform the library is compiled for
    ///
    /// If the platform is unknown (`Platform::Unknown`) this will return `"unknown"`
    ///
    pub const fn library_name() -> &'static str {
        match aleph_target::build::target_platform() {
            Platform::WindowsGNU => "coreclr.dll",
            Platform::WindowsMSVC => "coreclr.dll",
            Platform::Linux => "libcoreclr.so",
            Platform::Android => "libcoreclr.so",
            Platform::Unknown => "unknown",
        }
    }
}

///
/// This wraps an error code that can be returned from any of the coreclr raw functions
///
#[derive(Debug)]
#[repr(transparent)]
pub struct RuntimeError {
    error_code: c_int,
}

///
/// Error enum for the set of errors that can be emitted while getting a delegate from an assembly
///
pub enum CreateDelegateError {
    ///
    /// An error occurred within the coreclr library itself
    ///
    RuntimeError(RuntimeError),

    ///
    /// An error occurred when creating c-strings for the raw API
    ///
    CStringError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for CreateDelegateError {
    fn from(error: std::ffi::NulError) -> Self {
        CreateDelegateError::CStringError(error)
    }
}

///
/// Error enum for the set of errors that can be emitted by building a `Runtime` object with the
/// `RuntimeBuilder` builder
///
#[derive(Debug)]
pub enum RuntimeBuildError {
    ///
    /// An error occurred when trying to get the symbols from the dynamic library
    ///
    LibraryLoadError(libloading::Error),

    ///
    /// An error occurred within the coreclr library itself
    ///
    RuntimeError(RuntimeError),

    ///
    /// An IO error occurred
    ///
    IOError(std::io::Error),

    ///
    /// An error occurred when creating c-strings for the raw API
    ///
    CStringError(std::ffi::NulError),
}

impl From<libloading::Error> for RuntimeBuildError {
    fn from(error: libloading::Error) -> Self {
        RuntimeBuildError::LibraryLoadError(error)
    }
}

impl From<std::io::Error> for RuntimeBuildError {
    fn from(error: std::io::Error) -> Self {
        RuntimeBuildError::IOError(error)
    }
}

impl From<std::ffi::NulError> for RuntimeBuildError {
    fn from(error: std::ffi::NulError) -> Self {
        RuntimeBuildError::CStringError(error)
    }
}

///
/// A build struct for wrapping the construction of a `Runtime` object
///
pub struct RuntimeBuilder {
    exe_path: Option<String>,
    properties: HashMap<CString, CString>,
}

impl RuntimeBuilder {
    ///
    /// Creates a new builder with the default options
    ///
    pub fn new() -> Self {
        Self {
            exe_path: None,
            properties: Default::default(),
        }
    }

    ///
    /// Add a property with a given name where the value is a list of strings.
    ///
    /// This will automatically handle flattening the list of strings given down to a single string
    /// with a separator between them (how coreclr want's these lists) and abstract away the slight
    /// platform differences that are associated with doing this (the separator is different on
    /// windows thant it is elsewhere).
    ///
    pub fn property_string_list(
        mut self,
        property: &str,
        mut items: impl Iterator<Item = impl AsRef<str>>,
    ) -> Self {
        const SEPARATOR: char = if aleph_target::build::target_platform().is_windows() {
            ';'
        } else {
            ':'
        };

        let mut value = String::new();

        while let Some(item) = items.next() {
            let item = item.as_ref();
            value.push_str(item);
            value.push(SEPARATOR);
        }

        let property = CString::new(property).unwrap();
        let value = CString::new(value).unwrap();
        self.properties.insert(property, value);

        self
    }

    ///
    /// Specify the list of trusted assemblies.
    ///
    /// Maps to `TRUSTED_PLATFORM_ASSEMBLIES`. [more info](https://docs.microsoft.com/en-us/dotnet/core/dependency-loading/default-probing)
    ///
    pub fn trusted_platform_assemblies(
        self,
        assemblies: impl Iterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.property_string_list("TRUSTED_PLATFORM_ASSEMBLIES", assemblies)
    }

    ///
    /// Specify the set of app folders where app assemblies may be loaded from
    ///
    /// Maps to `APP_PATHS` [more info](https://docs.microsoft.com/en-us/dotnet/core/dependency-loading/default-probing)
    ///
    pub fn app_paths(self, paths: impl Iterator<Item = impl AsRef<str>>) -> Self {
        self.property_string_list("APP_PATHS", paths)
    }

    ///
    /// Specify list of folders where native libraries can be loaded from
    ///
    /// Maps to `NATIVE_DLL_SEARCH_DIRECTORIES` [more info](https://docs.microsoft.com/en-us/dotnet/core/dependency-loading/default-probing)
    ///
    pub fn native_dll_search_directories(
        self,
        paths: impl Iterator<Item = impl AsRef<str>>,
    ) -> Self {
        self.property_string_list("NATIVE_DLL_SEARCH_DIRECTORIES", paths)
    }

    ///
    /// Creates a coreclr runtime instance, loading functions from the given library instance and
    /// using the given name for the appdomain.
    ///
    /// This handles trying to load function pointers from the dynamically linked library and
    /// calling `coreclr_initialize` to create the runtime instance.
    ///
    pub fn build(
        self,
        library: &Library,
        app_domain_name: impl Into<String>,
    ) -> Result<Runtime, RuntimeBuildError> {
        // Load the needed functions from CoreCLR
        let library = &library.library;
        let coreclr_initialize: Symbol<coreclr_initialize_fn>;
        let coreclr_shutdown: Symbol<coreclr_shutdown_fn>;
        let coreclr_create_delegate: Symbol<coreclr_create_delegate_fn>;
        unsafe {
            coreclr_initialize = library.get(CORECLR_INITIALIZE_FN_SYMBOL.as_bytes())?;
            coreclr_shutdown = library.get(CORECLR_SHUTDOWN_FN_SYMBOL.as_bytes())?;
            coreclr_create_delegate = library.get(CORECLR_CREATE_DELEGATE_FN_SYMBOL.as_bytes())?;
        }

        // Get the specified exe_path or default to getting it ourselves
        let exe_path = if let Some(exe_path) = self.exe_path {
            exe_path
        } else {
            // Get current exe path as string, with a null terminator
            std::env::current_exe()?.to_str().unwrap().to_string()
        };
        let exe_path = CString::new(exe_path)?;
        let exe_path_ptr = exe_path.as_ptr();

        // Convert to string and push a null terminator as the raw API wants c-strings
        let app_domain_name = app_domain_name.into();
        let app_domain_name = CString::new(app_domain_name)?;
        let app_domain_name_ptr = app_domain_name.as_ptr();

        // Flatten the properties down to the lists required by the API
        let property_count = self.properties.len() as c_int;
        let mut property_keys = Vec::new();
        let mut property_vals = Vec::new();
        for (key, val) in self.properties.iter() {
            property_keys.push(key.as_ptr());
            property_vals.push(val.as_ptr());
        }
        let property_keys_ptr = if property_keys.is_empty() {
            std::ptr::null_mut()
        } else {
            property_keys.as_mut_ptr()
        };
        let property_vals_ptr = if property_keys.is_empty() {
            std::ptr::null_mut()
        } else {
            property_vals.as_mut_ptr()
        };

        let mut handle: *mut c_void = std::ptr::null_mut();
        let mut id: c_uint = 0;

        let retval: c_int = unsafe {
            coreclr_initialize(
                exe_path_ptr,
                app_domain_name_ptr,
                property_count,
                property_keys_ptr,
                property_vals_ptr,
                &mut handle as *mut *mut c_void,
                &mut id as *mut c_uint,
            )
        };
        if retval != 0 {
            let err = RuntimeError { error_code: retval };
            return Err(RuntimeBuildError::RuntimeError(err));
        }

        let handle = RuntimeHandle { handle, id };
        let out = Runtime {
            coreclr_shutdown,
            coreclr_create_delegate,
            handle: Mutex::new(handle),
        };

        Ok(out)
    }
}

///
/// A wrapper type that enforces that a delegate returned from the coreclr runtime does not outlive
/// the runtime.
///
pub struct Delegate<'runtime, T> {
    pointer: *mut T,
    phantom: PhantomData<&'runtime T>,
}

impl<'runtime, T> Deref for Delegate<'runtime, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Additional reference level for a dereference on `deref` return value.
            &*(&self.pointer as *const *mut _ as *const T)
        }
    }
}

///
/// Internal struct for wrapping the runtime struct in a mutex
///
struct RuntimeHandle {
    handle: *mut c_void,
    id: c_uint,
}

///
/// A rusty wrapper around the CoreCLR library
///
pub struct Runtime<'lib> {
    coreclr_shutdown: Symbol<'lib, coreclr_shutdown_fn>,
    coreclr_create_delegate: Symbol<'lib, coreclr_create_delegate_fn>,
    handle: Mutex<RuntimeHandle>,
}

impl<'lib> Runtime<'lib> {
    ///
    /// Gets a builder for creating a new `Runtime`
    ///
    pub fn builder() -> RuntimeBuilder {
        RuntimeBuilder::new()
    }

    ///
    /// Attempts to get a delegate from the runtime.
    ///
    /// This maps to the `coreclr_create_delegate` function from the coreclr library.
    ///
    /// # Arguments
    /// - assembly_name: The name of the assembly to find the function in
    /// - type_name: The name of the type in the assembly to look for the function in
    /// - method_name: The name of the static method to use as a delegate
    ///
    /// # Returns
    /// Returns either a function pointer for the delegate or an error if the underlying call to
    /// `coreclr_create_delegate` failed
    ///
    /// # Unsafety
    /// This is super super unsafe to call because there's no way to verify the actual type of the
    /// function pointer returned. As such it is unsafe to call this as it casts an untyped pointer
    /// to an arbitrary type.
    ///
    /// This is an unfortunate side effect of working with FFI like this. Make sure you pass a
    /// reasonable type as F (i.e unsafe extern "C" fn()).
    ///
    pub unsafe fn create_delegate<F, A: Into<String>, B: Into<String>, C: Into<String>>(
        &self,
        assembly_name: A,
        type_name: B,
        method_name: C,
    ) -> Result<Delegate<F>, RuntimeError> {
        // Get a null terminated string
        let assembly_name = CString::new(assembly_name.into()).unwrap();
        let assembly_name_ptr = assembly_name.as_ptr();

        // Get a null terminated string
        let type_name = CString::new(type_name.into()).unwrap();
        let type_name_ptr = type_name.as_ptr();

        // Get a null terminated string
        let method_name = CString::new(method_name.into()).unwrap();
        let method_name_ptr = method_name.as_ptr();

        // Do the actual call to coreclr_create_delegate
        let mut delegate: *mut c_void = std::ptr::null_mut();
        let handle = self.handle.lock().unwrap();
        let retval: c_int = (self.coreclr_create_delegate)(
            handle.handle,
            handle.id,
            assembly_name_ptr,
            type_name_ptr,
            method_name_ptr,
            &mut delegate as *mut *mut c_void,
        );

        // Check if we've encountered an error
        if retval != 0 {
            return Err(RuntimeError { error_code: retval });
        }

        let delegate = delegate as *mut F;
        let delegate = Delegate {
            pointer: delegate,
            phantom: Default::default(),
        };

        Ok(delegate)
    }
}

impl<'lib> Drop for Runtime<'lib> {
    fn drop(&mut self) {
        unsafe {
            let handle = self.handle.lock().unwrap();
            let retval: c_int = (self.coreclr_shutdown)(handle.handle, handle.id);
            assert_eq!(retval, 0, "coreclr_shutdown returned error {}", retval);
        }
    }
}
