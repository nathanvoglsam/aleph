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

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroU32;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::{ArcObject, ArcedObject};
use aleph_rhi_api::*;
use crossbeam::atomic::AtomicCell;

use crate::descriptor_set_layout::DescriptorBindingInfo;
use crate::fence::FenceState;
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::get_as_unwrapped;
use crate::semaphore::SemaphoreState;
use crate::texture::{ValidationImageView, ValidationViewType};
use crate::{
    ValidationAdapter, ValidationBuffer, ValidationCommandList, ValidationComputePipeline,
    ValidationContext, ValidationDescriptorArena, ValidationDescriptorPool,
    ValidationDescriptorSetLayout, ValidationFence, ValidationGraphicsPipeline,
    ValidationPipelineLayout, ValidationQueue, ValidationSampler, ValidationSemaphore,
    ValidationTexture,
};

pub struct ValidationDevice {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<ValidationContext>,
    pub(crate) _adapter: AnyArc<ValidationAdapter>,
    pub(crate) inner: AnyArc<dyn IDevice>,
    pub(crate) pool_counter: AtomicU64,
    pub(crate) general_queue: Option<AnyArc<ValidationQueue>>,
    pub(crate) compute_queue: Option<AnyArc<ValidationQueue>>,
    pub(crate) transfer_queue: Option<AnyArc<ValidationQueue>>,
}

declare_interfaces!(ValidationDevice, [IDevice]);

crate::impl_platform_interface_passthrough!(ValidationDevice);

impl IDevice for ValidationDevice {
    // ========================================================================================== //
    // ========================================================================================== //

    fn upgrade(&self) -> AnyArc<dyn IDevice> {
        AnyArc::map::<dyn IDevice, _>(self._this.upgrade().unwrap(), |v| v)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn garbage_collect(&self) {
        self.inner.garbage_collect()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {
        self.inner.wait_idle()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        let mut stage_set = HashSet::with_capacity(8);
        desc.shader_stages.iter().for_each(|v| {
            let duplicate_stage = !stage_set.insert(v.stage as u32);
            assert!(
                !duplicate_stage,
                "Provided multiple shader modules of the same type for a graphics pipeline"
            );
            assert_ne!(
                v.stage,
                ShaderType::Compute,
                "Passed a compute shader module to a graphics pipeline"
            );
        });

        let pipeline_layout = ValidationPipelineLayout::get_owned(desc.pipeline_layout);

        let new_desc = GraphicsPipelineDesc {
            shader_stages: desc.shader_stages,
            pipeline_layout: &pipeline_layout.inner,
            vertex_layout: desc.vertex_layout,
            input_assembly_state: desc.input_assembly_state,
            rasterizer_state: desc.rasterizer_state,
            depth_stencil_state: desc.depth_stencil_state,
            blend_state: desc.blend_state,
            render_target_formats: desc.render_target_formats,
            depth_stencil_format: desc.depth_stencil_format,
            name: desc.name,
        };

        let inner = self.inner.create_graphics_pipeline(&new_desc)?;
        let out = ValidationGraphicsPipeline {
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout,
            inner,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(GraphicsPipelineHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        let pipeline_layout = ValidationPipelineLayout::get_owned(desc.pipeline_layout);

        let new_desc = ComputePipelineDesc {
            shader_module: desc.shader_module,
            pipeline_layout: &pipeline_layout.inner,
            name: desc.name,
        };

        let inner = self.inner.create_compute_pipeline(&new_desc)?;
        let out = ValidationComputePipeline {
            _device: self._this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout,
            inner,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(ComputePipelineHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<DescriptorSetLayoutHandle, DescriptorSetLayoutCreateError> {
        Self::validate_descriptor_set_layout(desc);

        // Extract binding metadata we need for validation
        let mut binding_info = HashMap::new();
        for binding in desc.items {
            binding_info.insert(
                binding.binding_num,
                DescriptorBindingInfo {
                    r#type: binding.binding_type,
                    descriptor_count: binding.binding_count,
                },
            );
        }

        // Construct a new list of bindings matching the outer description, but with the inner
        // references unwrapped
        let mut items = Vec::with_capacity(desc.items.len());
        for v in desc.items.iter() {
            let item = DescriptorSetLayoutBinding {
                binding_num: v.binding_num,
                binding_type: v.binding_type,
                binding_count: v.binding_count,
            };
            items.push(item);
        }

        // Finally, make our unwrapped description to give to the inner implementation
        let new_desc = DescriptorSetLayoutDesc {
            visibility: desc.visibility,
            items: items.as_slice(),
            name: desc.name,
        };

        let inner = self.inner.create_descriptor_set_layout(&new_desc)?;
        let out = ValidationDescriptorSetLayout {
            _device: self._this.upgrade().unwrap(),
            inner,
            binding_info,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(DescriptorSetLayoutHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let inner_desc = get_as_unwrapped::descriptor_pool_desc(desc);
        let inner = self.inner.create_descriptor_pool(&inner_desc)?;

        let layout = ValidationDescriptorSetLayout::get_owned(desc.layout);

        let pool = Box::new(ValidationDescriptorPool {
            _device: self._this.upgrade().unwrap(),
            _layout: layout,
            inner,
            pool_id: self.pool_counter.fetch_add(1, Ordering::Relaxed),
            set_objects: Vec::with_capacity(desc.num_sets as usize),
            free_list: Vec::with_capacity(128),
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        let inner = self.inner.create_descriptor_arena(desc)?;

        let pool = Box::new(ValidationDescriptorArena {
            _device: self._this.upgrade().unwrap(),
            inner,
            pool_id: self.pool_counter.fetch_add(1, Ordering::Relaxed),
            set_objects: Cell::new(Vec::with_capacity(desc.num_sets as usize)),
            free_list: Cell::new(Vec::with_capacity(128)),
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<PipelineLayoutHandle, PipelineLayoutCreateError> {
        // TODO: implement validation for create_pipeline_layout

        // Unwrap the objects in 'set_layouts' into a new list so the layer below gets the correct
        // object implementations
        let inner =
            get_as_unwrapped::pipeline_layout_desc(desc, |v| self.inner.create_pipeline_layout(v))?;

        let mut push_constant_blocks = Vec::new();
        for block in desc.push_constant_blocks {
            if (block.size % 4) != 0 {
                return Err(PipelineLayoutCreateError::InvalidPushConstantBlockSize);
            }
            push_constant_blocks.push(block.clone());
        }

        let out = ValidationPipelineLayout {
            _device: self._this.upgrade().unwrap(),
            inner,
            push_constant_blocks,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(PipelineLayoutHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        assert!(
            ResourceUsageFlags::BUFFER_USAGE_MASK.contains(desc.usage),
            "Attempted to create a buffer with usage flags meant only for textures!"
        );
        let inner = self.inner.create_buffer(desc)?;
        let out = ValidationBuffer {
            _device: self._this.upgrade().unwrap(),
            size: desc.size,
            usage: desc.usage,
            inner,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        assert!(
            ResourceUsageFlags::TEXTURE_USAGE_MASK.contains(desc.usage),
            "Attempted to create a texture with usage flags meant only for buffers!"
        );
        assert_ne!(desc.width, 0, "desc.width must be > 0");
        assert_ne!(desc.height, 0, "desc.height must be > 0");
        assert_ne!(desc.depth, 0, "desc.depth must be > 0");
        assert_ne!(desc.mip_levels, 0, "desc.mip_levels must be > 0");
        assert_ne!(desc.array_size, 0, "desc.array_size must be > 0");
        let inner = self.inner.create_texture(desc)?;
        let out = Arc::new_cyclic(|v| {
            ArcedObject::new(ValidationTexture {
                _this: v.clone(),
                _device: self._this.upgrade().unwrap(),
                inner,
                desc: desc.clone().strip_name(),
                views: Default::default(),
                rtvs: Default::default(),
                dsvs: Default::default(),
            })
        });
        let out = ArcObject::from_object(out);
        unsafe { Ok(TextureHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        let inner = self.inner.create_sampler(desc)?;
        let out = ValidationSampler {
            _device: self._this.upgrade().unwrap(),
            inner,
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(SamplerHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let inner = self.inner.create_command_list(desc)?;
        let list = Box::new(ValidationCommandList {
            _device: self._this.upgrade().unwrap(),
            inner,
            list_type: desc.queue_type,
        });
        Ok(list)
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
        unsafe {
            writes
                .iter()
                .for_each(|v| Self::validate_descriptor_write(v));

            get_as_unwrapped::descriptor_set_updates(writes, |writes| {
                self.inner.update_descriptor_sets(writes)
            })
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let initial_state = if signalled {
            FenceState::ObservedAsSignalled
        } else {
            FenceState::NotSignalled
        };
        let inner = self.inner.create_fence(signalled)?;
        let fence = ValidationFence {
            _device: self._this.upgrade().unwrap(),
            inner,
            state: AtomicCell::new(initial_state),
        };
        let fence = ArcedObject::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let inner = self.inner.create_semaphore()?;
        let semaphore = ValidationSemaphore {
            _device: self._this.upgrade().unwrap(),
            inner,
            state: AtomicCell::new(SemaphoreState::Reset),
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
        fences.iter().for_each(|v| {
            let v = ValidationFence::get(v);
            if timeout == u32::MAX {
                let fence_state = v.state.load();
                assert_ne!(
                    fence_state,
                    FenceState::NotSignalled,
                    "It is invalid to wait on a fence with no pending work with a u32::MAX timeout"
                );
            }
        });

        let inner_fences: Vec<_> = fences
            .iter()
            .copied()
            .map(ValidationFence::get)
            .map(|v| &v.inner)
            .collect();
        let result = self.inner.wait_fences(&inner_fences, wait_all, timeout);

        if result == FenceWaitResult::Complete {
            // If we met the wait condition we can update the fence states as at least one of them
            // have been signalled
            fences.iter().for_each(|v| {
                let v = ValidationFence::get(v);

                // We can only update the state if we can prove the fence is signalled.
                //
                // If 'wait_all' is true we know that all the fences are signalled so we can update
                // the state without any further checks.
                //
                // If 'wait_all' is false then we only know that at least one fence is signalled.
                // We poll all the fences after the wait to confirm they are in fact signalled and
                // update the state accordingly.
                if wait_all {
                    v.state.store(FenceState::ObservedAsSignalled);
                } else {
                    // Will update the state as a side effect of calling poll_fence. Skips storing
                    // to the state twice
                    self.poll_fence(&v.inner);
                }
            });
        }

        result
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        let fence = ValidationFence::get(fence);

        let result = self.inner.poll_fence(&fence.inner);

        if result {
            fence.state.store(FenceState::ObservedAsSignalled);
        }

        result
    }

    fn reset_fences(&self, fences: &[&FenceHandle]) {
        fences.iter().for_each(|v| {
            let v = ValidationFence::get(v);
            let fence_state = v.state.load();
            assert_ne!(
                fence_state,
                FenceState::Pending,
                "It is invalid to reset a fence while it is still in use on a queue."
            );
        });

        let inner_fences: Vec<_> = fences
            .iter()
            .copied()
            .map(ValidationFence::get)
            .map(|v| &v.inner)
            .collect();

        self.inner.reset_fences(&inner_fences);

        fences.iter().for_each(|v| {
            let v = ValidationFence::get(v);
            v.state.store(FenceState::NotSignalled);
        });
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        self.inner.get_backend_api()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_id(&self, buffer: &BufferHandle) -> std::num::NonZeroU64 {
        let buffer = ValidationBuffer::get(buffer);
        self.inner.get_buffer_id(&buffer.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b> {
        let buffer = ValidationBuffer::get(buffer);
        self.inner.get_buffer_desc(&buffer.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn map_buffer(&self, buffer: &BufferHandle) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        let buffer = ValidationBuffer::get(buffer);
        self.inner.map_buffer(&buffer.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        let buffer = ValidationBuffer::get(buffer);
        self.inner.unmap_buffer(&buffer.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        let buffer = ValidationBuffer::get(buffer);
        buffer.validate_range(offset, len);
        self.inner.flush_buffer_range(&buffer.inner, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        let buffer = ValidationBuffer::get(buffer);
        buffer.validate_range(offset, len);
        self.inner
            .invalidate_buffer_range(&buffer.inner, offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_id(&self, texture: &TextureHandle) -> std::num::NonZeroU64 {
        let texture = ValidationTexture::get(texture);
        self.inner.get_texture_id(&texture.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_desc<'b>(&self, texture: &'b TextureHandle) -> &'b TextureDesc<'b> {
        let texture = ValidationTexture::get(texture);
        self.inner.get_texture_desc(&texture.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_view(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        let texture = ValidationTexture::get(texture);
        texture.get_view(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        let texture = ValidationTexture::get(texture);
        texture.get_rtv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        let texture = ValidationTexture::get(texture);
        texture.get_dsv(self, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_id(&self, sampler: &SamplerHandle) -> std::num::NonZeroU64 {
        let v = ValidationSampler::get(sampler);
        self.inner.get_sampler_id(&v.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_desc<'b>(&self, sampler: &'b SamplerHandle) -> &'b SamplerDesc<'b> {
        let v = ValidationSampler::get(sampler);
        self.inner.get_sampler_desc(&v.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_descriptor_set_layout_id(
        &self,
        set_layout: &DescriptorSetLayoutHandle,
    ) -> std::num::NonZeroU64 {
        ValidationDescriptorSetLayout::get(set_layout).get_id(self)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_pipeline_layout_id(
        &self,
        pipeline_layout: &PipelineLayoutHandle,
    ) -> std::num::NonZeroU64 {
        let v = ValidationPipelineLayout::get(pipeline_layout);
        self.inner.get_pipeline_layout_id(&v.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> std::num::NonZeroU64 {
        let v = ValidationGraphicsPipeline::get(pipeline);
        self.inner.get_graphics_pipeline_id(&v.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> std::num::NonZeroU64 {
        let v = ValidationComputePipeline::get(pipeline);
        self.inner.get_compute_pipeline_id(&v.inner)
    }
}

impl ValidationDevice {
    // ========================================================================================== //
    // ========================================================================================== //

    pub fn validate_descriptor_set_layout(desc: &DescriptorSetLayoutDesc) {
        for binding in desc.items.iter() {
            if binding.binding_count.is_some() {
                unimplemented!("Currently descriptor arrays are unimplemented");
            }
        }

        fn calculate_binding_range(v: &DescriptorSetLayoutBinding) -> (u32, u32) {
            let start = v.binding_num;
            let num = v.binding_count.map(NonZeroU32::get).unwrap_or(1);
            let end = start + num;
            (start, end)
        }

        desc.items.iter().enumerate().for_each(|(outer_i, outer)| {
            let (outer_start, outer_end) = calculate_binding_range(outer);

            desc.items.iter().enumerate().for_each(|(inner_i, inner)| {
                // Skip over outer_i so we don't check if the outer range intersects with itself
                if outer_i == inner_i {
                    return;
                }

                let (inner_start, inner_end) = calculate_binding_range(inner);

                let starts_inside_outer = inner_start >= outer_start && inner_start <= outer_end;
                let ends_inside_outer = inner_end >= outer_start && inner_end <= outer_end;

                assert!(
                    !starts_inside_outer || !ends_inside_outer,
                    "It is invalid for two descriptor binding ranges to intersect"
                );
            })
        });
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// # Safety
    ///
    /// This function does not check if the pointer inside [DescriptorWriteDesc].set is valid, as
    /// it is unknowable in isolation. It is the caller's responsibility to ensure the descriptor
    /// set being written to is still live and valid, and the caller's responsibility to synchronize
    /// access.
    pub unsafe fn validate_descriptor_write(write: &DescriptorWriteDesc) {
        let set = unsafe { DescriptorSet::ref_from_handle(&write.set) };
        let layout = set._layout.deref();

        // Checks if the binding is actually present in the descriptor set.
        let info = if let Some(info) = layout.get_binding_info(write.binding) {
            info
        } else {
            panic!(
                "Trying to write to a descriptor binding '{}' not present in the set",
                write.binding
            )
        };

        // Check if the user is requesting to write the correct descriptor type. That is, the
        // 'descriptor_type' in the write description must match the type declared in the descriptor
        // set layout.
        let expected_binding_type = info.r#type;
        let actual_binding_type = write.writes.descriptor_type();
        assert_eq!(
            expected_binding_type, actual_binding_type,
            "It is invalid to write the incorrect descriptor type into a binding."
        );

        // Check if the user is trying to write more than 1 descriptor into a non-array binding.
        let is_array_binding = info.descriptor_count.is_some();
        let num_writes = write.writes.len();
        assert!(
            !is_array_binding && num_writes <= 1,
            "It is invalid to write more than 1 descriptor into a non-array binding."
        );

        // Check if the user is trying to write outside of an array binding's range.
        let write_start = write.array_element as usize;
        let write_end = write_start + write.writes.len();
        let binding_start = 0;
        let binding_end = info.descriptor_count.map(NonZeroU32::get).unwrap_or(1) as usize;
        assert!(
            write_start >= binding_start && write_start < binding_end,
            "It is invalid to write outside of an array binding's bounds."
        );
        assert!(
            write_end > binding_start && write_end <= binding_end,
            "It is invalid to write outside of an array binding's bounds."
        );

        // Check that the declared descriptor type matches the DescriptorWrites variant provided.
        match &write.writes {
            DescriptorWrites::Texture(writes) => {
                for v in writes.iter() {
                    let image_view = unsafe {
                        &*std::mem::transmute::<_, *const ValidationImageView>(v.image_view)
                    };

                    Self::validate_image_view_type(image_view);

                    let texture = image_view
                        ._image
                        .upgrade()
                        .expect("Trying to write view for destroyed image");

                    Self::validate_texture_usage(&texture, ResourceUsageFlags::SHADER_RESOURCE);
                }
            }
            DescriptorWrites::TextureRW(writes) => {
                for v in writes.iter() {
                    let image_view = unsafe {
                        &*std::mem::transmute::<_, *const ValidationImageView>(v.image_view)
                    };

                    Self::validate_image_view_type(image_view);

                    let texture = image_view
                        ._image
                        .upgrade()
                        .expect("Trying to write view for destroyed image");

                    Self::validate_texture_usage(&texture, ResourceUsageFlags::UNORDERED_ACCESS);
                }
            }
            DescriptorWrites::UniformBuffer(writes)
            | DescriptorWrites::UniformBufferDynamic(writes) => {
                for write in writes.iter() {
                    let buffer = write
                        .buffer
                        .get()
                        .downcast_ref::<ValidationBuffer>()
                        .unwrap();
                    Self::validate_buffer_usage(buffer, ResourceUsageFlags::CONSTANT_BUFFER);
                    Self::validate_buffer_write_range(buffer, write);
                    Self::validate_uniform_buffer_offset_alignment(write);
                }
            }
            DescriptorWrites::StructuredBuffer(writes) => {
                for write in writes.iter() {
                    let buffer = write
                        .buffer
                        .get()
                        .downcast_ref::<ValidationBuffer>()
                        .unwrap();
                    Self::validate_buffer_usage(buffer, ResourceUsageFlags::SHADER_RESOURCE);
                    Self::validate_buffer_write_range(buffer, write);
                }
            }
            DescriptorWrites::StructuredBufferRW(writes) => {
                for write in writes.iter() {
                    let buffer = write
                        .buffer
                        .get()
                        .downcast_ref::<ValidationBuffer>()
                        .unwrap();
                    Self::validate_buffer_usage(buffer, ResourceUsageFlags::UNORDERED_ACCESS);
                    Self::validate_buffer_write_range(buffer, write);
                }
            }
            DescriptorWrites::ByteAddressBuffer(writes) => {
                for write in writes.iter() {
                    let buffer = write
                        .buffer
                        .get()
                        .downcast_ref::<ValidationBuffer>()
                        .unwrap();
                    Self::validate_buffer_usage(buffer, ResourceUsageFlags::SHADER_RESOURCE);
                    Self::validate_buffer_write_range(buffer, write);
                }
            }
            DescriptorWrites::ByteAddressBufferRW(writes) => {
                for write in writes.iter() {
                    let buffer = write
                        .buffer
                        .get()
                        .downcast_ref::<ValidationBuffer>()
                        .unwrap();
                    Self::validate_buffer_usage(buffer, ResourceUsageFlags::UNORDERED_ACCESS);
                    Self::validate_buffer_write_range(buffer, write);
                }
            }
            _ => {}
        }
    }

    fn validate_image_view_type(image_view: &ValidationImageView) {
        assert!(
            matches!(image_view.view_type, ValidationViewType::ResourceView),
            "Writing a resource view with an '{:?}' image view is invalid",
            image_view.view_type
        );
    }

    fn validate_texture_usage(texture: &ValidationTexture, required: ResourceUsageFlags) {
        let texture_usage = texture.desc.usage;
        assert!(
            texture_usage.contains(required),
            "Texture missing required usage '{:?}' for view",
            required
        );
    }

    fn validate_buffer_usage(buffer: &ValidationBuffer, required: ResourceUsageFlags) {
        let buffer_usage = buffer.usage;
        assert!(
            buffer_usage.contains(required),
            "Buffer missing required usage '{:?}' for view",
            required
        );
    }

    fn validate_buffer_write_range(buffer: &ValidationBuffer, write: &BufferDescriptorWrite) {
        let buffer_size = buffer.size;
        let view_end = write.offset + write.len as u64;
        assert!(
            view_end <= buffer_size,
            "Buffer view 'offset: {} len: {}' out of buffer bounds. Size '{}'.",
            write.offset,
            write.len,
            buffer_size
        );
    }

    fn validate_uniform_buffer_offset_alignment(write: &BufferDescriptorWrite) {
        assert!(
            write.offset % 256 == 0,
            "UniformBuffer offset '{}' does not maintain 256 byte alignment",
            write.offset
        );
    }
}
