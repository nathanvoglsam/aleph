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

use once_cell::sync::OnceCell;
use raw::windows::win32::direct3d12::D3D12_CACHED_PIPELINE_STATE;
use raw::windows::win32::direct3d12::D3D12_SHADER_BYTECODE;
use raw::windows::win32::system_services::GetProcAddress;
use raw::windows::win32::system_services::LoadLibraryW;
use utf16_lit::utf16_null;

pub struct DynamicLoadCell<T: Sized> {
    cell: OnceCell<T>,
    lib_name: &'static [u16],
    fn_name: &'static str,
}

impl<T: Sized> DynamicLoadCell<T> {
    pub const fn new(lib_name: &'static [u16], fn_name: &'static str) -> Self {
        Self {
            cell: OnceCell::new(),
            lib_name,
            fn_name,
        }
    }

    pub unsafe fn get(&self) -> Option<&T> {
        self.cell
            .get_or_try_init(|| {
                // Attempt to load the library
                let h_module = LoadLibraryW(self.lib_name.as_ptr());

                // If we fail to load the library emit an error
                if h_module == 0 {
                    return Err(());
                }

                // Attempt to find the function pointer we're after
                GetProcAddress(h_module, self.fn_name.as_ptr() as *const _)
                    .ok_or(())
                    .map(|v| std::mem::transmute_copy::<_, T>(&v))
            })
            .ok()
    }
}

///
/// Utility function that tries to set the main thread's name to "AlephMainThread", silently failing
/// if it doesn't
///
pub fn name_thread_as_main_thread() {
    use raw::windows::win32::system_services::GetCurrentThread;
    use raw::windows::win32::system_services::SetThreadDescription;

    unsafe {
        let handle = GetCurrentThread();
        let _ = SetThreadDescription(handle, utf16_null!("AlephMainThread").as_ptr());
    }
}

pub fn blob_to_shader(blob: &[u8]) -> D3D12_SHADER_BYTECODE {
    D3D12_SHADER_BYTECODE {
        p_shader_bytecode: blob.as_ptr() as _,
        bytecode_length: blob.len() as _,
    }
}

pub fn optional_blob_to_shader(blob: Option<&[u8]>) -> D3D12_SHADER_BYTECODE {
    match blob {
        None => D3D12_SHADER_BYTECODE {
            p_shader_bytecode: std::ptr::null_mut(),
            bytecode_length: 0,
        },
        Some(blob) => blob_to_shader(blob),
    }
}

pub fn blob_to_cached_pso(blob: &[u8]) -> D3D12_CACHED_PIPELINE_STATE {
    D3D12_CACHED_PIPELINE_STATE {
        p_cached_blob: blob.as_ptr() as _,
        cached_blob_size_in_bytes: blob.len() as _,
    }
}

pub fn optional_blob_to_cached_pso(blob: Option<&[u8]>) -> D3D12_CACHED_PIPELINE_STATE {
    match blob {
        None => D3D12_CACHED_PIPELINE_STATE {
            p_cached_blob: std::ptr::null_mut(),
            cached_blob_size_in_bytes: 0,
        },
        Some(blob) => blob_to_cached_pso(blob),
    }
}
