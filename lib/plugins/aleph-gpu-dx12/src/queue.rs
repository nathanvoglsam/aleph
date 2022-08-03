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

use crate::acquired_texture::AcquiredTexture;
use crate::command_list::CommandList;
use crate::internal::in_flight_command_list::{InFlightCommandList, ReturnToPool};
use crossbeam::queue::SegQueue;
use dx12::{AsWeakRef, D3D12Object};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterfaceBox};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    IAcquiredTexture, ICommandList, INamedObject, IQueue, ISwapChain, QueuePresentError,
    QueueSubmitError, QueueType,
};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Queue {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) queue_type: QueueType,
    pub(crate) handle: dx12::CommandQueue,
    pub(crate) submit_lock: Mutex<()>,
    pub(crate) fence: dx12::Fence,
    pub(crate) last_submitted_index: AtomicU64,
    pub(crate) last_completed_index: AtomicU64,
    pub(crate) in_flight: SegQueue<InFlightCommandList<CommandList>>,
}

declare_interfaces!(Queue, [IQueue]);

impl Queue {
    fn is_list_supported(queue_type: QueueType, list_type: QueueType) -> bool {
        return match (queue_type, list_type) {
            (QueueType::Compute, QueueType::General) => false,
            (QueueType::Transfer, QueueType::Compute | QueueType::General) => false,
            _ => true,
        };
    }

    #[inline]
    pub(crate) fn new(
        device: &dx12::Device,
        queue_type: QueueType,
        handle: dx12::CommandQueue,
    ) -> AnyArc<Self> {
        AnyArc::new_cyclic(|v| Self {
            this: v.clone(),
            queue_type,
            handle,
            submit_lock: Mutex::new(()),
            fence: device.create_fence(0, dx12::FenceFlags::NONE).unwrap(),
            last_submitted_index: Default::default(),
            last_completed_index: Default::default(),
            in_flight: Default::default(),
        })
    }

    pub(crate) fn wait_all_lists_completed(&self) {
        while let Some(mut v) = self.in_flight.pop() {
            self.fence.set_event_on_completion(v.index, None).unwrap();
            self.last_completed_index.store(v.index, Ordering::Relaxed);
            v.list.return_to_pool();
        }
    }

    pub(crate) fn clear_completed_lists(&self) {
        // Grab the index of the most recently completed command list on this queue and update
        // the queue's value
        let last_completed = self.fence.get_completed_value();
        self.last_completed_index
            .store(last_completed, Ordering::Relaxed);

        // Capture the current length of the queue. We then pop N items off the queue and check
        // to see if it is complete based on comparing the list's index with the last completed
        // index. If the list is done we drop it to release any resources that it was keeping
        // alive.
        let num = self.in_flight.len();
        for _ in 0..num {
            // Check if the
            let mut v = self.in_flight.pop().unwrap();
            if v.index > last_completed {
                self.in_flight.push(v);
            } else {
                v.list.return_to_pool();
            }
        }
    }
}

impl IQueue for Queue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    unsafe fn submit_list(
        &self,
        command_list: Box<dyn ICommandList>,
    ) -> Result<(), QueueSubmitError> {
        let command_list: Box<CommandList> =
            command_list.query_interface::<CommandList>().ok().unwrap();

        // Check that the queue supports submitting the provided command list type
        let (queue_type, list_type) = (self.queue_type, command_list.list_type);
        if !Self::is_list_supported(queue_type, list_type) {
            return Err(QueueSubmitError::InvalidEncoderType(list_type));
        }

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = self.submit_lock.lock();
            self.handle
                .execute_command_lists(&[command_list.list.as_weak()]);

            let index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            self.handle
                .signal(&self.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        self.in_flight.push(InFlightCommandList {
            index,
            list: command_list,
        });

        Ok(())
    }

    unsafe fn submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn ICommandList>>,
    ) -> Result<(), QueueSubmitError> {
        // Perform the actual submit operation
        let lists: Vec<Box<CommandList>> = command_lists
            .map(|v| v.query_interface::<CommandList>().ok().unwrap())
            .collect();

        // Check that the queue supports submitting the provided command list type
        for command_list in lists.iter() {
            let (queue_type, list_type) = (self.queue_type, command_list.list_type);
            if !Self::is_list_supported(queue_type, list_type) {
                return Err(QueueSubmitError::InvalidEncoderType(list_type));
            }
        }

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = self.submit_lock.lock();

            let handles: Vec<dx12::GraphicsCommandList> =
                lists.iter().map(|v| v.list.clone()).collect();

            self.handle.execute_command_lists_strong(&handles);

            let index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            self.handle
                .signal(&self.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        for list in lists {
            self.in_flight.push(InFlightCommandList { index, list });
        }

        Ok(())
    }

    unsafe fn present(&self, image: Box<dyn IAcquiredTexture>) -> Result<(), QueuePresentError> {
        let image = image.query_interface::<AcquiredTexture>().ok().unwrap();

        if !image
            .swap_chain
            .present_supported_on_queue(QueueType::General)
        {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                QueueType::General,
            ));
        }

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let _index = {
            let _lock = self.submit_lock.lock();

            image
                .swap_chain
                .swap_chain
                .present(0, 0)
                .map_err(|v| anyhow!(v))?;
            let index = self.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            self.handle
                .signal(&self.fence, index)
                .map_err(|v| anyhow!(v))?;

            // TODO: We need to track the lifetime of this operation and extend the swap image's
            //       lifetime until the present operation is complete.

            index
        };

        Ok(())
    }
}

impl INamedObject for Queue {
    fn set_name(&self, name: &str) {
        self.handle.set_name(name).unwrap()
    }
}