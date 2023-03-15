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

use core::mem::ManuallyDrop;

/// Takes a slice of `[ManuallyDrop<T>]` and returns a slice that views the same array as `[T]`.
///
/// # Safety
///
/// This has the same safety requirements as [ManuallyDrop]. The caller must ensure that all
/// [ManuallyDrop] instances are still live and have not been dropped.
///
/// [ManuallyDrop::drop] is unsafe, so this function can be safe as this function can not introduce
/// unsoundness without other unsafe code.
pub fn view_list_as_inner<'a, 'b, T: Sized + 'b>(v: &'a [ManuallyDrop<T>]) -> &'a [T] {
    let data = v.as_ptr() as *const T;
    let len = v.len();
    unsafe { core::slice::from_raw_parts(data, len) }
}

/// Takes a slice of `[ManuallyDrop<T>]` and returns a slice that views the same array as `[T]`.
///
/// # Safety
///
/// This has the same safety requirements as [ManuallyDrop]. The caller must ensure that all
/// [ManuallyDrop] instances are still live and have not been dropped.
///
/// [ManuallyDrop::drop] is unsafe, so this function can be safe as this function can not introduce
/// unsoundness without other unsafe code.
pub fn view_list_as_inner_mut<'a, 'b, T: Sized + 'b>(v: &'a mut [ManuallyDrop<T>]) -> &'a mut [T] {
    let data = v.as_mut_ptr() as *mut T;
    let len = v.len();
    unsafe { core::slice::from_raw_parts_mut(data, len) }
}
