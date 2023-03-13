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
use crate::fence::Fence;
use crate::internal::conv::{
    blend_factor_to_vk, blend_op_to_vk, compare_op_to_vk, cull_mode_to_vk, front_face_order_to_vk,
    polygon_mode_to_vk, primitive_topology_to_vk, stencil_op_to_vk, texture_format_to_vk,
    vertex_input_rate_to_vk,
};
use crate::internal::queues::Queues;
use crate::internal::set_name::set_name;
use crate::semaphore::Semaphore;
use crate::shader::Shader;
use byteorder::{ByteOrder, NativeEndian};
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
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
    // ========================================================================================== //
    // ========================================================================================== //

    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self.this.upgrade().unwrap(), |v| v)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn garbage_collect(&self) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        let builder = vk::GraphicsPipelineCreateInfoBuilder::new();

        // Translate the vertex input state
        let vertex_binding_descriptions: Vec<_> = Self::translate_vertex_bindings(desc);
        let vertex_attribute_descriptions: Vec<_> = Self::translate_vertex_attributes(desc);
        let vertex_input_state = Self::translate_vertex_input_state(
            &vertex_binding_descriptions,
            &vertex_attribute_descriptions,
        );

        let input_assembly_state = Self::translate_input_assembly_state(desc);
        let rasterization_state = Self::translate_rasterization_state(desc);
        let depth_stencil_state = Self::translate_depth_stencil_state(desc);

        let attachments = Self::translate_color_attachment_state(desc);
        let color_blend_state = Self::translate_color_blend_state(&attachments);

        let builder = builder.vertex_input_state(&vertex_input_state);
        let builder = builder.input_assembly_state(&input_assembly_state);
        let builder = builder.rasterization_state(&rasterization_state);
        let builder = builder.depth_stencil_state(&depth_stencil_state);
        let builder = builder.color_blend_state(&color_blend_state);

        let pipeline = unsafe {
            self.device_loader
                .create_graphics_pipelines(vk::PipelineCache::null(), &[builder], None)
                .map_err(|v| anyhow!(v))?
        };
        let pipeline = pipeline[0];

        set_name(&self.device_loader, pipeline, desc.name);

        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

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

        let pipeline = unsafe {
            self.device_loader
                .create_compute_pipelines(vk::PipelineCache::null(), &[builder], None)
                .map_err(|v| anyhow!(v))?
        };
        let pipeline = pipeline[0];

        set_name(&self.device_loader, pipeline, desc.name);

        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

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

            set_name(&self.device_loader, module, options.name);

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

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        _desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        _desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        _desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, _desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        _desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        _desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        _desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_queue(&self, _queue_type: QueueType) -> Option<AnyArc<dyn IQueue>> {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, _writes: &[DescriptorWriteDesc]) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self) -> Result<AnyArc<dyn IFence>, FenceCreateError> {
        let fence = unsafe {
            let info = vk::FenceCreateInfoBuilder::new();
            self.device_loader
                .create_fence(&info, None)
                .map_err(|v| anyhow!(v))?
        };

        let fence = AnyArc::new_cyclic(move |v| Fence {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            fence,
        });
        Ok(AnyArc::map::<dyn IFence, _>(fence, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<AnyArc<dyn ISemaphore>, SemaphoreCreateError> {
        let semaphore = unsafe {
            let info = vk::SemaphoreCreateInfoBuilder::new();
            self.device_loader
                .create_semaphore(&info, None)
                .map_err(|v| anyhow!(v))?
        };

        let semaphore = AnyArc::new_cyclic(move |v| Semaphore {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            semaphore,
        });
        Ok(AnyArc::map::<dyn ISemaphore, _>(semaphore, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(&self, fences: &[&dyn IFence], wait_all: bool, timeout: u32) -> FenceWaitResult {
        let timeout = if timeout == u32::MAX {
            u64::MAX
        } else {
            timeout as u64 * 1000000 // Convert to nanoseconds
        };

        let fences: Vec<_> = fences
            .iter()
            .map(|v| {
                v.query_interface::<Fence>()
                    .expect("Unknown IFence implementation")
                    .fence
            })
            .collect();

        let result = unsafe {
            self.device_loader
                .wait_for_fences(&fences, wait_all, timeout)
        };

        match result.raw {
            vk::Result::SUCCESS => FenceWaitResult::Complete,
            vk::Result::TIMEOUT => FenceWaitResult::Timeout,
            _ => {
                result.unwrap();
                unreachable!()
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &dyn IFence) -> bool {
        let fence = fence
            .query_interface::<Fence>()
            .expect("Unknown IFence implementation");

        let result = unsafe { self.device_loader.get_fence_status(fence.fence) };

        match result.raw {
            vk::Result::SUCCESS => true,
            vk::Result::NOT_READY => false,
            _ => {
                result.unwrap();
                unreachable!()
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, fences: &[&dyn IFence]) {
        let fences: Vec<_> = fences
            .iter()
            .map(|v| {
                v.query_interface::<Fence>()
                    .expect("Unknown IFence implementation")
                    .fence
            })
            .collect();

        unsafe { self.device_loader.reset_fences(&fences).unwrap() }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }
}

impl Device {
    fn translate_vertex_bindings(
        desc: &GraphicsPipelineDesc,
    ) -> Vec<vk::VertexInputBindingDescriptionBuilder<'static>> {
        desc.vertex_layout
            .input_bindings
            .iter()
            .map(|v| {
                vk::VertexInputBindingDescriptionBuilder::new()
                    .binding(v.binding)
                    .stride(v.stride)
                    .input_rate(vertex_input_rate_to_vk(v.input_rate))
            })
            .collect()
    }

    fn translate_vertex_attributes(
        desc: &GraphicsPipelineDesc,
    ) -> Vec<vk::VertexInputAttributeDescriptionBuilder<'static>> {
        desc.vertex_layout
            .input_attributes
            .iter()
            .map(|v| {
                vk::VertexInputAttributeDescriptionBuilder::new()
                    .location(v.location)
                    .binding(v.binding)
                    .offset(v.offset)
                    .format(texture_format_to_vk(v.format))
            })
            .collect()
    }

    fn translate_vertex_input_state<'a>(
        vertex_binding_descriptions: &'a [vk::VertexInputBindingDescriptionBuilder],
        vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescriptionBuilder],
    ) -> vk::PipelineVertexInputStateCreateInfoBuilder<'a> {
        vk::PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(vertex_binding_descriptions)
            .vertex_attribute_descriptions(vertex_attribute_descriptions)
    }

    fn translate_input_assembly_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineInputAssemblyStateCreateInfoBuilder<'static> {
        let topology = primitive_topology_to_vk(desc.input_assembly_state.primitive_topology);
        vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(topology)
            .primitive_restart_enable(false)
    }

    fn translate_rasterization_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineRasterizationStateCreateInfoBuilder<'static> {
        let polygon_mode = polygon_mode_to_vk(desc.rasterizer_state.polygon_mode);
        let cull_mode = cull_mode_to_vk(desc.rasterizer_state.cull_mode);
        let front_face = front_face_order_to_vk(desc.rasterizer_state.front_face);
        vk::PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(polygon_mode)
            .cull_mode(cull_mode)
            .front_face(front_face)
            .depth_clamp_enable(true)
            .rasterizer_discard_enable(false) // No support in dx12
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .line_width(1.0)
    }

    fn translate_depth_stencil_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineDepthStencilStateCreateInfoBuilder<'static> {
        const fn translate_stencil_op_state(
            state: &StencilOpState,
            compare_mask: u32,
            write_mask: u32,
        ) -> vk::StencilOpState {
            vk::StencilOpState {
                fail_op: stencil_op_to_vk(state.fail_op),
                pass_op: stencil_op_to_vk(state.pass_op),
                depth_fail_op: stencil_op_to_vk(state.depth_fail_op),
                compare_op: compare_op_to_vk(state.compare_op),
                compare_mask,
                write_mask,
                reference: 0,
            }
        }

        vk::PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_test_enable(desc.depth_stencil_state.depth_test)
            .depth_write_enable(desc.depth_stencil_state.depth_write)
            .depth_compare_op(compare_op_to_vk(desc.depth_stencil_state.depth_compare_op))
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
            .depth_bounds_test_enable(desc.depth_stencil_state.depth_bounds_enable)
            .min_depth_bounds(desc.depth_stencil_state.min_depth_bounds)
            .max_depth_bounds(desc.depth_stencil_state.max_depth_bounds)
    }

    fn translate_color_attachment_state(
        desc: &GraphicsPipelineDesc,
    ) -> Vec<vk::PipelineColorBlendAttachmentStateBuilder<'static>> {
        desc.blend_state
            .attachments
            .iter()
            .map(|v| {
                vk::PipelineColorBlendAttachmentStateBuilder::new()
                    .blend_enable(v.blend_enabled)
                    .src_color_blend_factor(blend_factor_to_vk(v.src_factor))
                    .dst_color_blend_factor(blend_factor_to_vk(v.dst_factor))
                    .color_blend_op(blend_op_to_vk(v.blend_op))
                    .src_alpha_blend_factor(blend_factor_to_vk(v.alpha_src_factor))
                    .dst_alpha_blend_factor(blend_factor_to_vk(v.alpha_dst_factor))
                    .alpha_blend_op(blend_op_to_vk(v.alpha_blend_op))
                    .color_write_mask(vk::ColorComponentFlags::from_bits_truncate(
                        v.color_write_mask.bits() as _,
                    ))
            })
            .collect()
    }

    fn translate_color_blend_state<'a>(
        attachments: &'a [vk::PipelineColorBlendAttachmentStateBuilder],
    ) -> vk::PipelineColorBlendStateCreateInfoBuilder<'a> {
        vk::PipelineColorBlendStateCreateInfoBuilder::new()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
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
