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

/// Utility function that is made available to _all_ platforms for wrapping execution of the given
/// closure in an autorelease pool.
///
/// Useful for injecting into places where one may be needed in platform-agnostic code.
///
/// # Caveat
///
/// The pool object that the 'objc' crate uses can't be exposed because the type isn't opaque. Only
/// objc/swift code can really make use of this pool. The expected use for this is only for catching
/// ffi autorelease calls.
#[inline]
pub fn autoreleasepool<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    internal_autoreleasepool(f)
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[inline]
fn internal_autoreleasepool<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    objc2::rc::autoreleasepool(|_| f())
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[inline]
fn internal_autoreleasepool<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}
