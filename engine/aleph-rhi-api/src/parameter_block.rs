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

use std::ptr::NonNull;

use crate::*;

/// Opaque handle to an allocated parameter block.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ParameterBlockHandle(NonNull<()>);

impl ParameterBlockHandle {
    /// Unsafe utility function for constructing a new [`ParameterBlockHandle`] from a raw pointer.
    ///
    /// # Safety
    ///
    /// This technically doesn't cause immediate UB, the implementation is safe, but doing this
    /// outside of an RHI implementation is almost certainly incorrect. This function is marked as
    /// unsafe to discourage using it. There should be zero need to call this unless you're
    /// constructing handles from internal RHI types.
    ///
    /// This function exists to avoid using *actual* unsafe via [core::mem::transmute] to allow
    /// RHI implementations to construct this otherwise opaque type safely.
    pub unsafe fn from_raw(v: NonNull<()>) -> Self {
        ParameterBlockHandle(v)
    }

    /// The inverse of [`ParameterBlockHandle::from_Raw`].
    pub fn into_raw<U>(self) -> NonNull<U> {
        self.0.cast()
    }

    /// # Safety
    ///
    /// See [ParameterBlockHandle::from_raw]
    pub unsafe fn from_raw_int(v: u64) -> Option<Self> {
        NonNull::new(v as *mut ()).map(ParameterBlockHandle)
    }
}

unsafe impl Send for ParameterBlockHandle {}
unsafe impl Sync for ParameterBlockHandle {}

#[derive(Clone)]
pub enum ParameterWrite<'a> {
    Sampler(SamplerWrite<'a>),
    Texture(TextureWrite),
    Buffer(BufferWrite<'a>),
    TextureBuffer(TextureBufferWrite<'a>),
}

/// Describes the parameters of a descriptor to write when writing into a sampler binding.
#[derive(Clone)]
pub struct SamplerWrite<'a> {
    /// The sampler target.
    pub sampler: &'a SamplerHandle,
}

impl<'a> SamplerWrite<'a> {
    pub const fn new(sampler: &'a SamplerHandle) -> Self {
        Self { sampler }
    }
}

impl<'a> From<SamplerWrite<'a>> for ParameterWrite<'a> {
    #[inline(always)]
    fn from(val: SamplerWrite<'a>) -> Self {
        ParameterWrite::Sampler(val)
    }
}

/// Describes the parameters of a descriptor to write when writing into a texture binding.
#[derive(Clone, Hash, Debug)]
pub struct TextureWrite {
    /// The image target.
    pub image_view: ImageView,

    /// The layout of the image
    pub image_layout: ImageLayout,
}

impl TextureWrite {
    pub const fn new(image_view: ImageView, image_layout: ImageLayout) -> Self {
        Self {
            image_view,
            image_layout,
        }
    }

    pub const fn srv(image_view: ImageView) -> Self {
        Self {
            image_view,
            image_layout: ImageLayout::ShaderReadOnly,
        }
    }

    pub const fn uav(image_view: ImageView) -> Self {
        Self {
            image_view,
            image_layout: ImageLayout::UnorderedAccess,
        }
    }
}

impl From<TextureWrite> for ParameterWrite<'static> {
    #[inline(always)]
    fn from(val: TextureWrite) -> Self {
        ParameterWrite::Texture(val)
    }
}

/// Describes the parameters of a descriptor to write when writing into a simple buffer like
/// binding.
#[derive(Clone)]
pub struct BufferWrite<'a> {
    /// The buffer target
    pub buffer: &'a BufferHandle,

    /// The offset in bytes from the start of buffer. Access to buffer memory via this descriptor
    /// uses addressing that is relative to this starting offset.
    pub offset: u64,

    /// The size in bytes that is used for this descriptor update, or [u32::MAX] to use the range
    /// from offset to the end of the buffer.
    pub len: u32,

    /// The stride/size of an individual structure in the structured buffer, in bytes. This is only
    /// relevant for structured buffers. All other buffer types will ignore this field.
    pub structure_byte_stride: u32,
}

impl<'a> BufferWrite<'a> {
    pub const fn cbv(buffer: &'a BufferHandle, len: u32) -> Self {
        Self::cbv_offset(buffer, 0, len)
    }

    pub const fn cbv_offset(buffer: &'a BufferHandle, offset: u64, len: u32) -> Self {
        Self {
            buffer,
            offset,
            len,
            structure_byte_stride: 0,
        }
    }

    pub const fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }

    pub const fn with_len(mut self, len: u32) -> Self {
        self.len = len;
        self
    }

    pub const fn with_stride(mut self, stride: u32) -> Self {
        self.structure_byte_stride = stride;
        self
    }
}

impl<'a> From<BufferWrite<'a>> for ParameterWrite<'a> {
    #[inline(always)]
    fn from(val: BufferWrite<'a>) -> Self {
        ParameterWrite::Buffer(val)
    }
}

/// Describes the parameters of a descriptor to write when writing into a texel buffer binding.
#[derive(Clone)]
pub struct TextureBufferWrite<'a> {
    /// The buffer target
    pub buffer: &'a BufferHandle,

    /// The texel format the buffer should be interpreted as.
    pub format: Format,

    /// The offset in bytes from the start of buffer. Access to buffer memory via this descriptor
    /// uses addressing that is relative to this starting offset.
    pub offset: u64,

    /// The size in bytes that is used for this descriptor update, or [u32::MAX] to use the range
    /// from offset to the end of the buffer.
    pub len: u32,
}

impl<'a> From<TextureBufferWrite<'a>> for ParameterWrite<'a> {
    #[inline(always)]
    fn from(val: TextureBufferWrite<'a>) -> Self {
        ParameterWrite::TextureBuffer(val)
    }
}
