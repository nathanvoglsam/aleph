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

use aleph_any::{AnyArc, AnyWeak, box_downcast, declare_interfaces};
use aleph_rhi_api::*;
use crossbeam::queue::ArrayQueue;
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::ProtocolObject;
use objc2_foundation::ns_string;
use objc2_metal::*;
use parking_lot::{Mutex, MutexGuard};

use crate::command_list::{CommandList, ListState};
use crate::device::Device;
use crate::fence::Fence;
use crate::semaphore::Semaphore;
use crate::swap_image::SwapImage;

pub struct Queue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyWeak<Device>,
    pub(crate) queue_type: QueueType,

    pub(crate) objects: QueueObjects,

    /// Lock used to serialize submissions to the command queue.
    pub(crate) submit_lock: Mutex<()>,

    /// A ring-buffer that tracks all currently in flight queue submissions. This is used in
    /// conjunction with [IQueue::garbage_collect] to track when resources are no longer in use on
    /// the GPU timeline and are safe to destroy.
    pub(crate) in_flight: ArrayQueue<QueueSubmission>,
}

declare_interfaces!(Queue, [IQueue]);

impl IGetPlatformInterface for Queue {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl Queue {
    #[inline]
    pub(crate) fn new(device: &Device, queue_type: QueueType) -> Option<AnyArc<Self>> {
        // TODO: should this be configurable? metal 4 will make this go away anyway I think?
        let queue = device.device.newCommandQueueWithMaxCommandBufferCount(64)?;
        match queue_type {
            QueueType::General => queue.setLabel(Some(ns_string!("GeneralQueue"))),
            QueueType::Compute => queue.setLabel(Some(ns_string!("ComputeQueue"))),
            QueueType::Transfer => queue.setLabel(Some(ns_string!("TransferQueue"))),
        }

        let out = AnyArc::new_cyclic(|v| Self {
            _this: v.clone(),
            _device: device.this.clone(),
            queue_type,
            objects: QueueObjects { queue },
            submit_lock: Mutex::new(()),
            in_flight: ArrayQueue::new(256),
        });

        Some(out)
    }
}

impl IQueue for Queue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        QueueProperties {
            min_image_transfer_granularity: Extent3D::default(),
        }
    }

    fn garbage_collect(&self) {
        // Lock access to the queue to ensure nobody submits while we're running the GC cycle.
        let _lock = self.submit_lock.lock();
        autoreleasepool(|_| {
            self.garbage_collect_internal();
        })
    }

    fn wait_idle(&self) {
        // Lock access to the queue to ensure nobody submits while we're waiting for all outstanding
        // work to complete
        let _lock = self.submit_lock.lock();
        autoreleasepool(|_| {
            self.wait_idle_internal();
        })
    }

    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        autoreleasepool(|_| {
            let _lock = self.submit_lock.lock();

            let lists: Vec<_> = desc
                .command_lists
                .iter()
                .map(|v| {
                    let v = v.take().unwrap();
                    box_downcast::<_, CommandList>(v).ok().unwrap()
                })
                .collect();

            let wait_list = if !desc.wait_semaphores.is_empty() {
                let list = self.objects.queue.commandBuffer().unwrap();

                for semaphore in desc.wait_semaphores {
                    let semaphore = Semaphore::get(semaphore);
                    list.encodeWaitForEvent_value(
                        semaphore.objects.event.as_ref(),
                        semaphore.get_wait_value(),
                    );
                }

                Some(list)
            } else {
                None
            };

            match lists.last() {
                Some(last) => {
                    for semaphore in desc.signal_semaphores {
                        let semaphore = Semaphore::get(semaphore);

                        last.objects.list.encodeSignalEvent_value(
                            semaphore.objects.event.as_ref(),
                            semaphore.get_next_signal_value(),
                        );
                    }
                    if let Some(fence) = desc.fence {
                        let fence = Fence::get(fence);

                        last.objects.list.encodeSignalEvent_value(
                            fence.objects.event.as_ref(),
                            fence.get_next_signal_value(),
                        );
                    }
                }
                None => panic!("Can't call IQueue::submit with zero command buffers!"),
            }

            if let Some(wait_list) = wait_list {
                wait_list.commit();
            }

            for list in lists {
                assert_eq!(list.list_type, self.queue_type);
                assert_eq!(list.state, ListState::Closed);

                // TODO: flush barriers between submits?

                list.objects.list.commit();
            }

            Ok(())
        })
    }

    unsafe fn present(&self, swap_image: AnyArc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        autoreleasepool(|_| {
            let _lock = self.submit_lock.lock();

            let mut swap_image = {
                let v = swap_image;
                let v = v
                    .query_interface::<SwapImage>()
                    .expect("Unknown ISwapImage implementation");
                v
            };
            let swap_image = AnyArc::get_mut(&mut swap_image).unwrap();

            let list = &swap_image.objects.list;
            list.presentDrawable(swap_image.objects.drawable.as_ref());
            list.commit();

            Ok(())
        })
    }
}

impl Queue {
    pub fn wait_idle_internal(&self) {
        while let Some(mut v) = self.in_flight.pop() {
            for list in v.lists.drain(..) {
                list.objects.list.waitUntilCompleted();
            }
        }
    }

    pub fn garbage_collect_internal(&self) {
        let num = self.in_flight.len();
        for _ in 0..num {
            let mut v = self.in_flight.pop().unwrap();

            v.lists.retain(|v| match v.objects.list.status() {
                MTLCommandBufferStatus::NotEnqueued => unreachable!(),
                MTLCommandBufferStatus::Enqueued => unreachable!(),
                MTLCommandBufferStatus::Committed => true,
                MTLCommandBufferStatus::Scheduled => true,
                MTLCommandBufferStatus::Completed => false,
                MTLCommandBufferStatus::Error => panic!("CommandBuffer status = 'error'"),
                _ => panic!("CommandBuffer status = 'unknown'"),
            });

            if !v.lists.is_empty() {
                self.in_flight.push(v).ok().unwrap();
            }
        }
    }

    pub fn submit_lock(&self) -> MutexGuard<'_, ()> {
        self.submit_lock.lock()
    }
}

pub struct QueueSubmission {
    /// We separate the command lists in the submission into their own list so they can be easily
    /// filtered out and recycled later
    pub lists: Vec<Box<CommandList>>,
}

/// Wrapper over the MTL objects to limit scope of our 'unsafe impl Send+Sync'
pub struct QueueObjects {
    pub queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

// Safety: Needed for 'MTLCommandQueue'
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}
