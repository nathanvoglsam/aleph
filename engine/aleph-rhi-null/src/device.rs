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

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::Object;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::owned_desc::{OwnedBufferDesc, OwnedSamplerDesc, OwnedTextureDesc};

use crate::internal::unwrap;
use crate::{
    NullAdapter, NullBindingSignature, NullBuffer, NullCommandList, NullComputePipeline,
    NullContext, NullDescriptorArena, NullDescriptorPool, NullFence, NullGraphicsPipeline,
    NullParameterBlockLayout, NullQueue, NullSampler, NullSemaphore, NullTexture,
};

pub struct NullDevice {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<NullContext>,
    pub(crate) _adapter: AnyArc<NullAdapter>,
    pub(crate) general_queue: Option<AnyArc<NullQueue>>,
    pub(crate) compute_queue: Option<AnyArc<NullQueue>>,
    pub(crate) transfer_queue: Option<AnyArc<NullQueue>>,
    pub(crate) object_counter: ObjectCounter,
}

declare_interfaces!(NullDevice, [IDevice]);

crate::impl_platform_interface_passthrough!(NullDevice);

impl IDevice for NullDevice {
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

    fn garbage_collect(&self) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_idle(&self) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_parameter_block_layout(
        &self,
        _desc: &ParameterBlockDesc,
    ) -> Result<AnyArc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        let out = AnyArc::new_cyclic(move |v| NullParameterBlockLayout {
            this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            id: self.object_counter.next_parameter_block_layout(),
        });
        Ok(AnyArc::map::<dyn IParameterBlockLayout, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_binding_signature(
        &self,
        _desc: &BindingSignatureDesc,
    ) -> Result<AnyArc<dyn IBindingSignature>, BindingSignatureCreateError> {
        let out = AnyArc::new_cyclic(move |v| NullBindingSignature {
            this: v.clone(),
            _device: self._this.upgrade().unwrap(),
            id: self.object_counter.next_binding_signature(),
        });
        Ok(AnyArc::map::<dyn IBindingSignature, _>(out, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        let pipeline = NullGraphicsPipeline {
            _device: self._this.upgrade().unwrap(),
            _binding_signature: binding_signature.this.upgrade().unwrap(),
            id: self.object_counter.next_compute_pipeline(),
        };
        let v = Object::new_arc_opaque(pipeline);
        unsafe { Ok(GraphicsPipelineHandle::new(v)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError> {
        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        let pipeline = NullComputePipeline {
            _device: self._this.upgrade().unwrap(),
            _binding_signature: binding_signature.this.upgrade().unwrap(),
            id: self.object_counter.next_compute_pipeline(),
        };
        let v = Object::new_arc_opaque(pipeline);
        unsafe { Ok(ComputePipelineHandle::new(v)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::parameter_block_layout(desc.layout);
        let pool = Box::new(NullDescriptorPool {
            _device: self._this.upgrade().unwrap(),
            _layout: layout.this.upgrade().unwrap(),
            counter: 1,
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        _desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        let pool = Box::new(NullDescriptorArena {
            _device: self._this.upgrade().unwrap(),
            counter: Cell::new(1),
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        let out = NullBuffer {
            _device: self._this.upgrade().unwrap(),
            id: self.object_counter.next_buffer(),
            desc: OwnedBufferDesc::new(desc.clone()),
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
        let out = NullTexture {
            _device: self._this.upgrade().unwrap(),
            id: self.object_counter.next_texture(),
            desc: OwnedTextureDesc::new(desc.clone()),
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(TextureHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        let sampler = NullSampler {
            _device: self._this.upgrade().unwrap(),
            id: self.object_counter.next_sampler(),
            desc: OwnedSamplerDesc::new(desc.clone()),
        };
        let v = Object::new_arc_opaque(sampler);
        unsafe { Ok(SamplerHandle::new(v)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        _desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let list = Box::new(NullCommandList {
            _device: self._this.upgrade().unwrap(),
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

    unsafe fn update_parameter_block(
        &self,
        _layout: &dyn IParameterBlockLayout,
        _block: ParameterBlockHandle,
        _base: u32,
        _writes: &[ParameterWrite],
    ) {
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, _signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let fence = NullFence {
            _device: self._this.upgrade().unwrap(),
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let fence = NullSemaphore {
            _device: self._this.upgrade().unwrap(),
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(SemaphoreHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(
        &self,
        _fences: &[&FenceHandle],
        _wait_all: bool,
        _timeout: u32,
    ) -> FenceWaitResult {
        FenceWaitResult::Complete
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, _fence: &FenceHandle) -> bool {
        true
    }

    fn reset_fences(&self, _fences: &[&FenceHandle]) {}

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Null
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_id(&self, buffer: &BufferHandle) -> std::num::NonZeroU64 {
        NullBuffer::get(buffer).get_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b> {
        NullBuffer::get(buffer).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn map_buffer(&self, buffer: &BufferHandle) -> Result<std::ptr::NonNull<u8>, ResourceMapError> {
        let _ = NullBuffer::get(buffer);
        unimplemented!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        let _ = NullBuffer::get(buffer);
        Ok(())
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, _offset: u64, _len: u64) {
        let _ = NullBuffer::get(buffer);
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, _offset: u64, _len: u64) {
        let _ = NullBuffer::get(buffer);
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_id(&self, texture: &TextureHandle) -> std::num::NonZeroU64 {
        NullTexture::get(texture).get_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_desc<'b>(&self, texture: &'b TextureHandle) -> &'b TextureDesc<'b> {
        NullTexture::get(texture).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_view(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        NullTexture::get(texture).get_view(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        NullTexture::get(texture).get_rtv(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        NullTexture::get(texture).get_dsv(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_id(&self, sampler: &SamplerHandle) -> std::num::NonZeroU64 {
        NullSampler::get(sampler).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_sampler_desc<'b>(&self, sampler: &'b SamplerHandle) -> &'b SamplerDesc<'b> {
        NullSampler::get(sampler).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> std::num::NonZeroU64 {
        NullGraphicsPipeline::get(pipeline).id
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> std::num::NonZeroU64 {
        NullComputePipeline::get(pipeline).id
    }
}
