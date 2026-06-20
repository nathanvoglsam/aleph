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
use std::sync::{Arc, Weak};

use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedParameterBlockDesc;

use crate::device::Device;
use crate::internal::unwrap;

pub struct ParameterBlockLayout {
    pub(crate) this: Weak<Self>,
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) compiled: CompiledParameterBlockLayout,
    pub(crate) desc: OwnedParameterBlockDesc,
}

impl IParameterBlockLayout for ParameterBlockLayout {
    fn upgrade(&self) -> Arc<dyn IParameterBlockLayout> {
        self.this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &ParameterBlockDesc<'_> {
        self.desc.get()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    fn is_compatible(&self, other: &dyn IParameterBlockLayout) -> bool {
        let other = unwrap::parameter_block_layout(other);
        self.desc.get().is_compatible(other.desc.get())
    }
}

impl ParameterBlockLayout {
    pub(crate) fn create(
        device: &Device,
        desc: &ParameterBlockDesc,
    ) -> Result<Arc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        let compiled = CompiledParameterBlockLayout::new(desc);
        let out = Arc::new_cyclic(move |v| ParameterBlockLayout {
            this: v.clone(),
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_parameter_block_layout(),
            compiled,
            desc: OwnedParameterBlockDesc::new(desc),
        });
        Ok(out)
    }
}

/// Backend specific information derived from the [`ParameterBlockDesc`] used to create a parameter
/// block layout.
pub struct CompiledParameterBlockLayout {
    /// The total number of arguments the block layout will consume to serve all parameters in the
    /// block.
    pub num_arguments: usize,
}

impl CompiledParameterBlockLayout {
    /// Create a new [`CompiledParameterBlockLayout`] derived from the given [`ParameterBlockDesc`].
    pub fn new(desc: &ParameterBlockDesc) -> Self {
        let mut num_arguments = 0;
        for param in desc.params {
            num_arguments += param.array_size.count() as usize;
        }

        Self { num_arguments }
    }
}
