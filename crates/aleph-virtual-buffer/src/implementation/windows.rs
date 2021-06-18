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

#![allow(unused)]

use aleph_windows_raw as windows;

use windows::Win32::SystemServices::{VirtualAlloc, VirtualFree};

use crate::VirtualBuffer;
use aleph_windows_raw::Win32::SystemServices::{
    VirtualAlloc_flAllocationType, VirtualFree_dwFreeType, PAGE_TYPE,
};

#[inline]
pub unsafe fn reserve_virtual_buffer(pages: usize) -> std::io::Result<VirtualBuffer> {
    let alloc_type = VirtualAlloc_flAllocationType::MEM_RESERVE;
    let page_type = PAGE_TYPE::PAGE_READWRITE;

    let result = VirtualAlloc(std::ptr::null_mut(), pages * 4096, alloc_type, page_type);

    if result.is_null() {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(VirtualBuffer {
            data: result as _,
            len: pages * 4096,
        })
    }
}

#[inline]
pub unsafe fn free_virtual_buffer(base: *mut u8, _pages: usize) -> std::io::Result<()> {
    let free_type = VirtualFree_dwFreeType::MEM_RELEASE;

    // The number of pages to free isn't needed on the windows implementation
    if VirtualFree(base as _, 0, free_type).as_bool() {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[inline]
pub unsafe fn commit_virtual_address_range(base: *mut u8, pages: usize) -> std::io::Result<()> {
    let alloc_type = VirtualAlloc_flAllocationType::MEM_COMMIT;
    let page_type = PAGE_TYPE::PAGE_READWRITE;

    let result = VirtualAlloc(base as _, pages * 4096, alloc_type, page_type);

    if result.is_null() {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[inline]
pub unsafe fn release_virtual_address_range(base: *mut u8, pages: usize) -> std::io::Result<()> {
    let free_type = VirtualFree_dwFreeType::MEM_DECOMMIT;

    if VirtualFree(base as _, pages * 4096, free_type).as_bool() {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub const fn requires_committing() -> bool {
    true
}
