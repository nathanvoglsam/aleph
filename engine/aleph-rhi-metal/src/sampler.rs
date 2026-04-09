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
use std::sync::Arc;

use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedSamplerDesc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::device::Device;
use crate::internal::conv;

pub struct Sampler {
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) desc: OwnedSamplerDesc,
    pub(crate) objects: SamplerObjects,
}

unsafe_impl_iobject!(Sampler, "01980753-5c4f-7ae3-be3b-97225f3e91be");

impl Sampler {
    pub(crate) fn create(
        device: &Device,
        desc: &SamplerDesc,
    ) -> Result<SamplerHandle, SamplerCreateError> {
        let mtl_desc = MTLSamplerDescriptor::new();

        mtl_desc.setMinFilter(conv::sampler_filter_to_mtl(desc.min_filter));
        mtl_desc.setMagFilter(conv::sampler_filter_to_mtl(desc.mag_filter));
        mtl_desc.setMipFilter(conv::sampler_mip_filter_to_mtl(desc.mip_filter));

        mtl_desc.setRAddressMode(conv::address_mode_to_mtl(desc.address_mode_u));
        mtl_desc.setSAddressMode(conv::address_mode_to_mtl(desc.address_mode_v));
        mtl_desc.setTAddressMode(conv::address_mode_to_mtl(desc.address_mode_w));

        mtl_desc.setLodMinClamp(desc.min_lod);
        mtl_desc.setLodMaxClamp(desc.max_lod);
        // TODO: LOD BIAS?

        if desc.enable_anisotropy {
            mtl_desc.setMaxAnisotropy(desc.max_anisotropy as usize);
        }

        if let Some(op) = desc.compare_op {
            mtl_desc.setCompareFunction(conv::compare_op_to_mtl(op));
        }

        mtl_desc.setBorderColor(conv::border_color_to_mtl(desc.border_color));
        mtl_desc.setSupportArgumentBuffers(true);

        if let Some(name) = desc.name
            && device.context.debug
        {
            let mtl_name = NSString::from_str(name);
            mtl_desc.setLabel(Some(&mtl_name));
        }

        let sampler = match device.device.newSamplerStateWithDescriptor(&mtl_desc) {
            Some(v) => v,
            None => {
                log::error!("Failed to construct 'MTLSamplerState'.");
                panic!("Failed to construct 'MTLSamplerState'.");
            }
        };

        let out = Sampler {
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_sampler(),
            desc: OwnedSamplerDesc::new(desc.clone()),
            objects: SamplerObjects { sampler },
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(SamplerHandle::new(out)) }
    }

    pub(crate) fn get(v: &SamplerHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Sampler implementation!")
    }

    pub(crate) const fn desc(&self) -> &SamplerDesc<'_> {
        self.desc.get()
    }
}

/// Wrapper to scope our 'unsafe impl Send'
pub struct SamplerObjects {
    pub sampler: Retained<ProtocolObject<dyn MTLSamplerState>>,
}

unsafe impl Send for SamplerObjects {}
unsafe impl Sync for SamplerObjects {}
