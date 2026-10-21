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

use std::collections::VecDeque;
use std::sync::Arc;
use std::thread::JoinHandle;

use aleph_alloc::{BVec, Blink};
use crossbeam::channel::{Receiver, SendError, Sender, TryRecvError, unbounded};
use tokio::sync::oneshot;

/// Result type for async fence operations.
pub type AsyncFenceResult = Result<(), rhi::FenceWaitError>;

/// Sender half of a channel used to submit fences to an [`AsyncFenceWaiter`].
#[derive(Clone)]
pub struct AsyncFenceWaiterQueue {
    sender: Sender<PendingFence>,
}

impl AsyncFenceWaiterQueue {
    /// Queues a fence until it reaches `value`, then sends the result to `completion`.
    pub fn wait_for(
        &self,
        fence: rhi::FenceHandle,
        value: u64,
        completion: oneshot::Sender<AsyncFenceResult>,
    ) -> Result<(), SendError<()>> {
        self.sender
            .send(PendingFence {
                fence,
                value,
                completion,
            })
            .map_err(|_| SendError(()))
    }
}

/// Worker that waits for submitted GPU fences on a dedicated thread.
pub struct AsyncFenceWaiter {
    receiver: Receiver<PendingFence>,
    device: Arc<dyn rhi::IDevice>,
    pending: VecDeque<PendingFence>,
    wait_arena: Blink,
    min_timeout_ms: u32,
    max_timeout_ms: u32,
}

impl AsyncFenceWaiter {
    /// Spawns the fence waiter thread.
    ///
    /// Returns the thread's join handle and a queue that can submit fences to it.
    pub fn spawn(device: Arc<dyn rhi::IDevice>) -> (JoinHandle<()>, AsyncFenceWaiterQueue) {
        Self::spawn_with_timeout_range(device, 1, 8)
    }

    /// Spawns the fence waiter thread with a configurable wait timeout range, in milliseconds.
    ///
    /// The minimum timeout is used initially and after a successful wait. Each timeout doubles
    /// the current value up to the maximum.
    ///
    /// # Panics
    ///
    /// Panics if `min_timeout_ms` is greater than `max_timeout_ms`.
    pub fn spawn_with_timeout_range(
        device: Arc<dyn rhi::IDevice>,
        min_timeout_ms: u32,
        max_timeout_ms: u32,
    ) -> (JoinHandle<()>, AsyncFenceWaiterQueue) {
        assert!(
            min_timeout_ms <= max_timeout_ms,
            "minimum fence wait timeout must not exceed maximum timeout"
        );

        let (sender, receiver) = unbounded();
        let queue = AsyncFenceWaiterQueue { sender };
        let waiter = Self {
            receiver,
            device,
            pending: VecDeque::new(),
            wait_arena: Blink::new(),
            min_timeout_ms,
            max_timeout_ms,
        };

        let handle = std::thread::Builder::new()
            .name("async-fence-waiter".into())
            .spawn(move || waiter.run())
            .expect("Failed to spawn AsyncFenceWaiter thread");

        (handle, queue)
    }

    fn run(mut self) {
        let mut wait_timeout_ms = self.min_timeout_ms;

        let error = 'fail: {
            'success: loop {
                // Go to sleep if there are no pending fences, and wait for at least one to arrive.
                if self.pending.is_empty() {
                    match self.receiver.recv() {
                        Ok(fence) => self.pending.push_back(fence),
                        Err(_) => break 'success,
                    }
                }

                // Drain the receiver of any additional fences that have arrived.
                'receive: loop {
                    match self.receiver.try_recv() {
                        Ok(fence) => self.pending.push_back(fence),
                        Err(TryRecvError::Empty | TryRecvError::Disconnected) => break 'receive,
                    }
                }

                // Wait for the pending fences to be signaled, with a timeout to allow polling.
                match self.wait_with_timeout(wait_timeout_ms) {
                    Ok(rhi::FenceWaitResult::Complete) => {
                        wait_timeout_ms = self.min_timeout_ms;
                        if let Err(error) = self.poll_pending() {
                            log::error!("Async fence waiter failed to poll a fence: {error:?}");
                            break 'fail error;
                        }
                    }
                    Ok(rhi::FenceWaitResult::Timeout) => {
                        // Wait timeout means nothing completed, so we can just continue to the next
                        // iteration and poll again.
                        wait_timeout_ms =
                            wait_timeout_ms.saturating_mul(2).min(self.max_timeout_ms);
                    }
                    Err(error) => {
                        log::error!("Async fence waiter failed to wait for fences: {error:?}");
                        break 'fail error;
                    }
                }
            }

            // If we reach this point, the receiver has been dropped. We should wait for any fences
            // that are still pending, and then exit.
            while let Some(entry) = self.pending.pop_front() {
                match self
                    .device
                    .wait_fences(&[&entry.fence], &[entry.value], false, u32::MAX)
                {
                    Ok(_) => {
                        let _ = entry.completion.send(Ok(()));
                    }
                    Err(error) => {
                        log::error!("Async fence waiter failed to wait for fences: {error:?}");
                        let _ = entry.completion.send(Err(error));
                        break 'fail error;
                    }
                }
            }
            return;
        };

        // If we reach this point, an error occurred. We should notify all pending fences of the
        // error.
        //
        // We treat any error as fatal, and we will not attempt to recover from it. This is because
        // the error may indicate that the device has been lost, and we cannot safely continue to
        // wait for fences in that case.
        while let Some(entry) = self.pending.pop_front() {
            let _ = entry.completion.send(Err(error));
        }
    }

    fn wait_with_timeout(
        &mut self,
        timeout_ms: u32,
    ) -> Result<rhi::FenceWaitResult, rhi::FenceWaitError> {
        let mut fences = BVec::with_capacity_in(self.pending.len(), self.wait_arena.allocator());
        fences.extend(self.pending.iter().map(|entry| &entry.fence));
        let mut values = BVec::with_capacity_in(self.pending.len(), self.wait_arena.allocator());
        values.extend(self.pending.iter().map(|entry| entry.value));

        let out = self.device.wait_fences(&fences, &values, false, timeout_ms);

        drop(fences);
        drop(values);

        self.wait_arena.reset();
        out
    }

    fn poll_pending(&mut self) -> Result<(), rhi::FenceWaitError> {
        let mut temp = BVec::with_capacity_in(self.pending.len(), self.wait_arena.allocator());

        while let Some(entry) = self.pending.pop_front() {
            match self.device.get_fence_signaled_value(&entry.fence) {
                Ok(signaled_value) => {
                    if signaled_value >= entry.value {
                        let _ = entry.completion.send(Ok(()));
                    } else {
                        temp.push(entry);
                    }
                }
                Err(error) => {
                    let error = map_poll_error(error);
                    let _ = entry.completion.send(Err(error));

                    self.pending.extend(temp.into_iter());
                    self.wait_arena.reset();
                    return Err(error);
                }
            }
        }

        self.pending.extend(temp.into_iter());
        self.wait_arena.reset();
        Ok(())
    }
}

struct PendingFence {
    fence: rhi::FenceHandle,
    value: u64,
    completion: oneshot::Sender<AsyncFenceResult>,
}

fn map_poll_error(error: rhi::FencePollError) -> rhi::FenceWaitError {
    match error {
        rhi::FencePollError::DeviceLost => rhi::FenceWaitError::DeviceLost,
        rhi::FencePollError::Platform => rhi::FenceWaitError::Platform,
        _ => rhi::FenceWaitError::Platform,
    }
}
