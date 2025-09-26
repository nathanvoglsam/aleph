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
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::owned_desc::OwnedParameterBlockDesc;
use objc2_metal::MTLRenderStages;

use crate::device::Device;
use crate::internal::unwrap;

pub struct ParameterBlockLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) compiled: CompiledParameterBlockLayout,
    pub(crate) desc: OwnedParameterBlockDesc,
}

declare_interfaces!(ParameterBlockLayout, [IParameterBlockLayout]);

impl IParameterBlockLayout for ParameterBlockLayout {
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
    ) -> Result<AnyArc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        let compiled = CompiledParameterBlockLayout::new(desc);
        let out = AnyArc::new_cyclic(move |v| ParameterBlockLayout {
            this: v.clone(),
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_parameter_block_layout(),
            compiled,
            desc: OwnedParameterBlockDesc::new(desc),
        });
        Ok(AnyArc::map::<dyn IParameterBlockLayout, _>(out, |v| v))
    }
}

/// Backend specific information derived from the [`ParameterBlockDesc`] used to create a parameter
/// block layout.
pub struct CompiledParameterBlockLayout {
    /// The shader stages the parameter block is accessible from.
    pub visibility: MTLRenderStages,

    /// The total number of arguments the block layout will consume to serve all parameters in the
    /// block.
    pub num_arguments: usize,

    /// Table that associates with 'params' in [`ParameterBlockDesc`]. Stores the base offset of
    /// the parameter into the read-only used resources set.
    pub use_read_bases: BVec<usize, RhiSystem>,

    /// The number of read-only parameters used by this block layout.
    pub num_reads: usize,

    /// Table that associates with 'params' in [`ParameterBlockDesc`]. Stores the base offset of
    /// the parameter into the writeable used resources set.
    pub use_write_bases: BVec<usize, RhiSystem>,

    /// The number of writeable parameters used by this block layout.
    pub num_writes: usize,
}

impl CompiledParameterBlockLayout {
    /// Create a new [`CompiledParameterBlockLayout`] derived from the given [`ParameterBlockDesc`].
    pub fn new(desc: &ParameterBlockDesc) -> Self {
        let visibility = match desc.visibility {
            // Map compute to this as a dummy value. If visibility is 'compute' then it's illegal
            // to bind any parameter blocks with this layout to the graphics pipeline. We don't read
            // this on the compute encoder paths so it can just be set to a dummy value.
            DescriptorShaderVisibility::All | DescriptorShaderVisibility::Compute => {
                MTLRenderStages::Vertex
                    | MTLRenderStages::Fragment
                    | MTLRenderStages::Object
                    | MTLRenderStages::Mesh
            }
            DescriptorShaderVisibility::Vertex => MTLRenderStages::Vertex,
            DescriptorShaderVisibility::Fragment => MTLRenderStages::Fragment,
            DescriptorShaderVisibility::Amplification => MTLRenderStages::Object,
            DescriptorShaderVisibility::Mesh => MTLRenderStages::Mesh,
            DescriptorShaderVisibility::Hull => unimplemented!(),
            DescriptorShaderVisibility::Domain => unimplemented!(),
            DescriptorShaderVisibility::Geometry => unimplemented!(),
        };

        let mut num_arguments = 0;
        let mut num_reads = 0;
        let mut num_writes = 0;
        let mut use_read_bases = BVec::new_in(RhiSystem::default());
        let mut use_write_bases = BVec::new_in(RhiSystem::default());

        for param in desc.params {
            num_arguments += param.array_size.count() as usize;

            // Take the current number of reads/writes as the offset into the respective arrays used
            // for hazard tracking.
            use_write_bases.push(num_writes);
            use_read_bases.push(num_reads);

            if param.ty == ParameterType::SamplerState {
                // Do nothing
            } else if param.ty.is_srv() || param.ty.is_constant_buffer() {
                num_reads += param.array_size.count() as usize;
            } else if param.ty.is_uav() {
                num_writes += param.array_size.count() as usize;
            } else {
                unreachable!();
            }
        }

        Self {
            visibility,
            num_arguments,
            use_read_bases,
            num_reads,
            use_write_bases,
            num_writes,
        }
    }
}
