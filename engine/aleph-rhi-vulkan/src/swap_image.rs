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

use std::any::TypeId;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use aleph_alloc::BVec;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::{RhiRoot, abort_on_unwind};
use ash::vk::{self, Handle};
use parking_lot::Mutex;

use crate::internal::queue_present_support::QueuePresentSupportFlags;
use crate::internal::semaphore_pool::SemaphorePool;
use crate::swap_chain::SwapChain;

pub struct SwapImage {
    pub(crate) swap_chain: Arc<SwapChain>,
    pub(crate) index: u32,
    pub(crate) texture: Option<TextureHandle>,
    pub(crate) ready_semaphore: AtomicU64,
    pub(crate) work_semaphores: Mutex<BVec<WorkSemaphore, RhiRoot>>,
    pub(crate) work_semaphore_pool: Arc<SemaphorePool>,
}

impl IGetPlatformInterface for SwapImage {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl Drop for SwapImage {
    fn drop(&mut self) {
        abort_on_unwind(|| {
            if self.texture.is_some() {
                let support = self.swap_chain.queue_support;
                let result = if support.contains(QueuePresentSupportFlags::GENERAL) {
                    self.present_on_queue(QueueType::General)
                } else if support.contains(QueuePresentSupportFlags::COMPUTE) {
                    self.present_on_queue(QueueType::Compute)
                } else if support.contains(QueuePresentSupportFlags::TRANSFER) {
                    self.present_on_queue(QueueType::Transfer)
                } else {
                    unreachable!("How we have swap chain without queue that can present????")
                };
                match result {
                    Ok(_) => {}
                    Err(QueuePresentError::DeviceLost) => {}
                    Err(QueuePresentError::QueuePresentationNotSupported(_)) => unreachable!(),
                    Err(QueuePresentError::SubOptimal) => {}
                    Err(QueuePresentError::OutOfDate) => {}
                    Err(QueuePresentError::SurfaceLost) => {}
                    Err(QueuePresentError::Platform) => panic!("Failed to issue fallback present"),
                }
            }
        });
    }
}

impl ISwapImage for SwapImage {
    fn texture(&self) -> &TextureHandle {
        abort_on_unwind(|| self.texture.as_ref().unwrap())
    }

    fn texture_desc(&self) -> &TextureDesc<'_> {
        abort_on_unwind(|| self.swap_chain.device.get_texture_desc(self.texture()))
    }
}

impl SwapImage {
    pub fn take_ready_semaphore(&self) -> vk::Semaphore {
        let out = self.ready_semaphore.swap(0, Ordering::Acquire);
        vk::Semaphore::from_raw(out)
    }

    fn present_on_queue(&mut self, queue: QueueType) -> Result<(), QueuePresentError> {
        let queue = self.swap_chain.device.get_queue_internal(queue).unwrap();
        let queue = queue._this.upgrade().unwrap();
        queue.present_internal(self, true)
    }
}

pub struct WorkSemaphore {
    /// The semaphore object itself. This semaphore must have been had a signal operation enqueued
    /// on the specified queue type to be present int the 'work_semaphores' list.
    pub handle: vk::Semaphore,

    /// The queue the semaphore will be signalled by once the associated work is retired.
    pub queue_type: QueueType,

    /// The submission index on the queue when this semaphore was enqueued to be signalled.
    ///
    /// Internally we always use a timeline semaphore to track each submission, and this index
    /// corresponds to the value in the timeline semaphore when this semaphore is expected to be
    /// signalled.
    ///
    /// This is needed so we can clean up the work semaphores under various failure conditions.
    pub submission_index: u64,
}
