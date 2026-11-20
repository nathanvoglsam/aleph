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

use std::alloc::Allocator;

pub mod drain;
pub mod into_iter;

// TODO: Remove this function once `Box::from` is stable with a custom allocator.
/// Copies `s` into a new `Box<str, A>` allocated with `alloc`.
pub fn box_str_in<A: Allocator>(s: &str, alloc: A) -> Box<str, A> {
    let mut bytes = Vec::with_capacity_in(s.len(), alloc);
    bytes.extend_from_slice(s.as_bytes());
    let boxed: Box<[u8], A> = bytes.into_boxed_slice();
    let (ptr, alloc) = Box::into_raw_with_allocator(boxed);
    // Safety: the bytes were copied from a `&str`, so they are valid UTF-8.
    unsafe { Box::from_raw_in(ptr as *mut str, alloc) }
}
