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

use aleph_alloc::BVec;
use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::{
    BindingSignatureCreateError, BindingSignatureDesc, IBindingSignature, PushConstantBlock,
};
use aleph_rhi_impl_utils::RhiSystem;

use crate::device::Device;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct BindingSignature {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) _parameter_block_layouts: BVec<AnyArc<ParameterBlockLayout>, RhiSystem>,
    pub(crate) push_constant_block: Option<PushConstantBlock>,
}

declare_interfaces!(BindingSignature, [IBindingSignature]);

impl IBindingSignature for BindingSignature {
    fn upgrade(&self) -> AnyArc<dyn IBindingSignature> {
        AnyArc::map::<dyn IBindingSignature, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.id
    }
}

impl BindingSignature {
    pub(crate) fn create(
        device: &Device,
        desc: &BindingSignatureDesc,
    ) -> Result<AnyArc<dyn IBindingSignature>, BindingSignatureCreateError> {
        let mut block_layouts =
            BVec::with_capacity_in(desc.parameter_block_layouts.len(), Default::default());
        block_layouts.extend(
            desc.parameter_block_layouts
                .iter()
                .map(unwrap::parameter_block_layout_d)
                .map(|v| v.this.upgrade().unwrap()),
        );

        let out = AnyArc::new_cyclic(move |v| BindingSignature {
            this: v.clone(),
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_binding_signature(),
            _parameter_block_layouts: block_layouts,
            push_constant_block: desc.push_constant_block.clone(),
        });
        Ok(AnyArc::map::<dyn IBindingSignature, _>(out, |v| v))
    }
}
