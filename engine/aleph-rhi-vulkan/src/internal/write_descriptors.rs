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

use std::hint::unreachable_unchecked;

use aleph_alloc::alloc::Allocator;
use aleph_alloc::vec::Vec as BVec;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_layout_visitor::ParameterBlockLayoutVisitor;
use ash::vk;

use crate::buffer::Buffer;
use crate::internal::conv::*;
use crate::sampler::Sampler;

pub fn translate_descriptor_writes<'a, A: Allocator + Copy + 'a>(
    layout_desc: &ParameterBlockDesc,
    base: u32,
    writes: &[ParameterWrite],
    set: vk::DescriptorSet,
    a: A,
) -> BVec<vk::WriteDescriptorSet<'a>, A> {
    let mut descriptor_writes = BVec::with_capacity_in(writes.len(), a);

    let visitor = ParameterBlockLayoutVisitor::new(layout_desc, base as u64, writes).unwrap();
    for v in visitor {
        let mut new_write = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(v.binding as u32)
            .dst_array_element(v.element as u32)
            .descriptor_type(parameter_type_to_vk(&v.ty));

        match v.ty {
            ParameterType::ConstantBuffer
            | ParameterType::StructuredBuffer
            | ParameterType::RWStructuredBuffer
            | ParameterType::ByteAddressBuffer
            | ParameterType::RWByteAddressBuffer => {
                let translator = v.writes.iter().map(|v| unsafe { buffer_write_mapper(v) });
                let mut buffer_infos = BVec::new_in(a);
                buffer_infos.extend(translator);
                let buffer_infos = BVec::leak(buffer_infos);
                new_write = new_write.buffer_info(buffer_infos);
            }
            ParameterType::Buffer | ParameterType::RWBuffer => todo!(),
            ParameterType::Texture1D
            | ParameterType::RWTexture1D
            | ParameterType::Texture2D
            | ParameterType::RWTexture2D
            | ParameterType::Texture3D
            | ParameterType::RWTexture3D
            | ParameterType::Texture1DArray
            | ParameterType::RWTexture1DArray
            | ParameterType::Texture2DArray
            | ParameterType::RWTexture2DArray
            | ParameterType::Texture3DArray
            | ParameterType::RWTexture3DArray
            | ParameterType::Texture2DMS
            | ParameterType::RWTexture2DMS
            | ParameterType::Texture2DMSArray
            | ParameterType::RWTexture2DMSArray
            | ParameterType::TextureCube
            | ParameterType::TextureCubeArray => {
                let translator = v.writes.iter().map(|v| unsafe { image_write_mapper(v) });
                let mut image_infos = BVec::new_in(a);
                image_infos.extend(translator);
                let image_infos = BVec::leak(image_infos);
                new_write = new_write.image_info(image_infos);
            }
            ParameterType::SamplerState => {
                let translator = v.writes.iter().map(|v| unsafe { sampler_write_mapper(v) });
                let mut image_infos = BVec::new_in(a);
                image_infos.extend(translator);
                let image_infos = BVec::leak(image_infos);
                new_write = new_write.image_info(image_infos)
            }
            ParameterType::AccelerationStructure => todo!(),
        }

        // Add the translated write to the vulkan description
        descriptor_writes.push(new_write);
    }

    descriptor_writes
}

unsafe fn buffer_write_mapper(v: &ParameterWrite) -> vk::DescriptorBufferInfo {
    if let ParameterWrite::Buffer(v) = v {
        let buffer = v.buffer.get().downcast_ref::<Buffer>().unwrap();
        let len = buffer.clamp_max_size_for_view(v.len);
        vk::DescriptorBufferInfo::default()
            .buffer(buffer.buffer)
            .offset(v.offset)
            .range(len)
    } else {
        unsafe { unreachable_unchecked() }
    }
}
unsafe fn image_write_mapper(v: &ParameterWrite) -> vk::DescriptorImageInfo {
    if let ParameterWrite::Texture(v) = v {
        vk::DescriptorImageInfo::default()
            .image_view(unsafe { std::mem::transmute(v.image_view) })
            .image_layout(image_layout_to_vk(v.image_layout))
    } else {
        unsafe { unreachable_unchecked() }
    }
}
unsafe fn sampler_write_mapper(v: &ParameterWrite) -> vk::DescriptorImageInfo {
    if let ParameterWrite::Sampler(v) = v {
        vk::DescriptorImageInfo::default().sampler(Sampler::get(v.sampler).sampler)
    } else {
        unsafe { unreachable_unchecked() }
    }
}
