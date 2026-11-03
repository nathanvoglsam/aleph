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

#![cfg(windows)]

use std::env;
use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use aleph_windows::Win32::Foundation::{E_FAIL, FARPROC, HMODULE};
use aleph_windows::Win32::System::LibraryLoader::{
    GetProcAddress, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR, LoadLibraryExW, LoadLibraryW,
};
use aleph_windows::core::{PCSTR, PCWSTR};

#[repr(C)]
pub struct ID3D12CommandList {
    _private: (),
}

#[repr(C)]
pub struct ID3D12CommandQueue {
    _private: (),
}

/// The shim DLL built from `cmake/shim.cpp`. It is first loaded by name from the OS DLL search
/// path, then from each fallback directory. `LoadLibraryW` implies the `.dll` extension for the
/// bare name, so the file name is only needed for fallback paths.
const SHIM_LIBRARY_NAME: &str = "aleph_pix_shim";
const SHIM_LIBRARY_FILE_NAME: &str = "aleph_pix_shim.dll";

/// Holds a resolved function pointer for every export of the PIX shim DLL.
pub struct PixShimFunctions {
    pub begin_event_on_thread: PIXBeginEventOnThreadFn,
    pub set_marker_on_thread: PIXSetMarkerOnThreadFn,
    pub begin_event_data_on_thread: PIXBeginEventDataOnThreadFn,
    pub set_marker_data_on_thread: PIXSetMarkerDataOnThreadFn,
    pub begin_event_on_command_list: PIXBeginEventOnCommandListFn,
    pub begin_event_on_command_queue: PIXBeginEventOnCommandQueueFn,
    pub begin_event_data_on_command_list: PIXBeginEventDataOnCommandListFn,
    pub begin_event_data_on_command_queue: PIXBeginEventDataOnCommandQueueFn,
    pub set_marker_on_command_list: PIXSetMarkerOnCommandListFn,
    pub set_marker_on_command_queue: PIXSetMarkerOnCommandQueueFn,
    pub set_marker_data_on_command_list: PIXSetMarkerDataOnCommandListFn,
    pub set_marker_data_on_command_queue: PIXSetMarkerDataOnCommandQueueFn,
    pub end_event_on_thread: PIXEndEventOnThreadFn,
    pub end_event_on_command_list: PIXEndEventOnCommandListFn,
    pub end_event_on_command_queue: PIXEndEventOnCommandQueueFn,
}

impl PixShimFunctions {
    /// Loads the shim DLL and resolves every exported function.
    ///
    /// The DLL is first loaded by name from the OS DLL search path. If that fails, we look for the
    /// well known location relative to the 'ALEPH_BUILD_ROOT' env var. This fallback is for
    /// development builds in a cargo workspace to work.
    ///
    /// # Safety
    ///
    /// Loading a DLL runs its initialisation code. Each symbol is transmuted to the function
    /// pointer type declared in this crate, so the DLL's exports must match those signatures.
    #[rustfmt::skip]
    pub unsafe fn load() -> aleph_windows::core::Result<Self> {
        unsafe {
            let module = load_shim_library()?;
            Ok(Self {
                begin_event_on_thread: load_symbol(module, PIX_BEGIN_EVENT_ON_THREAD_NAME)?,
                set_marker_on_thread: load_symbol(module, PIX_SET_MARKER_ON_THREAD_NAME)?,
                begin_event_data_on_thread: load_symbol(module, PIX_BEGIN_EVENT_DATA_ON_THREAD_NAME)?,
                set_marker_data_on_thread: load_symbol(module, PIX_SET_MARKER_DATA_ON_THREAD_NAME)?,
                begin_event_on_command_list: load_symbol(module, PIX_BEGIN_EVENT_ON_COMMAND_LIST_NAME)?,
                begin_event_on_command_queue: load_symbol(module, PIX_BEGIN_EVENT_ON_COMMAND_QUEUE_NAME)?,
                begin_event_data_on_command_list: load_symbol(module, PIX_BEGIN_EVENT_DATA_ON_COMMAND_LIST_NAME)?,
                begin_event_data_on_command_queue: load_symbol(module, PIX_BEGIN_EVENT_DATA_ON_COMMAND_QUEUE_NAME)?,
                set_marker_on_command_list: load_symbol(module, PIX_SET_MARKER_ON_COMMAND_LIST_NAME)?,
                set_marker_on_command_queue: load_symbol(module, PIX_SET_MARKER_ON_COMMAND_QUEUE_NAME)?,
                set_marker_data_on_command_list: load_symbol(module, PIX_SET_MARKER_DATA_ON_COMMAND_LIST_NAME)?,
                set_marker_data_on_command_queue: load_symbol(module, PIX_SET_MARKER_DATA_ON_COMMAND_QUEUE_NAME)?,
                end_event_on_thread: load_symbol(module, PIX_END_EVENT_ON_THREAD_NAME)?,
                end_event_on_command_list: load_symbol(module, PIX_END_EVENT_ON_COMMAND_LIST_NAME)?,
                end_event_on_command_queue: load_symbol(module, PIX_END_EVENT_ON_COMMAND_QUEUE_NAME)?,
            })
        }
    }
}

/// Converts `s` to a null-terminated UTF-16 string for the wide Win32 APIs.
fn to_wide_null(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(std::iter::once(0)).collect()
}

/// # Safety
///
/// Loading a DLL runs its initialisation code.
unsafe fn load_shim_library() -> aleph_windows::core::Result<HMODULE> {
    unsafe {
        let name = to_wide_null(OsStr::new(SHIM_LIBRARY_NAME));
        let mut module = LoadLibraryW(PCWSTR(name.as_ptr()));

        if module.is_ok() {
            return module;
        }

        if let Some(build_root) = env::var_os("ALEPH_BUILD_ROOT") {
            let arch = if cfg!(target_arch = "x86_64") {
                "x64"
            } else if cfg!(target_arch = "aarch64") {
                "ARM64"
            } else {
                panic!("Unsupported architecture")
            };

            let mut path = PathBuf::from(build_root);
            path.extend(["pix", arch, SHIM_LIBRARY_FILE_NAME]);

            let path = to_wide_null(path.as_os_str());
            module = LoadLibraryExW(
                PCWSTR(path.as_ptr()),
                None,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
            );
        }

        module
    }
}

/// # Safety
///
/// `T` must be a function pointer type whose signature matches the exported symbol `name`.
unsafe fn load_symbol<T: Copy>(module: HMODULE, name: &CStr) -> aleph_windows::core::Result<T> {
    unsafe {
        const { assert!(size_of::<T>() == size_of::<FARPROC>()) };
        let proc = GetProcAddress(module, PCSTR(name.as_ptr().cast()))
            .ok_or_else(|| aleph_windows::core::Error::from(E_FAIL))?;
        Ok(std::mem::transmute_copy(&proc))
    }
}

#[rustfmt::skip]
mod hack {
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use crate::{ID3D12CommandList, ID3D12CommandQueue};

    pub type PIXBeginEventOnThreadFn = unsafe extern "C" fn(colour: u64, string: *const c_char);
    pub const PIX_BEGIN_EVENT_ON_THREAD_NAME: &CStr = c"SHIM_PIXBeginEventOnThread";

    pub type PIXBeginEventOnCommandListFn = unsafe extern "C" fn(context: *mut ID3D12CommandList, colour: u64, string: *const c_char);
    pub const PIX_BEGIN_EVENT_ON_COMMAND_LIST_NAME: &CStr = c"SHIM_PIXBeginEventOnCommandList";

    pub type PIXBeginEventOnCommandQueueFn = unsafe extern "C" fn(context: *mut ID3D12CommandQueue, colour: u64, string: *const c_char);
    pub const PIX_BEGIN_EVENT_ON_COMMAND_QUEUE_NAME: &CStr = c"SHIM_PIXBeginEventOnCommandQueue";

    pub type PIXBeginEventDataOnThreadFn = unsafe extern "C" fn(colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_BEGIN_EVENT_DATA_ON_THREAD_NAME: &CStr = c"SHIM_PIXBeginEventDataOnThread";

    pub type PIXBeginEventDataOnCommandListFn = unsafe extern "C" fn(context: *mut ID3D12CommandList, colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_BEGIN_EVENT_DATA_ON_COMMAND_LIST_NAME: &CStr = c"SHIM_PIXBeginEventDataOnCommandList";

    pub type PIXBeginEventDataOnCommandQueueFn = unsafe extern "C" fn(context: *mut ID3D12CommandQueue, colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_BEGIN_EVENT_DATA_ON_COMMAND_QUEUE_NAME: &CStr = c"SHIM_PIXBeginEventDataOnCommandQueue";

    pub type PIXSetMarkerOnThreadFn = unsafe extern "C" fn(colour: u64, string: *const c_char);
    pub const PIX_SET_MARKER_ON_THREAD_NAME: &CStr = c"SHIM_PIXSetMarkerOnThread";

    pub type PIXSetMarkerOnCommandListFn = unsafe extern "C" fn(context: *mut ID3D12CommandList, colour: u64, string: *const c_char);
    pub const PIX_SET_MARKER_ON_COMMAND_LIST_NAME: &CStr = c"SHIM_PIXSetMarkerOnCommandList";

    pub type PIXSetMarkerOnCommandQueueFn = unsafe extern "C" fn(context: *mut ID3D12CommandQueue, colour: u64, string: *const c_char);
    pub const PIX_SET_MARKER_ON_COMMAND_QUEUE_NAME: &CStr = c"SHIM_PIXSetMarkerOnCommandQueue";

    pub type PIXSetMarkerDataOnThreadFn = unsafe extern "C" fn(colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_SET_MARKER_DATA_ON_THREAD_NAME: &CStr = c"SHIM_PIXSetMarkerDataOnThread";

    pub type PIXSetMarkerDataOnCommandListFn = unsafe extern "C" fn(context: *mut ID3D12CommandList, colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_SET_MARKER_DATA_ON_COMMAND_LIST_NAME: &CStr = c"SHIM_PIXSetMarkerDataOnCommandList";

    pub type PIXSetMarkerDataOnCommandQueueFn = unsafe extern "C" fn(context: *mut ID3D12CommandQueue, colour: u64, string: *const c_char, data: *const c_char);
    pub const PIX_SET_MARKER_DATA_ON_COMMAND_QUEUE_NAME: &CStr = c"SHIM_PIXSetMarkerDataOnCommandQueue";

    pub type PIXEndEventOnThreadFn = unsafe extern "C" fn();
    pub const PIX_END_EVENT_ON_THREAD_NAME: &CStr = c"SHIM_PIXEndEventOnThread";

    pub type PIXEndEventOnCommandListFn = unsafe extern "C" fn(context: *mut ID3D12CommandList);
    pub const PIX_END_EVENT_ON_COMMAND_LIST_NAME: &CStr = c"SHIM_PIXEndEventOnCommandList";

    pub type PIXEndEventOnCommandQueueFn = unsafe extern "C" fn(context: *mut ID3D12CommandQueue);
    pub const PIX_END_EVENT_ON_COMMAND_QUEUE_NAME: &CStr = c"SHIM_PIXEndEventOnCommandQueue";
}

pub use hack::*;
