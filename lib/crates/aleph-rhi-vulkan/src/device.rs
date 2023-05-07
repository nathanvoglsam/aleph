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
use crate::command_list::CommandList;
use crate::context::Context;
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::fence::Fence;
use crate::internal::conv::*;
use crate::internal::set_name::set_name;
use crate::internal::unwrap;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::PipelineLayout;
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::shader::Shader;
use crate::texture::Texture;
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use anyhow::anyhow;
use byteorder::{ByteOrder, NativeEndian};
use erupt::{vk, ExtendableFrom};
use std::any::TypeId;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use vulkan_alloc::vma;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device_loader: ManuallyDrop<erupt::DeviceLoader>,
    pub(crate) allocator: ManuallyDrop<vma::Allocator>,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
}

declare_interfaces!(Device, [IDevice]);

impl IGetPlatformInterface for Device {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: Expose the device loader through an arc or something
        None
    }
}

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
        if let Some(queue) = &self.general_queue {
            queue.garbage_collect();
        }
        if let Some(queue) = &self.compute_queue {
            queue.garbage_collect();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.garbage_collect();
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        // We need to take all of the queue locks to meet vulkan sync requirements.
        let _lock_ness_monster = (
            self.general_queue.as_ref().map(|v| v.submit_lock.lock()),
            self.compute_queue.as_ref().map(|v| v.submit_lock.lock()),
            self.transfer_queue.as_ref().map(|v| v.submit_lock.lock()),
        );

        unsafe { self.device_loader.device_wait_idle().unwrap() }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError> {
        let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout);

        let builder =
            vk::GraphicsPipelineCreateInfoBuilder::new().layout(pipeline_layout.pipeline_layout);

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(&dynamic_states);

        // Translate the vertex input state
        let vertex_binding_descriptions: Vec<_> = Self::translate_vertex_bindings(desc);
        let vertex_attribute_descriptions: Vec<_> = Self::translate_vertex_attributes(desc);
        let vertex_input_state = Self::translate_vertex_input_state(
            &vertex_binding_descriptions,
            &vertex_attribute_descriptions,
        );

        let viewport_state = vk::PipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(1)
            .scissor_count(1);
        let input_assembly_state = Self::translate_input_assembly_state(desc);
        let rasterization_state = Self::translate_rasterization_state(desc);
        let multisample_state = vk::PipelineMultisampleStateCreateInfoBuilder::new()
            .rasterization_samples(vk::SampleCountFlagBits::_1)
            .sample_shading_enable(false)
            .min_sample_shading(0.0)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);
        let depth_stencil_state = Self::translate_depth_stencil_state(desc);

        let mut color_formats = Vec::with_capacity(desc.render_target_formats.len());
        let mut dynamic_rendering = Self::translate_framebuffer_info(desc, &mut color_formats);

        let attachments = Self::translate_color_attachment_state(desc);
        let color_blend_state = Self::translate_color_blend_state(&attachments);

        let stages: Vec<_> = desc
            .shader_stages
            .iter()
            .map(unwrap::shader_d)
            .map(|v| {
                vk::PipelineShaderStageCreateInfoBuilder::new()
                    .stage(v.vk_shader_type)
                    .module(v.module)
                    .name(v.entry_point.as_ref())
            })
            .collect();

        let builder = builder.dynamic_state(&dynamic_state);
        let builder = builder.stages(&stages);
        let builder = builder.vertex_input_state(&vertex_input_state);
        let builder = builder.viewport_state(&viewport_state);
        let builder = builder.input_assembly_state(&input_assembly_state);
        let builder = builder.rasterization_state(&rasterization_state);
        let builder = builder.multisample_state(&multisample_state);
        let builder = builder.depth_stencil_state(&depth_stencil_state);
        let builder = builder.extend_from(&mut dynamic_rendering);
        let builder = builder.color_blend_state(&color_blend_state);

        let pipeline = unsafe {
            self.device_loader
                .create_graphics_pipelines(vk::PipelineCache::null(), &[builder], None)
                .map_err(|v| anyhow!(v))?
        };
        let pipeline = pipeline[0];

        set_name(&self.device_loader, pipeline, desc.name);

        let out = AnyArc::new_cyclic(move |v| GraphicsPipeline {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
            pipeline,
        });
        Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError> {
        let module = unwrap::shader(desc.shader_module);
        let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout);

        let builder = vk::ComputePipelineCreateInfoBuilder::new()
            .layout(pipeline_layout.pipeline_layout)
            .stage(
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

        let out = AnyArc::new_cyclic(move |v| ComputePipeline {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
            pipeline,
        });
        Ok(AnyArc::map::<dyn IComputePipeline, _>(out, |v| v))
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
                vk_shader_type: shader_type_to_vk(options.shader_type),
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
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        let stage_flags = descriptor_shader_visibility_to_vk(desc.visibility);

        let mut _samplers = Vec::new();
        let mut static_samplers = Vec::new();
        for v in desc.items {
            if let Some(samplers) = v.static_samplers {
                for sampler in unwrap::sampler_iter(samplers) {
                    _samplers.push(sampler._this.upgrade().unwrap());
                    static_samplers.push(sampler.sampler);
                }
            }
        }

        let mut sampler_i = 0;
        let mut sizes = [0; 11];
        let mut bindings = Vec::with_capacity(desc.items.len());
        for v in desc.items {
            let descriptor_type = descriptor_type_to_vk(v.binding_type);
            let descriptor_count = v.binding_count.map(|v| v.get()).unwrap_or(1);

            sizes[descriptor_type.0 as usize] += descriptor_count;

            let binding = vk::DescriptorSetLayoutBindingBuilder::new()
                .binding(v.binding_num)
                .descriptor_type(descriptor_type)
                .descriptor_count(descriptor_count)
                .stage_flags(stage_flags);

            let binding = if let Some(samplers) = v.static_samplers {
                let base = sampler_i;
                sampler_i += samplers.len();
                binding.immutable_samplers(&static_samplers[base..sampler_i])
            } else {
                binding
            };

            bindings.push(binding);
        }

        let mut pool_sizes = Vec::with_capacity(sizes.len());
        for (i, v) in sizes.iter().copied().enumerate() {
            // Accumulate any non-zero pool size into the list
            if v > 0 {
                pool_sizes.push(
                    vk::DescriptorPoolSizeBuilder::new()
                        ._type(vk::DescriptorType(i as i32))
                        .descriptor_count(v),
                );
            }
        }

        let create_info = vk::DescriptorSetLayoutCreateInfoBuilder::new().bindings(&bindings);

        let descriptor_set_layout = unsafe {
            self.device_loader
                .create_descriptor_set_layout(&create_info, None)
                .map_err(|v| anyhow!(v))?
        };

        set_name(&self.device_loader, descriptor_set_layout, desc.name);

        let out = AnyArc::new_cyclic(move |v| DescriptorSetLayout {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            _samplers,
            descriptor_set_layout,
            pool_sizes,
        });
        Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::descriptor_set_layout(desc.layout)
            ._this
            .upgrade()
            .unwrap();

        let mut pool_sizes = layout.pool_sizes.clone();
        for size in &mut pool_sizes {
            size.descriptor_count *= desc.num_sets;
        }

        let create_info = vk::DescriptorPoolCreateInfoBuilder::new()
            .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET) // TODO: perhaps this could be exposed to the API? D3D12 could do it easy as just a bump allocator
            .max_sets(desc.num_sets)
            .pool_sizes(&pool_sizes);

        let descriptor_pool = unsafe {
            self.device_loader
                .create_descriptor_pool(&create_info, None)
                .map_err(|v| anyhow!(v))?
        };

        set_name(&self.device_loader, descriptor_pool, desc.name);

        let pool = Box::new(DescriptorPool {
            _device: self.this.upgrade().unwrap(),
            _layout: layout,
            descriptor_pool,
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        let mut set_layouts = Vec::with_capacity(desc.set_layouts.len());
        for v in desc.set_layouts {
            let v = unwrap::descriptor_set_layout_d(v);
            set_layouts.push(v.descriptor_set_layout);
        }

        let mut offset = 0;
        let mut ranges = Vec::with_capacity(desc.push_constant_blocks.len());
        for v in desc.push_constant_blocks {
            let range = vk::PushConstantRangeBuilder::new()
                .stage_flags(descriptor_shader_visibility_to_vk(v.visibility))
                .offset(offset)
                .size(v.size as u32);
            ranges.push(range);

            offset += v.size as u32;
        }

        let create_info = vk::PipelineLayoutCreateInfoBuilder::new()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&ranges);

        let pipeline_layout = unsafe {
            self.device_loader
                .create_pipeline_layout(&create_info, None)
                .map_err(|v| anyhow!(v))?
        };

        set_name(&self.device_loader, pipeline_layout, desc.name);

        let out = AnyArc::new_cyclic(move |v| PipelineLayout {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            pipeline_layout,
            push_constant_blocks: ranges,
        });
        Ok(AnyArc::map::<dyn IPipelineLayout, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        let mut usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;

        if desc.allow_unordered_access {
            usage |= vk::BufferUsageFlags::STORAGE_BUFFER;
        }
        if desc.allow_texel_buffer {
            if desc.allow_unordered_access {
                usage |= vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER
                    | vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER;
            } else {
                usage |= vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER;
            }
        }
        if desc.is_vertex_buffer {
            usage |= vk::BufferUsageFlags::VERTEX_BUFFER;
        }
        if desc.is_index_buffer {
            usage |= vk::BufferUsageFlags::INDEX_BUFFER;
        }
        if desc.is_constant_buffer {
            usage |= vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if desc.is_indirect_draw_args {
            usage |= vk::BufferUsageFlags::INDIRECT_BUFFER;
        }
        if desc.is_accel_struct_build_input {
            usage |= vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR;
        }
        if desc.is_accel_struct_storage {
            usage |= vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR;
        }

        let create_info = vk::BufferCreateInfoBuilder::new()
            .size(desc.size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let usage = match desc.cpu_access {
            CpuAccessMode::None => vma::MemoryUsage::GPUOnly,
            CpuAccessMode::Read => vma::MemoryUsage::GPUToCPU,
            CpuAccessMode::Write => vma::MemoryUsage::CPUToGPU,
        };
        let alloc_info = vma::AllocationCreateInfo::builder()
            .flags(vma::AllocationCreateFlags::empty())
            .usage(usage);

        let (buffer, allocation) = unsafe {
            self.allocator
                .create_buffer(&create_info, &alloc_info)
                .map_err(|v| anyhow!(v))?
        };

        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let out = AnyArc::new_cyclic(move |v| Buffer {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            buffer,
            allocation,
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        let image_type = match desc.dimension {
            TextureDimension::Texture1D => vk::ImageType::_1D,
            TextureDimension::Texture2D => vk::ImageType::_2D,
            TextureDimension::Texture3D => vk::ImageType::_3D,
        };

        let format = texture_format_to_vk(desc.format);

        let samples = match desc.sample_count {
            1 => vk::SampleCountFlagBits::_1,
            2 => vk::SampleCountFlagBits::_2,
            4 => vk::SampleCountFlagBits::_4,
            8 => vk::SampleCountFlagBits::_8,
            16 => vk::SampleCountFlagBits::_16,
            32 => vk::SampleCountFlagBits::_32,
            _ => return Err(TextureCreateError::InvalidSampleCount(desc.sample_count)),
        };

        let mut usage = vk::ImageUsageFlags::SAMPLED;
        if desc.allow_copy_dest {
            usage |= vk::ImageUsageFlags::TRANSFER_DST
        }
        if desc.allow_copy_source {
            usage |= vk::ImageUsageFlags::TRANSFER_SRC
        }
        if desc.allow_unordered_access {
            usage |= vk::ImageUsageFlags::STORAGE
        }
        if desc.is_render_target {
            if desc.format.is_depth_stencil() {
                usage |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
            } else {
                usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT
            }
        }

        let mut flags = vk::ImageCreateFlags::empty();
        if desc.allow_cube_face {
            flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
        }

        let create_info = vk::ImageCreateInfoBuilder::new()
            .flags(flags)
            .image_type(image_type)
            .format(format)
            .extent(vk::Extent3D {
                width: desc.width,
                height: desc.height,
                depth: desc.depth,
            })
            .mip_levels(desc.mip_levels)
            .array_layers(desc.array_size)
            .samples(samples)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED);

        let alloc_info = vma::AllocationCreateInfo::builder()
            .flags(vma::AllocationCreateFlags::empty())
            .usage(vma::MemoryUsage::GPUOnly);

        let (image, allocation) = unsafe {
            self.allocator
                .create_image(&create_info, &alloc_info)
                .map_err(|v| anyhow!(v))?
        };

        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let out = AnyArc::new_cyclic(move |v| Texture {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            image,
            allocation: Some(allocation),
            is_owned: true,
            views: Default::default(),
            rtvs: Default::default(),
            dsvs: Default::default(),
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn ITexture, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        let mut create_info = vk::SamplerCreateInfoBuilder::new()
            .mag_filter(sampler_filter_to_vk(desc.mag_filter))
            .min_filter(sampler_filter_to_vk(desc.min_filter))
            .mipmap_mode(sampler_mip_filter_to_vk(desc.mip_filter))
            .address_mode_u(sampler_address_mode_to_vk(desc.address_mode_u))
            .address_mode_v(sampler_address_mode_to_vk(desc.address_mode_v))
            .address_mode_w(sampler_address_mode_to_vk(desc.address_mode_w))
            .mip_lod_bias(desc.lod_bias)
            .anisotropy_enable(desc.enable_anisotropy)
            .max_anisotropy(desc.max_anisotropy as f32)
            .min_lod(desc.min_lod)
            .max_lod(desc.max_lod)
            .border_color(sampler_border_color_to_vk(desc.border_color))
            .unnormalized_coordinates(false);

        if let Some(v) = desc.compare_op {
            create_info = create_info
                .compare_enable(true)
                .compare_op(compare_op_to_vk(v))
        }

        let sampler = unsafe {
            self.device_loader
                .create_sampler(&create_info, None)
                .map_err(|v| anyhow!(v))?
        };

        set_name(&self.device_loader, sampler, desc.name);

        let name = desc.name.map(String::from);
        let out = AnyArc::new_cyclic(move |v| Sampler {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            sampler,
            desc: desc.clone().strip_name(),
            name,
        });
        Ok(AnyArc::map::<dyn ISampler, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let family_index = match desc.queue_type {
            QueueType::General => self.general_queue.as_ref().unwrap().info.family_index,
            QueueType::Compute => self.compute_queue.as_ref().unwrap().info.family_index,
            QueueType::Transfer => self.transfer_queue.as_ref().unwrap().info.family_index,
        };

        let create_info = vk::CommandPoolCreateInfoBuilder::new()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT)
            .queue_family_index(family_index);
        let command_pool = unsafe {
            self.device_loader
                .create_command_pool(&create_info, None)
                .map_err(|v| anyhow!(v))?
        };

        let allocate_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let command_buffer = unsafe {
            self.device_loader
                .allocate_command_buffers(&allocate_info)
                .map_err(|v| anyhow!(v))?
        };
        let command_buffer = command_buffer[0];

        set_name(&self.device_loader, command_pool, desc.name);
        set_name(&self.device_loader, command_buffer, desc.name);

        let out = Box::new(CommandList {
            _device: self.this.upgrade().unwrap(),
            pool: command_pool,
            buffer: command_buffer,
            list_type: desc.queue_type,
        });

        Ok(out)
    }

    // ========================================================================================== //
    // ========================================================================================== //

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

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]) {
        let mut image_infos = Vec::new();
        let mut buffer_infos = Vec::new();
        let mut texel_buffer_infos = Vec::new();
        for write in writes {
            match write.writes {
                DescriptorWrites::Sampler(v) => {
                    for v in v {
                        let image_info = vk::DescriptorImageInfoBuilder::new()
                            .sampler(unwrap::sampler(v.sampler).sampler);
                        image_infos.push(image_info);
                    }
                }
                DescriptorWrites::Image(v) => {
                    for v in v {
                        let image_info = vk::DescriptorImageInfoBuilder::new()
                            .image_view(std::mem::transmute(v.image_view))
                            .image_layout(image_layout_to_vk(v.image_layout));
                        image_infos.push(image_info);
                    }
                }
                DescriptorWrites::Buffer(v) => {
                    for v in v {
                        let buffer = unwrap::buffer(v.buffer).buffer;
                        let buffer_info = vk::DescriptorBufferInfoBuilder::new()
                            .buffer(buffer)
                            .offset(v.offset)
                            .range(v.len as _);
                        buffer_infos.push(buffer_info);
                    }
                }
                DescriptorWrites::StructuredBuffer(v) => {
                    for v in v {
                        let buffer = unwrap::buffer(v.buffer).buffer;
                        let buffer_info = vk::DescriptorBufferInfoBuilder::new()
                            .buffer(buffer)
                            .offset(v.offset)
                            .range(v.len as _);
                        buffer_infos.push(buffer_info);
                    }
                }
                DescriptorWrites::TexelBuffer(v) => {
                    for v in v {
                        texel_buffer_infos.push(vk::BufferView::null());
                    }
                }
                DescriptorWrites::InputAttachment(v) => {
                    for v in v {
                        let image_info = vk::DescriptorImageInfoBuilder::new()
                            .image_view(std::mem::transmute(v.image_view))
                            .image_layout(image_layout_to_vk(v.image_layout));
                        image_infos.push(image_info);
                    }
                }
            }
        }

        let mut image_info_idx = 0;
        let mut buffer_info_idx = 0;
        let mut texel_buffer_info_idx = 0;
        let mut descriptor_writes = Vec::with_capacity(writes.len());
        for write in writes {
            let d_type = match write.descriptor_type {
                DescriptorType::Sampler => vk::DescriptorType::SAMPLER,
                DescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
                DescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
                DescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
                DescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
                DescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
                DescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
                DescriptorType::StructuredBuffer => vk::DescriptorType::STORAGE_BUFFER,
                DescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
            };

            let new_write = vk::WriteDescriptorSetBuilder::new()
                .dst_set(std::mem::transmute(write.set.clone()))
                .dst_binding(write.binding)
                .dst_array_element(write.array_element);

            let new_write = match write.writes {
                DescriptorWrites::Sampler(v) => {
                    let base = image_info_idx;
                    image_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .image_info(&image_infos[base..image_info_idx])
                }
                DescriptorWrites::Image(v) => {
                    let base = image_info_idx;
                    image_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .image_info(&image_infos[base..image_info_idx])
                }
                DescriptorWrites::Buffer(v) => {
                    let base = buffer_info_idx;
                    buffer_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .buffer_info(&buffer_infos[base..buffer_info_idx])
                }
                DescriptorWrites::StructuredBuffer(v) => {
                    let base = buffer_info_idx;
                    image_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .buffer_info(&buffer_infos[base..buffer_info_idx])
                }
                DescriptorWrites::TexelBuffer(v) => {
                    let base = texel_buffer_info_idx;
                    texel_buffer_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .texel_buffer_view(&texel_buffer_infos[base..texel_buffer_info_idx])
                }
                DescriptorWrites::InputAttachment(v) => {
                    let base = image_info_idx;
                    image_info_idx += v.len();
                    new_write
                        .descriptor_type(d_type)
                        .image_info(&image_infos[base..image_info_idx])
                }
            };

            descriptor_writes.push(new_write);
        }

        self.device_loader
            .update_descriptor_sets(&descriptor_writes, &[]);
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

        let fences: Vec<_> = unwrap::fence_iter(fences).map(|v| v.fence).collect();

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
        let fence = unwrap::fence(fence);

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
        let fences: Vec<_> = unwrap::fence_iter(fences).map(|v| v.fence).collect();

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

    fn translate_framebuffer_info<'a, 'b>(
        desc: &'b GraphicsPipelineDesc,
        color_formats: &'a mut Vec<vk::Format>,
    ) -> vk::PipelineRenderingCreateInfoBuilder<'a> {
        let builder = vk::PipelineRenderingCreateInfoBuilder::new();

        let iter = desc
            .render_target_formats
            .iter()
            .copied()
            .map(texture_format_to_vk);
        color_formats.extend(iter);

        let builder = if let Some(v) = desc.depth_stencil_format {
            builder
                .depth_attachment_format(texture_format_to_vk(v))
                .stencil_attachment_format(texture_format_to_vk(v))
        } else {
            builder
        };

        builder
            .view_mask(0)
            .color_attachment_formats(color_formats.as_slice())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            if let Some(queue) = self.general_queue.take() {
                self.device_loader.queue_wait_idle(queue.handle).unwrap();
                self.device_loader.destroy_semaphore(queue.semaphore, None);
            }
            if let Some(queue) = self.compute_queue.take() {
                self.device_loader.queue_wait_idle(queue.handle).unwrap();
                self.device_loader.destroy_semaphore(queue.semaphore, None);
            }
            if let Some(queue) = self.transfer_queue.take() {
                self.device_loader.queue_wait_idle(queue.handle).unwrap();
                self.device_loader.destroy_semaphore(queue.semaphore, None);
            }

            ManuallyDrop::drop(&mut self.allocator);

            self.device_loader.destroy_device(None);
            ManuallyDrop::drop(&mut self.device_loader);
        }
    }
}
