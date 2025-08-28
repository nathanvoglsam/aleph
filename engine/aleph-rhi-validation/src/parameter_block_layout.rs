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

use std::num::NonZeroU64;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_layout_visitor::ParameterBlockLayoutVisitor;

use crate::ValidationBuffer;
use crate::device::ValidationDevice;
use crate::internal::unwrap;
use crate::texture::ValidationImageView;

pub struct ValidationParameterBlockLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IParameterBlockLayout>,
}

declare_interfaces!(ValidationParameterBlockLayout, [IParameterBlockLayout]);

impl IParameterBlockLayout for ValidationParameterBlockLayout {
    fn upgrade(&self) -> AnyArc<dyn IParameterBlockLayout> {
        AnyArc::map::<dyn IParameterBlockLayout, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &ParameterBlockDesc<'_> {
        self.inner.desc()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.inner.get_id()
    }

    fn is_compatible(&self, other: &dyn IParameterBlockLayout) -> bool {
        let other = unwrap::parameter_block_layout(other);
        self.inner.is_compatible(other)
    }
}

impl ValidationParameterBlockLayout {
    pub fn validate_updates(&self, base: u32, writes: &[ParameterWrite]) {
        let desc = self.desc();

        let visitor = ParameterBlockLayoutVisitor::new(desc, base as u64, writes).unwrap();
        for write_block in visitor {
            let p_base = write_block.index as usize;
            let p_end = p_base + write_block.writes.len();
            let params = &desc.params[p_base..p_end];

            for (i, (write, param)) in write_block.writes.iter().zip(params.iter()).enumerate() {
                let param_i = p_base + i;
                match write {
                    ParameterWrite::Sampler(_v) => {
                        assert!(
                            param.ty.is_sampler(),
                            "Invalid to write 'Sampler' to parameter[{}] of type '{}'",
                            param_i,
                            param.ty
                        );
                    }
                    ParameterWrite::Texture(v) => {
                        assert!(
                            param.ty.is_texture(),
                            "Invalid to write 'Texture' to parameter[{}] of type '{}'",
                            param_i,
                            param.ty
                        );
                        let view = unsafe { ValidationImageView::get(&v.image_view) };
                        let texture = view._image.upgrade().unwrap();
                        let usage = texture.desc.usage;
                        let resource_name = texture.desc.name.unwrap_or("<unnamed resource>");
                        validate_resource_usage(param, usage, "Texture", resource_name);
                    }
                    ParameterWrite::Buffer(v) => {
                        assert!(
                            param.ty.is_buffer(),
                            "Invalid to write 'Buffer' to parameter[{}] of type '{}'",
                            param_i,
                            param.ty
                        );
                        let buffer = ValidationBuffer::get(v.buffer);
                        let resource_name = buffer.name.as_deref().unwrap_or("<unnamed resource>");
                        validate_resource_usage(param, buffer.usage, "Buffer", resource_name);
                    }
                    ParameterWrite::TextureBuffer(v) => {
                        assert!(
                            param.ty.is_texture_buffer(),
                            "Invalid to write 'Texture Buffer' to parameter[{}] of type '{}'",
                            param_i,
                            param.ty
                        );
                        let buffer = ValidationBuffer::get(v.buffer);
                        let resource_name = buffer.name.as_deref().unwrap_or("<unnamed resource>");
                        validate_resource_usage(param, buffer.usage, "Buffer", resource_name);
                    }
                }
            }
        }
    }
}

fn validate_resource_usage(
    param: &ParameterDesc,
    usage: ResourceUsageFlags,
    resource_type: &'static str,
    resource_name: &str,
) {
    if param.ty.is_uav() {
        assert!(
            usage.contains(ResourceUsageFlags::UNORDERED_ACCESS),
            "{} '{}' must support 'UNORDERED_ACCESS' to be used in parameter type '{}'. Only supports {:?}",
            resource_type,
            resource_name,
            param.ty,
            usage,
        );
    } else if param.ty.is_srv() {
        assert!(
            usage.contains(ResourceUsageFlags::SHADER_RESOURCE),
            "{} '{}' must support 'SHADER_RESOURCE' to be used in parameter type '{}'. Only supports {:?}",
            resource_type,
            resource_name,
            param.ty,
            usage,
        );
    } else if param.ty.is_constant_buffer() {
        assert!(
            usage.contains(ResourceUsageFlags::CONSTANT_BUFFER),
            "{} '{}' must support 'CONSTANT_BUFFER' to be used in parameter type '{}'. Only supports {:?}",
            resource_type,
            resource_name,
            param.ty,
            usage,
        );
    } else {
        unreachable!()
    }
}
