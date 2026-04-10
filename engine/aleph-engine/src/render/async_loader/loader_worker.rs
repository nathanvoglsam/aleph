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

use std::time::Duration;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use crossbeam::channel::{Receiver, RecvTimeoutError};
use mg::async_resource_loader::buffer_upload_range::BufferUploadRange;
use mg::async_resource_loader::{
    AllocateRangeError, AsyncResourceLoader, BufferLoadHandle, FlushError, RetireError,
};

use crate::core::alloc::EngineSystem;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

pub struct AsyncLoaderWorker {
    queue: Option<Receiver<WorkerMessage>>,
    loader: AsyncResourceLoader<ResourceLoadHandle>,
}

impl AsyncLoaderWorker {
    pub fn run(&mut self) {
        let queue = self.queue.take().unwrap();

        let mut inner = Inner {
            state: InnerState::AtRest,
            queue,
            working_set: BVec::new_in(system()),
        };
        inner.run(&self.loader);

        self.queue = Some(inner.queue);
    }
}

struct Inner<'a> {
    state: InnerState,
    queue: Receiver<WorkerMessage>,
    working_set: BVec<LoadState<'a>, EngineSystem>,
}

impl<'a> Inner<'a> {
    fn run(&mut self, loader: &'a AsyncResourceLoader<ResourceLoadHandle>) {
        'main: loop {
            match self.state {
                InnerState::AtRest => match self.at_rest() {
                    None => break 'main,
                    Some(_) => continue 'main,
                },
                InnerState::InFlight { .. } if self.working_set.is_empty() => {
                    match self.in_flight_empty(loader) {
                        None => break 'main,
                        Some(_) => continue 'main,
                    }
                }
                InnerState::InFlight { next } => match self.in_flight(loader, next) {
                    None => break 'main,
                    Some(_) => continue 'main,
                },
            }
        }
    }

    fn at_rest(&mut self) -> Option<()> {
        debug_assert!(self.working_set.is_empty());

        // Fully sleep on the queue to put the thread to sleep so it stops stealing
        // CPU time.
        //
        // This is fine because we should only have entered this state when there is
        // no GPU work in-flight and no in-progress uploads. Under these conditions
        // there's nothing left to do but wait for more work.
        let message = match self.queue.recv() {
            Ok(v) => v,
            Err(_) => {
                log::error!("AsyncLoaderWorker::queue disconnected.");
                return None;
            }
        };

        // Pop the request into the working set and then hand-ball off to the
        // 'in flight' branch.
        self.working_set.push(LoadState::Initial { message });

        self.state = InnerState::InFlight { next: 0 };
        Some(())
    }

    fn in_flight_empty(
        &mut self,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
    ) -> Option<()> {
        // If we've fully uploaded all our in-flight requests then we should
        // force-flush the loader. Without this the last batch of uploads may never
        // get flushed automatically as the loader internally only flushes if enough
        // work is queued.
        match loader.flush_submitted_uploads() {
            Ok(_) => {}
            Err(FlushError::DeviceLost) => {
                log::error!("GPU device lost.");
                return None;
            }
            Err(FlushError::RendererDisconnected) => {
                log::error!("Target renderer has been destroyed.");
                return None;
            }
            Err(e @ FlushError::CommandRecordingFailure) | Err(e @ FlushError::WaitFailure) => {
                panic!("Fatal: {e:?}")
            }
        }

        // We optimistically try and poll a new request from the queue with a short
        // timeout so we don't immediately go to sleep.
        let message = match self.queue.recv_timeout(Duration::from_millis(8)) {
            Ok(v) => v,
            Err(RecvTimeoutError::Timeout) => {
                // If we fail to get a new request from the queue within our timeout
                // window we block on the GPU to complete all the in-flight upload
                // work. Then we enter the 'at rest' state and restart the state
                // machine.
                //
                // Ultimately our goal here is to go to sleep as no work is
                // in-flight.
                match loader.wait_all_submissions() {
                    Ok(_) => {
                        self.state = InnerState::AtRest;
                        return Some(());
                    }
                    Err(RetireError::DeviceLost) => {
                        log::error!("GPU device lost.");
                        return None;
                    }
                    Err(RetireError::RendererDisconnected) => {
                        log::error!("Target renderer has been destroyed.");
                        return None;
                    }
                    Err(e @ RetireError::WaitFailure) => panic!("Fatal: {e:?}"),
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                log::error!("AsyncLoaderWorker::queue disconnected.");
                return None;
            }
        };

        // If we reach this point then we now have a new request, and we didn't go
        // to sleep. (Just in the nick of time)
        //
        // Pop the request into the working set and hand-ball to another loop of
        // the worker state machine.
        self.working_set.push(LoadState::Initial { message });

        self.state = InnerState::InFlight { next: 0 };
        Some(())
    }

    fn in_flight(
        &mut self,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        next: usize,
    ) -> Option<()> {
        // We reach this branch of the match if we're 'in flight' _and_ we still have
        // data to read from disk.
        match &mut self.working_set[next] {
            LoadState::Initial { message } => {
                let handle = match loader.begin_buffer_load(message.size, message.cookie) {
                    Ok(v) => v,
                    Err(e) => {
                        // If we failed to create the GPU resource then we should remove
                        // the upload from the working set and try and process another
                        // upload instead.
                        //
                        // The magnesium loader handles notifying the renderer. We just
                        // log a message.
                        log::error!("Failed to create GPU resource '{e:?}'.");
                        self.working_set.remove(next);

                        // We don't increment 'next' here because we just removed the
                        // old upload that was in that slot.
                        return Some(());
                    }
                };

                // Update the upload state because it now has a handle for the magnesium
                // loader.
                self.working_set[next] = LoadState::WantsRange { handle };

                // Don't increment 'next' because we want to process the same upload
                // again.
                Some(())
            }
            LoadState::WantsRange { handle } => {
                let mut attempt = 0;
                let range = 'alloc: loop {
                    match loader.allocate_range_for_buffer_load(*handle, u64::MAX) {
                        Ok(v) => break 'alloc v,
                        Err(AllocateRangeError::UploadComplete) => {
                            // This case is entered when the request is not yet retired
                            // but does not need any more data.
                            self.working_set.remove(next);

                            // We don't increment 'next' here because we just removed
                            // the old upload that was in that slot.
                            return Some(());
                        }
                        Err(AllocateRangeError::OutstandingRange) => {
                            // Our internal state machine should mean it's impossible to
                            // hit this case. We never ask for more data before
                            // submitting an existing block.
                            unreachable!();
                        }
                        Err(AllocateRangeError::NotEnoughUploadMemory) => {
                            // If we run out of memory in the internal pool then we must
                            // wait for all the in-flight GPU work to finish to free up
                            // space for the upload.
                            //
                            // Successfully waiting loops the alloc loop which will
                            // retry getting the range. If it fails a second time then
                            // we bail as we're stuck.
                            if attempt == 0 {
                                match loader.wait_all_submissions() {
                                    Ok(_) => {
                                        attempt += 1;
                                        continue 'alloc;
                                    }
                                    Err(RetireError::DeviceLost) => {
                                        log::error!("GPU device lost.");
                                        return None;
                                    }
                                    Err(RetireError::RendererDisconnected) => {
                                        log::error!("Target renderer has been destroyed.");
                                        return None;
                                    }
                                    Err(e @ RetireError::WaitFailure) => {
                                        panic!("Fatal: {e:?}")
                                    }
                                }
                            } else {
                                log::error!("Not enough memory in async loader pool.");
                                return None;
                            }
                        }
                        Err(AllocateRangeError::LoadHandleInvalid) => {
                            // It's possible to hit this case if a completed upload has
                            // already been removed from the magnesium loader before
                            // we've flushed it from the working set. We treat this the
                            // same as 'UploadComplete'.
                            self.working_set.remove(next);

                            // We don't increment 'next' here because we just removed
                            // the old upload that was in that slot.
                            return Some(());
                        }
                        Err(AllocateRangeError::DeviceLost) => {
                            log::error!("GPU device lost.");
                            return None;
                        }
                        Err(e @ AllocateRangeError::WaitFailure) => {
                            panic!("Fatal: {e:?}")
                        }
                        Err(AllocateRangeError::RendererDisconnected) => {
                            log::error!("Target renderer has been destroyed.");
                            return None;
                        }
                    }
                };

                self.working_set[next] = LoadState::HasRange {
                    handle: *handle,
                    range: Some(range),
                };

                // Don't increment 'next' because we want to process the same upload
                // again.
                Some(())
            }
            LoadState::HasRange { handle, range } => {
                let range = range.take().unwrap();

                // TODO: read file data into range

                match range.submit() {
                    Ok(_) => {}
                    Err(FlushError::DeviceLost) => {
                        log::error!("GPU device lost.");
                        return None;
                    }
                    Err(FlushError::RendererDisconnected) => {
                        log::error!("Target renderer has been destroyed.");
                        return None;
                    }
                    Err(e @ FlushError::CommandRecordingFailure)
                    | Err(e @ FlushError::WaitFailure) => {
                        panic!("Fatal: {e:?}")
                    }
                }

                // We've processed the allocated range. We go back to the previous state
                // so next time we process the upload we'll try and get another range.
                self.working_set[next] = LoadState::WantsRange { handle: *handle };

                // Increment the 'next' index to the next upload to process, round-robin
                // style.
                self.state = InnerState::InFlight {
                    next: (next + 1) % self.working_set.len(),
                };
                Some(())
            }
        }
    }
}

struct WorkerMessage {
    size: u64,
    cookie: ResourceLoadHandle,
}

enum LoadState<'a> {
    Initial {
        message: WorkerMessage,
    },
    WantsRange {
        handle: BufferLoadHandle,
    },
    HasRange {
        handle: BufferLoadHandle,
        range: Option<BufferUploadRange<'a, ResourceLoadHandle>>,
    },
}

enum InnerState {
    /// This state is entered when the loader is fully at rest. When there are no in-flight uploads,
    /// when the async loader has no in-flight GPU copies, and when there are no more requests in
    /// the queue to take.
    ///
    /// This is the initial state of the worker's inner loop.
    ///
    /// In this state the loader is expected to fully block on the queue with no timeout.
    AtRest,

    InFlight {
        next: usize,
    },
}
