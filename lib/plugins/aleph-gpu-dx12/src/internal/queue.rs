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

use crate::internal::in_flight_command_list::{InFlightCommandList, ReturnToPool};
use crossbeam::queue::SegQueue;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Queue<T: ReturnToPool> {
    pub handle: dx12::CommandQueue,
    pub submit_lock: Mutex<()>,
    pub fence: dx12::Fence,
    pub last_submitted_index: AtomicU64,
    pub last_completed_index: AtomicU64,
    pub in_flight: SegQueue<InFlightCommandList<T>>,
}

impl<T: ReturnToPool> Queue<T> {
    #[inline]
    pub fn new(device: &dx12::Device, handle: dx12::CommandQueue) -> Self {
        Self {
            handle,
            submit_lock: Mutex::new(()),
            fence: device.create_fence(0, dx12::FenceFlags::NONE).unwrap(),
            last_submitted_index: Default::default(),
            last_completed_index: Default::default(),
            in_flight: Default::default(),
        }
    }

    pub fn wait_all_lists_completed(&self) {
        while let Some(mut v) = self.in_flight.pop() {
            self.fence.set_event_on_completion(v.index, None).unwrap();
            self.last_completed_index.store(v.index, Ordering::Relaxed);
            v.list.return_to_pool();
        }
    }

    pub fn clear_completed_lists(&self) {
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
