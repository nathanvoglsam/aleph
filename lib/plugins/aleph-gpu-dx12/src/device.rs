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

use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_pool::CommandPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::internal::conv::{
    blend_factor_to_dx12, blend_op_to_dx12, border_color_to_dx12, compare_op_to_dx12,
    cull_mode_to_dx12, front_face_order_to_dx12, polygon_mode_to_dx12, primitive_topology_to_dx12,
    sampler_address_mode_to_dx12, sampler_filters_to_dx12, shader_visibility_to_dx12,
    stencil_op_to_dx12, texture_create_clear_value_to_dx12, texture_create_desc_to_dx12,
    texture_format_to_dxgi,
};
use crate::internal::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::internal::descriptor_heap_info::DescriptorHeapInfo;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::{PipelineLayout, PushConstantBlockInfo};
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::shader::Shader;
use crate::texture::{PlainTexture, Texture, TextureInner};
use aleph_windows::Win32::Graphics::Direct3D12::*;
use aleph_windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use crossbeam::queue::SegQueue;
use dx12::{dxgi, D3D12Object};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use interfaces::anyhow;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BlendStateDesc, BufferCreateError, BufferDesc, CommandPoolCreateError,
    ComputePipelineCreateError, ComputePipelineDesc, CpuAccessMode, DepthStencilStateDesc,
    DescriptorSetLayoutCreateError, DescriptorSetLayoutDesc, DescriptorType,
    GraphicsPipelineCreateError, GraphicsPipelineDesc, IBuffer, ICommandPool, IComputePipeline,
    IDescriptorSetLayout, IDevice, IGraphicsPipeline, INamedObject, IPipelineLayout, IQueue,
    ISampler, IShader, ITexture, PipelineLayoutCreateError, PipelineLayoutDesc, QueueType,
    RasterizerStateDesc, SamplerCreateError, SamplerDesc, ShaderBinary, ShaderCreateError,
    ShaderOptions, ShaderType, StencilOpState, TextureCreateError, TextureDesc, VertexInputRate,
    VertexInputStateDesc,
};
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) device: dx12::Device,
    pub(crate) debug_message_cookie: Option<u32>,
    pub(crate) descriptor_heap_info: DescriptorHeapInfo,
    pub(crate) rtv_heap: DescriptorAllocatorCPU,
    pub(crate) dsv_heap: DescriptorAllocatorCPU,
    pub(crate) _sampler_heap: DescriptorAllocatorCPU,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
}

declare_interfaces!(Device, [IDevice, IDeviceExt]);

impl IDevice for Device {
    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn garbage_collect(&self) {
        if let Some(queue) = &self.general_queue {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.compute_queue {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.clear_completed_lists();
        }
    }

    fn wait_idle(&self) {
        if let Some(queue) = &self.general_queue {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.compute_queue {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.wait_all_lists_completed();
        }
    }

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        // Unwrap the pipeline layout trait object into the concrete implementation
        let pipeline_layout = desc
            .pipeline_layout
            .upgrade()
            .query_interface::<PipelineLayout>()
            .unwrap();

        let builder = dx12::GraphicsPipelineStateStreamBuilder::new();

        // Add all shaders in the list to their corresponding slot
        let builder = Self::translate_shader_stage_list(desc.shader_stages, builder)?;

        let builder = builder.root_signature(pipeline_layout.root_signature.clone());

        let (input_binding_strides, input_layout) =
            Self::translate_vertex_input_state_desc(desc.vertex_layout);
        let builder = builder.input_layout(&input_layout);

        let (builder, primitive_topology) =
            Self::translate_input_assembly_state_desc(desc, builder);

        let rasterizer_state = Self::translate_rasterizer_state_desc(desc.rasterizer_state);
        let builder = builder.rasterizer_state(rasterizer_state);

        let depth_stencil_state = Self::translate_depth_stencil_desc(desc.depth_stencil_state);
        let builder = builder.depth_stencil_state(depth_stencil_state);

        let blend_state = Self::translate_blend_state_desc(desc.blend_state);
        let builder = builder.blend_state(blend_state);

        let builder = builder.sample_mask(u32::MAX); // TODO: Why?

        // Render target format translation is straight forward, just convert the formats and add
        let rtv_formats: Vec<dxgi::Format> = desc
            .render_target_formats
            .iter()
            .copied()
            .map(texture_format_to_dxgi)
            .collect();
        let builder = builder.rtv_formats(&rtv_formats);
        let builder =
            if let Some(dsv_format) = desc.depth_stencil_format.map(texture_format_to_dxgi) {
                builder.dsv_format(dsv_format)
            } else {
                builder
            };

        // Construct the D3D12 pipeline object
        let state_stream = builder.build();
        let pipeline = self
            .device
            .create_graphics_pipeline_state(&state_stream)
            .map_err(|v| anyhow!(v))?;

        let pipeline = AnyArc::new_cyclic(move |v| GraphicsPipeline {
            this: v.clone(),
            pipeline,
            pipeline_layout,
            primitive_topology,
            input_binding_strides,
        });
        Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(pipeline, |v| v))
    }

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        // Unwrap the pipeline layout trait object into the concrete implementation
        let pipeline_layout = desc
            .pipeline_layout
            .upgrade()
            .query_interface::<PipelineLayout>()
            .expect("Unknown IPipelineLayout implementation");

        let module = desc
            .shader_module
            .query_interface::<Shader>()
            .expect("Unknown IShader implementation");

        let pipeline_desc = dx12::ComputePipelineStateDesc {
            root_signature: pipeline_layout.root_signature.clone(),
            shader: &module.data,
            node_mask: 0,
            cached_pso: None,
        };

        let pipeline = self
            .device
            .create_compute_pipeline_state(&pipeline_desc)
            .map_err(|v| anyhow!(v))?;

        let pipeline = AnyArc::new_cyclic(move |v| ComputePipeline {
            this: v.clone(),
            pipeline,
            _pipeline_layout: pipeline_layout,
        });
        Ok(AnyArc::map::<dyn IComputePipeline, _>(pipeline, |v| v))
    }

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        if let ShaderBinary::Dxil(data) = options.data {
            // Empty shader binary is invalid
            if data.is_empty() {
                return Err(ShaderCreateError::InvalidInputSize(0));
            }

            let shader = AnyArc::new_cyclic(move |v| Shader {
                this: v.clone(),
                shader_type: options.shader_type,
                data: data.to_vec(),
                entry_point: options.entry_point.to_string(),
            });
            Ok(AnyArc::map::<dyn IShader, _>(shader, |v| v))
        } else {
            Err(ShaderCreateError::UnsupportedShaderFormat)
        }
    }

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        let visibility = shader_visibility_to_dx12(desc.visibility);

        // TODO: Currently we always create a descriptor table. In the future we could use some
        //       optimization heuristics to detect when a root descriptor is better.

        // First we produce a descriptor table for the non-sampler descriptors. Samplers have to go
        // in their own descriptor heap and so we can't emit a single descriptor table for the
        // layout.
        //
        // Any non-immutable samplers require a second descriptor table.
        let resource_table = self.build_resource_table_layout(desc);
        let (sampler_table, static_samplers) = self.build_sampler_table_layout(desc)?;

        // Convert an empty sampler table into none to better encode the meaning in the type
        let sampler_table = if sampler_table.is_empty() {
            None
        } else {
            Some(sampler_table)
        };

        let descriptor_set_layout = AnyArc::new_cyclic(move |v| DescriptorSetLayout {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            visibility,
            resource_table,
            static_samplers,
            sampler_table,
        });
        Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(
            descriptor_set_layout,
            |v| v,
        ))
    }

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        // Bundle up all the table layouts after we patch them for use in this layout as we need to
        // extend the lifetime for the call to create the root signature
        let mut resource_tables = Vec::with_capacity(desc.set_layouts.len());
        let mut static_samplers = Vec::new();
        for (i, layout) in desc.set_layouts.iter().enumerate() {
            let layout = layout
                .query_interface::<DescriptorSetLayout>()
                .expect("Unknown IDescriptorSetLayout impl");

            // Take a copy of the pre-calculated layout and patch the register space to match the
            // set index that it is being used for
            let mut table = layout.resource_table.clone();
            for binding in table.iter_mut() {
                binding.register_space = i as u32;
            }
            resource_tables.push((table, layout.visibility));

            // Extend our list of static samplers based on the provided list for this binding
            static_samplers.extend(layout.static_samplers.iter().map(|v| {
                let mut out = v.clone();
                out.register_space = i as u32;
                out
            }));
        }

        let mut parameters =
            Vec::with_capacity(desc.set_layouts.len() + desc.push_constant_blocks.len());
        for (ranges, visibility) in &resource_tables {
            let param = dx12::RootParameter1::DescriptorTable {
                visibility: visibility.clone(),
                ranges: ranges.as_slice(),
            };
            parameters.push(param);
        }
        // TODO: Putting root constants after all descriptors may have performance implications.
        //       D3D12 requires priority to lower root parameter indices so, (on AMD) having push
        //       constants after descriptors means the constants are more likely to spill into
        //       memory instead of being in the registers.
        let mut push_constant_blocks = Vec::new();
        for block in desc.push_constant_blocks {
            if (block.size % 4) != 0 {
                return Err(PipelineLayoutCreateError::InvalidPushConstantBlockSize);
            }
            let num32_bit_values = (block.size / 4) as u32;
            let range = dx12::RootParameter1::Constants {
                visibility: shader_visibility_to_dx12(block.visibility),
                constants: dx12::RootConstants {
                    shader_register: block.binding,
                    register_space: 1024, // A reserved space for root/push constants
                    num32_bit_values,
                },
            };
            push_constant_blocks.push(PushConstantBlockInfo {
                size: num32_bit_values * 4,
                root_parameter_index: parameters.len() as u32,
            });
            parameters.push(range);
        }

        // TODO: dynamic samplers

        // TODO: Investigate a better way for handling 'allow input assembler' flag as currently we
        //       just unconditionally enable it. Supposedly it can be a minor optimization on some
        //       hardware.
        let desc = dx12::RootSignatureDesc1::builder()
            .parameters(&parameters)
            .static_samplers(&static_samplers)
            .flags(dx12::RootSignatureFlags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT)
            .build();
        let desc = dx12::VersionedRootSignatureDesc::Desc1(desc);
        let root_signature = unsafe {
            let blob = dx12::RootSignatureBlob::new(&desc).map_err(|v| anyhow!(v))?;
            self.device
                .create_root_signature(&blob)
                .map_err(|v| anyhow!(v))?
        };

        let pipeline_layout = AnyArc::new_cyclic(move |v| PipelineLayout {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            root_signature,
            push_constant_blocks,
        });
        Ok(AnyArc::map::<dyn IPipelineLayout, _>(
            pipeline_layout,
            |v| v,
        ))
    }

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        let mut resource_desc = D3D12_RESOURCE_DESC1 {
            // Fields that will be the same regardless of the requested buffer desc
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: 0,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Default::default(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            SamplerFeedbackMipRegion: Default::default(),
            Flags: Default::default(),
        };

        resource_desc.Width = desc.size;

        if desc.allow_unordered_access {
            resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
        }

        let heap_type = match desc.cpu_access {
            CpuAccessMode::None => D3D12_HEAP_TYPE_DEFAULT,
            CpuAccessMode::Read => D3D12_HEAP_TYPE_READBACK,
            CpuAccessMode::Write => D3D12_HEAP_TYPE_UPLOAD,
        };

        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: heap_type,
            CPUPageProperty: Default::default(),
            MemoryPoolPreference: Default::default(),
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource = unsafe {
            self.device
                .as_raw()
                .CreateCommittedResource3::<_, ID3D12Resource>(
                    &heap_properties,
                    Default::default(),
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT::UNDEFINED,
                    core::ptr::null(),
                    None,
                    0,
                    std::ptr::null(),
                )
                .map(|v| std::mem::transmute::<_, dx12::Resource>(v))
                .map_err(|v| anyhow!(v))?
        };
        let base_address = resource.get_gpu_virtual_address().unwrap();

        let buffer = AnyArc::new_cyclic(move |v| Buffer {
            this: v.clone(),
            resource,
            base_address,
            desc: desc.clone(),
            debug_mapped_tracker: Default::default(),
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(buffer, |v| v))
    }

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            CPUPageProperty: Default::default(),
            MemoryPoolPreference: Default::default(),
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource_desc = texture_create_desc_to_dx12(desc)?;
        let optimized_clear_value =
            texture_create_clear_value_to_dx12(desc, resource_desc.Format.try_into().unwrap())?;

        let resource = unsafe {
            let optimized_clear_value = optimized_clear_value.map(D3D12_CLEAR_VALUE::from);
            let optimized_clear_value_ref = match optimized_clear_value.as_ref() {
                None => core::ptr::null(),
                Some(v) => v as *const D3D12_CLEAR_VALUE,
            };

            self.device
                .as_raw()
                .CreateCommittedResource3::<_, ID3D12Resource>(
                    &heap_properties,
                    Default::default(),
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT::UNDEFINED,
                    optimized_clear_value_ref,
                    None,
                    0,
                    std::ptr::null(), // TODO: We could use this maybe?
                )
                .map(|v| std::mem::transmute::<_, dx12::Resource>(v))
                .map_err(|v| anyhow!(v))?
        };

        let texture = AnyArc::new_cyclic(move |v| Texture {
            this: v.clone(),
            inner: TextureInner::Plain(PlainTexture {
                device: self.this.upgrade().unwrap(),
                resource,
                desc: desc.clone(),
                dxgi_format: resource_desc.Format.try_into().unwrap(),
                rtv_cache: RwLock::new(HashMap::new()),
                dsv_cache: RwLock::new(HashMap::new()),
            }),
        });
        Ok(AnyArc::map::<dyn ITexture, _>(texture, |v| v))
    }

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        let sampler = AnyArc::new_cyclic(move |v| Sampler {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            desc: desc.clone(),
        });
        Ok(AnyArc::map::<dyn ISampler, _>(sampler, |v| v))
    }

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError> {
        let pool = AnyArc::new_cyclic(move |v| CommandPool {
            this: v.clone(),
            device: self.this.upgrade().unwrap(),
            general_free_list: SegQueue::new(),
            _compute_free_list: SegQueue::new(),
            _transfer_free_list: SegQueue::new(),
        });
        Ok(AnyArc::map::<dyn ICommandPool, _>(pool, |v| v))
    }

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        match queue_type {
            QueueType::General => self
                .general_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Compute => self
                .compute_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
            QueueType::Transfer => self
                .transfer_queue
                .clone()
                .map(|v| AnyArc::map::<dyn IQueue, _>(v, |v| v)),
        }
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }
}

impl Device {
    pub unsafe fn create_views_for_swap_images(
        &self,
        swap_chain: &dxgi::SwapChain,
        format: dxgi::Format,
        count: u32,
    ) -> anyhow::Result<Vec<(dx12::Resource, dx12::CPUDescriptorHandle)>> {
        let mut images = Vec::new();
        for i in 0..count {
            let buffer = swap_chain.get_buffer(i).map_err(|e| anyhow!(e))?;
            let view = self.rtv_heap.allocate().unwrap();

            let desc = dx12::RenderTargetViewDesc::Texture2D {
                format,
                texture_2d: dx12::Tex2DRtv {
                    mip_slice: 0,
                    plane_slice: 0,
                },
            };
            self.device.create_render_target_view(&buffer, &desc, view);

            images.push((buffer, view));
        }
        Ok(images)
    }

    /// Internal function for translating the list of [IShader] stages into the pipeline description
    fn translate_shader_stage_list<'a, 'b>(
        shader_stages: &'a [&'a dyn IShader],
        mut builder: dx12::GraphicsPipelineStateStreamBuilder<'a>,
    ) -> Result<dx12::GraphicsPipelineStateStreamBuilder<'a>, GraphicsPipelineCreateError> {
        for shader in shader_stages {
            let shader = shader.query_interface::<Shader>().unwrap();
            builder = match shader.shader_type {
                ShaderType::Vertex => builder.vertex_shader(&shader.data),
                ShaderType::Hull => builder.hull_shader(&shader.data),
                ShaderType::Domain => builder.domain_shader(&shader.data),
                ShaderType::Geometry => builder.geometry_shader(&shader.data),
                ShaderType::Fragment => builder.pixel_shader(&shader.data),
                ShaderType::Compute | ShaderType::Amplification | ShaderType::Mesh => {
                    todo!()
                }
            }
        }
        Ok(builder)
    }

    /// Internal function for translating the [VertexInputStateDesc] field of a pipeline
    /// description
    fn translate_vertex_input_state_desc(
        desc: &VertexInputStateDesc,
    ) -> ([u32; 16], Vec<dx12::InputElementDesc<'static>>) {
        // Copy the input binding strides into a buffer the pipeline will hold on to so it can be
        // used in the command encoders. Vulkan bakes these in the pipeline, d3d12 gets the values
        // when the input bindings are bound
        let mut input_binding_strides = [0u32; 16];
        for (binding, stride) in desc.input_bindings.iter().zip(&mut input_binding_strides) {
            *stride = binding.stride;
        }

        // Translate the vertex input description
        let mut input_layout = Vec::new();
        for attribute in desc.input_attributes {
            // DX12 describes vertex attributes differently. The RHI exposes the Vulkan way as it
            // is easier to map vulkan->dx12 here than the other way around, and is more robust.
            //
            // DX12 duplicates some of the "binding" description in every attribute, Vulkan uses
            // a level of indirection by separating attributes from the actual buffers bound to get
            // data from.
            //
            // We have to re-duplicate the data for DX12. Some of the data is also fully dynamic.
            // Buffer binding "stride" is part of the pipeline state object on Vulkan, while on DX12
            // it's only known once `IASetVertexBuffers` is recorded on a command buffer. Vulkan is
            // again easier to map to dx12 so we choose vulkan's behavior. We need to store the
            // stride on our pipeline object so it can be sourced when recording.
            //
            // This requires binding the pipeline before vertex buffers, and re-binding when the
            // pipeline changes as the stride may have changed. This *will* require some extra work
            // in the command buffer recording abstraction.
            let binding = desc
                .input_bindings
                .iter()
                .find(|v| v.binding == attribute.binding)
                .unwrap();

            // We always use a semantic of "A" for vertex attributes for DX12. We only expose an
            // attribute "location" index like vulkan so only the number of the semantic means
            // anything to consumers of our RHI.
            //
            // This requires some modification to existing shaders to be compatible but makes
            // mapping Vulkan easier. It is also much simpler, just an "index" compared to a string
            // identifier + index combo.
            let semantic_name = cstr::cstr!("A").into();
            let semantic_index = attribute.location;

            // Input slot directly translates to Vulkan's concept of a vertex attribute binding
            // index. They are the same thing, an index that describes which bound vertex buffer to
            // load data from for the vertex attribute being described.
            let input_slot = attribute.binding;

            // Aligned byte offset also translates directly, but one some of dx12's convenience
            // features. If set to '0', dx12 can synthesize this value based on the other input
            // elements and the vertex format. Vulkan requires manual specification, so we just
            // leave it to the RHI caller like Vulkan.
            let aligned_byte_offset = attribute.offset;

            // Vertex input rate is defined on the buffer binding and not the attribute on Vulkan.
            // Mapping dx12->vulkan here requires extra checks, so instead we adopt Vulkan's model.
            //
            // We've fetched the binding and extract the values for input_slot_class and
            // instance_data_step_rate from the binding description.
            let (input_slot_class, instance_data_step_rate) = match binding.input_rate {
                VertexInputRate::PerVertex => (dx12::InputClassification::PerVertex, 0),
                VertexInputRate::PerInstance => (dx12::InputClassification::PerInstance, 1),
            };

            input_layout.push(dx12::InputElementDesc {
                semantic_name,
                semantic_index,
                format: texture_format_to_dxgi(attribute.format),
                input_slot,
                aligned_byte_offset,
                input_slot_class,
                instance_data_step_rate,
            });
        }

        (input_binding_strides, input_layout)
    }

    /// Internal function for translating the [InputAssemblyStateDesc] field of a pipeline
    /// description
    fn translate_input_assembly_state_desc<'a, 'b>(
        desc: &'a GraphicsPipelineDesc,
        mut builder: dx12::GraphicsPipelineStateStreamBuilder<'b>,
    ) -> (
        dx12::GraphicsPipelineStateStreamBuilder<'b>,
        dx12::PrimitiveTopology,
    ) {
        // Once again, we adopt a Vulkan model when handling primitive topology. DX12's pipeline
        // state object only takes a "primitive class" of point, line or triangle. Whether it's a
        // line strip/line list or triangle strip/triangle list is only known once
        // IASetPrimitiveTopology is called.
        //
        // Vulkan can't replicate this so we need to follow Vulkan's convention here. We *do* select
        // the "primitive class" here, as we should. We also need to store the *actual* primitive
        // topology on the pipeline so we can call IASetPrimitiveTopology with the correct value
        // when we bind the pipeline.
        let (r#type, topo) =
            primitive_topology_to_dx12(desc.input_assembly_state.primitive_topology);
        builder = builder.primitive_topology_type(r#type);
        (builder, topo)
    }

    /// Internal function for translating the [RasterizerStateDesc] field of a pipeline
    /// description
    fn translate_rasterizer_state_desc(desc: &RasterizerStateDesc) -> dx12::RasterizerDesc {
        let fill_mode = polygon_mode_to_dx12(desc.polygon_mode);
        let cull_mode = cull_mode_to_dx12(desc.cull_mode);
        let front_counter_clockwise = front_face_order_to_dx12(desc.front_face);
        dx12::RasterizerDesc {
            fill_mode,
            cull_mode,
            front_counter_clockwise,
            depth_bias: 0,                         // TODO: translate
            depth_bias_clamp: 0.0,                 // TODO: translate
            slope_scaled_depth_bias: 0.0,          // TODO: translate
            depth_clip_enable: dx12::Bool::TRUE,   // Vulkan has no option, so always true
            multisample_enable: dx12::Bool::FALSE, // TODO: translate
            antialiased_line_enable: dx12::Bool::FALSE,
            forced_sample_count: 0,
            conservative_raster: dx12::ConservativeRasterizationMode::Off,
        }
    }

    /// Internal function for translating the [DepthStencilStateDesc] field of a pipeline
    /// description
    fn translate_depth_stencil_desc(desc: &DepthStencilStateDesc) -> dx12::DepthStencilDesc {
        /// Internal function for translating our [StencilOpState] into the D3D12 equivalent
        fn translate_depth_stencil_op_desc(desc: &StencilOpState) -> dx12::DepthStencilOpDesc {
            let stencil_fail_op = stencil_op_to_dx12(desc.fail_op);
            let stencil_depth_fail_op = stencil_op_to_dx12(desc.depth_fail_op);
            let stencil_pass_op = stencil_op_to_dx12(desc.pass_op);
            let stencil_func = compare_op_to_dx12(desc.compare_op);
            dx12::DepthStencilOpDesc {
                stencil_fail_op,
                stencil_depth_fail_op,
                stencil_pass_op,
                stencil_func,
            }
        }

        let depth_enable = dx12::Bool::from(desc.depth_test);
        let depth_write_mask = if desc.depth_write {
            dx12::DepthWriteMask::All
        } else {
            dx12::DepthWriteMask::Zero
        };
        let depth_func = compare_op_to_dx12(desc.depth_compare_op);
        let stencil_enable = dx12::Bool::from(desc.stencil_test);
        let stencil_read_mask = desc.stencil_read_mask;
        let stencil_write_mask = desc.stencil_write_mask;

        let front_face = translate_depth_stencil_op_desc(&desc.stencil_front);
        let back_face = translate_depth_stencil_op_desc(&desc.stencil_back);

        dx12::DepthStencilDesc {
            depth_enable,
            depth_write_mask,
            depth_func,
            stencil_enable,
            stencil_read_mask,
            stencil_write_mask,
            front_face,
            back_face,
        }
    }

    fn translate_blend_state_desc(desc: &BlendStateDesc) -> dx12::BlendDesc {
        // TODO: Figure out if alpha to coverage is possible to expose
        let alpha_to_coverage_enable = dx12::Bool::FALSE;
        let independent_blend_enable = dx12::Bool::TRUE;

        // TODO: Once we cast aleph-dx12 into the void we should replace this with a 'zeroed' struct
        //       for faster initialization. We can't zero this struct because our enum wrappers dont
        //       allow for zero as a valid value, so zeroing this causes immediate UB.
        // Use our default attachment to initialize the array dx12 needs. Only the first 'n' values
        // will be read, where 'n' is the number of render targets in the pipeline desc, all other
        // items in the array will be ignored so they don't need to be in a well defined state.
        let mut render_targets = [
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
            dx12::RenderTargetBlendDesc::default(),
        ];

        for (i, attachment) in desc.attachments.iter().enumerate() {
            let blend_enable = dx12::Bool::from(attachment.blend_enabled);

            let logic_op_enable = dx12::Bool::FALSE;
            let logic_op = dx12::LogicOp::Clear;

            let src_blend = blend_factor_to_dx12(attachment.src_factor);
            let dest_blend = blend_factor_to_dx12(attachment.dst_factor);
            let blend_op = blend_op_to_dx12(attachment.blend_op);

            let src_blend_alpha = blend_factor_to_dx12(attachment.alpha_src_factor);
            let dest_blend_alpha = blend_factor_to_dx12(attachment.alpha_dst_factor);
            let blend_op_alpha = blend_op_to_dx12(attachment.alpha_blend_op);

            let render_target_write_mask =
                dx12::ColorWriteEnable(attachment.color_write_mask.bits());

            render_targets[i] = dx12::RenderTargetBlendDesc {
                blend_enable,
                logic_op_enable,
                src_blend,
                dest_blend,
                blend_op,
                src_blend_alpha,
                dest_blend_alpha,
                blend_op_alpha,
                logic_op,
                render_target_write_mask,
            };
        }

        dx12::BlendDesc {
            alpha_to_coverage_enable,
            independent_blend_enable,
            render_targets,
        }
    }

    fn build_resource_table_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Vec<dx12::DescriptorRange1> {
        let mut offset = 0;
        let mut table = Vec::with_capacity(desc.items.len());
        for item in desc
            .items
            .iter()
            .filter(|v| v.binding_type != DescriptorType::Sampler)
        {
            if item.binding_count.is_some() {
                // Descriptor arrays are currently unimplemented pending a solution for mapping
                // how they surface in SPIR-V vs D3D12.
                //
                // - Vulkan uses a single binding for the whole array.
                // - D3D12 uses a register per element.
                //
                // We currently map binding_num directly to register number. Arrays break this
                // mapping, Vulkan will work but D3D12 will not. We either have to force asinine
                // D3D12 behavior on Vulkan or
                //
                unimplemented!("Currently descriptor arrays are unimplemented");
            }
            // The concrete descriptor type depends on the resource class and potential write access
            //
            // - Anything that can be written to will be accessed via an unordered access view.
            // - Constant buffers have a dedicated view type (constant buffer view).
            // - Textures and other buffer types are shader resource views.
            // - Samplers get filtered out so don't matter for this conversion.
            let range_type = match (item.binding_type, item.allow_writes) {
                (
                    DescriptorType::Texture
                    | DescriptorType::StructuredBuffer
                    | DescriptorType::RawBuffer
                    | DescriptorType::TypedBuffer,
                    false,
                ) => dx12::DescriptorRangeType::SRV,
                (
                    DescriptorType::Texture
                    | DescriptorType::StructuredBuffer
                    | DescriptorType::RawBuffer
                    | DescriptorType::TypedBuffer,
                    true,
                ) => dx12::DescriptorRangeType::UAV,
                (DescriptorType::ConstantBuffer, _) => dx12::DescriptorRangeType::CBV,
                (DescriptorType::Sampler, _) => unreachable!(),
            };
            let num_descriptors = match item.binding_count {
                None => 1,
                Some(v) => v.get(),
            };
            let base_shader_register = item.binding_num;
            let flags = dx12::DescriptorRangeFlags::DATA_VOLATILE
                | dx12::DescriptorRangeFlags::DESCRIPTORS_VOLATILE;
            let item = dx12::DescriptorRange1 {
                range_type,
                num_descriptors,
                base_shader_register,
                register_space: 0,
                flags, // TODO: temp fix for existing renderer, remove in future
                offset_in_descriptors_from_table_start: offset,
            };
            table.push(item);
            offset += self.descriptor_heap_info.resource_inc * num_descriptors;
        }
        table
    }

    fn build_sampler_table_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<
        (Vec<dx12::DescriptorRange1>, Vec<dx12::StaticSamplerDesc>),
        DescriptorSetLayoutCreateError,
    > {
        let mut offset = 0;
        let mut sampler_table = Vec::new();
        let mut static_samplers = Vec::new();
        for item in desc
            .items
            .iter()
            .filter(|v| v.binding_type == DescriptorType::Sampler)
        {
            if item.binding_count.is_some() {
                // we don't support sampler array bindings due to strict limits imposed on D3D12.
                // - (Tier 1) max 16 samplers in a single root signature
                // - (Tier 2+) max 2048 samplers in a single root signature
                // - max 2048 samplers in a single device-visible descriptor heap
                //
                // Only 2048 samplers can ever be addressed at once, making bindless difficult as
                // the limit is very small, and non-bindless capable hardware can only have 16
                // samplers in a root signature meaning static sized arrays will typically be so
                // small it makes using an array redundant.
                unimplemented!("Sampler Arrays are currently un-implemented");
            }

            // Dynamic samplers require a descriptor table as they're dynamic. There is a separate
            // part of a root signature that handles static samplers.
            //
            // We switch how we output the binding based on the presence of static samplers
            if let Some(samplers) = item.static_samplers {
                for sampler in samplers {
                    let sampler = sampler.query_interface::<Sampler>().unwrap();
                    let filter = sampler_filters_to_dx12(
                        sampler.desc.min_filter,
                        sampler.desc.mag_filter,
                        sampler.desc.mip_filter,
                        sampler.desc.compare_op.is_some(),
                        sampler.desc.enable_anisotropy,
                    );
                    static_samplers.push(dx12::StaticSamplerDesc {
                        filter,
                        address_u: sampler_address_mode_to_dx12(sampler.desc.address_mode_u),
                        address_v: sampler_address_mode_to_dx12(sampler.desc.address_mode_v),
                        address_w: sampler_address_mode_to_dx12(sampler.desc.address_mode_w),
                        mip_lod_bias: sampler.desc.lod_bias,
                        max_anisotropy: sampler.desc.max_anisotropy as u32,
                        comparison_func: sampler
                            .desc
                            .compare_op
                            .map(compare_op_to_dx12)
                            .unwrap_or(dx12::ComparisonFunc::Always),
                        border_color: border_color_to_dx12(sampler.desc.border_color),
                        min_lod: sampler.desc.min_lod,
                        max_lod: sampler.desc.max_lod,
                        shader_register: item.binding_num,
                        register_space: 0,
                        shader_visibility: dx12::ShaderVisibility::All,
                    });
                }
            } else {
                // Handle dynamic samplers by inserting them into a descriptor table.
                let num_descriptors = match item.binding_count {
                    None => 1,
                    Some(v) => v.get(),
                };
                let base_shader_register = item.binding_num;
                let flags = dx12::DescriptorRangeFlags::DATA_VOLATILE
                    | dx12::DescriptorRangeFlags::DESCRIPTORS_VOLATILE;
                let item = dx12::DescriptorRange1 {
                    range_type: dx12::DescriptorRangeType::Sampler,
                    num_descriptors,
                    base_shader_register,
                    register_space: 0,
                    flags, // TODO: temp fix for existing renderer, remove in future
                    offset_in_descriptors_from_table_start: offset,
                };
                sampler_table.push(item);
                offset += self.descriptor_heap_info.sampler_inc * num_descriptors;
            }
        }
        Ok((sampler_table, static_samplers))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // SAFETY: This should be safe but I can't prove it
        unsafe {
            if let Some(cookie) = self.debug_message_cookie {
                let _sink = self.device.unregister_message_callback(cookie);
            }
        }
    }
}

pub trait IDeviceExt: IDevice {
    fn get_raw_handle(&self) -> dx12::Device;
    fn get_raw_general_queue(&self) -> Option<dx12::CommandQueue>;
    fn get_raw_compute_queue(&self) -> Option<dx12::CommandQueue>;
    fn get_raw_transfer_queue(&self) -> Option<dx12::CommandQueue>;
}

impl IDeviceExt for Device {
    fn get_raw_handle(&self) -> dx12::Device {
        self.device.clone()
    }

    fn get_raw_general_queue(&self) -> Option<dx12::CommandQueue> {
        self.general_queue.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_compute_queue(&self) -> Option<dx12::CommandQueue> {
        self.compute_queue.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_transfer_queue(&self) -> Option<dx12::CommandQueue> {
        self.transfer_queue.as_ref().map(|v| v.handle.clone())
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl INamedObject for Device {
    fn set_name(&self, name: &str) {
        self.device.set_name(name).unwrap()
    }
}
