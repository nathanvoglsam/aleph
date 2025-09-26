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

//!
//! This module holds a collection of internal utilities used for unwrapping interface structs that
//! contain references to interface objects.
//!

use std::cell::Cell;

use aleph_alloc::BVec;
use aleph_any::{QueryInterface, box_downcast};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;

use crate::internal::unwrap;
use crate::texture::ValidationImageView;
use crate::{
    ValidationBuffer, ValidationCommandList, ValidationFence, ValidationSampler,
    ValidationSemaphore, ValidationSwapImage, ValidationTexture,
};

pub fn buffer(buffer: &BufferHandle) -> &BufferHandle {
    &buffer
        .get()
        .downcast_ref::<ValidationBuffer>()
        .expect("Unknown Buffer Implementation")
        .inner
}

pub fn texture(texture: &TextureHandle) -> &TextureHandle {
    &texture
        .get()
        .downcast_ref::<ValidationTexture>()
        .expect("Unknown Texture Implementation")
        .inner
}

pub unsafe fn parameter_writes<'a>(writes: &[ParameterWrite<'a>]) -> Vec<ParameterWrite<'a>> {
    let mut new_writes = Vec::with_capacity(writes.len());

    for write in writes {
        let new_write = match write {
            ParameterWrite::Sampler(v) => {
                let inner = &ValidationSampler::get(v.sampler).inner;
                SamplerWrite::new(inner).into()
            }
            ParameterWrite::Texture(v) => {
                let inner = unsafe { ValidationImageView::get(&v.image_view) };
                let inner = inner.image_view;
                TextureWrite::new(inner, v.image_layout).into()
            }
            ParameterWrite::Buffer(v) => {
                let inner = &ValidationBuffer::get(&v.buffer).inner;
                BufferWrite {
                    buffer: inner,
                    offset: v.offset,
                    len: v.len,
                    structure_byte_stride: v.structure_byte_stride,
                }
                .into()
            }
            ParameterWrite::TextureBuffer(v) => {
                let inner = &ValidationBuffer::get(&v.buffer).inner;
                TextureBufferWrite {
                    buffer: inner,
                    format: v.format,
                    offset: v.offset,
                    len: v.len,
                }
                .into()
            }
        };
        new_writes.push(new_write);
    }

    new_writes
}

pub fn queue_submit_desc<Return>(
    desc: &QueueSubmitDesc,
    f: impl FnOnce(&QueueSubmitDesc) -> Return,
) -> Return {
    let mut command_lists = BVec::with_capacity_in(desc.command_lists.len(), RhiSystem::default());
    command_lists.extend(desc.command_lists.iter().map(|v| {
        let v = v.take().unwrap();
        let v = box_downcast::<_, ValidationCommandList>(v).ok().unwrap();
        let v = v.inner;
        Cell::new(Some(v))
    }));

    let mut wait_semaphores =
        BVec::with_capacity_in(desc.command_lists.len(), RhiSystem::default());
    wait_semaphores.extend(
        desc.wait_semaphores
            .iter()
            .copied()
            .map(ValidationSemaphore::get)
            .map(|v| &v.inner),
    );

    let mut signal_semaphores =
        BVec::with_capacity_in(desc.command_lists.len(), RhiSystem::default());
    signal_semaphores.extend(
        desc.signal_semaphores
            .iter()
            .copied()
            .map(ValidationSemaphore::get)
            .map(|v| &v.inner),
    );

    let fence = desc.fence.map(|v| &ValidationFence::get(v).inner);
    let swap_image = desc
        .swap_image
        .map(|v| {
            let v = v
                .query_interface::<ValidationSwapImage>()
                .expect("Unknown ISwapImage implementation");
            v.inner.as_deref()
        })
        .flatten();

    let new_desc = QueueSubmitDesc {
        command_lists: command_lists.as_slice(),
        wait_semaphores: wait_semaphores.as_slice(),
        signal_semaphores: signal_semaphores.as_slice(),
        fence,
        swap_image,
    };

    f(&new_desc)
}

pub fn input_assembly_buffer_binding<'a>(
    binding: &'a InputAssemblyBufferBinding<'a>,
) -> InputAssemblyBufferBinding<'a> {
    let buffer = buffer(binding.buffer);
    InputAssemblyBufferBinding {
        buffer,
        offset: binding.offset,
    }
}

pub fn rendering_color_attachment_info(
    info: &RenderingColorAttachmentInfo,
) -> RenderingColorAttachmentInfo {
    let image_view = unsafe {
        let image_view = ValidationImageView::get(&info.image_view);
        (*image_view).image_view
    };
    RenderingColorAttachmentInfo {
        image_view,
        image_layout: info.image_layout,
        load_op: info.load_op.clone(),
        store_op: info.store_op,
    }
}

pub fn rendering_depth_stencil_attachment_info(
    info: &RenderingDepthStencilAttachmentInfo,
) -> RenderingDepthStencilAttachmentInfo {
    let image_view = unsafe {
        let image_view = ValidationImageView::get(&info.image_view);
        (*image_view).image_view
    };
    RenderingDepthStencilAttachmentInfo {
        image_view,
        image_layout: info.image_layout,
        depth: info.depth.clone(),
        stencil: info.stencil.clone(),
    }
}

pub fn buffer_barrier<'a>(barrier: &'a BufferBarrier<'a>) -> BufferBarrier<'a> {
    let buffer = buffer(barrier.buffer.unwrap());

    BufferBarrier {
        buffer: Some(buffer),
        offset: barrier.offset,
        size: barrier.size,
        before_sync: barrier.before_sync,
        after_sync: barrier.after_sync,
        before_access: barrier.before_access,
        after_access: barrier.after_access,
        queue_transition: barrier.queue_transition,
    }
}

pub fn texture_barrier<'a>(barrier: &'a TextureBarrier<'a>) -> TextureBarrier<'a> {
    let texture = texture(barrier.texture.unwrap());

    TextureBarrier {
        texture: Some(texture),
        before_sync: barrier.before_sync,
        after_sync: barrier.after_sync,
        before_access: barrier.before_access,
        after_access: barrier.after_access,
        before_layout: barrier.before_layout,
        after_layout: barrier.after_layout,
        queue_transition: barrier.queue_transition,
        subresource_range: barrier.subresource_range.clone(),
    }
}

pub fn descriptor_pool_desc<'a>(desc: &'a DescriptorPoolDesc<'a>) -> DescriptorPoolDesc<'a> {
    let layout = unwrap::parameter_block_layout(desc.layout);
    DescriptorPoolDesc {
        layout: layout.inner.as_ref(),
        num_blocks: desc.num_blocks,
        name: desc.name,
    }
}
