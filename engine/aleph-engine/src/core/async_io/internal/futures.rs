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

use std::pin::Pin;
use std::task::{Context, Poll};

/// Future wrapper that turns panics during polling into aborts.
#[repr(transparent)]
pub struct AbortUnwindFuture<F>(F);

impl<F> AbortUnwindFuture<F> {
    pub fn new(future: F) -> Self {
        Self(future)
    }
}

impl<F> From<F> for AbortUnwindFuture<F> {
    fn from(future: F) -> Self {
        Self::new(future)
    }
}

impl<F: Future> std::future::Future for AbortUnwindFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: The inner future is structurally pinned and is not moved after this point.
        unsafe {
            let this = self.get_unchecked_mut();
            abort_unwind(|| Pin::new_unchecked(&mut this.0).poll(cx))
        }
    }
}

/// Future wrapper that profiles each poll of the inner future.
pub struct ProfiledFuture<F> {
    future: F,

    #[allow(dead_code)]
    name: &'static str,
}

impl<F> ProfiledFuture<F> {
    pub fn new(future: F, name: &'static str) -> Self {
        Self { future, name }
    }
}

impl<F: Future> std::future::Future for ProfiledFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: The inner future is structurally pinned and is not moved after this point.
        unsafe {
            let this = self.get_unchecked_mut();
            aleph_profile::scope_named!("ProfiledFuture::poll", this.name);
            Pin::new_unchecked(&mut this.future).poll(cx)
        }
    }
}

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
