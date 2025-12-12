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

use std::alloc::System;

use allocator_api2::alloc::Allocator;

/// A trait implemented by [`Allocator`] types which are simply zero sized handles to some global
/// allocator.
///
/// # Safety
///
/// It is up to the user to ensure this is only implemented on types where the allocator state is
/// global, and that any particular instance of the impl type is interchangable. An example is the
/// [`System`] allocator, which is a zero sized struct that simply defers to the system's global
/// allocator. Any [`System`] instance is the same, so this trait is correct to implement on it.
pub unsafe trait AllocatorGlobalHandle: Allocator {
    /// Constructs a new handle
    fn make_handle() -> Self;
}

unsafe impl AllocatorGlobalHandle for System {
    fn make_handle() -> Self {
        Self
    }
}
