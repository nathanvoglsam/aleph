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

use crate::acquired_texture::AcquiredTexture;
use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_pool::CommandPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::general_command_list::GeneralCommandList;
use crate::internal::conv::{
    resource_state_to_dx12, texture_create_clear_value_to_dx12, texture_create_desc_to_dx12,
    texture_format_to_dxgi,
};
use crate::internal::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::internal::descriptor_heap_info::DescriptorHeapInfo;
use crate::internal::in_flight_command_list::InFlightCommandList;
use crate::internal::queue::Queue;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::PipelineLayout;
use crate::sampler::Sampler;
use crate::shader::Shader;
use crate::texture::Texture;
use crossbeam::queue::SegQueue;
use dx12::{dxgi, AsWeakRef, D3D12Object};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface, QueryInterfaceBox};
use interfaces::anyhow;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BlendFactor, BlendOp, BlendStateDesc, BufferCreateError, BufferDesc,
    CommandPoolCreateError, CompareOp, ComputePipelineCreateError, ComputePipelineDesc,
    CpuAccessMode, CullMode, DepthStencilStateDesc, DescriptorSetLayoutCreateError,
    DescriptorSetLayoutDesc, FrontFaceOrder, GraphicsPipelineCreateError, GraphicsPipelineDesc,
    IAcquiredTexture, IBuffer, ICommandPool, IComputePipeline, IDescriptorSetLayout, IDevice,
    IGeneralCommandList, IGraphicsPipeline, INamedObject, ISampler, IShader, ISwapChain, ITexture,
    PolygonMode, PrimitiveTopology, QueuePresentError, QueueSubmitError, QueueType,
    RasterizerStateDesc, SamplerCreateError, SamplerDesc, ShaderBinary, ShaderCreateError,
    ShaderOptions, ShaderType, StencilOp, StencilOpState, TextureCreateError, TextureDesc,
    VertexInputRate, VertexInputStateDesc,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) device: dx12::Device,
    pub(crate) debug_message_cookie: Option<u32>,
    pub(crate) descriptor_heap_info: DescriptorHeapInfo,
    pub(crate) rtv_heap: DescriptorAllocatorCPU,
    pub(crate) dsv_heap: DescriptorAllocatorCPU,
    pub(crate) _sampler_heap: DescriptorAllocatorCPU,
    pub(crate) queues: Queues,
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
        if let Some(queue) = &self.queues.general {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.queues.compute {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.queues.transfer {
            queue.clear_completed_lists();
        }
    }

    fn wait_idle(&self) {
        if let Some(queue) = &self.queues.general {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.queues.compute {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.queues.transfer {
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

        // TODO: translate render target desc

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
            pipeline_layout,
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
        let descriptor_set_layout = AnyArc::new_cyclic(move |v| DescriptorSetLayout {
            this: v.clone(),
            visibility: desc.visibility,
            items: desc.items.to_vec(),
        });
        Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(
            descriptor_set_layout,
            |v| v,
        ))
    }

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        let mut resource_desc = dx12::ResourceDesc {
            // Fields that will be the same regardless of the requested buffer desc
            dimension: dx12::ResourceDimension::Buffer,
            layout: dx12::TextureLayout::RowMajor,
            format: dxgi::Format::Unknown,
            alignment: 0,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 1,
            sample_desc: dxgi::SampleDesc {
                count: 1,
                quality: 0,
            },

            // Fields based on the description
            width: 0,
            flags: dx12::ResourceFlags::NONE,
        };

        resource_desc.width = desc.size;

        if desc.allow_unordered_access {
            resource_desc.flags |= dx12::ResourceFlags::ALLOW_UNORDERED_ACCESS;
        }

        let (heap_type, initial_state) = match desc.cpu_access {
            CpuAccessMode::None => {
                (dx12::HeapType::Default, dx12::ResourceStates::COMMON) // TODO: Figure this out
            }
            CpuAccessMode::Read => (dx12::HeapType::ReadBack, dx12::ResourceStates::COPY_DEST),
            CpuAccessMode::Write => (dx12::HeapType::Upload, dx12::ResourceStates::GENERIC_READ),
        };

        let heap_properties = dx12::HeapProperties {
            r#type: heap_type,
            cpu_page_property: Default::default(),
            memory_pool_preference: Default::default(),
            creation_node_mask: 0,
            visible_node_mask: 0,
        };
        let resource = unsafe {
            self.device
                .create_committed_resource(
                    &heap_properties,
                    dx12::HeapFlags::NONE,
                    &resource_desc,
                    initial_state,
                    None,
                )
                .map_err(|v| anyhow!(v))?
        };

        let buffer = AnyArc::new_cyclic(move |v| Buffer {
            this: v.clone(),
            resource,
            desc: desc.clone(),
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(buffer, |v| v))
    }

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        let heap_properties = dx12::HeapProperties {
            r#type: dx12::HeapType::Default,
            ..Default::default()
        };
        let heap_flags = dx12::HeapFlags::NONE;
        let resource_desc = texture_create_desc_to_dx12(desc)?;
        let initial_state = resource_state_to_dx12(desc.initial_state);
        let optimized_clear_value = texture_create_clear_value_to_dx12(desc, resource_desc.format)?;

        let resource = unsafe {
            self.device
                .create_committed_resource(
                    &heap_properties,
                    heap_flags,
                    &resource_desc,
                    initial_state,
                    optimized_clear_value,
                )
                .map_err(|v| anyhow!(v))?
        };

        let texture = AnyArc::new_cyclic(move |v| Texture {
            this: v.clone(),
            device: self.this.upgrade().unwrap(),
            resource,
            desc: desc.clone(),
            dxgi_format: resource_desc.format,
            rtv_cache: RwLock::new(HashMap::new()),
            dsv_cache: RwLock::new(HashMap::new()),
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

    unsafe fn general_queue_submit_list(
        &self,
        command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), QueueSubmitError> {
        let queue = self
            .queues
            .general
            .as_ref()
            .ok_or(QueueSubmitError::QueueNotAvailable(QueueType::General))?;

        let command_list: Box<GeneralCommandList> = command_list
            .query_interface::<GeneralCommandList>()
            .ok()
            .unwrap();

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = queue.submit_lock.lock();
            queue
                .handle
                .execute_command_lists(&[command_list.list.as_weak()]);

            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        queue.in_flight.push(InFlightCommandList {
            index,
            list: command_list,
        });

        Ok(())
    }

    unsafe fn general_queue_submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), QueueSubmitError> {
        let queue = self
            .queues
            .general
            .as_ref()
            .ok_or(QueueSubmitError::QueueNotAvailable(QueueType::General))?;

        // Perform the actual submit operation
        let lists: Vec<Box<GeneralCommandList>> = command_lists
            .map(|v| v.query_interface::<GeneralCommandList>().ok().unwrap())
            .collect();

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = queue.submit_lock.lock();

            let handles: Vec<dx12::GraphicsCommandList> =
                lists.iter().map(|v| v.list.clone()).collect();

            queue.handle.execute_command_lists_strong(&handles);

            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        for list in lists {
            queue.in_flight.push(InFlightCommandList { index, list });
        }

        Ok(())
    }

    unsafe fn general_queue_present(
        &self,
        texture: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError> {
        let image = texture.query_interface::<AcquiredTexture>().ok().unwrap();

        if !image
            .swap_chain
            .present_supported_on_queue(QueueType::General)
        {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                QueueType::General,
            ));
        }

        let queue = self
            .queues
            .general
            .as_ref()
            .ok_or(QueuePresentError::QueueNotAvailable(QueueType::General))?;

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let _index = {
            let _lock = queue.submit_lock.lock();

            image
                .swap_chain
                .swap_chain
                .present(0, 0)
                .map_err(|v| anyhow!(v))?;
            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            // TODO: We need to track the lifetime of this operation and extend the swap image's
            //       lifetime until the present operation is complete.

            index
        };

        Ok(())
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
        let topo = match desc.input_assembly_state.primitive_topology {
            PrimitiveTopology::PointList => {
                builder = builder.primitive_topology_type(dx12::PrimitiveTopologyType::Point);
                dx12::PrimitiveTopology::PointList
            }
            PrimitiveTopology::LineList => {
                builder = builder.primitive_topology_type(dx12::PrimitiveTopologyType::Line);
                dx12::PrimitiveTopology::LineList
            }
            PrimitiveTopology::LineStrip => {
                builder = builder.primitive_topology_type(dx12::PrimitiveTopologyType::Line);
                dx12::PrimitiveTopology::LineStrip
            }
            PrimitiveTopology::TriangleList => {
                builder = builder.primitive_topology_type(dx12::PrimitiveTopologyType::Triangle);
                dx12::PrimitiveTopology::TriangleList
            }
            PrimitiveTopology::TriangleStrip => {
                builder = builder.primitive_topology_type(dx12::PrimitiveTopologyType::Triangle);
                dx12::PrimitiveTopology::TriangleStrip
            }
        };
        (builder, topo)
    }

    /// Internal function for translating the [RasterizerStateDesc] field of a pipeline
    /// description
    fn translate_rasterizer_state_desc(desc: &RasterizerStateDesc) -> dx12::RasterizerDesc {
        let fill_mode = match desc.polygon_mode {
            PolygonMode::Fill => dx12::FillMode::Solid,
            PolygonMode::Line => dx12::FillMode::Wireframe,
        };
        let cull_mode = match desc.cull_mode {
            CullMode::None => dx12::CullMode::None,
            CullMode::Back => dx12::CullMode::Back,
            CullMode::Front => dx12::CullMode::Front,
        };
        let front_counter_clockwise = match desc.front_face {
            FrontFaceOrder::CounterClockwise => dx12::Bool::TRUE,
            FrontFaceOrder::Clockwise => dx12::Bool::FALSE,
        };
        dx12::RasterizerDesc {
            fill_mode,
            cull_mode,
            front_counter_clockwise,
            depth_bias: todo!(),
            depth_bias_clamp: todo!(),
            slope_scaled_depth_bias: todo!(),
            depth_clip_enable: todo!(),
            multisample_enable: todo!(),
            antialiased_line_enable: dx12::Bool::FALSE,
            forced_sample_count: 0,
            conservative_raster: dx12::ConservativeRasterizationMode::Off,
        }
    }

    /// Internal function for translating the [DepthStencilStateDesc] field of a pipeline
    /// description
    fn translate_depth_stencil_desc(desc: &DepthStencilStateDesc) -> dx12::DepthStencilDesc {
        /// Internal function for translating our [CompareOp] to the D3D12 equivalent
        fn translate_compare_op(op: CompareOp) -> dx12::ComparisonFunc {
            match op {
                CompareOp::Never => dx12::ComparisonFunc::Never,
                CompareOp::Always => dx12::ComparisonFunc::Always,
                CompareOp::Equal => dx12::ComparisonFunc::Equal,
                CompareOp::NotEqual => dx12::ComparisonFunc::NotEqual,
                CompareOp::Less => dx12::ComparisonFunc::Less,
                CompareOp::LessEqual => dx12::ComparisonFunc::LessEqual,
                CompareOp::Greater => dx12::ComparisonFunc::Greater,
                CompareOp::GreaterOrEqual => dx12::ComparisonFunc::GreaterEqual,
            }
        }

        /// Internal function for translating our [StencilOpState] into the D3D12 equivalent
        fn translate_depth_stencil_op_desc(desc: &StencilOpState) -> dx12::DepthStencilOpDesc {
            /// Internal function for translating our [StencilOp] into the D3D12 equivalent
            fn translate_stencil_op(op: StencilOp) -> dx12::StencilOp {
                match op {
                    StencilOp::Keep => dx12::StencilOp::Keep,
                    StencilOp::Zero => dx12::StencilOp::Zero,
                    StencilOp::Replace => dx12::StencilOp::Replace,
                    StencilOp::IncrementClamp => dx12::StencilOp::IncrementSaturate,
                    StencilOp::DecrementClamp => dx12::StencilOp::DecrementSaturate,
                    StencilOp::Invert => dx12::StencilOp::Invert,
                    StencilOp::IncrementWrap => dx12::StencilOp::Increment,
                    StencilOp::DecrementWrap => dx12::StencilOp::Decrement,
                }
            }
            let stencil_fail_op = translate_stencil_op(desc.fail_op);
            let stencil_depth_fail_op = translate_stencil_op(desc.depth_fail_op);
            let stencil_pass_op = translate_stencil_op(desc.pass_op);
            let stencil_func = translate_compare_op(desc.compare_op);
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
        let depth_func = translate_compare_op(desc.depth_compare_op);
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
        fn translate_blend_factor(factor: BlendFactor) -> dx12::Blend {
            match factor {
                BlendFactor::Zero => dx12::Blend::Zero,
                BlendFactor::One => dx12::Blend::One,
                BlendFactor::SrcColor => dx12::Blend::SrcColor,
                BlendFactor::OneMinusSrcColor => dx12::Blend::SrcColorInv,
                BlendFactor::DstColor => dx12::Blend::DestColor,
                BlendFactor::OneMinusDstColor => dx12::Blend::DestColorInv,
                BlendFactor::SrcAlpha => dx12::Blend::SrcAlpha,
                BlendFactor::OneMinusSrcAlpha => dx12::Blend::SrcAlphaInv,
                BlendFactor::DstAlpha => dx12::Blend::DestAlpha,
                BlendFactor::OneMinusDstAlpha => dx12::Blend::DestAlphaInv,
                BlendFactor::SrcAlphaSaturate => dx12::Blend::SrcAlphaSaturated,
                BlendFactor::BlendFactor => dx12::Blend::BlendFactor,
                BlendFactor::OneMinusBlendFactor => dx12::Blend::BlendFactorInv,
            }
        }

        fn translate_blend_op(op: BlendOp) -> dx12::BlendOp {
            match op {
                BlendOp::Add => dx12::BlendOp::Add,
                BlendOp::Subtract => dx12::BlendOp::Subtract,
                BlendOp::ReverseSubtract => dx12::BlendOp::SubtractReverse,
                BlendOp::Min => dx12::BlendOp::Min,
                BlendOp::Max => dx12::BlendOp::Max,
            }
        }

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

            let src_blend = translate_blend_factor(attachment.src_factor);
            let dest_blend = translate_blend_factor(attachment.dst_factor);
            let blend_op = translate_blend_op(attachment.blend_op);

            let src_blend_alpha = translate_blend_factor(attachment.alpha_src_factor);
            let dest_blend_alpha = translate_blend_factor(attachment.alpha_dst_factor);
            let blend_op_alpha = translate_blend_op(attachment.alpha_blend_op);

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
        self.queues.general.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_compute_queue(&self) -> Option<dx12::CommandQueue> {
        self.queues.compute.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_transfer_queue(&self) -> Option<dx12::CommandQueue> {
        self.queues.transfer.as_ref().map(|v| v.handle.clone())
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl INamedObject for Device {
    fn set_name(&self, name: &str) {
        self.device.set_name(name).unwrap()
    }
}

/// Internal struct that logically associates all queues into a single block
///
/// # Info
///
/// I'm not sure if I need a mutex on D3D12, but vkQueue requires external synchronization so I am
/// just going to be safe for now and lock for the D3D12 backend too for now.
///
/// I can just remove them later
pub struct Queues {
    pub general: Option<Queue<GeneralCommandList>>,
    pub compute: Option<Queue<()>>,
    pub transfer: Option<Queue<()>>,
}
