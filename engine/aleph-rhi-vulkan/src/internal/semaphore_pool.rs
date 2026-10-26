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

use std::ops::DerefMut;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_rhi_impl_utils::RhiRoot;
use ash::vk;
use parking_lot::Mutex;

use crate::internal::allocation_callbacks::GLOBAL;

/// Internal semaphore recycling pool.
///
/// Maintains two lists of semaphores: one for immediately reusable semaphores and one for
/// semaphores pending recycling.
///
/// Depending on the use case semaphores may be recycled immediately, or added to a queue to be
/// recycled at a known safepoint later.
///
/// # Global 'swap ready' semaphores
///
/// The semaphores used for swap image acquire operations are maintained in a single global pool.
/// The reuse semantics of these are such that once the acquire op signals them and using a swap
/// image on a queue waits it. This means that the signal and wait operations are always balanced.
/// This also means that these semaphores can be safely recycled immediately after use, and can use
/// 'push_immediately' to return them to the pool without waiting for a safepoint.
///
/// # Per-image 'swap work' semaphores
///
/// Semaphores associated with individual swap images are managed separately. These semaphores may
/// not be immediately reusable after use, as they might still be in use by the GPU. Instead, they
/// are added to the recycle queue and moved to the reusable set once it is safe to do so.
///
/// When a swap image is used in a queue submission, a 'work semaphore' is pulled from the pool and
/// added to be signalled when the submission is retired. It is not safe to return the work
/// semaphore to the pool once the submission has been garbage collected because the semaphore wait
/// operation is not yet balanced.
///
/// A 'work semaphore' is waited by a swap presentation operation. There is no explicit way in
/// Vulkan (without extensions we can't rely on) to know when the presentation has retired and the
/// semaphore can be safely recycled. The only way to know a presentation is complete is to observe
/// the same swap image index being given in an acquire operation on the same swap chain.
///
/// So 'work semaphore' objects are instead added to the 'to_recycle' queue and only get released to
/// be reused when the same image index is acquired again.
///
/// # Note
///
/// The same pool type is used to service both cases.
///
/// The global pool will never have 'recycle' called, so do not attempt to use the recycle queue for
/// it.
pub struct SemaphorePool {
    semaphores: Mutex<SemaphorePoolInner>,
}

impl SemaphorePool {
    pub const fn new() -> Self {
        Self {
            semaphores: Mutex::new(SemaphorePoolInner {
                semaphores: BVec::new_in(system()),
                to_recycle: BVec::new_in(system()),
            }),
        }
    }

    pub unsafe fn get(&self, device: &ash::Device) -> vk::Semaphore {
        let mut semaphores = self.semaphores.lock();
        if let Some(semaphore) = semaphores.semaphores.pop() {
            semaphore
        } else {
            unsafe {
                let info = vk::SemaphoreCreateInfo::default();
                device.create_semaphore(&info, GLOBAL).unwrap()
            }
        }
    }

    pub fn recycle(&self) {
        let mut semaphores = self.semaphores.lock();
        let semaphores = semaphores.deref_mut();
        semaphores
            .semaphores
            .extend(semaphores.to_recycle.drain(..));
    }

    pub fn push_immediately(&self, semaphore: vk::Semaphore) {
        let mut semaphores = self.semaphores.lock();
        semaphores.semaphores.push(semaphore);
    }

    pub fn push_to_recycle_queue(&self, semaphore: vk::Semaphore) {
        let mut semaphores = self.semaphores.lock();
        semaphores.to_recycle.push(semaphore);
    }

    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        let semaphores = Mutex::get_mut(&mut self.semaphores);
        for semaphore in semaphores.semaphores.drain(..) {
            unsafe {
                device.destroy_semaphore(semaphore, GLOBAL);
            }
        }
        for semaphore in semaphores.to_recycle.drain(..) {
            unsafe {
                device.destroy_semaphore(semaphore, GLOBAL);
            }
        }
    }
}

struct SemaphorePoolInner {
    /// The set of immediately reusable semaphores.
    semaphores: BVec<vk::Semaphore, RhiRoot>,

    /// A set of semaphores that are pending recycling and will be moved to the reusable set later.
    to_recycle: BVec<vk::Semaphore, RhiRoot>,
}
