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

use aleph_object_system::ArcObject;
use thiserror::Error;

#[derive(Clone)]
pub struct FenceHandle {
    inner: ArcObject,
}

impl FenceHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a semaphore objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`FenceHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`FenceHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FenceWaitResult {
    /// The wait condition was met and the call has returned successfully.
    Complete,

    /// The timeout time was reached before the condition was met.
    Timeout,
}

impl std::fmt::Display for FenceWaitResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenceWaitResult::Complete => f.write_str("Complete"),
            FenceWaitResult::Timeout => f.write_str("Timeout"),
        }
    }
}

/// Set of errors that can occur when waiting on a fence with [`crate::IDevice::wait_fences`]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FenceWaitError {
    #[error("The platform API device was lost.")]
    DeviceLost,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}

/// Set of errors that can occur when polling on a fence with
/// [`crate::IDevice::get_fence_signaled_value`]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FencePollError {
    #[error("The platform API device was lost.")]
    DeviceLost,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}

/// Set of errors that can occur when signaling on a fence with [`crate::IDevice::signal_fence`]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FenceSignalError {
    #[error("The platform API device was lost.")]
    DeviceLost,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
