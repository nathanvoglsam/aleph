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

// Copyright (c) 2022 The blink-alloc project developers
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

mod api;
mod arena;
mod blink;
mod drop_list;
mod global;
mod local;

mod sync;

mod cache;

#[cfg(test)]
mod tests;

pub use api::BlinkAllocator;
pub use blink::{Blink, Emplace, IteratorExt, SendBlink};
pub use cache::BlinkAllocCache;
pub use global::local::UnsafeGlobalBlinkAlloc;
pub use global::sync::GlobalBlinkAlloc;
pub use local::BlinkAlloc;
pub use sync::{LocalBlinkAlloc, SyncBlinkAlloc};

pub(crate) trait ResultExt<T> {
    fn safe_ok(self) -> T;
}

impl<T> ResultExt<T> for Result<T, std::convert::Infallible> {
    #[inline]
    fn safe_ok(self) -> T {
        match self {
            Ok(value) => value,
            Err(never) => match never {},
        }
    }
}

#[inline]
unsafe fn in_place<'a, T, I>(ptr: *mut T, init: I, f: impl FnOnce(I) -> T) -> &'a mut T {
    // Ask compiler very nicely to store return directly into memory.
    unsafe {
        std::ptr::write(ptr, f(init));
        &mut *ptr
    }
}

#[cold]
fn cold() {}

// #[cfg(debug_assertions)]
// #[track_caller]
// unsafe fn unreachable_unchecked() -> ! {
//     unreachable!()
// }

// #[cfg(not(debug_assertions))]
// unsafe fn unreachable_unchecked() -> ! {
//     unsafe { std::hint::unreachable_unchecked() }
// }
