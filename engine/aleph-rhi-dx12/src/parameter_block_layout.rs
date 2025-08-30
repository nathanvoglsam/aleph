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

//! D3D12 has no "descriptor set layout" like object, it only has the root signature which is
//! similar to the VkPipelineLayout object.
//!
//! We fake a 'VkDescriptorSetLayout' like object by just copying the input to the
//! create_descriptor_set_layout call so we can collect and use it when we create the root
//! signature.

use std::num::NonZeroU64;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedParameterBlockDesc;
use windows::Win32::Graphics::Direct3D12::*;

use crate::device::Device;
use crate::internal::{conv, unwrap};

pub struct ParameterBlockLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) desc: OwnedParameterBlockDesc,
    pub(crate) compiled: CompiledParameterBlockLayout,
}

declare_interfaces!(ParameterBlockLayout, [IParameterBlockLayout]);
unsafe_impl_iobject!(ParameterBlockLayout, "01944fed-ed20-7631-ab3e-c6683ac06428");

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

/// Internal struct, encapsulates data compiled from [`ParameterBlockDesc`] that D3D12 needs in
/// order to implement the parameter block abstraction.
///
/// This includes pre-computed parts of the root signature description as well as lookup tables for
/// mapping parameter indices to the correct place in the root signature.
pub struct CompiledParameterBlockLayout {
    /// D3D12 form of [`DescriptorShaderVisibility`]. Which shader stages is the block visible to.
    pub visibility: D3D12_SHADER_VISIBILITY,

    /// Table that associates with 'params' in [`ParameterBlockDesc`] that is used for looking up
    /// descriptor table and root parameter offsets for all parameters in the block layout.
    pub mapping: CompiledParameterMappingInfo,

    /// Pre-computed information about the resource segment of the parameter block. Binding
    /// signature creation will use the table array as template, patching the register space, when
    /// constructing the [`ID3D12RootSignature`].
    pub resources: CompiledResourceLayout,

    /// Pre-copmuted information about the sampler segment of the parameter block. Used in much the
    /// same way as 'resource'.
    pub samplers: CompiledSamplerLayout,

    /// The total number of root parameters consumed by this parameter block as laid out into a
    /// root signature. This number varies based on both the number of parameters in the block
    /// layout and which flags are set.
    pub num_root_parameters: u32,

    /// The total number of DWORDs this block layout consumes in a root signature for all its root
    /// parameters.
    pub num_dwords: u32,
}

impl CompiledParameterBlockLayout {
    /// Create a new [`CompiledParameterBlockLayout`] derived from the given [`ParameterBlockDesc`].
    pub fn new(desc: &ParameterBlockDesc) -> Self {
        if desc.flags.contains(ParameterBlockFlags::PUSH_DESCRIPTOR) {
            Self::new_for_push_descriptor(desc)
        } else {
            Self::new_default(desc)
        }
    }

    /// If [`ParameterBlockFlags::PUSH_DESCRIPTOR`] is _not_ set
    fn new_default(desc: &ParameterBlockDesc) -> Self {
        let mut num_registers_table = [0; 4];
        let mut mapping = CompiledParameterMappingInfo::default();
        let mut resources = CompiledResourceLayout::default();
        let mut samplers = CompiledSamplerLayout::default();
        for param in desc.params {
            let range_type = conv::parameter_type_to_range_dx12(param.ty);
            let num = param.array_size.count() as u32;

            let register_offset = num_registers_table[range_type.0 as usize];
            num_registers_table[range_type.0 as usize] += num;

            match param.ty {
                ParameterType::SamplerState => {
                    assert_eq!(num, 1, "We don't support sampler arrays!");

                    let offset = samplers.num_samplers();
                    samplers.sampler_num += num;

                    let info = CompiledParameterInfo {
                        register_offset,
                        storage_offset: offset,
                    };
                    let template = Self::sampler_descriptor_range_template(range_type, num, &info);

                    samplers.sampler_tables.push(template);
                    mapping.params.push(info);
                }
                ParameterType::ConstantBuffer
                | ParameterType::StructuredBuffer
                | ParameterType::RWStructuredBuffer
                | ParameterType::ByteAddressBuffer
                | ParameterType::RWByteAddressBuffer
                | ParameterType::Buffer
                | ParameterType::RWBuffer
                | ParameterType::Texture1D
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
                | ParameterType::TextureCubeArray
                | ParameterType::AccelerationStructure => {
                    let offset = resources.num_resources();
                    resources.resource_num += num;

                    let info = CompiledParameterInfo {
                        register_offset,
                        storage_offset: offset,
                    };
                    let template = Self::resource_descriptor_range_template(range_type, num, &info);

                    resources.resource_table.push(template);
                    mapping.params.push(info);
                }
            }
        }

        // The offsets for samplers are only partially derived by the above loop. Only after the
        // first loop to discover the total number of _resource_ descriptors has finished can we
        // fix-up the sampler mappings. We always place samplers after the resources.
        if resources.resource_num != 0 {
            // We don't consume a root parameter for the resource table if there are no non-sampler
            // parameters in the parameter block layout. If there _are_ any resources then we
            // take the first slot so we have to increment the sampler offsets by 1 to make space.
            for (i, param) in desc.params.iter().enumerate() {
                match param.ty {
                    ParameterType::SamplerState => {
                        // Offset the offsets by the total number of resources. All samplers
                        // come after the resources.
                        let mapper = &mut mapping.params[i];
                        mapper.storage_offset += 1;
                    }
                    _ => {}
                }
            }
        }

        // Total number of root parameters is 'num_samplers' + 'num_resources != 0 ? 1 : 0'
        let mut num_root_parameters = 0;
        if resources.resource_num != 0 {
            num_root_parameters += 1;
        }
        num_root_parameters += samplers.num_samplers();

        // Total DWORDs in this case is just number of root parameters as all parameters in this
        // layout mode are descriptor tables.
        let num_dwords = num_root_parameters;

        Self {
            visibility: conv::shader_visibility_to_dx12(desc.visibility),
            mapping,
            resources,
            samplers,
            num_root_parameters,
            num_dwords,
        }
    }

    /// If [`ParameterBlockFlags::PUSH_DESCRIPTOR`] _is_ set.
    fn new_for_push_descriptor(desc: &ParameterBlockDesc) -> Self {
        let mut num_registers_table = [0; 4];
        let mut mapping = CompiledParameterMappingInfo::default();
        let mut resources = CompiledResourceLayout::default();
        let mut samplers = CompiledSamplerLayout::default();
        for param in desc.params {
            let range_type = conv::parameter_type_to_range_dx12(param.ty);
            let num = param.array_size.count() as u32;

            let register_offset = num_registers_table[range_type.0 as usize];
            num_registers_table[range_type.0 as usize] += num;

            match param.ty {
                ParameterType::SamplerState => {
                    // we don't support sampler arrays due to strict limits imposed on D3D12.
                    // - (Tier 1) max 16 samplers in a single root signature
                    // - (Tier 2+) max 2048 samplers in a single root signature
                    // - max 2048 samplers in a single device-visible descriptor heap
                    assert_eq!(num, 1, "We don't support sampler arrays!");

                    let offset = samplers.num_samplers();
                    samplers.sampler_num += num;

                    let info = CompiledParameterInfo {
                        register_offset,
                        storage_offset: offset,
                    };
                    let template = Self::sampler_descriptor_range_template(range_type, num, &info);

                    samplers.sampler_tables.push(template);
                    mapping.params.push(info);
                }
                ParameterType::ConstantBuffer
                | ParameterType::StructuredBuffer
                | ParameterType::RWStructuredBuffer
                | ParameterType::ByteAddressBuffer
                | ParameterType::RWByteAddressBuffer
                | ParameterType::Buffer
                | ParameterType::RWBuffer
                | ParameterType::Texture1D
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
                | ParameterType::TextureCubeArray
                | ParameterType::AccelerationStructure => {
                    assert_eq!(num, 1, "We don't support push descriptors arrays!");

                    let offset = resources.num_resources();
                    resources.resource_num += num;

                    let parameter_type = conv::parameter_type_to_param_dx12(param.ty).unwrap();
                    resources.resource_desciptors.push(CompiledRootDescriptor {
                        parameter_type,
                        register_offset,
                    });
                    mapping.params.push(CompiledParameterInfo {
                        register_offset,
                        storage_offset: offset,
                    });
                }
            }
        }

        // The offsets for samplers are only partially derived by the above loop. Only after the
        // first loop to discover the total number of _resource_ descriptors has finished can we
        // fix-up the sampler mappings. We always place samplers after the resources.
        for (i, param) in desc.params.iter().enumerate() {
            match param.ty {
                ParameterType::SamplerState => {
                    let mapper = &mut mapping.params[i];

                    // Offset the offsets by the total number of resources. All samplers
                    // come after the resources.
                    mapper.storage_offset += resources.resource_num;
                }
                _ => {}
            }
        }

        // Total number of root parameters is 'num_samplers' + 'num_resources'
        let num_root_parameters = resources.num_resources() + samplers.num_samplers();

        // Total number of DWORDs consumed in this layout mode is 1 for each sampler
        // (descriptor table) and 2 for each resource parameter (root descriptor).
        let num_dwords = (resources.num_resources() * 2) + samplers.num_samplers();

        Self {
            visibility: conv::shader_visibility_to_dx12(desc.visibility),
            mapping,
            resources,
            samplers,
            num_root_parameters,
            num_dwords,
        }
    }

    fn resource_descriptor_range_template(
        range_type: D3D12_DESCRIPTOR_RANGE_TYPE,
        num: u32,
        info: &CompiledParameterInfo,
    ) -> D3D12_DESCRIPTOR_RANGE1 {
        D3D12_DESCRIPTOR_RANGE1 {
            RangeType: range_type,
            NumDescriptors: num,
            BaseShaderRegister: info.register_offset,
            RegisterSpace: 0, // Only known when binding sig is being created
            Flags: Default::default(),
            OffsetInDescriptorsFromTableStart: info.storage_offset,
        }
    }

    fn sampler_descriptor_range_template(
        range_type: D3D12_DESCRIPTOR_RANGE_TYPE,
        num: u32,
        info: &CompiledParameterInfo,
    ) -> D3D12_DESCRIPTOR_RANGE1 {
        D3D12_DESCRIPTOR_RANGE1 {
            RangeType: range_type,
            NumDescriptors: num,
            BaseShaderRegister: info.register_offset,
            RegisterSpace: 0, // Only known when binding sig is being created
            Flags: Default::default(),
            OffsetInDescriptorsFromTableStart: 0,
        }
    }
}

/// Internal struct, encapsulates the data compiled from [`ParameterBlockDesc`] for the non-sampler
/// resources declared on the parameter block layout.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct CompiledResourceLayout {
    /// Precompiled list of descriptor ranges that map the resource registers into the root
    /// signature.
    ///
    /// Resources are mapped into a single descriptor table. The parameters are flattened out into
    /// that table based on the order they're declared in the [`ParameterBlockDesc`]. They are cheap
    /// to manage and cheap to bind.
    ///
    /// # Push Descriptors?
    ///
    /// Well... unless you're using push descriptors. The [`ParameterBlockFlags::PUSH_DESCRIPTOR`]
    /// flag significantly changes how we must handle the parameter block layout.
    ///
    /// Instead, the resources will be flattened into _root descriptors_. These consume a lot of
    /// space in the root signature. Push descriptors are an optimization tool for small, frequently
    /// changing parameters that allows avoiding allocating parameter blocks entirely.
    pub resource_table: Vec<D3D12_DESCRIPTOR_RANGE1>,

    /// A sibling to 'resource_table'. This field is used as the template for the path where the
    /// [`ParameterBlockFlags::PUSH_DESCRIPTOR`] flag is set.
    ///
    /// This will contain a flattened list of root descriptor parameters.
    pub resource_desciptors: Vec<CompiledRootDescriptor>,

    /// The total number of descriptors, totalled across all parameters and their array dimension.
    pub resource_num: u32,
}

impl CompiledResourceLayout {
    /// Slice of pre-compiled descriptor ranges to be included when creating a root signature with
    /// this parameter block layout.
    pub const fn ranges(&self) -> &[D3D12_DESCRIPTOR_RANGE1] {
        self.resource_table.as_slice()
    }

    /// Slice of pre-compiled root descriptors to be used when creating a root signature with this
    /// parameter block layout.
    pub const fn descriptors(&self) -> &[CompiledRootDescriptor] {
        self.resource_desciptors.as_slice()
    }

    /// Total number of resource views used by the _single_ descriptor table that is used by the
    /// parameter block layout for all non-sampler descriptors.
    ///
    /// This is how big the descriptor table needs to be when allocated from a heap.
    pub const fn num_resources(&self) -> u32 {
        self.resource_num
    }
}

/// Internal struct, encapsulates the data compiled from [`ParameterBlockDesc`] for the sampler
/// resources declared on the parameter block layout.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct CompiledSamplerLayout {
    /// Precompiled list of descriptor ranges that map the sampler registers into the root
    /// signature.
    ///
    /// Samplers are handled differently to resource descriptors. Instead of a single table, each
    /// sampler register is mapped to a single table of 1 sampler. This means we consume 1
    /// descriptor table entry per shader.
    ///
    /// # Why?
    ///
    /// Because the sampler heap is tiny (2048 handles) we can't create a table in the sampler heap
    /// like resources do. There's just not enough descriptors to go around. Instead, each sampler
    /// object gets a single slot in the sampler heap. We then use the descriptor table as a pointer
    /// to those slots.
    ///
    /// This way we won't run out of space in the sampler heap.
    ///
    /// The downside is that samplers consume excessive root parameter space. Only a small number
    /// of samplers are typically used so the common case is not too bad. If the root parameter
    /// space is at a premium, use bindless samplers.
    pub sampler_tables: Vec<D3D12_DESCRIPTOR_RANGE1>,

    /// The total number of samplers, totalled across all sampler params and their array dimension.
    pub sampler_num: u32,
}

impl CompiledSamplerLayout {
    /// Slice of pre-compiled descriptor ranges to be included when creating a root signature with
    /// this parameter block layout.
    pub const fn ranges(&self) -> &[D3D12_DESCRIPTOR_RANGE1] {
        self.sampler_tables.as_slice()
    }

    /// The number of total samplers in this parameter block layout. Each sampler consumes a single
    /// descriptor table.
    pub const fn num_samplers(&self) -> u32 {
        self.sampler_num
    }
}

/// Internal struct, encapsulates the information to map the parameter index into the root signature
/// and descriptor table that parameter blocks of this layout will use.
///
/// This is used for mapping parameter indices into offsets within a descriptor table, or locations
/// in the root signature to bind root descriptors or descriptor tables.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct CompiledParameterMappingInfo {
    /// This table associates with the 'params' array in [`ParameterBlockDesc`]. Each entry in this
    /// array describes where the associated parameter is placed, either as an offset into a
    /// descriptor table, or as an offset into the root signature.
    pub params: Vec<CompiledParameterInfo>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CompiledParameterInfo {
    /// This is an offset into one of the 4 register types (c, b, t, u). This value encodes which
    /// register the parameter starts consuming registers from. The number of registers consumed
    /// depends on the array dimension the parameter is declared with.
    pub register_offset: u32,

    /// This is an offset into whatever the parameter's underlying storage mechanism is. How this is
    /// interpreted depends on the type of parameter, and flags set on the block layout.
    ///
    /// # Resources
    ///
    /// For plain resources this value encodes an offset, in descriptors, into a descriptor table
    /// that the parameter should be written into. This is used by
    /// [`IDevice::update_parameter_block`] to determine where to write parameters into.
    ///
    /// # Resources - [`ParameterBlockFlags::PUSH_DESCRIPTOR`] edition
    ///
    /// This flag on a layout completely changes how resource parameters are laid out, and how
    /// they're bound. In this case this value encodes an offset, in root parameters, from the
    /// parameter block's base parameter index in the specific binding signature it's being used
    /// from. This is used by [`IComputeEncoder::push_parameters`] to know where in the root sig
    /// to place the root descriptors.
    ///
    /// # Samplers
    ///
    /// Samplers are always encoded with one sampler -> one descriptor table. The interpretation is
    /// similar to resources with 'PUSH_DESCRIPTOR' set. This value encodes an offset, in root
    /// parameters, from the parameter block's base index in the specific binding signature it's
    /// being used from. This is used by both [`IComputeEncoder::push_parameters`] and
    /// [`IComputeEncoder::bind_parameter_blocks`] to know where in the root sig to place the
    /// descriptor tables for each sampler binding.
    pub storage_offset: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct CompiledRootDescriptor {
    /// The type of parameter this root descriptor encodes. This will also determine which register
    /// type is consumed.
    pub parameter_type: D3D12_ROOT_PARAMETER_TYPE,

    /// This is an offset into one of the 4 register types (c, b, t, u). This value encodes which
    /// register the parameter starts consuming registers from. The number of registers consumed
    /// depends on the array dimension the parameter is declared with.
    pub register_offset: u32,
}
