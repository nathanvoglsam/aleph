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

use std::os::raw;

//
// On Windows-x86 the coreclr library uses stdcall
//
#[cfg(all(target_os = "windows", target_arch = "x86"))]
macro_rules! core_clr_func {
    ($($arg:tt)*) => {
        unsafe extern "stdcall" fn ($($arg)*) -> std::os::raw::c_int;
    };
}

//
// On every other platform (windows x86-64 included) the C calling convention is used
//
// thanks microsoft
//
#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
macro_rules! core_clr_func {
    ($($arg:tt)*) => {
        unsafe extern "C" fn ($($arg)*) -> std::os::raw::c_int
    };
}

///
/// Initialize the CoreCLR. Creates and starts CoreCLR host and creates an app domain
///
/// # Parameters
/// - exePath: Absolute path of the executable that invoked the ExecuteAssembly (the native host application)
/// - appDomainFriendlyName: Friendly name of the app domain that will be created to execute the assembly
/// - propertyCount: Number of properties (elements of the following two arguments)
/// - propertyKeys: Keys of properties of the app domain
/// - propertyValues: Values of properties of the app domain
/// - hostHandle: Output parameter, handle of the created host
/// - domainId: Output parameter, id of the created app domain
///
/// # Returns
///  HRESULT indicating status of the operation. S_OK if the assembly was successfully executed
///
#[allow(non_camel_case_types)]
pub type coreclr_initialize_fn = core_clr_func!(
    *const raw::c_char,      // exePath
    *const raw::c_char,      // appDomainFriendlyName
    raw::c_int,              // propertyCount
    *mut *const raw::c_char, // propertyKeys
    *mut *const raw::c_char, // propertyValues
    *mut *mut raw::c_void,   // hostHandle
    *mut raw::c_uint,        // domainId
);

///
/// The symbol name of the coreclr_initialize function in the coreclr library
///
pub const CORECLR_INITIALIZE_FN_SYMBOL: &str = "coreclr_initialize\0";

///
/// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host.
///
/// # Parameters
/// - hostHandle: Handle of the host
/// - domainId: Id of the domain
///
/// # Returns
/// HRESULT indicating status of the operation. S_OK if the assembly was successfully executed
///
#[allow(non_camel_case_types)]
pub type coreclr_shutdown_fn = core_clr_func!(
    *mut raw::c_void, // hostHandle
    raw::c_uint,      // domainId
);

///
/// The symbol name of the coreclr_shutdown function in the coreclr library
///
pub const CORECLR_SHUTDOWN_FN_SYMBOL: &str = "coreclr_shutdown\0";

///
/// Shutdown CoreCLR. It unloads the app domain and stops the CoreCLR host.
///
/// # Parameters
/// - hostHandle: Handle of the host
/// - domainId: Id of the domain
/// - latchedExitCode: Latched exit code after domain unloaded
///
/// # Returns
/// HRESULT indicating status of the operation. S_OK if the assembly was successfully executed
///
#[allow(non_camel_case_types)]
pub type coreclr_shutdown_2_fn = core_clr_func!(
    *mut raw::c_void, // hostHandle
    raw::c_uint,      // domainId,
    *mut raw::c_char, // latchedExitCode
);

///
/// The symbol name of the coreclr_shutdown_2 function in the coreclr library
///
pub const CORECLR_SHUTDOWN_2_FN_SYMBOL: &str = "coreclr_shutdown_2\0";

///
/// Create a native callable function pointer for a managed method.
///
/// # Parameters
/// - hostHandle: Handle of the host
/// - domainId: Id of the domain
/// - entryPointAssemblyName: Name of the assembly which holds the custom entry point
/// - entryPointTypeName: Name of the type which holds the custom entry point
/// - entryPointMethodName: Name of the method which is the custom entry point
/// - delegate: Output parameter, the function stores a native callable function pointer to the delegate at the specified address
///
/// # Returns
/// HRESULT indicating status of the operation. S_OK if the assembly was successfully executed
///
#[allow(non_camel_case_types)]
pub type coreclr_create_delegate_fn = core_clr_func!(
    *mut raw::c_void,      // hostHandle
    raw::c_uint,           // domainId
    *const raw::c_char,    // entryPointAssemblyName
    *const raw::c_char,    // entryPointTypeName
    *const raw::c_char,    // entryPointMethodName
    *mut *mut raw::c_void, // delegate
);

///
/// The symbol name of the coreclr_create_delegate function in the coreclr library
///
pub const CORECLR_CREATE_DELEGATE_FN_SYMBOL: &str = "coreclr_create_delegate\0";

///
/// Execute a managed assembly with given arguments
///
/// # Parameters
/// - hostHandle: Handle of the host
/// - domainId: Id of the domain
/// - argc: Number of arguments passed to the executed assembly
/// - argv: Array of arguments passed to the executed assembly
/// - managedAssemblyPath: Path of the managed assembly to execute (or NULL if using a custom entrypoint).
/// - exitCode: Exit code returned by the executed assembly
///
/// # Returns
/// HRESULT indicating status of the operation. S_OK if the assembly was successfully executed
///
#[allow(non_camel_case_types)]
pub type coreclr_execute_assembly_fn = core_clr_func!(
    *mut raw::c_void,        // hostHandle
    raw::c_uint,             // domainId
    raw::c_int,              // argc
    *mut *const raw::c_char, // argv
    *const raw::c_char,      // managedAssemblyPath
    *mut raw::c_uint,        // exitCode
);

///
/// The symbol name of the coreclr_execute_assembly function in the coreclr library
///
pub const CORECLR_EXECUTE_ASSEMBLY_FN_SYMBOL: &str = "coreclr_execute_assembly\0";
