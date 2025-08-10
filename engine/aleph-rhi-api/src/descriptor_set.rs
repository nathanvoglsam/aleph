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

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DescriptorSetHandle(NonNull<()>);

impl DescriptorSetHandle {
    /// Unsafe utility function for constructing a new [DescriptorSetHandle] from a raw pointer.
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
        DescriptorSetHandle(v)
    }

    /// # Safety
    ///
    /// See [DescriptorSetHandle::from_raw]
    pub unsafe fn from_raw_int(v: u64) -> Option<Self> {
        NonNull::new(v as *mut ()).map(DescriptorSetHandle)
    }
}

impl From<DescriptorSetHandle> for NonNull<()> {
    fn from(val: DescriptorSetHandle) -> Self {
        val.0
    }
}

unsafe impl Send for DescriptorSetHandle {}
unsafe impl Sync for DescriptorSetHandle {}

/// A description of a descriptor write. Specifies the target descriptor set, binding index and
/// array element. Then specifies the type of, number of, and target of the descriptors to write.
#[derive(Clone)]
pub struct DescriptorWriteDesc<'a> {
    /// The descriptor set that will be the target of this write operation.
    pub set: DescriptorSetHandle,

    /// The descriptor binding index that will be the target of the write operation.
    pub binding: u32,

    /// The array element in the binding to write. Ignored for non-array bindings.
    pub array_element: u32,

    /// The list of descriptor writes to perform. The variant to use depends on `descriptor_type`.
    pub writes: DescriptorWrites<'a>,
}

macro_rules! descriptor_write_wrappers {
    ($fn_name: ident, $fn_at_name: ident, $dw_type: ident, $inner_type: ident, $lt: lifetime) => {
        pub const fn $fn_name(
            set: DescriptorSetHandle,
            binding: u32,
            v: &$lt $inner_type<$lt>,
        ) -> Self {
            Self::$fn_at_name(set, binding, 0, v)
        }

        pub const fn $fn_at_name(
            set: DescriptorSetHandle,
            binding: u32,
            array_element: u32,
            v: &$lt $inner_type<$lt>,
        ) -> Self {
            let writes = DescriptorWrites::$dw_type(::std::slice::from_ref(v));
            Self::new_at(set, binding, array_element, writes)
        }
    };
}

macro_rules! descriptor_write_wrappers_no_lt {
    ($fn_name: ident, $fn_at_name: ident, $dw_type: ident, $inner_type: ident, $lt: lifetime) => {
        pub const fn $fn_name(
            set: DescriptorSetHandle,
            binding: u32,
            v: &$lt $inner_type,
        ) -> Self {
            Self::$fn_at_name(set, binding, 0, v)
        }

        pub const fn $fn_at_name(
            set: DescriptorSetHandle,
            binding: u32,
            array_element: u32,
            v: &$lt $inner_type,
        ) -> Self {
            let writes = DescriptorWrites::$dw_type(::std::slice::from_ref(v));
            Self::new_at(set, binding, array_element, writes)
        }
    };
}

impl<'a> DescriptorWriteDesc<'a> {
    pub const fn new_at(
        set: DescriptorSetHandle,
        binding: u32,
        array_element: u32,
        writes: DescriptorWrites<'a>,
    ) -> Self {
        Self {
            set,
            binding,
            array_element,
            writes,
        }
    }

    pub const fn sampler(
        set: DescriptorSetHandle,
        binding: u32,
        v: &'a SamplerDescriptorWrite<'a>,
    ) -> Self {
        let writes = DescriptorWrites::Sampler(std::slice::from_ref(v));
        Self::new_at(set, binding, 0, writes)
    }

    descriptor_write_wrappers!(texel_buffer, texel_buffer_at, TexelBuffer, TexelBufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(texel_buffer_rw, texel_buffer_rw_at, TexelBufferRW, TexelBufferDescriptorWrite, 'a);
    descriptor_write_wrappers_no_lt!(texture, texture_at, Texture, ImageDescriptorWrite, 'a);
    descriptor_write_wrappers_no_lt!(texture_rw, texture_rw_at, TextureRW, ImageDescriptorWrite, 'a);
    descriptor_write_wrappers!(uniform_buffer, uniform_buffer_at, UniformBuffer, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(uniform_buffer_dynamic, uniform_buffer_dynamic_at, UniformBufferDynamic, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(structured_buffer, structured_buffer_at, StructuredBuffer, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(structured_buffer_rw, structured_buffer_rw_at, StructuredBufferRW, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(byte_address_buffer, byte_address_buffer_at, ByteAddressBuffer, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers!(byte_address_buffer_rw, byte_address_buffer_rw_at, ByteAddressBufferRW, BufferDescriptorWrite, 'a);
    descriptor_write_wrappers_no_lt!(input_attachment, input_attachment_at, InputAttachment, ImageDescriptorWrite, 'a);
}

/// The set of descriptor write types.
///
/// Each descriptor type needs different pieces of information in order to construct or write the
/// descriptors into the device-visible set memory. Each variant of this enum covers some of the
/// types in [DescriptorType].
#[derive(Clone)]
pub enum DescriptorWrites<'a> {
    Sampler(&'a [SamplerDescriptorWrite<'a>]),
    TexelBuffer(&'a [TexelBufferDescriptorWrite<'a>]),
    TexelBufferRW(&'a [TexelBufferDescriptorWrite<'a>]),
    Texture(&'a [ImageDescriptorWrite]),
    TextureRW(&'a [ImageDescriptorWrite]),
    UniformBuffer(&'a [BufferDescriptorWrite<'a>]),
    UniformBufferDynamic(&'a [BufferDescriptorWrite<'a>]),
    StructuredBuffer(&'a [BufferDescriptorWrite<'a>]),
    StructuredBufferRW(&'a [BufferDescriptorWrite<'a>]),
    ByteAddressBuffer(&'a [BufferDescriptorWrite<'a>]),
    ByteAddressBufferRW(&'a [BufferDescriptorWrite<'a>]),
    InputAttachment(&'a [ImageDescriptorWrite]),
}

impl<'a> DescriptorWrites<'a> {
    pub const fn descriptor_type(&self) -> DescriptorType {
        match self {
            DescriptorWrites::Sampler(_) => DescriptorType::Sampler,
            DescriptorWrites::TexelBuffer(_) => DescriptorType::TexelBuffer,
            DescriptorWrites::TexelBufferRW(_) => DescriptorType::TexelBufferRW,
            DescriptorWrites::Texture(_) => DescriptorType::Texture,
            DescriptorWrites::TextureRW(_) => DescriptorType::TextureRW,
            DescriptorWrites::UniformBuffer(_) => DescriptorType::UniformBuffer,
            DescriptorWrites::UniformBufferDynamic(_) => DescriptorType::UniformBufferDynamic,
            DescriptorWrites::StructuredBuffer(_) => DescriptorType::StructuredBuffer,
            DescriptorWrites::StructuredBufferRW(_) => DescriptorType::StructuredBufferRW,
            DescriptorWrites::ByteAddressBuffer(_) => DescriptorType::ByteAddressBuffer,
            DescriptorWrites::ByteAddressBufferRW(_) => DescriptorType::ByteAddressBufferRW,
            DescriptorWrites::InputAttachment(_) => DescriptorType::InputAttachment,
        }
    }

    /// Returns true if the array stored on the active variant of `self` is empty, that is: when
    /// `self.len() == 0`.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of array elements are contained in the array stored on the active variant
    /// of `self`.
    pub const fn len(&self) -> usize {
        match self {
            Self::Sampler(v) => v.len(),
            Self::TexelBuffer(v) => v.len(),
            Self::TexelBufferRW(v) => v.len(),
            Self::Texture(v) => v.len(),
            Self::TextureRW(v) => v.len(),
            Self::UniformBuffer(v) => v.len(),
            Self::UniformBufferDynamic(v) => v.len(),
            Self::StructuredBuffer(v) => v.len(),
            Self::StructuredBufferRW(v) => v.len(),
            Self::ByteAddressBuffer(v) => v.len(),
            Self::ByteAddressBufferRW(v) => v.len(),
            Self::InputAttachment(v) => v.len(),
        }
    }
}

/// Describes the parameters of a descriptor to write when writing into a sampler binding.
#[derive(Clone)]
pub struct SamplerDescriptorWrite<'a> {
    /// The sampler target.
    pub sampler: &'a SamplerHandle,
}

impl<'a> SamplerDescriptorWrite<'a> {
    pub const fn new(sampler: &'a SamplerHandle) -> Self {
        Self { sampler }
    }
}

/// Describes the parameters of a descriptor to write when writing into a texture binding.
#[derive(Clone, Hash, Debug)]
pub struct ImageDescriptorWrite {
    /// The image target.
    pub image_view: ImageView,

    /// The layout of the image
    pub image_layout: ImageLayout,
}

impl ImageDescriptorWrite {
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

/// Describes the parameters of a descriptor to write when writing into a simple buffer like
/// binding.
#[derive(Clone)]
pub struct BufferDescriptorWrite<'a> {
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

impl<'a> BufferDescriptorWrite<'a> {
    pub const fn uniform_buffer(buffer: &'a BufferHandle, len: u32) -> Self {
        Self::uniform_buffer_offset(buffer, 0, len)
    }

    pub const fn uniform_buffer_offset(buffer: &'a BufferHandle, offset: u64, len: u32) -> Self {
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

/// Describes the parameters of a descriptor to write when writing into a texel buffer binding.
#[derive(Clone)]
pub struct TexelBufferDescriptorWrite<'a> {
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
