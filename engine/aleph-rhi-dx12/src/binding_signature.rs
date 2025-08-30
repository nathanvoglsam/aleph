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

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec as BVec;
use std::num::NonZeroU64;
use windows::Win32::Graphics::Direct3D12::*;

use crate::device::Device;
use crate::internal::conv::shader_visibility_to_dx12;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct BindingSignature {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) parameter_block_layouts: Vec<AnyArc<ParameterBlockLayout>>,
    pub(crate) root_signature: ID3D12RootSignature,
    pub(crate) compiled: CompiledBindingSignature,
}

declare_interfaces!(BindingSignature, [IBindingSignature]);
unsafe_impl_iobject!(BindingSignature, "01944fef-c9e8-7563-b77a-5bda76bb4330");

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
    /// Translates our [`BindingSignatureDesc`] into a [`D3D12_VERSIONED_ROOT_SIGNATURE_DESC`] that
    /// can be used to construct a [`ID3D12RootSignature`].
    ///
    /// # Warning
    ///
    /// The output 'D3D12_VERSIONED_ROOT_SIGNATURE_DESC' will contain pointers tied to the
    /// lifetime of the allocator parameter. Ensure that the allocator lives longer than the desc
    /// and you'll be fine.
    pub fn translate_root_signature_desc<A: Allocator + Copy>(
        layouts: &[AnyArc<ParameterBlockLayout>],
        compiled: &CompiledBindingSignature,
        allocator: A,
    ) -> D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        let mut parameters =
            BVec::with_capacity_in(compiled.root_parameter_count as usize, allocator);

        // Push the root constants as the first parameter. They're always the first.
        if let Some(block) = compiled.push_constant_block.as_ref() {
            parameters.push(root_constants(block));
        }

        for (block_i, layout) in layouts.iter().enumerate() {
            // We must encode our parameters differently depending on whether the layout has the
            // 'PUSH_DESCRIPTOR' flag set.
            let layout_flags = layout.desc.get().flags;
            if layout_flags.contains(ParameterBlockFlags::PUSH_DESCRIPTOR) {
                Self::push_resource_push_descriptors(&mut parameters, block_i, layout.as_ref());
            } else {
                Self::push_resource_descriptor_table(&mut parameters, block_i, layout.as_ref());
            }

            // Take a copy of the template descriptor ranges from the compiled block layout
            let sampler_templates = layout.compiled.samplers.ranges();
            let mut samplers = BVec::with_capacity_in(sampler_templates.len(), allocator);
            samplers.extend(sampler_templates.iter().cloned());
            let samplers = BVec::leak(samplers);

            // Patch the register space in the copied templates
            for sampler in samplers.iter_mut() {
                sampler.RegisterSpace = block_i as u32;
            }

            // Emit a single root table for each sampler in the set layout
            for sampler in samplers.iter() {
                let ranges = std::slice::from_ref(sampler);
                parameters.push(descriptor_table(layout.compiled.visibility, ranges));
            }
        }

        let parameters = BVec::leak(parameters);
        D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            Version: D3D_ROOT_SIGNATURE_VERSION_1_1,
            Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
                Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1 {
                    NumParameters: parameters.len() as _,
                    pParameters: parameters.as_ptr(),
                    NumStaticSamplers: 0,
                    pStaticSamplers: std::ptr::null(),
                    Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
                },
            },
        }
    }

    pub fn push_resource_descriptor_table<A: Allocator + Copy>(
        parameters: &mut BVec<D3D12_ROOT_PARAMETER1, A>,
        block_i: usize,
        layout: &ParameterBlockLayout,
    ) {
        // Copy the descriptor ranges template from the block layout
        let resource_templates = layout.compiled.resources.ranges();
        let mut resources =
            BVec::with_capacity_in(resource_templates.len(), parameters.allocator());
        resources.extend(resource_templates.iter().cloned());
        let resources = BVec::leak(resources);

        // Patch the register space in the copied templates
        for resource in resources.iter_mut() {
            resource.RegisterSpace = block_i as u32;
        }

        // Push a single descriptor table for all non-sampler resources
        parameters.push(descriptor_table(layout.compiled.visibility, resources));
    }

    pub fn push_resource_push_descriptors<A: Allocator + Copy>(
        parameters: &mut BVec<D3D12_ROOT_PARAMETER1, A>,
        block_i: usize,
        layout: &ParameterBlockLayout,
    ) {
        let resource_templates = layout.compiled.resources.descriptors();

        let iter = resource_templates.iter().map(|v| {
            root_descriptor(
                layout.compiled.visibility,
                v.parameter_type,
                v.register_offset,
                block_i as u32,
            )
        });
        parameters.extend(iter);
    }
}

pub struct CompiledBindingSignature {
    /// Table that associates with the index of each [`IParameterBlockLayout`] given in
    /// [`BindingSignatureDesc`] that is used to look up the base index in the root signature the
    /// block should be bound to.
    pub block_offsets: Vec<CompiledBlockOffset>,

    /// Table that is present when a push constant block was defined in [`BindingSignatureDesc`].
    /// Stores the root parameter index and the number of DWORDs the block consumes.
    pub push_constant_block: Option<CompiledPushConstantBlock>,

    /// The total number of root parameters consumed across all parameter blocks.
    pub root_parameter_count: u32,
}

impl CompiledBindingSignature {
    pub fn new(
        block_layouts: &[AnyArc<ParameterBlockLayout>],
        desc: &BindingSignatureDesc,
    ) -> Result<Self, BindingSignatureCreateError> {
        let mut num_dwords = 0;
        let mut num_parameters = 0;

        // Push constants always take the first block
        if desc.push_constant_block.is_some() {
            num_parameters += 1;
        }

        let mut block_offsets = Vec::with_capacity(block_layouts.len());
        for layout in block_layouts {
            num_parameters += layout.compiled.num_root_parameters;
            num_dwords += layout.compiled.num_dwords;
            block_offsets.push(CompiledBlockOffset {
                root_parameter_index: num_parameters,
            });
        }

        let push_constant_block = if let Some(block) = desc.push_constant_block.as_ref() {
            // Size must be a multiple of 4, as we can only allocate space at DWORD granularity.
            let size = block.size.get() as u32;
            if !size.is_multiple_of(4) {
                return Err(BindingSignatureCreateError::InvalidPushConstantBlockSize);
            }

            let size_dwords = size / 4;
            num_dwords += size_dwords;

            // Convert size in bytes to size in DWORDs. Always take the 0th parameter.
            Some(CompiledPushConstantBlock {
                visibility: shader_visibility_to_dx12(block.visibility),
                size: size_dwords,
                root_parameter_index: 0,
            })
        } else {
            None
        };

        assert!(num_dwords <= 64, "Too many DWORDs in root signature.");

        Ok(Self {
            block_offsets,
            push_constant_block,
            root_parameter_count: num_parameters,
        })
    }
}

pub struct CompiledBlockOffset {
    /// The parameter index of the parameter block within the root signature. This consumes 1 or
    /// more parameter slots, depending on the parameter block layout.
    pub root_parameter_index: u32,
}

/// Internal struct for caching information necessary for implementing command recording
pub struct CompiledPushConstantBlock {
    /// D3D12 form of [`DescriptorShaderVisibility`]. Which shader stages is the block visible to.
    pub visibility: D3D12_SHADER_VISIBILITY,

    /// The number of DWORDs consumed by the push constant block in the root signature.
    pub size: u32,

    /// The parameter index of the push constant block within the root signature. This consumes
    /// exactly one parameter slot.
    pub root_parameter_index: u32,
}

fn root_constants(block: &CompiledPushConstantBlock) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
        Anonymous: D3D12_ROOT_PARAMETER1_0 {
            Constants: D3D12_ROOT_CONSTANTS {
                ShaderRegister: 0,   // Always use b0 for push constants
                RegisterSpace: 1024, // A reserved space for push constants
                Num32BitValues: block.size,
            },
        },
        ShaderVisibility: block.visibility,
    }
}

fn root_descriptor(
    visibility: D3D12_SHADER_VISIBILITY,
    ty: D3D12_ROOT_PARAMETER_TYPE,
    register: u32,
    space: u32,
) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: ty,
        Anonymous: D3D12_ROOT_PARAMETER1_0 {
            Descriptor: D3D12_ROOT_DESCRIPTOR1 {
                ShaderRegister: register,
                RegisterSpace: space,
                Flags: Default::default(),
            },
        },
        ShaderVisibility: visibility,
    }
}

fn descriptor_table(
    visibility: D3D12_SHADER_VISIBILITY,
    ranges: &[D3D12_DESCRIPTOR_RANGE1],
) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
        Anonymous: D3D12_ROOT_PARAMETER1_0 {
            DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1 {
                NumDescriptorRanges: ranges.len() as u32,
                pDescriptorRanges: ranges.as_ptr(),
            },
        },
        ShaderVisibility: visibility,
    }
}
