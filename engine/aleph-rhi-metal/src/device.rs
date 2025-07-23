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
use std::sync::atomic::AtomicU64;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BlinkCell;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::owned_desc::{OwnedBufferDesc, OwnedSamplerDesc, OwnedTextureDesc};
use blink_alloc::Blink;
use crossbeam::queue::ArrayQueue;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::adapter::Adapter;
use crate::buffer::{Buffer, BufferObjects};
use crate::command_list::{CommandList, CommandListObjects, ListState};
use crate::context::Context;
use crate::descriptor_arena::DescriptorArena;
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::fence::{Fence, FenceObjects};
use crate::internal::conv;
use crate::pipeline::{
    CachedGraphicsInfo, ComputePipeline, ComputePipelineObjects, GraphicsPipeline,
    GraphicsPipelineObjects,
};
use crate::pipeline_layout::PipelineLayout;
use crate::queue::Queue;
use crate::sampler::{Sampler, SamplerObjects};
use crate::semaphore::Semaphore;
use crate::texture::{Texture, TextureObjects};

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) adapter: AnyArc<Adapter>,
    pub(crate) device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
    pub(crate) object_counter: ObjectCounter,
}

// Safety: Needed because of 'MTLDevice'
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

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
        todo!()
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

            let mtl_desc = MTLRenderPipelineDescriptor::new();

            for stage in desc.shader_stages {
                match stage.stage {
                    ShaderType::Compute => panic!("Graphics pipelines can't use compute shaders!"),
                    ShaderType::Vertex => {
                        // todo: get this from the shader bytes
                        mtl_desc.setVertexFunction(None);
                    }
                    ShaderType::Hull => unimplemented!(),
                    ShaderType::Domain => unimplemented!(),
                    ShaderType::Geometry => unimplemented!(),
                    ShaderType::Fragment => {
                        // todo: get this from the shader bytes
                        mtl_desc.setFragmentFunction(None);
                    }
                    ShaderType::Amplification => unimplemented!(),
                    ShaderType::Mesh => unimplemented!(),
                }
            }

            let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);

            // vertex_layout: &'a VertexInputStateDesc<'a>,
            // TODO

            let primitive_topology = desc.input_assembly_state.primitive_topology;
            let primitive_type = conv::primitive_topology_to_mtl(primitive_topology);
            unsafe {
                let mtl_topology = conv::primitive_topology_to_mtl_class(primitive_topology);
                mtl_desc.setInputPrimitiveTopology(mtl_topology);
            }

            let cull_mode = conv::cull_mode_to_mtl(desc.rasterizer_state.cull_mode);
            let front_face = conv::front_face_order_to_mtl(desc.rasterizer_state.front_face);
            let polygon_mode = conv::polygon_mode_to_mtl(desc.rasterizer_state.polygon_mode);
            let depth_bias = desc.rasterizer_state.depth_bias;
            let depth_bias_clamp = desc.rasterizer_state.depth_bias_clamp;
            let depth_bias_slope_factor = desc.rasterizer_state.depth_bias_slope_factor;

            // TODO depth bounds
            let mtl_depth_desc = unsafe { MTLDepthStencilDescriptor::new() };

            // 'depth_test = false' just decays to 'always'
            let compare_fn = match (
                desc.depth_stencil_state.depth_test,
                desc.depth_stencil_state.depth_compare_op,
            ) {
                (true, v) => conv::compare_op_to_mtl(v),
                (false, _) => MTLCompareFunction::Always,
            };
            mtl_depth_desc.setDepthCompareFunction(compare_fn);
            mtl_depth_desc.setDepthWriteEnabled(desc.depth_stencil_state.depth_write);

            fn apply_stencil_op_state(to: &MTLStencilDescriptor, v: &StencilOpState) {
                to.setStencilFailureOperation(conv::stencil_op_to_mtl(v.fail_op));
                to.setDepthStencilPassOperation(conv::stencil_op_to_mtl(v.pass_op));
                to.setDepthFailureOperation(conv::stencil_op_to_mtl(v.depth_fail_op));
                to.setStencilCompareFunction(conv::compare_op_to_mtl(v.compare_op));
            }
            if desc.depth_stencil_state.stencil_test {
                let front = unsafe { MTLStencilDescriptor::new() };
                front.setReadMask(desc.depth_stencil_state.stencil_read_mask as u32);
                front.setWriteMask(desc.depth_stencil_state.stencil_write_mask as u32);
                apply_stencil_op_state(&front, &desc.depth_stencil_state.stencil_front);
                mtl_depth_desc.setFrontFaceStencil(Some(&front));

                let back = unsafe { MTLStencilDescriptor::new() };
                back.setReadMask(desc.depth_stencil_state.stencil_read_mask as u32);
                back.setWriteMask(desc.depth_stencil_state.stencil_write_mask as u32);
                apply_stencil_op_state(&back, &desc.depth_stencil_state.stencil_back);
                mtl_depth_desc.setBackFaceStencil(Some(&back));
            }

            let depth_stencil_state = self
                .device
                .newDepthStencilStateWithDescriptor(&mtl_depth_desc);
            let depth_stencil_state = match depth_stencil_state {
                Some(v) => v,
                None => {
                    log::error!(
                        "Failed to create depth stencil state for pipeline! Reason: {}",
                        "unknown"
                    );
                    return Err(PipelineCreateError::Platform);
                }
            };

            let attachments = mtl_desc.colorAttachments();
            for (i, (format, blend)) in desc
                .render_target_formats
                .iter()
                .zip(desc.blend_state.attachments)
                .enumerate()
            {
                let mtl_attachment = unsafe { MTLRenderPipelineColorAttachmentDescriptor::new() };

                mtl_attachment.setPixelFormat(conv::format_to_pixel_mtl(*format));
                mtl_attachment.setWriteMask(conv::write_mask_to_mtl(blend.color_write_mask));

                if blend.blend_enabled {
                    mtl_attachment.setBlendingEnabled(blend.blend_enabled);

                    let v = conv::blend_factor_to_mtl(blend.src_factor);
                    mtl_attachment.setSourceRGBBlendFactor(v);

                    let v = conv::blend_factor_to_mtl(blend.dst_factor);
                    mtl_attachment.setDestinationRGBBlendFactor(v);

                    let v = conv::alpha_blend_factor_to_mtl(blend.alpha_src_factor);
                    mtl_attachment.setSourceAlphaBlendFactor(v);

                    let v = conv::alpha_blend_factor_to_mtl(blend.alpha_dst_factor);
                    mtl_attachment.setDestinationAlphaBlendFactor(v);

                    let v = conv::blend_op_to_mtl(blend.blend_op);
                    mtl_attachment.setRgbBlendOperation(v);

                    let v = conv::blend_op_to_mtl(blend.alpha_blend_op);
                    mtl_attachment.setAlphaBlendOperation(v);
                }

                unsafe {
                    attachments.setObject_atIndexedSubscript(Some(&mtl_attachment), i);
                }
            }

            if let Some(fmt) = desc.depth_stencil_format {
                let mtl_format = conv::format_to_pixel_mtl(fmt);
                mtl_desc.setDepthAttachmentPixelFormat(mtl_format);

                if fmt.is_stencil() {
                    mtl_desc.setStencilAttachmentPixelFormat(mtl_format);
                }
            }

            if let Some(name) = desc.name
                && self.context.debug
            {
                let mtl_name = NSString::from_str(name);
                mtl_desc.setLabel(Some(&mtl_name));
            }

            if self.context.validation {
                unsafe {
                    mtl_desc.setShaderValidation(MTLShaderValidation::Enabled);
                }
            }

            let pipeline = self
                .device
                .newRenderPipelineStateWithDescriptor_error(&mtl_desc);
            let pipeline = match pipeline {
                Ok(v) => v,
                Err(_err) => {
                    log::error!(
                        "Failed to create render pipeline state! Reason: {}",
                        "unknown"
                    );
                    return Err(PipelineCreateError::Platform);
                }
            };

            let out = GraphicsPipeline {
                _device: self.this.upgrade().unwrap(),
                _pipeline_layout: pipeline_layout,
                id: self.object_counter.next_graphics_pipeline(),
                objects: GraphicsPipelineObjects {
                    pipeline,
                    depth_stencil_state,
                },
                info: CachedGraphicsInfo {
                    primitive_type,
                    cull_mode,
                    front_face,
                    polygon_mode,
                    depth_bias,
                    depth_bias_clamp,
                    depth_bias_slope_factor,
                },
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
        let mtl_desc = MTLComputePipelineDescriptor::new();

        // shader_module: ShaderBinary<'a>,
        // TODO

        // pipeline_layout: &'a PipelineLayoutHandle,
        let pipeline_layout = PipelineLayout::get_owned(desc.pipeline_layout);
        // TODO

        if let Some(name) = desc.name
            && self.context.debug
        {
            let mtl_name = NSString::from_str(name);
            mtl_desc.setLabel(Some(&mtl_name));
        }

        if self.context.validation {
            unsafe {
                mtl_desc.setShaderValidation(MTLShaderValidation::Enabled);
            }
        }

        let pipeline = unsafe {
            self.device
                .newComputePipelineStateWithDescriptor_options_reflection_error(
                    &mtl_desc,
                    MTLPipelineOption::empty(),
                    None,
                )
        };

        let pipeline = match pipeline {
            Ok(v) => v,
            Err(_err) => {
                log::error!(
                    "Failed to create render pipeline state! Reason: {}",
                    "unknown"
                );
                return Err(PipelineCreateError::Platform);
            }
        };

        let out = ComputePipeline {
            _device: self.this.upgrade().unwrap(),
            _pipeline_layout: pipeline_layout,
            id: self.object_counter.next_compute_pipeline(),
            objects: ComputePipelineObjects { pipeline },
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
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut samplers = Vec::new();
            for v in desc.items {
                if let Some(static_samplers) = v.static_samplers {
                    for sampler in static_samplers.iter().copied() {
                        let sampler = Sampler::get_owned(sampler);
                        samplers.push(sampler);
                    }
                }
            }

            let out = DescriptorSetLayout {
                _device: self.this.upgrade().unwrap(),
                _samplers: samplers,
                id: self.object_counter.next_set_layout(),
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

            let pool: Box<dyn IDescriptorPool> = Box::new(DescriptorPool {
                _device: self.this.upgrade().unwrap(),
                _layout: layout,
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

            let pool: Box<dyn IDescriptorArena> = Box::new(DescriptorArena {
                _device: self.this.upgrade().unwrap(),
                arena: Blink::new(),
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

            let out = PipelineLayout {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_pipeline_layout(),
            };
            let out = ArcedObject::new_arc_opaque(out);
            unsafe { Ok(PipelineLayoutHandle::new(out)) }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        let length = desc.size as usize;

        let mut options = MTLResourceOptions::HazardTrackingModeTracked;
        match desc.cpu_access {
            CpuAccessMode::None => options |= MTLResourceOptions::StorageModePrivate,
            CpuAccessMode::Read => options |= MTLResourceOptions::StorageModeShared,
            CpuAccessMode::Write => {
                options |= MTLResourceOptions::StorageModeShared
                    | MTLResourceOptions::CPUCacheModeWriteCombined
            }
        }

        let buffer = match self.device.newBufferWithLength_options(length, options) {
            Some(v) => v,
            None => return Err(BufferCreateError::Platform),
        };

        if let Some(name) = desc.name
            && self.context.debug
        {
            let mtl_name = NSString::from_str(name);
            buffer.setLabel(Some(&mtl_name));
        }

        let out = Buffer {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_buffer(),
            desc: OwnedBufferDesc::new(desc.clone()),
            objects: BufferObjects { buffer },
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        let mtl_desc = unsafe { MTLTextureDescriptor::new() };
        let texture = match self.device.newTextureWithDescriptor(&mtl_desc) {
            Some(v) => v,
            None => return Err(TextureCreateError::Platform),
        };

        let out = Texture {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_texture(),
            views: Default::default(),
            objects: TextureObjects { texture },
            rtvs: Default::default(),
            dsvs: Default::default(),
            desc: OwnedTextureDesc::new(desc.clone()),
        };
        let out = ArcedObject::new_arc_opaque(out);
        unsafe { Ok(TextureHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        let mtl_desc = MTLSamplerDescriptor::new();

        mtl_desc.setMinFilter(conv::sampler_filter_to_mtl(desc.min_filter));
        mtl_desc.setMagFilter(conv::sampler_filter_to_mtl(desc.mag_filter));
        mtl_desc.setMipFilter(conv::sampler_mip_filter_to_mtl(desc.mip_filter));

        mtl_desc.setRAddressMode(conv::address_mode_to_mtl(desc.address_mode_u));
        mtl_desc.setSAddressMode(conv::address_mode_to_mtl(desc.address_mode_v));
        mtl_desc.setTAddressMode(conv::address_mode_to_mtl(desc.address_mode_w));

        mtl_desc.setLodMinClamp(desc.min_lod);
        mtl_desc.setLodMaxClamp(desc.max_lod);

        if desc.enable_anisotropy {
            mtl_desc.setMaxAnisotropy(desc.max_anisotropy as usize);
        }

        if let Some(op) = desc.compare_op {
            mtl_desc.setCompareFunction(conv::compare_op_to_mtl(op));
        }

        mtl_desc.setBorderColor(conv::border_color_to_mtl(desc.border_color));
        // mtl_desc.setSupportArgumentBuffers(true);

        if let Some(name) = desc.name
            && self.context.debug
        {
            let mtl_name = NSString::from_str(name);
            mtl_desc.setLabel(Some(&mtl_name));
        }

        let sampler = match self.device.newSamplerStateWithDescriptor(&mtl_desc) {
            Some(v) => v,
            None => {
                log::error!("Failed to construct 'MTLSamplerState'.");
                panic!("Failed to construct 'MTLSamplerState'.");
            }
        };

        let out = Sampler {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_sampler(),
            desc: OwnedSamplerDesc::new(desc.clone()),
            objects: SamplerObjects { sampler },
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
        let queue = match self.get_queue_internal(desc.queue_type) {
            Some(v) => v,
            None => return Err(CommandListCreateError::NoSuchQueue(desc.queue_type)),
        };

        let list = match queue.objects.queue.commandBuffer() {
            Some(v) => v,
            None => return Err(CommandListCreateError::Platform),
        };

        if let Some(name) = desc.name {
            let mtl_name = NSString::from_str(name);
            list.setLabel(Some(&mtl_name));
        }

        let out: Box<dyn ICommandList> = Box::new(CommandList {
            _device: self.this.upgrade().unwrap(),
            list_type: desc.queue_type,
            state: ListState::Empty,
            objects: CommandListObjects { list },
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
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let event = match self.device.newSharedEvent() {
            Some(v) => v,
            None => return Err(FenceCreateError::Platform),
        };

        unsafe {
            if signalled {
                event.setSignaledValue(1);
            }
        }

        let fence = Fence {
            _device: self.this.upgrade().unwrap(),
            objects: FenceObjects { event },
            value: AtomicU64::new(2),
        };
        let fence = ArcedObject::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let semaphore = Semaphore {
            _device: self.this.upgrade().unwrap(),
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
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        let fence = Fence::get(fence);
        unsafe {
            // We use MTLSharedEvent::wait with a zero timer to poll the event status. This should
            // return true if the signalled value is equal to the expected value or higher.
            let value = fence.get_wait_value();
            fence
                .objects
                .event
                .waitUntilSignaledValue_timeoutMS(value, 0)
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, fences: &[&FenceHandle]) {
        todo!()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Metal
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
        Buffer::get(buffer).map_buffer()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        Buffer::get(buffer).unmap_buffer()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).flush_buffer_range(offset, len)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64) {
        Buffer::get(buffer).invalidate_buffer_range(offset, len)
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
    pub fn get_queue_internal(&self, queue_type: QueueType) -> Option<&Queue> {
        match queue_type {
            QueueType::General => self.general_queue.as_deref(),
            QueueType::Compute => self.compute_queue.as_deref(),
            QueueType::Transfer => self.transfer_queue.as_deref(),
        }
    }
}

thread_local! {
    pub static DEVICE_BUMP: BlinkCell = BlinkCell::new();
}

pub struct FreeCommandList {
    pub list_type: QueueType,
}

impl FreeCommandList {
    pub unsafe fn collect(&self, device: &Device) {}
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
