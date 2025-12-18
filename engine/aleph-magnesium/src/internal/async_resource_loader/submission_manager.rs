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

use std::cell::{Cell, RefCell};

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_alloc::offset_allocator::Allocation;

use crate::async_resource_loader::{BufferLoadHandle, RetireError, TextureLoadHandle};
use crate::internal::async_resource_loader::MgAsyncLdrSystem;
use crate::internal::async_resource_loader::renderer_channel::{
    LoaderSender, LoaderToRendererMessage,
};
use crate::internal::async_resource_loader::request_states::RequestStates;
use crate::internal::async_resource_loader::upload_memory_manager::UploadMemoryManager;

pub struct Submission {
    /// The value that will be signaled on the loader fence when this submission is fully retired
    /// on the copy queue.
    pub signal_value: u64,

    /// All the upload memory allocations that should be released to the allocator once this
    /// submission is retired on the copy queue.
    pub retired_allocations: BVec<Allocation, MgAsyncLdrSystem>,

    /// Collection of resource handles that are in use on the GPU in this submission, resources that
    /// must be kept alive while the submission is in-flight. Without this canceling a request may
    /// drop the resource while it's still in use.
    pub live_resources: BVec<LiveResource, MgAsyncLdrSystem>,

    /// Handles to all the resource upload requests that will become fully complete once this
    /// submission is retired on the copy queue. As soon as these are observed to be retired they
    /// should be dispatched to the render thread to be made available to the renderer.
    pub completed_uploads: BVec<CompletedResource, MgAsyncLdrSystem>,
}

impl Submission {
    pub fn retire<C: Send + 'static>(
        mut self,
        request_states: &mut RequestStates<C>,
        upload_memory_manager: &UploadMemoryManager,
        loader_sender: &LoaderSender<C>,
    ) -> Result<(), RetireError> {
        // Dispatch all the completed uploads to the renderer now that we have observed that
        // their final copies are complete.

        let mut maybe_failed = Ok(());
        for completed in self.completed_uploads.drain(..) {
            // Instead of immediately returning if the renderer has disconnected (discovered when we
            // try and send a message) we simply flag the error in 'maybe_failed' and continue.
            //
            // We must leave the loader in a valid state. This means we have to run this function
            // completely even if the renderer has disconnected from the channel. It's still valid
            // to run this code even with no listener.
            match completed {
                CompletedResource::Buffer(r) => {
                    let request = request_states.buffers.free(r);
                    match request {
                        Some(r) => {
                            let msg = LoaderToRendererMessage::BufferComplete {
                                cookie: r.cookie,
                                resource: r.buffer,
                            };

                            if loader_sender.send(msg).is_err() {
                                maybe_failed = Err(RetireError::RendererDisconnected);
                            }
                        }
                        None => {
                            // Intentionally do nothing, as if the handle was invalid it means the
                            // request was canceled. We should just drop the resource.
                        }
                    }
                }
                CompletedResource::Texture(r) => {
                    let request = request_states.textures.free(r);
                    match request {
                        Some(r) => {
                            let msg = LoaderToRendererMessage::TextureComplete {
                                cookie: r.cookie,
                                resource: r.texture,
                            };
                            if loader_sender.send(msg).is_err() {
                                maybe_failed = Err(RetireError::RendererDisconnected);
                            }
                        }
                        None => {
                            // Intentionally do nothing, as if the handle was invalid it means the
                            // request was canceled. We should just drop the resource.
                        }
                    }
                }
            }
        }

        // The allocations that were being read from by the GPU are now free to be reused
        // so we return them to the allocator, freeing the memory for other uses.
        for allocation in self.retired_allocations.drain(..) {
            upload_memory_manager.free_upload_range(allocation);
        }

        // Drop our stashed handles that were keeping the in-use resources alive. If they
        // were canceled then this will be where they are destroyed, if they were _not_
        // canceled then this just decrements the refcount.
        self.live_resources.clear();

        maybe_failed
    }
}

pub struct SubmissionManager {
    /// Tracks the next index we will signal
    pub next_submission_index: Cell<u64>,

    /// The set of all live submissions, and any resources they currently own that should be retired
    /// once the submission is observed to be complete on the copy queue.
    pub live: RefCell<BVec<Submission, MgAsyncLdrSystem>>,
}

impl SubmissionManager {
    pub fn new() -> Self {
        Self {
            next_submission_index: Cell::new(1),
            live: RefCell::new(BVec::new_in(system())),
        }
    }

    pub fn new_submission(&self) -> Submission {
        Submission {
            signal_value: self.prepare_next_submission_index(),
            retired_allocations: BVec::new_in(system()),
            live_resources: BVec::new_in(system()),
            completed_uploads: BVec::new_in(system()),
        }
    }

    pub fn submit(&self, submission: Submission) {
        self.live.borrow_mut().push(submission);
    }

    pub fn prepare_next_submission_index(&self) -> u64 {
        let out = self.next_submission_index.get();
        let next = out
            .checked_add(1)
            .expect("If this ever overflows, god help us");
        self.next_submission_index.set(next);
        out
    }
}

/// 'allow(unused)' on the associated fields because we only store them for the 'drop' side effects.
pub enum LiveResource {
    Buffer(#[allow(unused)] rhi::BufferHandle),
    Texture(#[allow(unused)] rhi::TextureHandle),
}

impl From<rhi::BufferHandle> for LiveResource {
    fn from(value: rhi::BufferHandle) -> Self {
        Self::Buffer(value)
    }
}

impl From<rhi::TextureHandle> for LiveResource {
    fn from(value: rhi::TextureHandle) -> Self {
        Self::Texture(value)
    }
}

pub enum CompletedResource {
    Buffer(BufferLoadHandle),
    Texture(TextureLoadHandle),
}

impl From<BufferLoadHandle> for CompletedResource {
    fn from(value: BufferLoadHandle) -> Self {
        Self::Buffer(value)
    }
}

impl From<TextureLoadHandle> for CompletedResource {
    fn from(value: TextureLoadHandle) -> Self {
        Self::Texture(value)
    }
}
