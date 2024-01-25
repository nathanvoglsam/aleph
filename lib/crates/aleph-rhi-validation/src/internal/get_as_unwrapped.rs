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
//! For example, [PipelineLayoutDesc] contains a list of references to [IDescriptorSetLayout]
//! objects. An API user will have filled that array with [ValidationDescriptorSetLayout]
//! references, as that's what the validation layer gives out to the user, but the wrapped
//! implementation is expecting to get its own concrete implementation.
//!
//! At best this will cause the base implementation to panic, or it could cause UB and other
//! nastiness.
//!
//! The validation layer is *not* zero overhead (by definition). To solve this problem we make a
//! copy of any structs like [PipelineLayoutDesc] and unwrap all trait-object references inside
//! until we have the same struct a user would've made without the validation layer in the way. The
//! resulting struct is then given to the base API instead.
//!
//! This requires a big mess of partial clones, and a lot of heap allocations as we've got to
//! allocate new arrays for structs that take arrays of references.
//!

use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;

use aleph_any::{box_downcast, QueryInterface};
use aleph_rhi_api::*;

use crate::internal::descriptor_set::DescriptorSet;
use crate::texture::ValidationImageView;
use crate::{
    ValidationBuffer, ValidationCommandList, ValidationDescriptorSetLayout, ValidationFence,
    ValidationSampler, ValidationSemaphore, ValidationSwapChain, ValidationTexture,
};

pub fn buffer(buffer: &dyn IBuffer) -> &dyn IBuffer {
    buffer
        .query_interface::<ValidationBuffer>()
        .expect("Unknown IBuffer Implementation")
        .inner
        .deref()
}

pub fn texture(texture: &dyn ITexture) -> &dyn ITexture {
    texture
        .query_interface::<ValidationTexture>()
        .expect("Unknown ITexture Implementation")
        .inner
        .deref()
}

pub unsafe fn descriptor_set_handle(
    handle: &DescriptorSetHandle,
    expected_pool_id: Option<u64>,
) -> DescriptorSetHandle {
    // Unwrap and validate to get the inner DescriptorSetHandle
    DescriptorSet::validate(handle, expected_pool_id);
    let inner: NonNull<()> = handle.clone().into();
    let inner: NonNull<DescriptorSet> = inner.cast();
    inner.as_ref().inner.clone()
}

pub fn pipeline_layout_desc<Return>(
    desc: &PipelineLayoutDesc,
    f: impl FnOnce(&PipelineLayoutDesc) -> Return,
) -> Return {
    let mut set_layouts: Vec<&dyn IDescriptorSetLayout> = Vec::new();
    for v in desc.set_layouts {
        let v = v
            .query_interface::<ValidationDescriptorSetLayout>()
            .expect("Unknown IDescriptorSetLayout implementation");
        set_layouts.push(v.inner.as_ref());
    }

    let new_desc = PipelineLayoutDesc {
        set_layouts: &set_layouts,
        push_constant_blocks: desc.push_constant_blocks,
        name: desc.name,
    };

    f(&new_desc)
}

pub fn descriptor_set_updates<Return>(
    writes: &[DescriptorWriteDesc],
    f: impl FnOnce(&[DescriptorWriteDesc]) -> Return,
) -> Return {
    let mut new_descriptor_writes = Vec::new();
    for v in writes {
        let new_writes = descriptor_writes(&v.writes);
        new_descriptor_writes.push(new_writes);
    }

    let mut new_writes = Vec::new();
    for (i, v) in writes.iter().enumerate() {
        let set = unsafe { descriptor_set_handle(&v.set, None) };

        new_writes.push(DescriptorWriteDesc {
            set,
            binding: v.binding,
            array_element: v.array_element,
            writes: new_descriptor_writes[i].as_ref(),
        });
    }

    f(&new_writes)
}

pub fn queue_submit_desc<Return>(
    desc: &QueueSubmitDesc,
    f: impl FnOnce(&QueueSubmitDesc) -> Return,
) -> Return {
    let command_lists: Vec<_> = desc
        .command_lists
        .iter()
        .map(|v| {
            let v = v.take().unwrap();
            let v = box_downcast::<_, ValidationCommandList>(v).ok().unwrap();
            let v = v.inner;
            Cell::new(Some(v))
        })
        .collect();

    let wait_semaphores: Vec<_> = desc
        .wait_semaphores
        .iter()
        .map(|v| {
            v.query_interface::<ValidationSemaphore>()
                .expect("Unknown ISemaphore implementation")
                .inner
                .as_ref()
        })
        .collect();

    let signal_semaphores: Vec<_> = desc
        .signal_semaphores
        .iter()
        .map(|v| {
            v.query_interface::<ValidationSemaphore>()
                .expect("Unknown ISemaphore implementation")
                .inner
                .as_ref()
        })
        .collect();

    let fence = desc.fence.map(|v| {
        v.query_interface::<ValidationFence>()
            .expect("Unknown IFence Implementation")
            .inner
            .as_ref()
    });

    let new_desc = QueueSubmitDesc {
        command_lists: command_lists.as_slice(),
        wait_semaphores: wait_semaphores.as_slice(),
        signal_semaphores: signal_semaphores.as_slice(),
        fence,
    };

    f(&new_desc)
}

pub fn queue_present_desc<Return>(
    desc: &QueuePresentDesc,
    f: impl FnOnce(&QueuePresentDesc) -> Return,
) -> Return {
    let swap_chain = desc
        .swap_chain
        .query_interface::<ValidationSwapChain>()
        .expect("Unknown ISwapChain Implementation")
        .inner
        .as_ref();

    let wait_semaphores: Vec<_> = desc
        .wait_semaphores
        .iter()
        .map(|v| {
            v.query_interface::<ValidationSemaphore>()
                .expect("Unknown ISemaphore implementation")
                .inner
                .as_ref()
        })
        .collect();

    let new_desc = QueuePresentDesc {
        swap_chain,
        image_index: desc.image_index,
        wait_semaphores: &wait_semaphores,
    };

    f(&new_desc)
}

pub fn descriptor_writes<'a>(writes: &'a DescriptorWrites<'a>) -> OwnedDescriptorWrites<'a> {
    match writes {
        DescriptorWrites::Sampler(v) => {
            let writes: Vec<_> = v.iter().map(sampler_descriptor_write).collect();
            OwnedDescriptorWrites::Sampler(writes)
        }
        DescriptorWrites::TexelBuffer(v) => {
            let writes: Vec<_> = v.iter().map(texel_buffer_descriptor_write).collect();
            OwnedDescriptorWrites::TexelBuffer(writes)
        }
        DescriptorWrites::TexelBufferRW(v) => {
            let writes: Vec<_> = v.iter().map(texel_buffer_descriptor_write).collect();
            OwnedDescriptorWrites::TexelBufferRW(writes)
        }
        DescriptorWrites::Texture(v) => {
            let writes: Vec<_> = v.iter().map(image_descriptor_write).collect();
            OwnedDescriptorWrites::Texture(writes)
        }
        DescriptorWrites::TextureRW(v) => {
            let writes: Vec<_> = v.iter().map(image_descriptor_write).collect();
            OwnedDescriptorWrites::TextureRW(writes)
        }
        DescriptorWrites::UniformBuffer(v) => {
            let writes: Vec<_> = v.iter().map(buffer_descriptor_write).collect();
            OwnedDescriptorWrites::UniformBuffer(writes)
        }
        DescriptorWrites::StructuredBuffer(v) => {
            let writes: Vec<_> = v.iter().map(buffer_descriptor_write).collect();
            OwnedDescriptorWrites::StructuredBuffer(writes)
        }
        DescriptorWrites::StructuredBufferRW(v) => {
            let writes: Vec<_> = v.iter().map(buffer_descriptor_write).collect();
            OwnedDescriptorWrites::StructuredBufferRW(writes)
        }
        DescriptorWrites::ByteAddressBuffer(v) => {
            let writes: Vec<_> = v.iter().map(buffer_descriptor_write).collect();
            OwnedDescriptorWrites::ByteAddressBuffer(writes)
        }
        DescriptorWrites::ByteAddressBufferRW(v) => {
            let writes: Vec<_> = v.iter().map(buffer_descriptor_write).collect();
            OwnedDescriptorWrites::ByteAddressBufferRW(writes)
        }
        DescriptorWrites::InputAttachment(v) => {
            let writes: Vec<_> = v.iter().map(image_descriptor_write).collect();
            OwnedDescriptorWrites::InputAttachment(writes)
        }
    }
}

pub fn sampler_descriptor_write<'a>(
    write: &'a SamplerDescriptorWrite<'a>,
) -> SamplerDescriptorWrite<'a> {
    let sampler = write
        .sampler
        .query_interface::<ValidationSampler>()
        .expect("Unknown ISampler Implementation")
        .inner
        .deref();
    SamplerDescriptorWrite { sampler }
}

pub fn image_descriptor_write(write: &ImageDescriptorWrite) -> ImageDescriptorWrite {
    let image_view = unsafe {
        let image_view = std::mem::transmute::<_, *const ValidationImageView>(write.image_view);
        (&*image_view).image_view
    };
    ImageDescriptorWrite {
        image_view,
        image_layout: write.image_layout,
    }
}

pub fn buffer_descriptor_write<'a>(
    write: &'a BufferDescriptorWrite<'a>,
) -> BufferDescriptorWrite<'a> {
    let buffer = buffer(write.buffer);
    BufferDescriptorWrite {
        buffer,
        offset: write.offset,
        len: write.len,
        structure_byte_stride: write.structure_byte_stride,
    }
}

pub fn texel_buffer_descriptor_write<'a>(
    write: &'a TexelBufferDescriptorWrite<'a>,
) -> TexelBufferDescriptorWrite<'a> {
    let buffer = buffer(write.buffer);
    TexelBufferDescriptorWrite {
        buffer,
        format: write.format,
        offset: write.offset,
        len: write.len,
    }
}

pub enum OwnedDescriptorWrites<'a> {
    Sampler(Vec<SamplerDescriptorWrite<'a>>),
    TexelBuffer(Vec<TexelBufferDescriptorWrite<'a>>),
    TexelBufferRW(Vec<TexelBufferDescriptorWrite<'a>>),
    Texture(Vec<ImageDescriptorWrite>),
    TextureRW(Vec<ImageDescriptorWrite>),
    UniformBuffer(Vec<BufferDescriptorWrite<'a>>),
    StructuredBuffer(Vec<BufferDescriptorWrite<'a>>),
    StructuredBufferRW(Vec<BufferDescriptorWrite<'a>>),
    ByteAddressBuffer(Vec<BufferDescriptorWrite<'a>>),
    ByteAddressBufferRW(Vec<BufferDescriptorWrite<'a>>),
    InputAttachment(Vec<ImageDescriptorWrite>),
}

impl<'a> OwnedDescriptorWrites<'a> {
    pub fn as_ref(&self) -> DescriptorWrites {
        match self {
            Self::Sampler(v) => DescriptorWrites::Sampler(v.as_slice()),
            Self::TexelBuffer(v) => DescriptorWrites::TexelBuffer(v.as_slice()),
            Self::TexelBufferRW(v) => DescriptorWrites::TexelBufferRW(v.as_slice()),
            Self::Texture(v) => DescriptorWrites::Texture(v.as_slice()),
            Self::TextureRW(v) => DescriptorWrites::TextureRW(v.as_slice()),
            Self::UniformBuffer(v) => DescriptorWrites::UniformBuffer(v.as_slice()),
            Self::StructuredBuffer(v) => DescriptorWrites::StructuredBuffer(v.as_slice()),
            Self::StructuredBufferRW(v) => DescriptorWrites::StructuredBufferRW(v.as_slice()),
            Self::ByteAddressBuffer(v) => DescriptorWrites::ByteAddressBuffer(v.as_slice()),
            Self::ByteAddressBufferRW(v) => DescriptorWrites::ByteAddressBufferRW(v.as_slice()),
            Self::InputAttachment(v) => DescriptorWrites::InputAttachment(v.as_slice()),
        }
    }
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
        let image_view = std::mem::transmute::<_, *const ValidationImageView>(info.image_view);
        (&*image_view).image_view
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
        let image_view = std::mem::transmute::<_, *const ValidationImageView>(info.image_view);
        (&*image_view).image_view
    };
    RenderingDepthStencilAttachmentInfo {
        image_view,
        image_layout: info.image_layout,
        depth_load_op: info.depth_load_op.clone(),
        depth_store_op: info.depth_store_op,
        stencil_load_op: info.stencil_load_op.clone(),
        stencil_store_op: info.stencil_store_op,
    }
}

pub fn buffer_barrier<'a>(barrier: &'a BufferBarrier<'a>) -> BufferBarrier<'a> {
    let buffer = buffer(barrier.buffer);

    BufferBarrier {
        buffer,
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
    let texture = texture(barrier.texture);

    TextureBarrier {
        texture,
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
    let layout = desc
        .layout
        .query_interface::<ValidationDescriptorSetLayout>()
        .expect("Unknown IDescriptorSetLayout implementation");
    DescriptorPoolDesc {
        layout: layout.inner.as_ref(),
        num_sets: desc.num_sets,
        name: desc.name,
    }
}
