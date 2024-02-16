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

use std::any::TypeId;
use std::mem::ManuallyDrop;
use std::ops::Deref;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BumpCell;
use aleph_rhi_impl_utils::cstr;
use allocator_api2::alloc::Allocator;
use ash::prelude::VkResult;
use ash::vk;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use byteorder::{ByteOrder, NativeEndian};
use parking_lot::Mutex;
use vulkan_alloc::vma;

use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_list::CommandList;
use crate::context::Context;
use crate::descriptor_arena::DescriptorArena;
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::fence::Fence;
use crate::internal::allocation_callbacks::callbacks_from_rust_allocator;
use crate::internal::conv::*;
use crate::internal::render_pass_cache::RenderPassCache;
use crate::internal::set_name::set_name;
use crate::internal::unwrap;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::PipelineLayout;
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::texture::Texture;

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device: ManuallyDrop<ash::Device>,
    pub(crate) timeline_semaphore: ash::extensions::khr::TimelineSemaphore,
    pub(crate) create_renderpass_2: ash::extensions::khr::CreateRenderPass2,
    pub(crate) dynamic_rendering: Option<ash::extensions::khr::DynamicRendering>,
    pub(crate) swapchain: Option<ash::extensions::khr::Swapchain>,
    pub(crate) synchronization_2: Option<ash::extensions::khr::Synchronization2>,
    pub(crate) allocator: ManuallyDrop<vma::Allocator>,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
    pub(crate) render_pass_cache: Mutex<RenderPassCache>,
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

        unsafe { self.device.device_wait_idle().unwrap() }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout);

            let mut builder =
                vk::GraphicsPipelineCreateInfo::builder().layout(pipeline_layout.pipeline_layout);

            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_states);

            // Translate the vertex input state
            let vertex_binding_descriptions: BVec<_> = Self::translate_vertex_bindings(&bump, desc);
            let vertex_attribute_descriptions: BVec<_> =
                Self::translate_vertex_attributes(&bump, desc);
            let vertex_input_state = Self::translate_vertex_input_state(
                &vertex_binding_descriptions,
                &vertex_attribute_descriptions,
            );

            let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
                .viewport_count(1)
                .scissor_count(1);
            let input_assembly_state = Self::translate_input_assembly_state(desc);
            let rasterization_state = Self::translate_rasterization_state(desc);
            let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1)
                .sample_shading_enable(false)
                .min_sample_shading(0.0)
                .alpha_to_coverage_enable(false)
                .alpha_to_one_enable(false);
            let depth_stencil_state = Self::translate_depth_stencil_state(desc);

            let mut color_formats = BVec::with_capacity_in(desc.render_target_formats.len(), &bump);
            let mut dynamic_rendering = Self::translate_framebuffer_info(desc, &mut color_formats);
            let render_pass = if self.dynamic_rendering.is_none() {
                self.translate_framebuffer_info_fallback(desc, &bump)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            } else {
                vk::RenderPass::null()
            };

            let attachments = Self::translate_color_attachment_state(&bump, desc);
            let color_blend_state = Self::translate_color_blend_state(&attachments);

            let alloc_adapter = callbacks_from_rust_allocator(bump.deref().by_ref());
            let mut shader_modules = BVec::with_capacity_in(desc.shader_stages.len(), &bump);
            for (i, v) in desc.shader_stages.iter().enumerate() {
                let module = unsafe {
                    let shader_data = Self::unwrap_shader_bytecode(&bump, i, &v.data)?;
                    let create_info = vk::ShaderModuleCreateInfo::builder().code(shader_data);
                    self.device
                        .create_shader_module(&create_info, Some(&alloc_adapter))
                        .map_err(|v| log::error!("Platform Error: {:#?}", v))?
                };
                shader_modules.push((v.stage, module));
            }

            let mut stages = BVec::with_capacity_in(shader_modules.len(), &bump);
            for &(shader_type, module) in shader_modules.iter() {
                let info = vk::PipelineShaderStageCreateInfo::builder()
                    .stage(shader_type_to_vk(shader_type))
                    .module(module)
                    .name(cstr!("main"))
                    .build();
                stages.push(info);
            }

            builder = builder.dynamic_state(&dynamic_state);
            builder = builder.stages(&stages);
            builder = builder.vertex_input_state(&vertex_input_state);
            builder = builder.viewport_state(&viewport_state);
            builder = builder.input_assembly_state(&input_assembly_state);
            builder = builder.rasterization_state(&rasterization_state);
            builder = builder.multisample_state(&multisample_state);
            builder = builder.depth_stencil_state(&depth_stencil_state);
            if self.dynamic_rendering.is_some() {
                builder = builder.push_next(&mut dynamic_rendering);
            } else {
                builder = builder.render_pass(render_pass);
            }
            builder = builder.color_blend_state(&color_blend_state);

            let pipeline = unsafe {
                self.device
                    .create_graphics_pipelines(vk::PipelineCache::null(), &[builder.build()], None)
                    .map_err(|(_, v)| log::error!("Platform Error: {:#?}", v))?
            };
            let pipeline = pipeline[0];

            for (_, module) in shader_modules {
                unsafe {
                    self.device
                        .destroy_shader_module(module, Some(&alloc_adapter));
                }
            }

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                pipeline,
                desc.name,
            );

            let out = AnyArc::new_cyclic(move |v| GraphicsPipeline {
                _this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
                pipeline,
            });
            Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(out, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let shader_data = Self::unwrap_shader_bytecode(&bump, 0, &desc.shader_module)?;
            let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout);

            // Create a temporary shader module using
            let alloc_adapter = callbacks_from_rust_allocator(bump.deref().by_ref());
            let module = unsafe {
                let create_info = vk::ShaderModuleCreateInfo::builder().code(shader_data);
                self.device
                    .create_shader_module(&create_info, Some(&alloc_adapter))
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            let builder = vk::ComputePipelineCreateInfo::builder()
                .layout(pipeline_layout.pipeline_layout)
                .stage(
                    vk::PipelineShaderStageCreateInfo::builder()
                        .stage(vk::ShaderStageFlags::COMPUTE)
                        .module(module)
                        .name(cstr!("main"))
                        .build(),
                );

            let pipeline = unsafe {
                self.device
                    .create_compute_pipelines(vk::PipelineCache::null(), &[builder.build()], None)
                    .map_err(|(_, v)| log::error!("Platform Error: {:#?}", v))?
            };
            let pipeline = pipeline[0];

            // Destroy the temporary shader module
            unsafe {
                self.device
                    .destroy_shader_module(module, Some(&alloc_adapter))
            }

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                pipeline,
                desc.name,
            );

            let out = AnyArc::new_cyclic(move |v| ComputePipeline {
                _this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout._this.upgrade().unwrap(),
                pipeline,
            });
            Ok(AnyArc::map::<dyn IComputePipeline, _>(out, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let stage_flags = descriptor_shader_visibility_to_vk(desc.visibility);

            let mut _samplers = Vec::new();
            let mut static_samplers = BVec::new_in(&bump);
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
            let mut bindings = BVec::with_capacity_in(desc.items.len(), &bump);
            for v in desc.items {
                let descriptor_type = descriptor_type_to_vk(v.binding_type);
                let descriptor_count = v.binding_count.map(|v| v.get()).unwrap_or(1);

                sizes[descriptor_type.as_raw() as usize] += descriptor_count;

                let binding = vk::DescriptorSetLayoutBinding::builder()
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

                bindings.push(binding.build());
            }

            let mut pool_sizes = Vec::with_capacity(sizes.len());
            for (i, v) in sizes.iter().copied().enumerate() {
                // Accumulate any non-zero pool size into the list
                if v > 0 {
                    pool_sizes.push(
                        vk::DescriptorPoolSize::builder()
                            .ty(vk::DescriptorType::from_raw(i as i32))
                            .descriptor_count(v)
                            .build(),
                    );
                }
            }

            let create_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);

            let descriptor_set_layout = unsafe {
                self.device
                    .create_descriptor_set_layout(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                descriptor_set_layout,
                desc.name,
            );

            let out = AnyArc::new_cyclic(move |v| DescriptorSetLayout {
                _this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                _samplers,
                descriptor_set_layout,
                pool_sizes,
            });
            Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(out, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let layout = unwrap::descriptor_set_layout(desc.layout)
                ._this
                .upgrade()
                .unwrap();

            let mut pool_sizes = BVec::from_iter_in(layout.pool_sizes.iter().copied(), &bump);
            for size in &mut pool_sizes {
                size.descriptor_count *= desc.num_sets;
            }

            let create_info = vk::DescriptorPoolCreateInfo::builder()
                .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
                .max_sets(desc.num_sets)
                .pool_sizes(&pool_sizes);

            let descriptor_pool = unsafe {
                self.device
                    .create_descriptor_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                descriptor_pool,
                desc.name,
            );

            let pool: Box<dyn IDescriptorPool> = Box::new(DescriptorPool {
                _device: self.this.upgrade().unwrap(),
                _layout: layout,
                descriptor_pool,
            });

            Ok(pool)
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            const fn pool_size(
                ty: vk::DescriptorType,
                descriptor_count: u32,
            ) -> vk::DescriptorPoolSize {
                vk::DescriptorPoolSize {
                    ty,
                    descriptor_count,
                }
            }
            let mut pool_sizes = [
                pool_size(vk::DescriptorType::SAMPLER, 1),
                // pool_size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 0),
                pool_size(vk::DescriptorType::SAMPLED_IMAGE, 8),
                pool_size(vk::DescriptorType::STORAGE_IMAGE, 4),
                pool_size(vk::DescriptorType::UNIFORM_TEXEL_BUFFER, 2),
                pool_size(vk::DescriptorType::STORAGE_TEXEL_BUFFER, 2),
                pool_size(vk::DescriptorType::UNIFORM_BUFFER, 4),
                pool_size(vk::DescriptorType::STORAGE_BUFFER, 4),
                pool_size(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC, 2),
                // pool_size(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC, 0),
                // pool_size(vk::DescriptorType::INPUT_ATTACHMENT, 0),
            ];
            // Multiply the pool sizes by our multiplier. We encode the default sizes as the number
            // of descriptors expected per '2' sets so we can do some nice integer math instead of
            // icky float maths with fractional ratios
            let multiplier = desc.num_sets.div_ceil(2).min(2);
            for v in &mut pool_sizes {
                v.descriptor_count *= multiplier;
            }

            let flags = match desc.arena_type {
                DescriptorArenaType::Linear => vk::DescriptorPoolCreateFlags::empty(),
                DescriptorArenaType::Heap => vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
            };

            let create_info = vk::DescriptorPoolCreateInfo::builder()
                .flags(flags)
                .max_sets(desc.num_sets)
                .pool_sizes(&pool_sizes);

            let descriptor_pool = unsafe {
                self.device
                    .create_descriptor_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                descriptor_pool,
                desc.name,
            );

            let pool: Box<dyn IDescriptorArena> = Box::new(DescriptorArena {
                _device: self.this.upgrade().unwrap(),
                descriptor_pool,
            });

            Ok(pool)
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut set_layouts = BVec::with_capacity_in(desc.set_layouts.len(), &bump);
            for v in desc.set_layouts {
                let v = unwrap::descriptor_set_layout_d(v);
                set_layouts.push(v.descriptor_set_layout);
            }

            let mut offset = 0;
            let mut ranges = Vec::with_capacity(desc.push_constant_blocks.len());
            for v in desc.push_constant_blocks {
                let range = vk::PushConstantRange::builder()
                    .stage_flags(descriptor_shader_visibility_to_vk(v.visibility))
                    .offset(offset)
                    .size(v.size as u32);
                ranges.push(range.build());

                offset += v.size as u32;
            }

            let create_info = vk::PipelineLayoutCreateInfo::builder()
                .set_layouts(&set_layouts)
                .push_constant_ranges(&ranges);

            let pipeline_layout = unsafe {
                self.device
                    .create_pipeline_layout(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                pipeline_layout,
                desc.name,
            );

            let out = AnyArc::new_cyclic(move |v| PipelineLayout {
                _this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                pipeline_layout,
                push_constant_blocks: ranges,
            });
            Ok(AnyArc::map::<dyn IPipelineLayout, _>(out, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
        // Storage buffer is always enabled as this is the most basic usage, essentially meaning
        // "bag of bytes".
        let mut usage = vk::BufferUsageFlags::empty();

        if desc.usage.contains(ResourceUsageFlags::COPY_SOURCE) {
            usage |= vk::BufferUsageFlags::TRANSFER_SRC
        }
        if desc.usage.contains(ResourceUsageFlags::COPY_DEST) {
            usage |= vk::BufferUsageFlags::TRANSFER_DST
        }
        if desc.usage.contains(ResourceUsageFlags::SHADER_RESOURCE) {
            usage |= vk::BufferUsageFlags::STORAGE_BUFFER;
            usage |= vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER;
        }
        if desc.usage.contains(ResourceUsageFlags::UNORDERED_ACCESS) {
            usage |= vk::BufferUsageFlags::STORAGE_BUFFER;
            usage |= vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER;
        }
        if desc.usage.contains(ResourceUsageFlags::VERTEX_BUFFER) {
            usage |= vk::BufferUsageFlags::VERTEX_BUFFER;
        }
        if desc.usage.contains(ResourceUsageFlags::INDEX_BUFFER) {
            usage |= vk::BufferUsageFlags::INDEX_BUFFER;
        }
        if desc.usage.contains(ResourceUsageFlags::CONSTANT_BUFFER) {
            usage |= vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if desc.usage.contains(ResourceUsageFlags::INDIRECT_DRAW_ARGS) {
            usage |= vk::BufferUsageFlags::INDIRECT_BUFFER;
        }
        if desc
            .usage
            .contains(ResourceUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT)
        {
            usage |= vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR;
        }
        if desc
            .usage
            .contains(ResourceUsageFlags::ACCELERATION_STRUCTURE_STORAGE)
        {
            usage |= vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR;
        }

        let create_info = vk::BufferCreateInfo::builder()
            .size(desc.size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let usage = match desc.cpu_access {
            CpuAccessMode::None => vma::MemoryUsage::GpuOnly,
            CpuAccessMode::Read => vma::MemoryUsage::GpuToCpu,
            CpuAccessMode::Write => vma::MemoryUsage::CpuToGpu,
        };
        let alloc_info = vma::AllocationCreateInfo::builder()
            .flags(vma::AllocationCreateFlags::empty())
            .usage(usage);

        let (buffer, allocation, _) = unsafe {
            self.allocator
                .create_buffer(&create_info, &alloc_info)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
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
            TextureDimension::Texture1D => vk::ImageType::TYPE_1D,
            TextureDimension::Texture2D => vk::ImageType::TYPE_2D,
            TextureDimension::Texture3D => vk::ImageType::TYPE_3D,
        };

        let format = texture_format_to_vk(desc.format);

        let samples = match desc.sample_count {
            1 => vk::SampleCountFlags::TYPE_1,
            2 => vk::SampleCountFlags::TYPE_2,
            4 => vk::SampleCountFlags::TYPE_4,
            8 => vk::SampleCountFlags::TYPE_8,
            16 => vk::SampleCountFlags::TYPE_16,
            32 => vk::SampleCountFlags::TYPE_32,
            _ => return Err(TextureCreateError::InvalidSampleCount(desc.sample_count)),
        };

        let mut usage = vk::ImageUsageFlags::empty();
        if desc.usage.contains(ResourceUsageFlags::SHADER_RESOURCE) {
            usage |= vk::ImageUsageFlags::SAMPLED
        }
        if desc.usage.contains(ResourceUsageFlags::COPY_DEST) {
            usage |= vk::ImageUsageFlags::TRANSFER_DST
        }
        if desc.usage.contains(ResourceUsageFlags::COPY_SOURCE) {
            usage |= vk::ImageUsageFlags::TRANSFER_SRC
        }
        if desc.usage.contains(ResourceUsageFlags::UNORDERED_ACCESS) {
            usage |= vk::ImageUsageFlags::STORAGE
        }
        if desc.usage.contains(ResourceUsageFlags::RENDER_TARGET) {
            if desc.format.is_depth_stencil() {
                usage |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
            } else {
                usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT
            }
        }

        let mut flags = vk::ImageCreateFlags::empty();
        if desc.usage.contains(ResourceUsageFlags::CUBE_FACE) {
            flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
        }

        let create_info = vk::ImageCreateInfo::builder()
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
            .usage(vma::MemoryUsage::GpuOnly);

        let (image, allocation, _) = unsafe {
            self.allocator
                .create_image(&create_info, &alloc_info)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        let name = desc.name.map(String::from);
        let desc = desc.clone().strip_name();
        let out = AnyArc::new_cyclic(move |v| Texture {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            image,
            creation_flags: create_info.flags,
            // created_usage: create_info.usage,
            allocation: Some(allocation),
            is_owned: true,
            views: Default::default(),
            rtvs: Default::default(),
            dsvs: Default::default(),
            framebuffers: Default::default(),
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
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut create_info = vk::SamplerCreateInfo::builder()
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
                self.device
                    .create_sampler(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                sampler,
                desc.name,
            );

            let name = desc.name.map(String::from);
            let out = AnyArc::new_cyclic(move |v| Sampler {
                _this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                sampler,
                desc: desc.clone().strip_name(),
                name,
            });
            Ok(AnyArc::map::<dyn ISampler, _>(out, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let family_index = match desc.queue_type {
                QueueType::General => self.general_queue.as_ref().unwrap().info.family_index,
                QueueType::Compute => self.compute_queue.as_ref().unwrap().info.family_index,
                QueueType::Transfer => self.transfer_queue.as_ref().unwrap().info.family_index,
            };

            let create_info = vk::CommandPoolCreateInfo::builder()
                .flags(vk::CommandPoolCreateFlags::TRANSIENT)
                .queue_family_index(family_index);
            let command_pool = unsafe {
                self.device
                    .create_command_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command_buffer = unsafe {
                self.device
                    .allocate_command_buffers(&allocate_info)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };
            let command_buffer = command_buffer[0];

            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                command_pool,
                desc.name,
            );
            set_name(
                self.context.debug_loader.as_ref(),
                self.device.handle(),
                &bump,
                command_buffer,
                desc.name,
            );

            let out: Box<dyn ICommandList> = Box::new(CommandList {
                _device: self.this.upgrade().unwrap(),
                pool: command_pool,
                buffer: command_buffer,
                list_type: desc.queue_type,
            });

            Ok(out)
        })
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
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut descriptor_writes = BVec::with_capacity_in(writes.len(), &bump);
            for write in writes {
                let d_type = write.writes.descriptor_type();
                let d_type = descriptor_type_to_vk(d_type);
                let new_write = vk::WriteDescriptorSet::builder()
                    .dst_set(std::mem::transmute(write.set.clone()))
                    .dst_binding(write.binding)
                    .dst_array_element(write.array_element)
                    .descriptor_type(d_type);
                let new_write = match write.writes {
                    DescriptorWrites::Sampler(v) => {
                        let translator = v.iter().map(|v| {
                            vk::DescriptorImageInfo::builder()
                                .sampler(unwrap::sampler(v.sampler).sampler)
                                .build()
                        });
                        let image_infos = bump.alloc_slice_fill_iter(translator);
                        new_write.image_info(image_infos)
                    }
                    DescriptorWrites::TexelBufferRW(v) | DescriptorWrites::TexelBuffer(v) => {
                        let translator = v.iter().map(|_v| vk::BufferView::null());
                        let texel_buffer_infos = bump.alloc_slice_fill_iter(translator);
                        new_write.texel_buffer_view(texel_buffer_infos)
                    }
                    DescriptorWrites::InputAttachment(v)
                    | DescriptorWrites::TextureRW(v)
                    | DescriptorWrites::Texture(v) => {
                        let translator = v.iter().map(|v| {
                            vk::DescriptorImageInfo::builder()
                                .image_view(std::mem::transmute(v.image_view))
                                .image_layout(image_layout_to_vk(v.image_layout))
                                .build()
                        });
                        let image_infos = bump.alloc_slice_fill_iter(translator);
                        new_write.image_info(image_infos)
                    }
                    DescriptorWrites::ByteAddressBuffer(v)
                    | DescriptorWrites::ByteAddressBufferRW(v)
                    | DescriptorWrites::StructuredBufferRW(v)
                    | DescriptorWrites::StructuredBuffer(v)
                    | DescriptorWrites::UniformBuffer(v)
                    | DescriptorWrites::UniformBufferDynamic(v) => {
                        let translator = v.iter().map(|v| {
                            let buffer = unwrap::buffer(v.buffer);
                            let len = buffer.clamp_max_size_for_view(v.len);
                            vk::DescriptorBufferInfo::builder()
                                .buffer(buffer.buffer)
                                .offset(v.offset)
                                .range(len)
                                .build()
                        });
                        let buffer_infos = bump.alloc_slice_fill_iter(translator);
                        new_write.buffer_info(buffer_infos)
                    }
                };
                descriptor_writes.push(new_write.build());
            }

            self.device.update_descriptor_sets(&descriptor_writes, &[]);
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<AnyArc<dyn IFence>, FenceCreateError> {
        let fence = unsafe {
            let mut info = vk::FenceCreateInfo::builder();
            if signalled {
                info = info.flags(vk::FenceCreateFlags::SIGNALED)
            }
            self.device
                .create_fence(&info, None)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
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
            let info = vk::SemaphoreCreateInfo::builder();
            self.device
                .create_semaphore(&info, None)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
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
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let timeout = if timeout == u32::MAX {
                u64::MAX
            } else {
                timeout as u64 * 1000000 // Convert to nanoseconds
            };

            let iter = unwrap::fence_iter(fences).map(|v| v.fence);
            let fences = BVec::from_iter_in(iter, &bump);

            let result = unsafe { self.device.wait_for_fences(&fences, wait_all, timeout) };

            match result {
                Ok(_) => FenceWaitResult::Complete,
                Err(vk::Result::TIMEOUT) => FenceWaitResult::Timeout,
                v @ _ => {
                    v.unwrap();
                    unreachable!()
                }
            }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &dyn IFence) -> bool {
        let fence = unwrap::fence(fence);

        let result = unsafe { self.device.get_fence_status(fence.fence) };

        match result {
            Ok(_) => true,
            Err(vk::Result::NOT_READY) => false,
            v @ _ => {
                v.unwrap();
                unreachable!()
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, fences: &[&dyn IFence]) {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let iter = unwrap::fence_iter(fences).map(|v| v.fence);
            let fences = BVec::from_iter_in(iter, &bump);

            unsafe { self.device.reset_fences(&fences).unwrap() }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }
}

impl Device {
    pub(crate) fn get_queue_family_index(&self, queue_type: QueueType) -> u32 {
        match queue_type {
            QueueType::General => self.general_queue.as_ref().unwrap().info.family_index,
            QueueType::Compute => self.compute_queue.as_ref().unwrap().info.family_index,
            QueueType::Transfer => self.transfer_queue.as_ref().unwrap().info.family_index,
        }
    }

    fn unwrap_shader_bytecode<'a>(
        bump: &'a Bump,
        index: usize,
        shader: &ShaderBinary,
    ) -> Result<&'a [u32], PipelineCreateError> {
        if let ShaderBinary::Spirv(data) = shader {
            // Vulkan shaders must always have a buffer length that is a multiple of 4. SPIR-V's binary
            // representation is a sequence of u32 values.
            if data.len() % 4 != 0 || data.is_empty() {
                return Err(PipelineCreateError::InvalidInputSize(index, data.len()));
            }

            // We need to copy the data into a u32 buffer to satisfy alignment requirements
            let data_iter = data.chunks_exact(4).map(NativeEndian::read_u32);
            let data = bump.alloc_slice_fill_iter(data_iter);

            Ok(&*data)
        } else {
            Err(PipelineCreateError::UnsupportedShaderFormat(index))
        }
    }

    fn translate_vertex_bindings<'a>(
        bump: &'a Bump,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<'a, vk::VertexInputBindingDescription> {
        let iter = desc.vertex_layout.input_bindings.iter().map(|v| {
            vk::VertexInputBindingDescription::builder()
                .binding(v.binding)
                .stride(v.stride)
                .input_rate(vertex_input_rate_to_vk(v.input_rate))
                .build()
        });
        BVec::from_iter_in(iter, bump)
    }

    fn translate_vertex_attributes<'a>(
        bump: &'a Bump,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<'a, vk::VertexInputAttributeDescription> {
        let iter = desc.vertex_layout.input_attributes.iter().map(|v| {
            vk::VertexInputAttributeDescription::builder()
                .location(v.location)
                .binding(v.binding)
                .offset(v.offset)
                .format(texture_format_to_vk(v.format))
                .build()
        });
        BVec::from_iter_in(iter, bump)
    }

    fn translate_vertex_input_state<'a>(
        vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
        vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> vk::PipelineVertexInputStateCreateInfoBuilder<'a> {
        vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(vertex_binding_descriptions)
            .vertex_attribute_descriptions(vertex_attribute_descriptions)
    }

    fn translate_input_assembly_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineInputAssemblyStateCreateInfoBuilder<'static> {
        let topology = primitive_topology_to_vk(desc.input_assembly_state.primitive_topology);
        vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(topology)
            .primitive_restart_enable(false)
    }

    fn translate_rasterization_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineRasterizationStateCreateInfoBuilder<'static> {
        let polygon_mode = polygon_mode_to_vk(desc.rasterizer_state.polygon_mode);
        let cull_mode = cull_mode_to_vk(desc.rasterizer_state.cull_mode);
        let front_face = front_face_order_to_vk(desc.rasterizer_state.front_face);
        vk::PipelineRasterizationStateCreateInfo::builder()
            .polygon_mode(polygon_mode)
            .cull_mode(cull_mode)
            .front_face(front_face)
            .depth_clamp_enable(true)
            .rasterizer_discard_enable(false) // No support in dx12
            .depth_bias_enable(desc.rasterizer_state.depth_bias != 0)
            .depth_bias_constant_factor(desc.rasterizer_state.depth_bias as f32)
            .depth_bias_clamp(desc.rasterizer_state.depth_bias_clamp)
            .depth_bias_slope_factor(desc.rasterizer_state.depth_bias_slope_factor)
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

        vk::PipelineDepthStencilStateCreateInfo::builder()
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

    fn translate_color_attachment_state<'a>(
        bump: &'a Bump,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<'a, vk::PipelineColorBlendAttachmentState> {
        let iter = desc.blend_state.attachments.iter().map(|v| {
            vk::PipelineColorBlendAttachmentState::builder()
                .blend_enable(v.blend_enabled)
                .src_color_blend_factor(blend_factor_to_vk(v.src_factor))
                .dst_color_blend_factor(blend_factor_to_vk(v.dst_factor))
                .color_blend_op(blend_op_to_vk(v.blend_op))
                .src_alpha_blend_factor(blend_factor_to_vk(v.alpha_src_factor))
                .dst_alpha_blend_factor(blend_factor_to_vk(v.alpha_dst_factor))
                .alpha_blend_op(blend_op_to_vk(v.alpha_blend_op))
                .color_write_mask(vk::ColorComponentFlags::from_raw(
                    v.color_write_mask.bits() as _
                ))
                .build()
        });
        BVec::from_iter_in(iter, bump)
    }

    fn translate_color_blend_state(
        attachments: &[vk::PipelineColorBlendAttachmentState],
    ) -> vk::PipelineColorBlendStateCreateInfoBuilder {
        vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
    }

    fn translate_framebuffer_info<'a, 'b>(
        desc: &'b GraphicsPipelineDesc,
        color_formats: &'a mut BVec<vk::Format>,
    ) -> vk::PipelineRenderingCreateInfoBuilder<'a> {
        let builder = vk::PipelineRenderingCreateInfo::builder();

        let iter = desc
            .render_target_formats
            .iter()
            .copied()
            .map(texture_format_to_vk);
        color_formats.extend(iter);

        let builder = if let Some(v) = desc.depth_stencil_format {
            if v.is_stencil() {
                builder
                    .depth_attachment_format(texture_format_to_vk(v))
                    .stencil_attachment_format(texture_format_to_vk(v))
            } else {
                builder
                    .depth_attachment_format(texture_format_to_vk(v))
                    .stencil_attachment_format(vk::Format::UNDEFINED)
            }
        } else {
            builder
        };

        builder
            .view_mask(0)
            .color_attachment_formats(color_formats.as_slice())
    }

    fn translate_framebuffer_info_fallback(
        &self,
        desc: &GraphicsPipelineDesc,
        bump: &Bump,
    ) -> VkResult<vk::RenderPass> {
        // Number of attachments is number of color attachments + 1 if we have a depth attachment
        let num_attachments =
            desc.render_target_formats.len() + desc.depth_stencil_format.map(|_| 1).unwrap_or(0);

        let attachments: &mut [vk::AttachmentDescription2] =
            bump.alloc_slice_fill_default(num_attachments);
        let references: &mut [vk::AttachmentReference2] =
            bump.alloc_slice_fill_default(num_attachments);

        let mut attachment_index = 0usize;
        for v in desc.render_target_formats.iter().copied() {
            attachments[attachment_index] = vk::AttachmentDescription2::builder()
                .format(texture_format_to_vk(v))
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::NONE_EXT)
                .store_op(vk::AttachmentStoreOp::NONE_EXT)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::GENERAL)
                .build();

            references[attachment_index] = vk::AttachmentReference2::builder()
                .attachment(attachment_index as _)
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layout(vk::ImageLayout::GENERAL)
                .build();

            attachment_index += 1;
        }

        if let Some(v) = desc.depth_stencil_format {
            attachments[attachment_index] = vk::AttachmentDescription2::builder()
                .format(texture_format_to_vk(v))
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::NONE_EXT)
                .store_op(vk::AttachmentStoreOp::NONE_EXT)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::GENERAL)
                .build();

            references[attachment_index] = vk::AttachmentReference2::builder()
                .attachment(attachment_index as _)
                .aspect_mask(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
                .layout(vk::ImageLayout::GENERAL)
                .build();
        }

        let mut subpass = vk::SubpassDescription2::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .view_mask(0)
            .color_attachments(&references[0..desc.render_target_formats.len()]);
        if desc.depth_stencil_format.is_some() {
            subpass = subpass.depth_stencil_attachment(references.last().unwrap())
        }
        let subpass = &*bump.alloc(subpass);

        let create_info = vk::RenderPassCreateInfo2::builder()
            .attachments(attachments)
            .subpasses(std::slice::from_ref(&subpass));

        unsafe {
            self.render_pass_cache
                .lock()
                .get_render_pass_for_create_info(&self, &create_info)
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.render_pass_cache.get_mut().destroy(&self.device);

            if let Some(queue) = self.general_queue.take() {
                self.device.queue_wait_idle(queue.handle).unwrap();
                self.device.destroy_semaphore(queue.semaphore, None);
            }
            if let Some(queue) = self.compute_queue.take() {
                self.device.queue_wait_idle(queue.handle).unwrap();
                self.device.destroy_semaphore(queue.semaphore, None);
            }
            if let Some(queue) = self.transfer_queue.take() {
                self.device.queue_wait_idle(queue.handle).unwrap();
                self.device.destroy_semaphore(queue.semaphore, None);
            }

            ManuallyDrop::drop(&mut self.allocator);

            self.device.destroy_device(None);
            ManuallyDrop::drop(&mut self.device);
        }
    }
}

thread_local! {
    pub static DEVICE_BUMP: BumpCell = BumpCell::with_capacity(8192 * 2);
}
