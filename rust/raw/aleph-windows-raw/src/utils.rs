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
use crate::win32::direct3d12::D3D12_CACHED_PIPELINE_STATE;
use crate::win32::direct3d12::D3D12_SHADER_BYTECODE;
//use windows_raw::win32::system_services::GetProcAddress;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::os::raw::c_char;
use utf16_lit::utf16_null;
use crate::win32::system_services::LoadLibraryW;
use crate::win32::system_services::PWSTR;

// TODO: Fixme when windows-rs generates this correctly
mod temp {
    use std::ffi::c_void;
    use std::ptr::NonNull;

    #[link(name = "KERNEL32")]
    extern "system" {
        pub fn GetProcAddress(
            h_module: isize,
            lp_proc_name: *const i8,
        ) -> ::std::option::Option<NonNull<c_void>>;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CStrFFI<'a> {
    ptr: *const c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> From<&'a CStr> for CStrFFI<'a> {
    fn from(v: &'a CStr) -> Self {
        Self {
            ptr: v.as_ptr(),
            phantom: Default::default(),
        }
    }
}

impl<'a> CStrFFI<'a> {
    pub unsafe fn as_cstr(&self) -> &CStr {
        CStr::from_ptr(self.ptr)
    }
}

impl<'a> Hash for CStrFFI<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { self.as_cstr().hash(state) }
    }
}

impl<'a> Debug for CStrFFI<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { self.as_cstr().fmt(f) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Bool(u32);

impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        let num = *other as u32;
        self.0.eq(&num)
    }
}

impl Into<u32> for Bool {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<u32> for Bool {
    fn from(v: u32) -> Self {
        if v != 0 {
            Bool(1)
        } else {
            Bool(0)
        }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        if self.0 != 0 {
            true
        } else {
            false
        }
    }
}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        Bool(v as u32)
    }
}

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
                let h_module = LoadLibraryW(PWSTR(self.lib_name.as_ptr() as *mut u16));

                // If we fail to load the library emit an error
                if h_module == 0 {
                    return Err(());
                }

                // Attempt to find the function pointer we're after
                temp::GetProcAddress(h_module, self.fn_name.as_ptr() as *const _)
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
    use crate::win32::system_services::GetCurrentThread;
    use crate::win32::system_services::SetThreadDescription;

    unsafe {
        let handle = GetCurrentThread();
        let _ = SetThreadDescription(
            handle,
            PWSTR(utf16_null!("AlephMainThread").as_ptr() as *mut u16),
        );
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

pub fn optional_slice_to_num_ptr_pair<T>(slice: Option<&[T]>) -> (u32, *const T) {
    if let Some(slice) = slice {
        let num = slice.len() as u32;
        let ptr = slice.as_ptr();
        (num, ptr)
    } else {
        let num = 0;
        let ptr = std::ptr::null();
        (num, ptr)
    }
}

pub fn optional_ref_to_ptr<T>(option: Option<&T>) -> *const T {
    option.map(|v| v as *const T).unwrap_or(std::ptr::null())
}

#[macro_export]
macro_rules! flags_bitwise_impl {
    ($t:ident) => {
        impl std::ops::BitOr for $t {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $t {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }

        impl std::ops::BitAnd for $t {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $t {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }

        impl std::ops::BitXor for $t {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $t {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0
            }
        }
    };
}

#[macro_export]
macro_rules! deref_impl {
    ($t:ident, $d:ident) => {
        impl $t {
            pub fn as_raw(&self) -> & $d {
                &self.0
            }

            pub fn as_raw_mut(&mut self) -> &mut $d {
                &mut self.0
            }
        }
    };
}
