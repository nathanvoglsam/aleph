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

use aleph_rhi_api::{FencePollError, FenceSignalError, FenceWaitError};
use ash::vk;

#[inline]
pub fn map_error_class<T: ErrorClasses>(err: vk::Result) -> T {
    match err {
        vk::Result::ERROR_DEVICE_LOST => T::DEVICE_LOST,
        vk::Result::ERROR_OUT_OF_HOST_MEMORY => T::OUT_OF_HOST_MEMORY,
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY => T::OUT_OF_DEVICE_MEMORY,
        vk::Result::ERROR_OUT_OF_POOL_MEMORY => T::OUT_OF_POOL_MEMORY,
        _ => T::PLATFORM,
    }
}

pub trait ErrorClasses {
    const DEVICE_LOST: Self;
    const OUT_OF_HOST_MEMORY: Self;
    const OUT_OF_DEVICE_MEMORY: Self;
    const OUT_OF_POOL_MEMORY: Self;
    const PLATFORM: Self;
}

impl ErrorClasses for FenceWaitError {
    const DEVICE_LOST: Self = FenceWaitError::DeviceLost;
    const OUT_OF_HOST_MEMORY: Self = FenceWaitError::Platform;
    const OUT_OF_DEVICE_MEMORY: Self = FenceWaitError::Platform;
    const OUT_OF_POOL_MEMORY: Self = FenceWaitError::Platform;
    const PLATFORM: Self = FenceWaitError::Platform;
}

impl ErrorClasses for FencePollError {
    const DEVICE_LOST: Self = FencePollError::DeviceLost;
    const OUT_OF_HOST_MEMORY: Self = FencePollError::Platform;
    const OUT_OF_DEVICE_MEMORY: Self = FencePollError::Platform;
    const OUT_OF_POOL_MEMORY: Self = FencePollError::Platform;
    const PLATFORM: Self = FencePollError::Platform;
}

impl ErrorClasses for FenceSignalError {
    const DEVICE_LOST: Self = FenceSignalError::DeviceLost;
    const OUT_OF_HOST_MEMORY: Self = FenceSignalError::Platform;
    const OUT_OF_DEVICE_MEMORY: Self = FenceSignalError::Platform;
    const OUT_OF_POOL_MEMORY: Self = FenceSignalError::Platform;
    const PLATFORM: Self = FenceSignalError::Platform;
}
