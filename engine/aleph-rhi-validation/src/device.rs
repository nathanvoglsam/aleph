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

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

use aleph_alloc::{BBox, BVec};
use aleph_object_system::{ArcObject, Object};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::parameter_block_pool::ParameterBlockPool;

use crate::internal::parameter_block::ParameterBlock;
use crate::internal::{get_as_unwrapped, unwrap};
use crate::{
    ValidationAdapter, ValidationBindingSignature, ValidationBuffer, ValidationCommandList,
    ValidationComputePipeline, ValidationContext, ValidationDescriptorArena,
    ValidationDescriptorPool, ValidationFence, ValidationGraphicsPipeline,
    ValidationParameterBlockLayout, ValidationQueue, ValidationSampler, ValidationTexture,
};

pub struct ValidationDevice {
    pub(crate) _this: Weak<Self>,
    pub(crate) _context: Arc<ValidationContext>,
    pub(crate) _adapter: Arc<ValidationAdapter>,
    pub(crate) inner: Arc<dyn IDevice>,
    pub(crate) pool_counter: AtomicU64,
    pub(crate) general_queue: Option<Arc<ValidationQueue>>,
    pub(crate) compute_queue: Option<Arc<ValidationQueue>>,
    pub(crate) transfer_queue: Option<Arc<ValidationQueue>>,
}

crate::impl_platform_interface_passthrough!(ValidationDevice);

impl IDevice for ValidationDevice {
    // ========================================================================================== //
    // ========================================================================================== //

    fn upgrade(&self) -> Arc<dyn IDevice> {
        self._this.upgrade().unwrap()
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

    fn garbage_collect(&self) -> Result<(), QueueGarbageCollectError> {
        self.inner.garbage_collect()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) -> Result<(), QueueWaitError> {
        self.inner.wait_idle()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_parameter_block_layout(
        &self,
        desc: &ParameterBlockDesc,
    ) -> Result<Arc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        if desc.flags.contains(ParameterBlockFlags::PUSH_DESCRIPTOR) {
            for param in desc.params {
                match param.ty {
                    ParameterType::Buffer
                    | ParameterType::RWBuffer
                    | ParameterType::Texture1D
                    | ParameterType::RWTexture1D
                    | ParameterType::Texture2D
                    | ParameterType::RWTexture2D
                    | ParameterType::Texture3D
                    | ParameterType::RWTexture3D
                    | ParameterType::Texture1DArray
                    | ParameterType::RWTexture1DArray
                    | ParameterType::Texture2DArray
                    | ParameterType::RWTexture2DArray
                    | ParameterType::Texture3DArray
                    | ParameterType::RWTexture3DArray
                    | ParameterType::Texture2DMS
                    | ParameterType::RWTexture2DMS
                    | ParameterType::Texture2DMSArray
                    | ParameterType::RWTexture2DMSArray
                    | ParameterType::TextureCube
                    | ParameterType::TextureCubeArray
                    | ParameterType::SamplerState => {
                        panic!(
                            "Paremeter type '{}' is illegal in 'ParameterBlockDesc' when 'PUSH_DESCRIPTOR' flag is enabled",
                            param.ty
                        );
                    }
                    _ => {}
                }
                if param.array_size.is_array() {
                    panic!(
                        "Parameter arrays are illegal in 'ParameterBlockDesc' when 'PUSH_DESCRIPTOR' flag is enabled"
                    );
                }
            }
        }

        let inner = self.inner.create_parameter_block_layout(&desc)?;

        let out = Arc::new_cyclic(move |v| ValidationParameterBlockLayout {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
        });
        Ok(out)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_binding_signature(
        &self,
        desc: &BindingSignatureDesc,
    ) -> Result<Arc<dyn IBindingSignature>, BindingSignatureCreateError> {
        if let Some(block) = &desc.push_constant_block {
            if (block.size.get() as u32 % 4) != 0 {
                return Err(BindingSignatureCreateError::InvalidPushConstantBlockSize);
            }
        }

        let mut parameter_block_layouts =
            BVec::with_capacity_in(desc.parameter_block_layouts.len(), RhiSystem::default());
        parameter_block_layouts.extend(
            desc.parameter_block_layouts
                .iter()
                .map(|v| unwrap::parameter_block_layout_d(v)._this.upgrade().unwrap()),
        );
        let mut parameter_block_layouts_inner =
            BVec::with_capacity_in(desc.parameter_block_layouts.len(), RhiSystem::default());
        parameter_block_layouts_inner.extend(
            desc.parameter_block_layouts
                .iter()
                .map(|v| unwrap::parameter_block_layout_d(v).inner.as_ref()),
        );
        let push_constant_block = desc.push_constant_block.clone();
        let new_desc = BindingSignatureDesc {
            parameter_block_layouts: &parameter_block_layouts_inner,
            push_constant_block: push_constant_block.clone(),
            name: desc.name.clone(),
        };
        let inner = self.inner.create_binding_signature(&new_desc)?;

        let out = Arc::new_cyclic(move |v| ValidationBindingSignature {
            _this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            inner,
            parameter_block_layouts,
            push_constant_block,
        });
        Ok(out)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        let mut stage_set = HashSet::with_capacity(8);
        desc.shader_stages.iter().for_each(|v| {
            let stage = v.shader_type();
            let duplicate_stage = !stage_set.insert(stage as u32);
            assert!(
                !duplicate_stage,
                "Provided multiple shader modules of the same type for a graphics pipeline"
            );
            assert_ne!(
                stage,
                ShaderType::Compute,
                "Passed a compute shader module to a graphics pipeline"
            );
        });

        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        let new_desc = GraphicsPipelineDesc {
            shader_stages: desc.shader_stages,
            binding_signature: binding_signature.inner.as_ref(),
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
            _binding_signature: binding_signature._this.upgrade().unwrap(),
            inner,
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(GraphicsPipelineHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        let new_desc = ComputePipelineDesc {
            shader_module: desc.shader_module,
            binding_signature: binding_signature.inner.as_ref(),
            name: desc.name,
        };

        let inner = self.inner.create_compute_pipeline(&new_desc)?;
        let out = ValidationComputePipeline {
            _device: self._this.upgrade().unwrap(),
            _binding_signature: binding_signature._this.upgrade().unwrap(),
            inner,
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(ComputePipelineHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::parameter_block_layout(desc.layout);
        assert!(
            !layout
                .desc()
                .flags
                .contains(ParameterBlockFlags::PUSH_DESCRIPTOR),
            "Creating a IDescriptorPool from a IParameterBlockLayout with the 'PUSH_DESCRIPTOR' flag is not allowed"
        );

        let inner_desc = get_as_unwrapped::descriptor_pool_desc(desc);
        let inner = self.inner.create_descriptor_pool(&inner_desc)?;

        let factory = crate::descriptor_pool::PoolBlockFactory {
            pool_id: self.pool_counter.fetch_add(1, Ordering::Relaxed),
            inner_pool: inner,
        };
        let pool = Box::new(ValidationDescriptorPool {
            _device: self._this.upgrade().unwrap(),
            _layout: layout._this.upgrade().unwrap(),
            pool: ParameterBlockPool::new(factory, desc.num_blocks as usize),
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

        let factory = crate::descriptor_arena::ArenaBlockFactory {
            pool_id: self.pool_counter.fetch_add(1, Ordering::Relaxed),
            inner_pool: inner,
        };
        let pool = Box::new(ValidationDescriptorArena {
            _device: self._this.upgrade().unwrap(),
            pool: ParameterBlockPool::new(factory, desc.num_blocks as usize),
            arena_type: desc.arena_type,
        });

        Ok(pool)
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
            name: desc.name.map(|v| BBox::from(v)),
            inner,
        };
        let out = Object::new_arc_opaque(out);
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
            Object::new(ValidationTexture {
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
        let out = Object::new_arc_opaque(out);
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

    fn get_queue(&self, queue_type: QueueType) -> Option<Arc<dyn IQueue>> {
        let out = match queue_type {
            QueueType::General => self.general_queue.clone(),
            QueueType::Compute => self.compute_queue.clone(),
            QueueType::Transfer => self.transfer_queue.clone(),
        };
        match out {
            None => None,
            Some(v) => Some(v),
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_parameter_block(
        &self,
        layout: &dyn IParameterBlockLayout,
        block: ParameterBlockHandle,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        let layout = unwrap::parameter_block_layout(layout);

        assert!(
            !layout
                .desc()
                .flags
                .contains(ParameterBlockFlags::PUSH_DESCRIPTOR),
            "Can't call 'IDevice::update_parameter_block' on parameter block layout with 'PUSH_DESCRIPTOR' flag set"
        );
        layout.validate_updates(base, writes);

        let layout_inner = layout.inner.as_ref();
        let block = unsafe { block.into_raw::<ParameterBlock>().as_ref().inner.unwrap() };

        let new_writes = unsafe { get_as_unwrapped::parameter_writes(writes) };

        unsafe {
            self.inner
                .update_parameter_block(layout_inner, block, base, &new_writes);
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, value: u64) -> Result<FenceHandle, FenceCreateError> {
        let inner = self.inner.create_fence(value)?;
        let fence = ValidationFence {
            _device: self._this.upgrade().unwrap(),
            inner,
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        fences: &[&FenceHandle],
        values: &[u64],
        wait_all: bool,
        timeout: u32,
    ) -> Result<FenceWaitResult, FenceWaitError> {
        let inner_fences: Vec<_> = fences
            .iter()
            .copied()
            .map(ValidationFence::get)
            .map(|v| &v.inner)
            .collect();
        self.inner
            .wait_fences(&inner_fences, values, wait_all, timeout)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_fence_signaled_value(&self, fence: &FenceHandle) -> Result<u64, FencePollError> {
        let fence = ValidationFence::get(fence);
        self.inner.get_fence_signaled_value(&fence.inner)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn signal_fence(&self, fence: &FenceHandle, value: u64) -> Result<(), FenceSignalError> {
        let fence = ValidationFence::get(fence);
        unsafe { self.inner.signal_fence(&fence.inner, value) }
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
