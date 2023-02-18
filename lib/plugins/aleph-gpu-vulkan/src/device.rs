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
use crate::context::Context;
use crate::internal::conv::texture_format_to_vk;
use crate::internal::queues::Queues;
use crate::shader::Shader;
use byteorder::{ByteOrder, NativeEndian};
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BlendFactor, BlendOp, BufferCreateError, BufferDesc, CommandPoolCreateError,
    CompareOp, ComputePipelineCreateError, ComputePipelineDesc, CullMode,
    DescriptorPoolCreateError, DescriptorSetLayoutCreateError, DescriptorSetLayoutDesc,
    DescriptorWriteDesc, FrontFaceOrder, GraphicsPipelineCreateError, GraphicsPipelineDesc,
    IBuffer, ICommandPool, IComputePipeline, IDescriptorPool, IDescriptorSetLayout, IDevice,
    IFence, IGetPlatformInterface, IGraphicsPipeline, IPipelineLayout, IQueue, ISampler, IShader,
    ITexture, PipelineLayoutCreateError, PipelineLayoutDesc, PolygonMode, PrimitiveTopology,
    QueueType, SamplerCreateError, SamplerDesc, ShaderBinary, ShaderCreateError, ShaderOptions,
    StencilOp, StencilOpState, TextureCreateError, TextureDesc, VertexInputRate,
};
use std::any::TypeId;
use std::ffi::CString;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device_loader: erupt::DeviceLoader,
    pub(crate) queues: Queues,
}

declare_interfaces!(Device, [IDevice]);

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
        todo!()
    }

    fn wait_idle(&self) {
        todo!()
    }

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        let builder = vk::GraphicsPipelineCreateInfoBuilder::new();

        // Translate the vertex input state
        let vertex_binding_descriptions: Vec<_> = desc
            .vertex_layout
            .input_bindings
            .iter()
            .map(|v| {
                let input_rate = match v.input_rate {
                    VertexInputRate::PerVertex => vk::VertexInputRate::VERTEX,
                    VertexInputRate::PerInstance => vk::VertexInputRate::INSTANCE,
                };
                vk::VertexInputBindingDescriptionBuilder::new()
                    .binding(v.binding)
                    .stride(v.stride)
                    .input_rate(input_rate)
            })
            .collect();
        let vertex_attribute_descriptions: Vec<_> = desc
            .vertex_layout
            .input_attributes
            .iter()
            .map(|v| {
                vk::VertexInputAttributeDescriptionBuilder::new()
                    .location(v.location)
                    .binding(v.binding)
                    .offset(v.offset)
                    .format(texture_format_to_vk(v.format))
            })
            .collect();
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(&vertex_binding_descriptions)
            .vertex_attribute_descriptions(&vertex_attribute_descriptions);
        let builder = builder.vertex_input_state(&vertex_input_state);

        // Translate the input assembly state
        let topology = match desc.input_assembly_state.primitive_topology {
            PrimitiveTopology::PointList => vk::PrimitiveTopology::POINT_LIST,
            PrimitiveTopology::LineList => vk::PrimitiveTopology::LINE_LIST,
            PrimitiveTopology::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
            PrimitiveTopology::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
        };
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(topology)
            .primitive_restart_enable(false);
        let builder = builder.input_assembly_state(&input_assembly_state);

        // Translate the rasterization state
        let polygon_mode = match desc.rasterizer_state.polygon_mode {
            PolygonMode::Fill => vk::PolygonMode::FILL,
            PolygonMode::Line => vk::PolygonMode::LINE,
        };
        let cull_mode = match desc.rasterizer_state.cull_mode {
            CullMode::None => vk::CullModeFlags::NONE,
            CullMode::Back => vk::CullModeFlags::BACK,
            CullMode::Front => vk::CullModeFlags::FRONT,
        };
        let front_face = match desc.rasterizer_state.front_face {
            FrontFaceOrder::CounterClockwise => vk::FrontFace::COUNTER_CLOCKWISE,
            FrontFaceOrder::Clockwise => vk::FrontFace::CLOCKWISE,
        };
        let rasterization_state = vk::PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(polygon_mode)
            .cull_mode(cull_mode)
            .front_face(front_face)
            .depth_clamp_enable(todo!())
            .rasterizer_discard_enable(false) // No support in dx12
            .depth_bias_enable(todo!())
            .depth_bias_constant_factor(todo!())
            .depth_bias_clamp(0.0) // TODO: Why?
            .depth_bias_slope_factor(todo!())
            .line_width(1.0);
        let builder = builder.rasterization_state(&rasterization_state);

        // Translate the depth/stencil state
        const fn translate_compare_op(op: CompareOp) -> vk::CompareOp {
            match op {
                CompareOp::Never => vk::CompareOp::NEVER,
                CompareOp::Always => vk::CompareOp::ALWAYS,
                CompareOp::Equal => vk::CompareOp::EQUAL,
                CompareOp::NotEqual => vk::CompareOp::NOT_EQUAL,
                CompareOp::Less => vk::CompareOp::LESS,
                CompareOp::LessEqual => vk::CompareOp::LESS_OR_EQUAL,
                CompareOp::Greater => vk::CompareOp::GREATER,
                CompareOp::GreaterOrEqual => vk::CompareOp::GREATER_OR_EQUAL,
            }
        }
        const fn translate_stencil_op_state(
            state: &StencilOpState,
            compare_mask: u32,
            write_mask: u32,
        ) -> vk::StencilOpState {
            const fn translate_stencil_op(op: StencilOp) -> vk::StencilOp {
                match op {
                    StencilOp::Keep => vk::StencilOp::KEEP,
                    StencilOp::Zero => vk::StencilOp::ZERO,
                    StencilOp::Replace => vk::StencilOp::REPLACE,
                    StencilOp::IncrementClamp => vk::StencilOp::INCREMENT_AND_CLAMP,
                    StencilOp::DecrementClamp => vk::StencilOp::DECREMENT_AND_CLAMP,
                    StencilOp::Invert => vk::StencilOp::INVERT,
                    StencilOp::IncrementWrap => vk::StencilOp::INCREMENT_AND_WRAP,
                    StencilOp::DecrementWrap => vk::StencilOp::DECREMENT_AND_WRAP,
                }
            }
            vk::StencilOpState {
                fail_op: translate_stencil_op(state.fail_op),
                pass_op: translate_stencil_op(state.pass_op),
                depth_fail_op: translate_stencil_op(state.depth_fail_op),
                compare_op: translate_compare_op(state.compare_op),
                compare_mask,
                write_mask,
                reference: 0,
            }
        }
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_test_enable(desc.depth_stencil_state.depth_test)
            .depth_write_enable(desc.depth_stencil_state.depth_write)
            .depth_compare_op(translate_compare_op(
                desc.depth_stencil_state.depth_compare_op,
            ))
            .stencil_test_enable(desc.depth_stencil_state.stencil_test)
            .front(translate_stencil_op_state(
                &desc.depth_stencil_state.stencil_front,
                desc.depth_stencil_state.stencil_read_mask as _,
                desc.depth_stencil_state.stencil_write_mask as _,
            ))
            .back(translate_stencil_op_state(
                &desc.depth_stencil_state.stencil_back,
                desc.depth_stencil_state.stencil_read_mask as _,
                desc.depth_stencil_state.stencil_write_mask as _,
            ))
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0);
        let builder = builder.depth_stencil_state(&depth_stencil_state);

        // Translate color blend state
        let attachments: Vec<_> = desc
            .blend_state
            .attachments
            .iter()
            .map(|v| {
                const fn translate_blend_factor(factor: BlendFactor) -> vk::BlendFactor {
                    match factor {
                        BlendFactor::Zero => vk::BlendFactor::ZERO,
                        BlendFactor::One => vk::BlendFactor::ONE,
                        BlendFactor::SrcColor => vk::BlendFactor::SRC_COLOR,
                        BlendFactor::OneMinusSrcColor => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
                        BlendFactor::DstColor => vk::BlendFactor::DST_COLOR,
                        BlendFactor::OneMinusDstColor => vk::BlendFactor::ONE_MINUS_DST_COLOR,
                        BlendFactor::SrcAlpha => vk::BlendFactor::SRC_ALPHA,
                        BlendFactor::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                        BlendFactor::DstAlpha => vk::BlendFactor::DST_ALPHA,
                        BlendFactor::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
                        BlendFactor::SrcAlphaSaturate => vk::BlendFactor::SRC_ALPHA_SATURATE,
                        BlendFactor::BlendFactor => vk::BlendFactor::CONSTANT_COLOR,
                        BlendFactor::OneMinusBlendFactor => {
                            vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR
                        }
                    }
                }
                const fn translate_blend_op(op: BlendOp) -> vk::BlendOp {
                    match op {
                        BlendOp::Add => vk::BlendOp::ADD,
                        BlendOp::Subtract => vk::BlendOp::SUBTRACT,
                        BlendOp::ReverseSubtract => vk::BlendOp::REVERSE_SUBTRACT,
                        BlendOp::Min => vk::BlendOp::MIN,
                        BlendOp::Max => vk::BlendOp::MAX,
                    }
                }
                vk::PipelineColorBlendAttachmentStateBuilder::new()
                    .blend_enable(v.blend_enabled)
                    .src_color_blend_factor(translate_blend_factor(v.src_factor))
                    .dst_color_blend_factor(translate_blend_factor(v.dst_factor))
                    .color_blend_op(translate_blend_op(v.blend_op))
                    .src_alpha_blend_factor(translate_blend_factor(v.alpha_src_factor))
                    .dst_alpha_blend_factor(translate_blend_factor(v.alpha_dst_factor))
                    .alpha_blend_op(translate_blend_op(v.alpha_blend_op))
                    .color_write_mask(vk::ColorComponentFlags::from_bits_truncate(
                        v.color_write_mask.bits() as _,
                    ))
            })
            .collect();
        let color_blend_state = vk::PipelineColorBlendStateCreateInfoBuilder::new()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);
        let builder = builder.color_blend_state(&color_blend_state);

        let _todo = unsafe {
            self.device_loader.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[builder],
                None,
            )
        };

        todo!()
    }

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        let module = desc
            .shader_module
            .query_interface::<Shader>()
            .expect("Unknown IShader implementation");

        let builder = vk::ComputePipelineCreateInfoBuilder::new();

        // TODO: Pipeline layout

        let builder = builder.stage(
            vk::PipelineShaderStageCreateInfoBuilder::new()
                .stage(vk::ShaderStageFlagBits::COMPUTE)
                .module(module.module)
                .name(&module.entry_point)
                .build_dangling(),
        );

        let _todo = unsafe {
            self.device_loader
                .create_compute_pipelines(vk::PipelineCache::null(), &[builder], None)
        };
        todo!()
    }

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError> {
        if let ShaderBinary::Spirv(data) = options.data {
            // Vulkan shaders must always have a buffer length that is a multiple of 4. SPIR-V's binary
            // representation is a sequence of u32 values.
            if data.len() % 4 != 0 || data.is_empty() {
                return Err(ShaderCreateError::InvalidInputSize(data.len()));
            }

            // We need to copy the data into a u32 buffer to satisfy alignment requirements
            let data: Vec<u32> = data.chunks_exact(4).map(NativeEndian::read_u32).collect();

            let module = unsafe {
                let create_info = vk::ShaderModuleCreateInfoBuilder::new().code(&data);
                self.device_loader
                    .create_shader_module(&create_info, None)
                    .map_err(|v| anyhow!(v))?
            };

            let entry_point = CString::new(options.entry_point)
                .map_err(|_| ShaderCreateError::InvalidEntryPointName)?;

            let shader = AnyArc::new_cyclic(move |v| Shader {
                this: v.clone(),
                device: self.this.upgrade().unwrap(),
                shader_type: options.shader_type,
                module,
                entry_point,
            });
            Ok(AnyArc::map::<dyn IShader, _>(shader, |v| v))
        } else {
            Err(ShaderCreateError::UnsupportedShaderFormat)
        }
    }

    fn create_descriptor_set_layout(
        &self,
        _desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        todo!()
    }

    fn create_descriptor_pool(
        &self,
        _layout: &dyn IDescriptorSetLayout,
        _capacity: u32,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        todo!()
    }

    fn create_pipeline_layout(
        &self,
        _desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        todo!()
    }

    fn create_buffer(&self, _desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        todo!()
    }

    fn create_texture(
        &self,
        _desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        todo!()
    }

    fn create_sampler(
        &self,
        _desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        todo!()
    }

    fn create_command_pool(&self) -> Result<AnyArc<dyn ICommandPool>, CommandPoolCreateError> {
        todo!()
    }

    fn get_queue(&self, _queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        todo!()
    }

    unsafe fn update_descriptor_sets(&self, _writes: &[DescriptorWriteDesc]) {
        todo!()
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device_loader.destroy_device(None);
        }
    }
}

impl IGetPlatformInterface for Device {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: Expose the device loader through an arc or something
        // TODO: Expose the queues, somewhere (likely on a queue object)
        None
    }
}

impl Device {
    fn set_name(&self, name: &str) {
        let loader = &self.device_loader;
        if let Some(func) = loader.set_debug_utils_object_name_ext {
            let name = CString::new(name).unwrap();
            let info = vk::DebugUtilsObjectNameInfoEXTBuilder::new()
                .object_type(vk::ObjectType::DEVICE)
                .object_handle(self.device_loader.handle.object_handle())
                .object_name(&name);
            unsafe {
                (func)(loader.handle, &info.build_dangling());
            }
        }
    }
}
