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

use crate::core::PCSTR;
use crate::core::PCWSTR;
use crate::Win32::Foundation::{HANDLE, HINSTANCE};
use crate::Win32::Graphics::Direct3D12::{D3D12_CACHED_PIPELINE_STATE, D3D12_SHADER_BYTECODE};
use crate::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use once_cell::sync::OnceCell;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::os::raw::c_char;
use windows::Win32::Foundation::GetLastError;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CStrFFI<'a> {
    ptr: *const c_char,
    phantom: PhantomData<&'a ()>,
}

impl<'a> From<&'a CStr> for CStrFFI<'a> {
    #[inline]
    fn from(v: &'a CStr) -> Self {
        Self {
            ptr: v.as_ptr(),
            phantom: Default::default(),
        }
    }
}

impl<'a> CStrFFI<'a> {
    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl<'a> Hash for CStrFFI<'a> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cstr().hash(state)
    }
}

impl<'a> Debug for CStrFFI<'a> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_cstr().fmt(f)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Bool(u32);

impl Bool {
    pub const TRUE: Self = Self(1);
    pub const FALSE: Self = Self(0);
}

impl PartialEq<bool> for Bool {
    fn eq(&self, other: &bool) -> bool {
        let num = *other as u32;
        self.0.eq(&num)
    }
}

impl From<Bool> for u32 {
    fn from(v: Bool) -> Self {
        v.0
    }
}

impl From<u32> for Bool {
    #[inline]
    fn from(v: u32) -> Self {
        if v != 0 {
            Bool(1)
        } else {
            Bool(0)
        }
    }
}

impl From<Bool> for bool {
    #[inline]
    fn from(v: Bool) -> bool {
        v.0 != 0
    }
}

impl From<bool> for Bool {
    #[inline]
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

    ///
    /// # Safety
    ///
    /// This function does not check, nor have any way to check, the type of what is pointed to by
    /// the result of the GetProcAddress call. This function will blindly assume the pointer points
    /// to something with type `T`. It is the caller's responsibility to decide whether the type
    /// of `T` is correct.
    ///
    #[inline]
    pub unsafe fn get(&self) -> Option<&T> {
        self.cell
            .get_or_try_init(|| {
                // Attempt to load the library
                let h_module: HINSTANCE =
                    LoadLibraryW(PCWSTR(self.lib_name.as_ptr())).map_err(|_| ())?;

                if h_module.0 == 0 {
                    return Err(());
                }

                // Attempt to find the function pointer we're after
                GetProcAddress(h_module, PCSTR(self.fn_name.as_ptr()))
                    .ok_or(())
                    .map(|v| std::mem::transmute_copy::<_, T>(&v))
            })
            .ok()
    }
}

///
/// Utility for setting the name of the current thread.
///
/// Useful for getting descriptive names in debuggers and profilers.
///
/// # Safety
///
/// `name` must be a null terminated wchar string. If the null terminator is missing the behavior is
/// undefined.
///
#[inline]
pub unsafe fn name_current_thread(name: &[u16]) -> crate::windows::core::Result<()> {
    use crate::Win32::System::Threading::GetCurrentThread;
    use crate::Win32::System::Threading::SetThreadDescription;

    let handle: HANDLE = GetCurrentThread();
    if handle.is_invalid() {
        GetLastError().ok()?
    }
    let _ = SetThreadDescription(handle, PCWSTR(name.as_ptr()));

    Ok(())
}

#[inline]
pub fn blob_to_shader(blob: &[u8]) -> D3D12_SHADER_BYTECODE {
    D3D12_SHADER_BYTECODE {
        pShaderBytecode: blob.as_ptr() as _,
        BytecodeLength: blob.len() as _,
    }
}

#[inline]
pub fn optional_blob_to_shader(blob: Option<&[u8]>) -> D3D12_SHADER_BYTECODE {
    match blob {
        None => D3D12_SHADER_BYTECODE {
            pShaderBytecode: std::ptr::null_mut(),
            BytecodeLength: 0,
        },
        Some(blob) => blob_to_shader(blob),
    }
}

#[inline]
pub fn blob_to_cached_pso(blob: &[u8]) -> D3D12_CACHED_PIPELINE_STATE {
    D3D12_CACHED_PIPELINE_STATE {
        pCachedBlob: blob.as_ptr() as _,
        CachedBlobSizeInBytes: blob.len() as _,
    }
}

#[inline]
pub fn optional_blob_to_cached_pso(blob: Option<&[u8]>) -> D3D12_CACHED_PIPELINE_STATE {
    match blob {
        None => D3D12_CACHED_PIPELINE_STATE {
            pCachedBlob: std::ptr::null_mut(),
            CachedBlobSizeInBytes: 0,
        },
        Some(blob) => blob_to_cached_pso(blob),
    }
}

#[inline]
pub fn optional_slice_to_num_ptr_pair<T>(slice: Option<&[T]>) -> (u32, *const T) {
    if let Some(slice) = slice {
        if slice.is_empty() {
            let num = 0;
            let ptr = std::ptr::null();
            (num, ptr)
        } else {
            let num = slice.len() as u32;
            let ptr = slice.as_ptr();
            (num, ptr)
        }
    } else {
        let num = 0;
        let ptr = std::ptr::null();
        (num, ptr)
    }
}

#[inline]
pub fn optional_ref_to_ptr<T>(option: Option<&T>) -> *const T {
    option.map(|v| v as *const T).unwrap_or(std::ptr::null())
}

#[macro_export]
macro_rules! flags_bitwise_impl {
    ($t:ident) => {
        impl $t {
            #[inline]
            pub fn is_single_flag(&self) -> bool {
                self.0.count_ones() == 1
            }

            #[inline]
            pub fn intersects(&self, other: &Self) -> bool {
                (*self | *other).0 != 0
            }

            #[inline]
            pub fn is_subset_of(&self, other: &Self) -> bool {
                (*self | *other) == *self
            }

            #[inline]
            pub fn is_disjoint_from(&self, other: &Self) -> bool {
                !self.intersects(other)
            }
        }

        impl std::ops::BitOr for $t {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $t {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }

        impl std::ops::BitAnd for $t {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $t {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }

        impl std::ops::BitXor for $t {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $t {
            #[inline]
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
            #[inline]
            pub fn as_raw(&self) -> &$d {
                &self.0
            }

            #[inline]
            pub fn as_raw_mut(&mut self) -> &mut $d {
                &mut self.0
            }
        }
    };
}
