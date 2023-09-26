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

use crate::VirtualBuffer;

#[inline]
pub unsafe fn reserve_virtual_buffer(pages: usize) -> std::io::Result<VirtualBuffer> {
    let result: *mut libc::c_void = libc::mmap(
        std::ptr::null_mut(),
        pages * page_size(),
        libc::PROT_NONE,
        libc::MAP_SHARED | libc::MAP_ANON,
        -1,
        0,
    );

    if result == libc::MAP_FAILED {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(VirtualBuffer {
            data: result as _,
            len: pages * page_size(),
        })
    }
}

#[inline]
pub unsafe fn free_virtual_buffer(base: *mut u8, pages: usize) -> std::io::Result<()> {
    if libc::munmap(base as _, pages * page_size()) != 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[inline]
pub unsafe fn commit_virtual_address_range(base: *mut u8, pages: usize) -> std::io::Result<()> {
    let result = libc::mprotect(
        base as _,
        pages * page_size(),
        libc::PROT_READ | libc::PROT_WRITE,
    );

    if result != 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[inline]
pub unsafe fn release_virtual_address_range(base: *mut u8, pages: usize) -> std::io::Result<()> {
    let result = libc::mprotect(base as _, pages * page_size(), libc::PROT_NONE);

    if result != 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub const fn page_size() -> usize {
    if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            16384
        } else {
            4096
        }
    } else {
        4096
    }
}
