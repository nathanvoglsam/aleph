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
pub(crate) mod r#box;
pub(crate) mod clear_flags;
pub(crate) mod clear_value;
pub(crate) mod color_write_enable;
pub(crate) mod command_allocator;
pub(crate) mod command_list;
pub(crate) mod command_list_type;
pub(crate) mod command_queue;
pub(crate) mod command_queue_desc;
pub(crate) mod command_queue_flags;
pub(crate) mod command_signature;
pub(crate) mod comparison_func;
pub(crate) mod component_mapping;
pub(crate) mod conservative_rasterization_mode;
pub(crate) mod cpu_descriptor_handle;
pub(crate) mod cpu_page_property;
pub(crate) mod cull_mode;
pub(crate) mod debug;
pub(crate) mod depth_stencil_desc;
pub(crate) mod depth_stencil_op_desc;
pub(crate) mod depth_stencil_value;
pub(crate) mod depth_stencil_view_desc;
pub(crate) mod depth_write_mask;
pub(crate) mod descriptor_heap;
pub(crate) mod descriptor_heap_desc;
pub(crate) mod descriptor_heap_flags;
pub(crate) mod descriptor_heap_type;
pub(crate) mod descriptor_range;
pub(crate) mod descriptor_range_flags;
pub(crate) mod descriptor_range_type;
pub(crate) mod device;
pub(crate) mod discard_region;
pub(crate) mod event;
pub(crate) mod feature_level;
pub(crate) mod fence;
pub(crate) mod fence_flags;
pub(crate) mod fill_mode;
pub(crate) mod filter;
pub(crate) mod gpu_descriptor_handle;
pub(crate) mod graphics_pipeline_state_stream;
pub(crate) mod heap;
pub(crate) mod heap_flags;
pub(crate) mod heap_properties;
pub(crate) mod heap_type;
pub(crate) mod index_buffer_strip_cut_value;
pub(crate) mod index_buffer_view;
pub(crate) mod input_classification;
pub(crate) mod input_element_desc;
pub(crate) mod interfaces;
pub(crate) mod logic_op;
pub(crate) mod memory_pool;
pub(crate) mod mesh_shader_pipeline_desc;
pub(crate) mod pipeline_state;
pub(crate) mod pipeline_state_stream;
pub(crate) mod placed_subresource_footprint;
pub(crate) mod predication_op;
pub(crate) mod primitive_topology;
pub(crate) mod primitive_topology_type;
pub(crate) mod query_heap;
pub(crate) mod query_type;
pub(crate) mod rasterizer_desc;
pub(crate) mod rect;
pub(crate) mod render_target_blend_desc;
pub(crate) mod render_target_view_desc;
pub(crate) mod resource;
pub(crate) mod resource_barrier;
pub(crate) mod resource_barrier_flags;
pub(crate) mod resource_desc;
pub(crate) mod resource_dimension;
pub(crate) mod resource_flags;
pub(crate) mod resource_states;
pub(crate) mod root_constants;
pub(crate) mod root_descriptor;
pub(crate) mod root_descriptor_flags;
pub(crate) mod root_parameter;
pub(crate) mod root_signature;
pub(crate) mod root_signature_blob;
pub(crate) mod root_signature_desc;
pub(crate) mod root_signature_flags;
pub(crate) mod sampler_desc;
pub(crate) mod shader_resource_view_desc;
pub(crate) mod shader_visibility;
pub(crate) mod static_border_color;
pub(crate) mod static_sampler_desc;
pub(crate) mod stencil_op;
pub(crate) mod stream_output_buffer_view;
pub(crate) mod stream_output_declaration;
pub(crate) mod stream_output_desc;
pub(crate) mod subresource_footprint;
pub(crate) mod texture_address_mode;
pub(crate) mod texture_copy_location;
pub(crate) mod texture_layout;
pub(crate) mod tile_copy_flags;
pub(crate) mod tile_region_size;
pub(crate) mod tiled_resource_coordinate;
pub(crate) mod versioned_root_signature_desc;
pub(crate) mod vertex_buffer_view;
pub(crate) mod viewport;

pub use blend::Blend;
pub use blend_desc::BlendDesc;
pub use blend_desc::BlendDescBuilder;
pub use blend_op::BlendOp;
pub use clear_flags::ClearFlags;
pub use clear_value::ClearValue;
pub use color_write_enable::ColorWriteEnable;
pub use command_allocator::CommandAllocator;
pub use command_list::GraphicsCommandList;
pub use command_list_type::CommandListType;
pub use command_queue::CommandQueue;
pub use command_queue_desc::CommandQueueDesc;
pub use command_queue_desc::CommandQueueDescBuilder;
pub use command_queue_flags::CommandQueueFlags;
pub use command_signature::CommandSignature;
pub use comparison_func::ComparisonFunc;
pub use component_mapping::ComponentMapping;
pub use component_mapping::ComponentMappingValue;
pub use conservative_rasterization_mode::ConservativeRasterizationMode;
pub use cpu_descriptor_handle::CPUDescriptorHandle;
pub use cpu_page_property::CpuPageProperty;
pub use cull_mode::CullMode;
pub use debug::Debug;
pub use depth_stencil_desc::DepthStencilDesc;
pub use depth_stencil_op_desc::DepthStencilOpDesc;
pub use depth_stencil_value::DepthStencilValue;
pub use depth_stencil_view_desc::DepthStencilViewDesc;
pub use depth_stencil_view_desc::Tex1DArrayDsv;
pub use depth_stencil_view_desc::Tex1DDsv;
pub use depth_stencil_view_desc::Tex2DArrayDsv;
pub use depth_stencil_view_desc::Tex2DDsv;
pub use depth_stencil_view_desc::Tex2DMSArrayDsv;
pub use depth_stencil_view_desc::Tex2DMSDsv;
pub use depth_write_mask::DepthWriteMask;
pub use descriptor_heap::DescriptorHeap;
pub use descriptor_heap_desc::DescriptorHeapDesc;
pub use descriptor_heap_desc::DescriptorHeapDescBuilder;
pub use descriptor_heap_flags::DescriptorHeapFlags;
pub use descriptor_heap_type::DescriptorHeapType;
pub use descriptor_range::DescriptorRange;
pub use descriptor_range::DescriptorRange1;
pub use descriptor_range_flags::DescriptorRangeFlags;
pub use descriptor_range_type::DescriptorRangeType;
pub use device::Device;
pub use discard_region::DiscardRegion;
pub use event::Event;
pub use feature_level::FeatureLevel;
pub use fence::Fence;
pub use fence_flags::FenceFlags;
pub use fill_mode::FillMode;
pub use filter::Filter;
pub use gpu_descriptor_handle::GPUDescriptorHandle;
pub use graphics_pipeline_state_stream::GraphicsPipelineStateStream;
pub use graphics_pipeline_state_stream::GraphicsPipelineStateStreamBuilder;
pub use heap::Heap;
pub use heap_flags::HeapFlags;
pub use heap_properties::HeapProperties;
pub use heap_type::HeapType;
pub use index_buffer_strip_cut_value::IndexBufferStripCutValue;
pub use index_buffer_view::IndexBufferView;
pub use input_classification::InputClassification;
pub use input_element_desc::InputElementDesc;
pub use interfaces::D3D12DeviceChild;
pub use interfaces::D3D12Object;
pub use logic_op::LogicOp;
pub use memory_pool::MemoryPool;
pub use mesh_shader_pipeline_desc::MeshShaderPipelineStateDesc;
pub use pipeline_state::AsPipelineState;
pub use pipeline_state::ComputePipelineState;
pub use pipeline_state::GraphicsPipelineState;
pub use pipeline_state::PipelineState;
pub use placed_subresource_footprint::PlacedSubresourceFootprint;
pub use predication_op::PredicationOp;
pub use primitive_topology::PrimitiveTopology;
pub use primitive_topology_type::PrimitiveTopologyType;
pub use query_heap::QueryHeap;
pub use query_type::QueryType;
pub use r#box::Box;
pub use rasterizer_desc::RasterizerDesc;
pub use rasterizer_desc::RasterizerDescBuilder;
pub use rect::Rect;
pub use render_target_blend_desc::RenderTargetBlendDesc;
pub use render_target_view_desc::BufferRtv;
pub use render_target_view_desc::RenderTargetViewDesc;
pub use render_target_view_desc::Tex1DArrayRtv;
pub use render_target_view_desc::Tex1DRtv;
pub use render_target_view_desc::Tex2DArrayRtv;
pub use render_target_view_desc::Tex2DMSArrayRtv;
pub use render_target_view_desc::Tex2DMSRtv;
pub use render_target_view_desc::Tex2DRtv;
pub use render_target_view_desc::Tex3DRtv;
pub use resource::Resource;
pub use resource_barrier::ResourceBarrier;
pub use resource_barrier_flags::ResourceBarrierFlags;
pub use resource_desc::ResourceDesc;
pub use resource_desc::ResourceDescBuilder;
pub use resource_dimension::ResourceDimension;
pub use resource_flags::ResourceFlags;
pub use resource_states::ResourceStates;
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
pub use root_signature_flags::RootSignatureFlags;
pub use sampler_desc::SamplerDesc;
pub use sampler_desc::SamplerDescBuilder;
pub use shader_resource_view_desc::BufferSrv;
pub use shader_resource_view_desc::BufferSrvFlags;
pub use shader_resource_view_desc::RaytracingAccelerationStructureSrv;
pub use shader_resource_view_desc::ShaderResourceViewDesc;
pub use shader_resource_view_desc::Tex1DArraySrv;
pub use shader_resource_view_desc::Tex1DSrv;
pub use shader_resource_view_desc::Tex2DArraySrv;
pub use shader_resource_view_desc::Tex2DMSArraySrv;
pub use shader_resource_view_desc::Tex2DMSSrv;
pub use shader_resource_view_desc::Tex2DSrv;
pub use shader_resource_view_desc::Tex3DSrv;
pub use shader_resource_view_desc::TexCubeArraySrv;
pub use shader_resource_view_desc::TexCubeSrv;
pub use shader_visibility::ShaderVisibility;
pub use static_border_color::StaticBorderColor;
pub use static_sampler_desc::StaticSamplerDesc;
pub use stencil_op::StencilOp;
pub use stream_output_buffer_view::StreamOutputBufferView;
pub use stream_output_declaration::StreamOutputDeclaration;
pub use stream_output_desc::StreamOutputDesc;
pub use subresource_footprint::SubresourceFootprint;
pub use texture_address_mode::TextureAddressMode;
pub use texture_copy_location::TextureCopyLocation;
pub use texture_layout::TextureLayout;
pub use tile_copy_flags::TileCopyFlags;
pub use tile_region_size::TileRegionSize;
pub use tiled_resource_coordinate::TiledResourceCoordinate;
pub use versioned_root_signature_desc::VersionedRootSignatureDesc;
pub use vertex_buffer_view::VertexBufferView;
pub use viewport::Viewport;
