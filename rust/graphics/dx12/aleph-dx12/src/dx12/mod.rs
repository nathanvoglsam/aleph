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

pub(crate) mod blend;
pub(crate) mod blend_desc;
pub(crate) mod blend_op;
pub(crate) mod command_allocator;
pub(crate) mod command_list;
pub(crate) mod command_queue;
pub(crate) mod comparison_func;
pub(crate) mod conservative_rasterization_mode;
pub(crate) mod cpu_descriptor_handle;
pub(crate) mod cull_mode;
pub(crate) mod debug;
pub(crate) mod depth_stencil_desc;
pub(crate) mod depth_stencil_op_desc;
pub(crate) mod depth_write_mask;
pub(crate) mod descriptor_heap;
pub(crate) mod descriptor_range;
pub(crate) mod descriptor_range_flags;
pub(crate) mod descriptor_range_type;
pub(crate) mod device;
pub(crate) mod event;
pub(crate) mod feature_level;
pub(crate) mod fence;
pub(crate) mod fill_mode;
pub(crate) mod filter;
pub(crate) mod gpu_descriptor_handle;
pub(crate) mod graphics_pipeline_state_stream;
pub(crate) mod index_buffer_strip_cut_value;
pub(crate) mod input_classification;
pub(crate) mod input_element_desc;
pub(crate) mod interfaces;
pub(crate) mod logic_op;
pub(crate) mod mesh_shader_pipeline_desc;
pub(crate) mod pipeline_state_stream;
pub(crate) mod primitive_topology_type;
pub(crate) mod rasterizer_desc;
pub(crate) mod render_target_blend_desc;
pub(crate) mod root_constants;
pub(crate) mod root_descriptor;
pub(crate) mod root_descriptor_flags;
pub(crate) mod root_parameter;
pub(crate) mod root_signature;
pub(crate) mod root_signature_blob;
pub(crate) mod root_signature_desc;
pub(crate) mod shader_visibility;
pub(crate) mod static_border_color;
pub(crate) mod static_sampler_desc;
pub(crate) mod stencil_op;
pub(crate) mod stream_output_declaration;
pub(crate) mod stream_output_desc;
pub(crate) mod submission_builder;
pub(crate) mod swapchain;
pub(crate) mod texture_address_mode;
pub(crate) mod versioned_root_signature_desc;

pub use blend::Blend;
pub use blend_desc::BlendDesc;
pub use blend_op::BlendOp;
pub use command_allocator::CommandAllocator;
pub use command_list::ClosedGraphicsCommandList;
pub use command_list::CommandListType;
pub use command_list::OpenGraphicsCommandList;
pub use command_queue::CommandQueue;
pub use command_queue::CommandQueueBuilder;
pub use comparison_func::ComparisonFunc;
pub use conservative_rasterization_mode::ConservativeRasterizationMode;
pub use cpu_descriptor_handle::CPUDescriptorHandle;
pub use cull_mode::CullMode;
pub use debug::Debug;
pub use depth_stencil_desc::DepthStencilDesc;
pub use depth_stencil_op_desc::DepthStencilOpDesc;
pub use depth_write_mask::DepthWriteMask;
pub use descriptor_heap::DescriptorHeap;
pub use descriptor_heap::DescriptorHeapBuilder;
pub use descriptor_heap::DescriptorHeapType;
pub use descriptor_range::DescriptorRange;
pub use descriptor_range::DescriptorRange1;
pub use descriptor_range_flags::DescriptorRangeFlags;
pub use descriptor_range_type::DescriptorRangeType;
pub use device::Device;
pub use event::Event;
pub use feature_level::FeatureLevel;
pub use fence::Fence;
pub use fence::FenceBuilder;
pub use fill_mode::FillMode;
pub use filter::Filter;
pub use gpu_descriptor_handle::GPUDescriptorHandle;
pub use graphics_pipeline_state_stream::GraphicsPipelineStateStream;
pub use graphics_pipeline_state_stream::GraphicsPipelineStateStreamBuilder;
pub use index_buffer_strip_cut_value::IndexBufferStripCutValue;
pub use input_classification::InputClassification;
pub use input_element_desc::InputElementDesc;
pub use interfaces::D3D12DeviceChild;
pub use interfaces::D3D12Object;
pub use logic_op::LogicOp;
pub use mesh_shader_pipeline_desc::MeshShaderPipelineStateDesc;
pub use primitive_topology_type::PrimitiveTopologyType;
pub use rasterizer_desc::RasterizerDesc;
pub use render_target_blend_desc::RenderTargetBlendDesc;
pub use root_constants::RootConstants;
pub use root_descriptor::RootDescriptor;
pub use root_descriptor::RootDescriptor1;
pub use root_descriptor_flags::RootDescriptorFlags;
pub use root_parameter::RootParameter;
pub use root_parameter::RootParameter1;
pub use root_signature::RootSignature;
pub use root_signature_blob::RootSignatureBlob;
pub use root_signature_desc::RootSignatureDesc;
pub use root_signature_desc::RootSignatureDesc1;
pub use root_signature_desc::RootSignatureDesc1Builder;
pub use root_signature_desc::RootSignatureDescBuilder;
pub use shader_visibility::ShaderVisibility;
pub use static_border_color::StaticBorderColor;
pub use static_sampler_desc::StaticSamplerDesc;
pub use stencil_op::StencilOp;
pub use stream_output_declaration::StreamOutputDeclaration;
pub use stream_output_desc::StreamOutputDesc;
pub use submission_builder::SubmissionBuilder;
pub use swapchain::SwapChain;
pub use swapchain::SwapChainBuilder;
pub use texture_address_mode::TextureAddressMode;
pub use versioned_root_signature_desc::VersionedRootSignatureDesc;
