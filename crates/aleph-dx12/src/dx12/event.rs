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

use std::num::NonZeroIsize;
use windows_raw::Win32::SystemServices::{
    CreateEventW, WaitForMultipleObjects, WaitForSingleObject, BOOL, HANDLE, PWSTR,
    WAIT_RETURN_CAUSE,
};
use windows_raw::Win32::WindowsProgramming::CloseHandle;

/// Wrapper around the return value of `WaitForSingleObject`
#[repr(u32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum SingleWaitResponse {
    /// The state of the specified object is signaled.
    Signaled = 0,

    /// The specified object is a mutex object that was not released by the thread that owned the
    /// mutex object before the owning thread terminated. Ownership of the mutex object is granted
    /// to the calling thread and the mutex state is set to non-signaled.
    ///
    /// If the mutex was protecting persistent state information, you should check it for
    /// consistency.
    Abandoned = 0x00000080,

    /// The time-out interval elapsed, and the object's state is non-signaled.
    Timeout = 0x00000102,
}

impl SingleWaitResponse {
    #[inline]
    fn from_u32(v: u32) -> Result<Self, ()> {
        match v {
            0x00000080 => Ok(SingleWaitResponse::Abandoned),
            0x00000000 => Ok(SingleWaitResponse::Signaled),
            0x00000102 => Ok(SingleWaitResponse::Timeout),
            _ => Err(()),
        }
    }

    #[inline]
    fn from_cause(v: WAIT_RETURN_CAUSE) -> Result<Self, ()> {
        Self::from_u32(v.0)
    }
}

/// Wrapper around the return value of `WaitForMultipleObjects` with bWaitAll as true
pub enum MultipleWaitAllResponse {
    /// Indicates that all objects in the array are now signalled
    Signaled,

    /// Indicates that all objects are signalled, but at least one of the objects is an abandoned
    /// mutex object. See `SingleWaitResponse::Abandoned` for more info.
    Abandoned,

    /// The wait timeout elapsed.
    Timeout,
}

impl MultipleWaitAllResponse {
    #[inline]
    fn from_u32(v: u32) -> Result<Self, ()> {
        match v {
            0x00000080 => Ok(MultipleWaitAllResponse::Abandoned),
            0x00000000 => Ok(MultipleWaitAllResponse::Signaled),
            0x00000102 => Ok(MultipleWaitAllResponse::Timeout),
            _ => Err(()),
        }
    }

    #[inline]
    fn from_cause(v: WAIT_RETURN_CAUSE) -> Result<Self, ()> {
        Self::from_u32(v.0)
    }
}

/// Wrapper around the return value of `WaitForMultipleObjects` with bWaitAll as false
pub enum MultipleWaitAnyResponse {
    /// Indicates which object in the array was signalled. Multiple objects may be signalled, this
    /// will yield the index of the lowest indexed object in the array to have been signalled.
    Signaled(usize),

    /// Same as `SingleWaitResponse::Abandoned`, but also specifies the array index of the lowest
    /// item in the `Event` array that triggered this response.
    Abandoned(usize),

    /// The wait timeout elapsed.
    Timeout,
}

impl MultipleWaitAnyResponse {
    #[inline]
    fn from_u32(v: u32) -> Result<Self, ()> {
        if v == WAIT_RETURN_CAUSE::WAIT_TIMEOUT.0 {
            return Ok(MultipleWaitAnyResponse::Timeout);
        }

        let signaled_index = v - WAIT_RETURN_CAUSE::WAIT_OBJECT_0.0;
        if signaled_index <= 64 {
            return Ok(MultipleWaitAnyResponse::Signaled(signaled_index as usize));
        }

        let abandoned_index = v - WAIT_RETURN_CAUSE::WAIT_ABANDONED_0.0;
        if abandoned_index <= 64 {
            return Ok(MultipleWaitAnyResponse::Abandoned(abandoned_index as usize));
        }

        if v == WAIT_RETURN_CAUSE::WAIT_FAILED.0 {
            return Err(());
        }

        unreachable!()
    }

    #[inline]
    fn from_cause(v: WAIT_RETURN_CAUSE) -> Result<Self, ()> {
        Self::from_u32(v.0)
    }
}

#[repr(transparent)]
pub struct Event(pub(crate) NonZeroIsize);

impl Event {
    #[inline]
    pub fn new() -> Option<Self> {
        let event = unsafe {
            CreateEventW(
                std::ptr::null_mut(),
                BOOL::from(false),
                BOOL::from(false),
                PWSTR(std::ptr::null_mut()),
            )
        };

        NonZeroIsize::new(event.0).map(|v| Self(v))
    }

    #[inline]
    pub fn wait(&self, timeout: Option<u32>) -> Result<SingleWaitResponse, ()> {
        let ret = if let Some(timeout) = timeout {
            unsafe { WaitForSingleObject(HANDLE(self.0.get()), timeout) }
        } else {
            unsafe { WaitForSingleObject(HANDLE(self.0.get()), 4294967295) }
        };
        SingleWaitResponse::from_cause(ret)
    }
}

impl Drop for Event {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            assert_ne!(
                CloseHandle(HANDLE(self.0.get())),
                windows_raw::BOOL::from(false)
            );
        }
    }
}

pub trait WaitAll {
    fn wait_all(&self, timeout: Option<u32>) -> Result<MultipleWaitAllResponse, ()>;

    fn wait_any(&self, timeout: Option<u32>) -> Result<MultipleWaitAnyResponse, ()>;
}

impl WaitAll for [Event] {
    #[inline]
    fn wait_all(&self, timeout: Option<u32>) -> Result<MultipleWaitAllResponse, ()> {
        if self.len() > 64 {
            panic!("Can't use WaitForMultipleObjects with more than 64 objects");
        }
        let ret = if let Some(timeout) = timeout {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(true),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(true),
                    4294967295,
                )
            }
        };
        MultipleWaitAllResponse::from_cause(ret)
    }

    #[inline]
    fn wait_any(&self, timeout: Option<u32>) -> Result<MultipleWaitAnyResponse, ()> {
        if self.len() > 64 {
            panic!("Can't use WaitForMultipleObjects with more than 64 objects");
        }
        let ret = if let Some(timeout) = timeout {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(false),
                    timeout,
                )
            }
        } else {
            unsafe {
                WaitForMultipleObjects(
                    self.len() as _,
                    self.as_ptr() as *const _,
                    BOOL::from(false),
                    4294967295,
                )
            }
        };
        MultipleWaitAnyResponse::from_cause(ret)
    }
}
