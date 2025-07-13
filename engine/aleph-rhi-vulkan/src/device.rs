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
use std::mem::{ManuallyDrop, MaybeUninit};

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BlinkCell;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::owned_desc::{OwnedBufferDesc, OwnedSamplerDesc, OwnedTextureDesc};
use allocator_api2::vec::Vec as BVec;
use ash::vk;
use blink_alloc::{Blink, BlinkAlloc};
use byteorder::{ByteOrder, NativeEndian};
use crossbeam::queue::ArrayQueue;
use parking_lot::Mutex;
use vulkan_alloc::vma;

use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::descriptor_arena::DescriptorArena;
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::fence::Fence;
use crate::internal::allocation_callbacks::callbacks_from_rust_allocator;
use crate::internal::conv::*;
use crate::internal::set_name::set_name;
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
    pub(crate) timeline_semaphore: ash::khr::timeline_semaphore::Device,
    pub(crate) _create_renderpass_2: ash::khr::create_renderpass2::Device,
    pub(crate) dynamic_rendering: ash::khr::dynamic_rendering::Device,
    pub(crate) swapchain: Option<ash::khr::swapchain::Device>,
    pub(crate) synchronization_2: Option<ash::khr::synchronization2::Device>,
    pub(crate) debug_loader: Option<ash::ext::debug_utils::Device>,
    pub(crate) allocator: ManuallyDrop<vma::Allocator>,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
    pub(crate) command_list_pool: CommandListPool,
    pub(crate) object_counter: ObjectCounter,
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

    #[aleph_profile::function]
    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);

            let mut builder =
                vk::GraphicsPipelineCreateInfo::default().layout(pipeline_layout.pipeline_layout);

            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dynamic_state =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

            // Translate the vertex input state
            let vertex_binding_descriptions: BVec<_, _> =
                Self::translate_vertex_bindings(&bump, desc);
            let vertex_attribute_descriptions: BVec<_, _> =
                Self::translate_vertex_attributes(&bump, desc);
            let vertex_input_state = Self::translate_vertex_input_state(
                &vertex_binding_descriptions,
                &vertex_attribute_descriptions,
            );

            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1);
            let input_assembly_state = Self::translate_input_assembly_state(desc);
            let rasterization_state = Self::translate_rasterization_state(desc);
            let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1)
                .sample_shading_enable(false)
                .min_sample_shading(0.0)
                .alpha_to_coverage_enable(false)
                .alpha_to_one_enable(false);
            let depth_stencil_state = Self::translate_depth_stencil_state(desc);

            let mut color_formats =
                BVec::with_capacity_in(desc.render_target_formats.len(), bump.allocator());
            let mut dynamic_rendering = Self::translate_framebuffer_info(desc, &mut color_formats);

            let attachments = Self::translate_color_attachment_state(&bump, desc);
            let color_blend_state = Self::translate_color_blend_state(&attachments);

            let alloc_adapter = callbacks_from_rust_allocator(bump.allocator());
            let mut shader_modules =
                BVec::with_capacity_in(desc.shader_stages.len(), bump.allocator());
            for (i, v) in desc.shader_stages.iter().enumerate() {
                let module = unsafe {
                    let shader_data = Self::unwrap_shader_bytecode(&bump, i, &v.data)?;
                    let create_info = vk::ShaderModuleCreateInfo::default().code(shader_data);
                    self.device
                        .create_shader_module(&create_info, Some(&alloc_adapter))
                        .map_err(|v| log::error!("Platform Error: {:#?}", v))?
                };
                shader_modules.push((v.stage, module));
            }

            let mut stages = BVec::with_capacity_in(shader_modules.len(), bump.allocator());
            for &(shader_type, module) in shader_modules.iter() {
                let info = vk::PipelineShaderStageCreateInfo::default()
                    .stage(shader_type_to_vk(shader_type))
                    .module(module)
                    .name(c"main");
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
            builder = builder.push_next(&mut dynamic_rendering);
            builder = builder.color_blend_state(&color_blend_state);

            let pipeline = unsafe {
                self.device
                    .create_graphics_pipelines(vk::PipelineCache::null(), &[builder], None)
                    .map_err(|(_, v)| log::error!("Platform Error: {:#?}", v))?
            };
            let pipeline = pipeline[0];

            for (_, module) in shader_modules {
                unsafe {
                    self.device
                        .destroy_shader_module(module, Some(&alloc_adapter));
                }
            }

            set_name(self.debug_loader.as_ref(), &bump, pipeline, desc.name);

            let out = GraphicsPipeline {
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout,
                id: self.object_counter.next_graphics_pipeline(),
                pipeline,
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(GraphicsPipelineHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    #[aleph_profile::function]
    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let shader_data = Self::unwrap_shader_bytecode(&bump, 0, &desc.shader_module)?;
            let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);

            // Create a temporary shader module using
            let alloc_adapter = callbacks_from_rust_allocator(bump.allocator());
            let module = unsafe {
                let create_info = vk::ShaderModuleCreateInfo::default().code(shader_data);
                self.device
                    .create_shader_module(&create_info, Some(&alloc_adapter))
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            let builder = vk::ComputePipelineCreateInfo::default()
                .layout(pipeline_layout.pipeline_layout)
                .stage(
                    vk::PipelineShaderStageCreateInfo::default()
                        .stage(vk::ShaderStageFlags::COMPUTE)
                        .module(module)
                        .name(c"main"),
                );

            let pipeline = unsafe {
                self.device
                    .create_compute_pipelines(vk::PipelineCache::null(), &[builder], None)
                    .map_err(|(_, v)| log::error!("Platform Error: {:#?}", v))?
            };
            let pipeline = pipeline[0];

            // Destroy the temporary shader module
            unsafe {
                self.device
                    .destroy_shader_module(module, Some(&alloc_adapter))
            }

            set_name(self.debug_loader.as_ref(), &bump, pipeline, desc.name);

            let out = ComputePipeline {
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout,
                id: self.object_counter.next_compute_pipeline(),
                pipeline,
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(ComputePipelineHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<DescriptorSetLayoutHandle, DescriptorSetLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let stage_flags = descriptor_shader_visibility_to_vk(desc.visibility);

            let mut _samplers = Vec::new();
            let mut static_samplers = BVec::new_in(bump.allocator());
            for v in desc.items {
                if let Some(samplers) = v.static_samplers {
                    for sampler in samplers.iter().copied() {
                        let sampler = Sampler::get_owned(sampler);
                        static_samplers.push(sampler.sampler);
                        _samplers.push(sampler);
                    }
                }
            }

            let mut sampler_i = 0;
            let mut sizes = [0; 11];
            let mut bindings = BVec::with_capacity_in(desc.items.len(), bump.allocator());
            for v in desc.items {
                let descriptor_type = descriptor_type_to_vk(v.binding_type);
                let descriptor_count = v.binding_count.map(|v| v.get()).unwrap_or(1);

                sizes[descriptor_type.as_raw() as usize] += descriptor_count;

                let binding = vk::DescriptorSetLayoutBinding::default()
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
                        vk::DescriptorPoolSize::default()
                            .ty(vk::DescriptorType::from_raw(i as i32))
                            .descriptor_count(v),
                    );
                }
            }

            let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);

            let descriptor_set_layout = unsafe {
                self.device
                    .create_descriptor_set_layout(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.debug_loader.as_ref(),
                &bump,
                descriptor_set_layout,
                desc.name,
            );

            let out = DescriptorSetLayout {
                _device: self.this.upgrade().unwrap(),
                _samplers,
                id: self.object_counter.next_set_layout(),
                descriptor_set_layout,
                pool_sizes,
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(DescriptorSetLayoutHandle::new(out)) }
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

            let layout = DescriptorSetLayout::get_owned(desc.layout);

            let iter = layout.pool_sizes.iter().copied();
            let mut pool_sizes = BVec::new_in(bump.allocator());
            pool_sizes.extend(iter);
            for size in &mut pool_sizes {
                size.descriptor_count *= desc.num_sets;
            }

            let create_info = vk::DescriptorPoolCreateInfo::default()
                .flags(vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
                .max_sets(desc.num_sets)
                .pool_sizes(&pool_sizes);

            let descriptor_pool = unsafe {
                self.device
                    .create_descriptor_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.debug_loader.as_ref(),
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
            let multiplier = desc.num_sets.div_ceil(2).max(2);
            for v in &mut pool_sizes {
                v.descriptor_count *= multiplier;
            }

            let flags = match desc.arena_type {
                DescriptorArenaType::Linear => vk::DescriptorPoolCreateFlags::empty(),
                DescriptorArenaType::Heap => vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
            };

            let create_info = vk::DescriptorPoolCreateInfo::default()
                .flags(flags)
                .max_sets(desc.num_sets)
                .pool_sizes(&pool_sizes);

            let descriptor_pool = unsafe {
                self.device
                    .create_descriptor_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.debug_loader.as_ref(),
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
    ) -> Result<PipelineLayoutHandle, PipelineLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut set_layouts = BVec::with_capacity_in(desc.set_layouts.len(), bump.allocator());
            for v in desc.set_layouts {
                let v = DescriptorSetLayout::get(v);
                set_layouts.push(v.descriptor_set_layout);
            }

            let mut offset = 0;
            let mut ranges = Vec::with_capacity(desc.push_constant_blocks.len());
            for v in desc.push_constant_blocks {
                let range = vk::PushConstantRange::default()
                    .stage_flags(descriptor_shader_visibility_to_vk(v.visibility))
                    .offset(offset)
                    .size(v.size as u32);
                ranges.push(range);

                offset += v.size as u32;
            }

            let create_info = vk::PipelineLayoutCreateInfo::default()
                .set_layouts(&set_layouts)
                .push_constant_ranges(&ranges);

            let pipeline_layout = unsafe {
                self.device
                    .create_pipeline_layout(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(
                self.debug_loader.as_ref(),
                &bump,
                pipeline_layout,
                desc.name,
            );

            let out = PipelineLayout {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_pipeline_layout(),
                pipeline_layout,
                push_constant_blocks: ranges,
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(PipelineLayoutHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
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

        let create_info = vk::BufferCreateInfo::default()
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

        let out = Buffer {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_buffer(),
            buffer,
            allocation,
            map_state: Mutex::new(Default::default()),
            desc: OwnedBufferDesc::new(desc.clone()),
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let image_type = match desc.dimension {
                TextureDimension::Texture1D => vk::ImageType::TYPE_1D,
                TextureDimension::Texture2D => vk::ImageType::TYPE_2D,
                TextureDimension::Texture3D => vk::ImageType::TYPE_3D,
            };

            let format = texture_format_to_vk(desc.format);

            // Select our set of view-compatible formats
            let iter = desc
                .format
                .compatible_view_formats()
                .iter()
                .copied()
                .map(texture_format_to_vk);
            let mut format_list = BVec::new_in(bump.allocator());
            format_list.extend(iter);

            let mut format_flags = vk::ImageCreateFlags::empty();
            if format_list.len() > 1 {
                format_flags |= vk::ImageCreateFlags::MUTABLE_FORMAT
            }

            let mut format_list =
                vk::ImageFormatListCreateInfo::default().view_formats(&format_list);

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

            let create_info = vk::ImageCreateInfo::default()
                .flags(flags | format_flags)
                .image_type(image_type)
                .format(format)
                .extent(vk::Extent3D {
                    width: desc.width.max(1),
                    height: desc.height.max(1),
                    depth: desc.depth.max(1),
                })
                .mip_levels(desc.mip_levels.max(1))
                .array_layers(desc.array_size.max(1))
                .samples(samples)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .initial_layout(vk::ImageLayout::UNDEFINED);
            let create_info = create_info.push_next(&mut format_list);

            let alloc_info = vma::AllocationCreateInfo::builder()
                .flags(vma::AllocationCreateFlags::empty())
                .usage(vma::MemoryUsage::GpuOnly);

            let (image, allocation, _) = unsafe {
                self.allocator
                    .create_image(&create_info, &alloc_info)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            let out = Texture {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_texture(),
                image,
                // creation_flags: create_info.flags,
                // created_usage: create_info.usage,
                allocation: Some(allocation),
                is_owned: true,
                views: Default::default(),
                rtvs: Default::default(),
                dsvs: Default::default(),
                desc: OwnedTextureDesc::new(desc.clone()),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(TextureHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut create_info = vk::SamplerCreateInfo::default()
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

            set_name(self.debug_loader.as_ref(), &bump, sampler, desc.name);

            let out = Sampler {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_sampler(),
                sampler,
                desc: OwnedSamplerDesc::new(desc.clone()),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(SamplerHandle::new(out)) }
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

            // First we try and grab a command list from the free list. This way we reuse an old
            // list before we try and make a new one. This can save a lot of performance even if the
            // free list is a bit slow.
            //
            // Some drivers will lazily allocate pages for the command list on first use. If we're
            // only using fresh allocators then we hit that (very) slow path every time. To avoid
            // this we front creating new command pools with a free list so we recycle old ones
            // first.
            if let Some(list) = self.command_list_pool.get_for_queue_type(desc.queue_type) {
                set_name(self.debug_loader.as_ref(), &bump, list.pool, desc.name);
                set_name(self.debug_loader.as_ref(), &bump, list.buffer, desc.name);

                // It is assumed that only command lists that are safe to reuse are placed into the
                // free list.
                //
                // Typically, this will be done in 'garbage_collect'.
                let out: Box<dyn ICommandList> = Box::new(CommandList {
                    _device: self.this.upgrade().unwrap(),
                    pool: list.pool,
                    buffer: list.buffer,
                    list_type: list.list_type,
                    state: ListState::Empty,
                });
                return Ok(out);
            }

            log::warn!(
                "CommandList free-object-pool empty. Taking slow-path for creating a new object!"
            );

            let family_index = match desc.queue_type {
                QueueType::General => self.general_queue.as_ref().unwrap().info.family_index,
                QueueType::Compute => self.compute_queue.as_ref().unwrap().info.family_index,
                QueueType::Transfer => self.transfer_queue.as_ref().unwrap().info.family_index,
            };

            let create_info = vk::CommandPoolCreateInfo::default()
                .flags(vk::CommandPoolCreateFlags::TRANSIENT)
                .queue_family_index(family_index);
            let command_pool = unsafe {
                self.device
                    .create_command_pool(&create_info, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            let allocate_info = vk::CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command_buffer = unsafe {
                let mut buffer = MaybeUninit::uninit();
                let result = (self.device.fp_v1_0().allocate_command_buffers)(
                    self.device.handle(),
                    &allocate_info,
                    buffer.as_mut_ptr(),
                );
                result
                    .assume_init_on_success(buffer)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            set_name(self.debug_loader.as_ref(), &bump, command_pool, desc.name);
            set_name(self.debug_loader.as_ref(), &bump, command_buffer, desc.name);

            let out: Box<dyn ICommandList> = Box::new(CommandList {
                _device: self.this.upgrade().unwrap(),
                pool: command_pool,
                buffer: command_buffer,
                list_type: desc.queue_type,
                state: ListState::Empty,
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

            let mut descriptor_writes = BVec::with_capacity_in(writes.len(), bump.allocator());
            for write in writes {
                let d_type = write.writes.descriptor_type();
                let d_type = descriptor_type_to_vk(d_type);
                let new_write = vk::WriteDescriptorSet::default()
                    .dst_set(unsafe { std::mem::transmute(write.set) })
                    .dst_binding(write.binding)
                    .dst_array_element(write.array_element)
                    .descriptor_type(d_type);
                let new_write = match write.writes {
                    DescriptorWrites::Sampler(v) => {
                        let translator = v.iter().map(|v| {
                            vk::DescriptorImageInfo::default()
                                .sampler(Sampler::get(v.sampler).sampler)
                        });
                        let mut image_infos = BVec::new_in(bump.allocator());
                        image_infos.extend(translator);
                        let image_infos = BVec::leak(image_infos);
                        new_write.image_info(image_infos)
                    }
                    DescriptorWrites::TexelBufferRW(v) | DescriptorWrites::TexelBuffer(v) => {
                        let translator = v.iter().map(|_v| vk::BufferView::null());
                        let mut texel_buffer_infos = BVec::new_in(bump.allocator());
                        texel_buffer_infos.extend(translator);
                        let texel_buffer_infos = BVec::leak(texel_buffer_infos);
                        new_write.texel_buffer_view(texel_buffer_infos)
                    }
                    DescriptorWrites::InputAttachment(v)
                    | DescriptorWrites::TextureRW(v)
                    | DescriptorWrites::Texture(v) => {
                        let translator = v.iter().map(|v| {
                            vk::DescriptorImageInfo::default()
                                .image_view(unsafe { std::mem::transmute(v.image_view) })
                                .image_layout(image_layout_to_vk(v.image_layout))
                        });
                        let mut image_infos = BVec::new_in(bump.allocator());
                        image_infos.extend(translator);
                        let image_infos = BVec::leak(image_infos);
                        new_write.image_info(image_infos)
                    }
                    DescriptorWrites::ByteAddressBuffer(v)
                    | DescriptorWrites::ByteAddressBufferRW(v)
                    | DescriptorWrites::StructuredBufferRW(v)
                    | DescriptorWrites::StructuredBuffer(v)
                    | DescriptorWrites::UniformBuffer(v)
                    | DescriptorWrites::UniformBufferDynamic(v) => {
                        let translator = v.iter().map(|v| {
                            let buffer = v.buffer.get().downcast_ref::<Buffer>().unwrap();
                            let len = buffer.clamp_max_size_for_view(v.len);
                            vk::DescriptorBufferInfo::default()
                                .buffer(buffer.buffer)
                                .offset(v.offset)
                                .range(len)
                        });
                        let mut buffer_infos = BVec::new_in(bump.allocator());
                        buffer_infos.extend(translator);
                        let buffer_infos = BVec::leak(buffer_infos);
                        new_write.buffer_info(buffer_infos)
                    }
                };
                descriptor_writes.push(new_write);
            }

            unsafe { self.device.update_descriptor_sets(&descriptor_writes, &[]) };
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let fence = unsafe {
            let mut info = vk::FenceCreateInfo::default();
            if signalled {
                info = info.flags(vk::FenceCreateFlags::SIGNALED)
            }
            self.device
                .create_fence(&info, None)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        let fence = Fence {
            _device: self.this.upgrade().unwrap(),
            fence,
        };
        let fence = ArcedObject::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let semaphore = unsafe {
            let info = vk::SemaphoreCreateInfo::default();
            self.device
                .create_semaphore(&info, None)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        let semaphore = Semaphore {
            _device: self.this.upgrade().unwrap(),
            semaphore,
        };
        let semaphore = ArcedObject::new_arc_opaque(semaphore);
        unsafe { Ok(SemaphoreHandle::new(semaphore)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        fences: &[&FenceHandle],
        wait_all: bool,
        timeout: u32,
    ) -> FenceWaitResult {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let timeout = if timeout == u32::MAX {
                u64::MAX
            } else {
                timeout as u64 * 1000000 // Convert to nanoseconds
            };

            let iter = fences.iter().copied().map(Fence::get).map(|v| v.fence);
            let mut fences = BVec::new_in(bump.allocator());
            fences.extend(iter);

            let result = unsafe { self.device.wait_for_fences(&fences, wait_all, timeout) };

            match result {
                Ok(_) => FenceWaitResult::Complete,
                Err(vk::Result::TIMEOUT) => FenceWaitResult::Timeout,
                v => {
                    v.unwrap();
                    unreachable!()
                }
            }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        let fence = Fence::get(fence);

        let result = unsafe { self.device.get_fence_status(fence.fence) };

        match result {
            Ok(_) => true,
            Err(vk::Result::NOT_READY) => false,
            v => {
                v.unwrap();
                unreachable!()
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, fences: &[&FenceHandle]) {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let iter = fences.iter().copied().map(Fence::get).map(|v| v.fence);
            let mut fences = BVec::new_in(bump.allocator());
            fences.extend(iter);

            unsafe { self.device.reset_fences(&fences).unwrap() }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_id(&self, buffer: &BufferHandle) -> std::num::NonZeroU64 {
        Buffer::get(buffer).get_buffer_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b> {
        Buffer::get(buffer).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn map_buffer(&self, buffer: &BufferHandle) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        Buffer::get(buffer).map_buffer(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        Buffer::get(buffer).unmap_buffer(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).flush_buffer_range(self, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).invalidate_buffer_range(self, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_id(&self, texture: &TextureHandle) -> std::num::NonZeroU64 {
        Texture::get(texture).get_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_desc<'b>(&self, texture: &'b TextureHandle) -> &'b TextureDesc<'b> {
        Texture::get(texture).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_view(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_view(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_rtv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_dsv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_id(&self, sampler: &SamplerHandle) -> std::num::NonZeroU64 {
        Sampler::get(sampler).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_desc<'b>(&self, sampler: &'b SamplerHandle) -> &'b SamplerDesc<'b> {
        Sampler::get(sampler).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_descriptor_set_layout_id(
        &self,
        set_layout: &DescriptorSetLayoutHandle,
    ) -> std::num::NonZeroU64 {
        DescriptorSetLayout::get(set_layout).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_pipeline_layout_id(
        &self,
        pipeline_layout: &PipelineLayoutHandle,
    ) -> std::num::NonZeroU64 {
        PipelineLayout::get(pipeline_layout).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> std::num::NonZeroU64 {
        GraphicsPipeline::get(pipeline).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> std::num::NonZeroU64 {
        ComputePipeline::get(pipeline).id
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
        bump: &'a Blink,
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
            let mut data = BVec::new_in(bump.allocator());
            data.extend(data_iter);
            let data = BVec::leak(data);

            Ok(&*data)
        } else {
            Err(PipelineCreateError::UnsupportedShaderFormat(index))
        }
    }

    fn translate_vertex_bindings<'a>(
        bump: &'a Blink,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<vk::VertexInputBindingDescription, &'a BlinkAlloc> {
        let iter = desc.vertex_layout.input_bindings.iter().map(|v| {
            vk::VertexInputBindingDescription::default()
                .binding(v.binding)
                .stride(v.stride)
                .input_rate(vertex_input_rate_to_vk(v.input_rate))
        });
        let mut out = BVec::new_in(bump.allocator());
        out.extend(iter);
        out
    }

    fn translate_vertex_attributes<'a>(
        bump: &'a Blink,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<vk::VertexInputAttributeDescription, &'a BlinkAlloc> {
        let iter = desc.vertex_layout.input_attributes.iter().map(|v| {
            vk::VertexInputAttributeDescription::default()
                .location(v.location)
                .binding(v.binding)
                .offset(v.offset)
                .format(texture_format_to_vk(v.format))
        });
        let mut out = BVec::new_in(bump.allocator());
        out.extend(iter);
        out
    }

    fn translate_vertex_input_state<'a>(
        vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
        vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> vk::PipelineVertexInputStateCreateInfo<'a> {
        vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(vertex_binding_descriptions)
            .vertex_attribute_descriptions(vertex_attribute_descriptions)
    }

    fn translate_input_assembly_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineInputAssemblyStateCreateInfo<'static> {
        let topology = primitive_topology_to_vk(desc.input_assembly_state.primitive_topology);
        vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(topology)
            .primitive_restart_enable(false)
    }

    fn translate_rasterization_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineRasterizationStateCreateInfo<'static> {
        let polygon_mode = polygon_mode_to_vk(desc.rasterizer_state.polygon_mode);
        let cull_mode = cull_mode_to_vk(desc.rasterizer_state.cull_mode);
        let front_face = front_face_order_to_vk(desc.rasterizer_state.front_face);
        vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(polygon_mode)
            .cull_mode(cull_mode)
            .front_face(front_face)
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false) // No support in dx12
            .depth_bias_enable(desc.rasterizer_state.depth_bias != 0)
            .depth_bias_constant_factor(desc.rasterizer_state.depth_bias as f32)
            .depth_bias_clamp(desc.rasterizer_state.depth_bias_clamp)
            .depth_bias_slope_factor(desc.rasterizer_state.depth_bias_slope_factor)
            .line_width(1.0)
    }

    fn translate_depth_stencil_state(
        desc: &GraphicsPipelineDesc,
    ) -> vk::PipelineDepthStencilStateCreateInfo<'static> {
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

        vk::PipelineDepthStencilStateCreateInfo::default()
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
        bump: &'a Blink,
        desc: &GraphicsPipelineDesc,
    ) -> BVec<vk::PipelineColorBlendAttachmentState, &'a BlinkAlloc> {
        let iter = desc.blend_state.attachments.iter().map(|v| {
            vk::PipelineColorBlendAttachmentState::default()
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
        });
        let mut out = BVec::new_in(bump.allocator());
        out.extend(iter);
        out
    }

    fn translate_color_blend_state(
        attachments: &[vk::PipelineColorBlendAttachmentState],
    ) -> vk::PipelineColorBlendStateCreateInfo {
        vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
    }

    fn translate_framebuffer_info<'a>(
        desc: &GraphicsPipelineDesc,
        color_formats: &'a mut BVec<vk::Format, &BlinkAlloc>,
    ) -> vk::PipelineRenderingCreateInfo<'a> {
        let builder = vk::PipelineRenderingCreateInfo::default();

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
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
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

            self.command_list_pool.collect(self);

            ManuallyDrop::drop(&mut self.allocator);

            self.device.destroy_device(None);
            ManuallyDrop::drop(&mut self.device);
        }
    }
}

thread_local! {
    pub static DEVICE_BUMP: BlinkCell = BlinkCell::new();
}

pub struct FreeCommandList {
    pub pool: vk::CommandPool,
    pub buffer: vk::CommandBuffer,
    pub list_type: QueueType,
}

impl FreeCommandList {
    pub unsafe fn collect(&self, device: &Device) {
        unsafe {
            device.device.destroy_command_pool(self.pool, None);
        }
    }
}

pub struct CommandListPool {
    pub general: ArrayQueue<FreeCommandList>,
    pub compute: ArrayQueue<FreeCommandList>,
    pub transfer: ArrayQueue<FreeCommandList>,
}

impl CommandListPool {
    pub fn new() -> Self {
        // We should only really ever need <num_lists_per_frame> * <frames_in_flight>
        Self {
            general: ArrayQueue::new(64),
            compute: ArrayQueue::new(32),
            transfer: ArrayQueue::new(32),
        }
    }

    pub fn get_for_queue_type(&self, queue_type: QueueType) -> Option<FreeCommandList> {
        match queue_type {
            QueueType::General => self.general.pop(),
            QueueType::Compute => self.compute.pop(),
            QueueType::Transfer => self.transfer.pop(),
        }
    }

    pub fn get_pool_for_queue_type(&self, queue_type: QueueType) -> &ArrayQueue<FreeCommandList> {
        match queue_type {
            QueueType::General => &self.general,
            QueueType::Compute => &self.compute,
            QueueType::Transfer => &self.transfer,
        }
    }

    pub unsafe fn collect(&self, device: &Device) {
        unsafe {
            while let Some(list) = self.general.pop() {
                list.collect(device);
            }

            while let Some(list) = self.compute.pop() {
                list.collect(device);
            }

            while let Some(list) = self.transfer.pop() {
                list.collect(device);
            }
        }
    }
}
