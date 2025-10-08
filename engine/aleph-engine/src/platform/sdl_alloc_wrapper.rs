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

use std::ffi::c_void;

use aleph_alloc::mallocator::Mallocator;

use crate::platform::Sdl2System;

/// A very, very unsafe wrapper function used to inject malloc wrappers into SDL. These are used to
/// track all the allocations SDL makes and should be disabled when memory instrumentation is not
/// used.
///
/// # Safety
///
/// Inherits all the pitfalls of 'SDL_SetMemoryFunctions'. It needs to be the very first SDL call
/// before any other interaction with the library.
pub unsafe fn set_memory_functions() {
    unsafe {
        sdl3::sys::stdinc::SDL_SetMemoryFunctions(
            Some(malloc_func),
            Some(calloc_func),
            Some(realloc_func),
            Some(free_func),
        );
    }
}

unsafe extern "C" fn malloc_func(size: usize) -> *mut c_void {
    unsafe { Mallocator::new(Sdl2System::default()).malloc(size) }
}

unsafe extern "C" fn calloc_func(nmemb: usize, size: usize) -> *mut c_void {
    unsafe { Mallocator::new(Sdl2System::default()).calloc(nmemb, size) }
}

unsafe extern "C" fn realloc_func(mem: *mut c_void, size: usize) -> *mut c_void {
    unsafe { Mallocator::new(Sdl2System::default()).realloc(mem, size) }
}

unsafe extern "C" fn free_func(mem: *mut c_void) {
    unsafe { Mallocator::new(Sdl2System::default()).free(mem) }
}
