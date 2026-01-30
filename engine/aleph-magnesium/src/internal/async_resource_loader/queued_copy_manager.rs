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

use crate::async_resource_loader::buffer_upload_range::BufferUploadRange;
use crate::async_resource_loader::texture_upload_range::TextureUploadRange;
use crate::async_resource_loader::{BufferLoadHandle, TextureLoadHandle};
use crate::internal::async_resource_loader::MgAsyncLdrSystem;

pub struct QueuedCopyManager {
    /// Queue of submitted upload ranges that should have upload commands recorded for in the next
    /// flush operation.
    pub queue: RefCell<BVec<SubmittedCopy, MgAsyncLdrSystem>>,

    /// Sum total of the sizes of all upload ranges currently queued.
    pub queued_bytes: Cell<u64>,
}

impl QueuedCopyManager {
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(BVec::new_in(system())),
            queued_bytes: Cell::new(0),
        }
    }

    pub fn submit_buffer_upload_range<C: Send + 'static>(
        &self,
        mut range: BufferUploadRange<C>,
        is_final: bool,
    ) {
        let mut submitted_queue = self.queue.borrow_mut();

        // Submit the upload range to the queue. We _must_ use mem::take() on the allocation
        // field to leave it in the 'is_fail' state. This sentinel signals that the upload has
        // been submitted and prevents the drop implementation from freeing the allocation too
        // early.
        let copy = SubmittedBufferCopy {
            request: range.request,
            region: rhi::BufferCopyRegion {
                src_offset: range.allocation.offset as u64,
                dst_offset: range.dst_offset,
                size: range.data.len() as u64,
            },
            allocation: std::mem::take(&mut range.allocation),
            is_final,
        };
        submitted_queue.push(SubmittedCopy::Buffer(copy));

        // Add the size of the range to our running total of bytes in the queue
        let bytes = range.data.len() as u64;
        self.queued_bytes.update(|v| v + bytes);
    }

    pub fn submit_texture_upload_range<C: Send + 'static>(
        &self,
        mut range: TextureUploadRange<C>,
        is_final: bool,
    ) {
        let mut submitted_queue = self.queue.borrow_mut();

        let mut regions = BVec::new_in(system());
        regions.extend(range.wanted.iter().map(|v| v.copy_region()));

        // We _must_ use mem::take() on the allocation field to leave it in the 'is_fail' state.
        // This sentinel signals that the upload has been submitted and prevents the drop
        // implementation from freeing the allocation too early.
        let mut allocations = BVec::new_in(system());
        allocations.extend(
            range
                .wanted
                .iter_mut()
                .map(|v| std::mem::take(&mut v.allocation)),
        );

        // Submit the upload range to the queue.
        let copy = SubmittedTextureCopy {
            request: range.request,
            regions,
            allocations,
            needs_discard: range.needs_discard,
            is_final,
        };
        submitted_queue.push(SubmittedCopy::Texture(copy));

        // Add the size of the range to our running total of bytes in the queue
        let bytes = range.total_bytes() as u64;
        self.queued_bytes.update(|v| v + bytes);
    }
}

pub enum SubmittedCopy {
    Buffer(SubmittedBufferCopy),
    Texture(SubmittedTextureCopy),
}

pub struct SubmittedBufferCopy {
    /// Handle to the request this upload block is uploading into
    pub request: BufferLoadHandle,

    /// The dst buffer region
    pub region: rhi::BufferCopyRegion,

    /// Allocation handle that holds ownership over the region we're using in the upload block
    pub allocation: Allocation,

    /// Whether this is the last upload for this resource, and the resource should be dispatched
    /// to the renderer thread once the upload is observed as complete.
    pub is_final: bool,
}

pub struct SubmittedTextureCopy {
    /// Handle to the request this upload block is uploading into
    pub request: TextureLoadHandle,

    /// A list of dst copy regions, typically one for each target mip level, this upload block is
    /// trying to copy into.
    pub regions: BVec<rhi::BufferToTextureCopyRegion, MgAsyncLdrSystem>,

    /// Allocation handles that hold ownership over the regions we're using in the upload block
    pub allocations: BVec<Allocation, MgAsyncLdrSystem>,

    /// 'true' if this is the first copy into the texture. Requires issuing a discard barrier if
    /// 'true'.
    pub needs_discard: bool,

    /// Whether this is the last upload for this resource, and the resource should be dispatched
    /// to the renderer thread once the upload is observed as complete.
    pub is_final: bool,
}
