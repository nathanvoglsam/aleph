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
use std::cell::Cell;
use std::mem::{ManuallyDrop, size_of, transmute_copy};
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicU64;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_object_system::Object;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BlinkCell;
use aleph_rhi_impl_utils::object_counter::ObjectCounter;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use aleph_rhi_impl_utils::owned_desc::{
    OwnedBufferDesc, OwnedParameterBlockDesc, OwnedSamplerDesc, OwnedTextureDesc,
};
use aleph_rhi_impl_utils::parameter_block_layout_visitor::ParameterBlockLayoutVisitor;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use allocator_api2::alloc::Allocator;
use allocator_api2::vec::Vec as BVec;
use blink_alloc::{Blink, BlinkAlloc};
use crossbeam::queue::ArrayQueue;
use parking_lot::Mutex;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::System::Threading::*;
use windows::core::{BOOL, PCSTR};
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::adapter::Adapter;
use crate::binding_signature::{BindingSignature, CompiledBindingSignature};
use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::descriptor_arena::{DescriptorArenaHeap, DescriptorArenaLinear};
use crate::descriptor_pool::DescriptorPool;
use crate::fence::Fence;
use crate::internal::conv::{
    blend_factor_to_dx12, blend_op_to_dx12, compare_op_to_dx12, cull_mode_to_dx12,
    front_face_order_to_dx12, polygon_mode_to_dx12, primitive_topology_to_dx12, queue_type_to_dx12,
    stencil_op_to_dx12, texture_create_clear_value_to_dx12, texture_create_desc_to_dx12,
    texture_format_to_dxgi,
};
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::descriptor_heaps::DescriptorHeaps;
use crate::internal::graphics_pipeline_state_stream::{
    GraphicsPipelineStateStream, GraphicsPipelineStateStreamBuilder,
};
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::parameter_block_pool::ParameterBlockPool;
use crate::internal::register_message_callback::device_unregister_message_callback;
use crate::internal::root_signature_blob::RootSignatureBlob;
use crate::internal::set_name::set_name;
use crate::internal::{handle_wait_result, unwrap};
use crate::parameter_block_layout::{CompiledParameterBlockLayout, ParameterBlockLayout};
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::texture::{ImageViewObject, Texture};

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) device: ID3D12Device10,
    pub(crate) allocator: d3d12ma::Allocator,
    pub(crate) debug_message_cookie: Option<u32>,
    pub(crate) descriptor_heaps: DescriptorHeaps,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
    pub(crate) command_list_pool: CommandListPool,
    pub(crate) object_counter: ObjectCounter,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

declare_interfaces!(Device, [IDevice]);

impl IGetPlatformInterface for Device {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe { try_clone_value_into_slot::<ID3D12Device10>(&self.device, out, target) }
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
        if let Some(queue) = &self.general_queue {
            queue.wait_idle();
        }
        if let Some(queue) = &self.compute_queue {
            queue.wait_idle();
        }
        if let Some(queue) = &self.transfer_queue {
            queue.wait_idle();
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_parameter_block_layout(
        &self,
        desc: &ParameterBlockDesc,
    ) -> Result<AnyArc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError> {
        let compiled = CompiledParameterBlockLayout::new(desc);

        let layout = AnyArc::new_cyclic(move |v| ParameterBlockLayout {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_parameter_block_layout(),
            desc: OwnedParameterBlockDesc::new(desc),
            compiled,
        });

        Ok(AnyArc::map::<dyn IParameterBlockLayout, _>(layout, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_binding_signature(
        &self,
        desc: &BindingSignatureDesc,
    ) -> Result<AnyArc<dyn IBindingSignature>, BindingSignatureCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            let mut parameter_block_layouts =
                Vec::with_capacity(desc.parameter_block_layouts.len());
            for layout in desc.parameter_block_layouts {
                let layout = unwrap::parameter_block_layout_d(layout);
                parameter_block_layouts.push(layout.this.upgrade().unwrap());
            }

            let compiled = CompiledBindingSignature::new(&parameter_block_layouts, desc)?;

            let root_signature = unsafe {
                let desc = BindingSignature::translate_root_signature_desc(
                    &parameter_block_layouts,
                    &compiled,
                    bump.allocator(),
                );
                let blob = RootSignatureBlob::new(&desc)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
                self.device
                    .CreateRootSignature::<ID3D12RootSignature>(0, &blob)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            if let Some(name) = desc.name {
                set_name(&root_signature, name).unwrap();
            }

            let signature = AnyArc::new_cyclic(move |v| BindingSignature {
                this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_binding_signature(),
                _parameter_block_layouts: parameter_block_layouts,
                root_signature,
                compiled,
            });

            Ok(AnyArc::map::<dyn IBindingSignature, _>(signature, |v| v))
        })
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

            // Unwrap the binding signature trait object into the concrete implementation
            let binding_signature = unwrap::binding_signature(desc.binding_signature);

            let builder = GraphicsPipelineStateStreamBuilder::new();

            // Add all shaders in the list to their corresponding slot
            let builder = Self::translate_shader_stage_list(desc.shader_stages, builder)?;

            let builder = builder.root_signature(binding_signature.root_signature.clone());

            let (input_binding_strides, input_layout) =
                Self::translate_vertex_input_state_desc(bump.allocator(), desc.vertex_layout);
            let builder = builder.input_layout(&input_layout);

            let (builder, primitive_topology) =
                Self::translate_input_assembly_state_desc(desc, builder);

            let rasterizer_state = Self::translate_rasterizer_state_desc(desc.rasterizer_state);
            let builder = builder.rasterizer_state(rasterizer_state);

            let (depth_bounds, depth_stencil_state) =
                Self::translate_depth_stencil_desc(desc.depth_stencil_state);
            let builder = builder.depth_stencil_state(depth_stencil_state);

            let blend_state = Self::translate_blend_state_desc(desc.blend_state);
            let builder = builder.blend_state(blend_state);

            // TODO: we should be able to expose this in the API
            let builder = builder.sample_mask(u32::MAX);

            // Render target format translation is straight forward, just convert the formats and add
            let mut rtv_formats =
                BVec::with_capacity_in(desc.render_target_formats.len(), bump.allocator());
            for v in desc.render_target_formats.iter().copied() {
                rtv_formats.push(texture_format_to_dxgi(v))
            }
            let builder = builder.rtv_formats(&rtv_formats);
            let builder =
                if let Some(dsv_format) = desc.depth_stencil_format.map(texture_format_to_dxgi) {
                    builder.dsv_format(dsv_format)
                } else {
                    builder
                };

            // Construct the D3D12 pipeline object
            let state_stream = builder.build();
            let state_stream_ref = D3D12_PIPELINE_STATE_STREAM_DESC {
                SizeInBytes: std::mem::size_of_val(&state_stream),
                pPipelineStateSubobjectStream: &state_stream as *const GraphicsPipelineStateStream
                    as *mut _,
            };
            let pipeline = unsafe {
                self.device
                    .CreatePipelineState(&state_stream_ref)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            if let Some(name) = desc.name {
                set_name(&pipeline, name).unwrap();
            }

            let out = GraphicsPipeline {
                _device: self.this.upgrade().unwrap(),
                id: self.object_counter.next_graphics_pipeline(),
                pipeline,
                binding_signature: binding_signature.this.upgrade().unwrap(),
                primitive_topology,
                input_binding_strides,
                depth_bounds,
            };
            let out = Object::new_arc_opaque(out);
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
        // Unwrap the binding signature trait object into the concrete implementation
        let binding_signature = unwrap::binding_signature(desc.binding_signature);

        let shader = desc.shader_module.get_dxil();

        let pipeline_desc = D3D12_COMPUTE_PIPELINE_STATE_DESC {
            pRootSignature: unsafe { transmute_copy(&binding_signature.root_signature) },
            CS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: shader.as_ptr() as *const _,
                BytecodeLength: shader.len(),
            },
            NodeMask: 0,
            CachedPSO: D3D12_CACHED_PIPELINE_STATE {
                pCachedBlob: std::ptr::null(),
                CachedBlobSizeInBytes: 0,
            },
            Flags: D3D12_PIPELINE_STATE_FLAGS::default(),
        };

        let pipeline = unsafe {
            self.device
                .CreateComputePipelineState(&pipeline_desc)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        if let Some(name) = desc.name {
            set_name(&pipeline, name).unwrap();
        }

        let out = ComputePipeline {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_compute_pipeline(),
            pipeline,
            binding_signature: binding_signature.this.upgrade().unwrap(),
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

        let num_resources = layout.compiled.resources.num_resources();
        let num_samplers = layout.compiled.samplers.num_samplers();

        let resource_arena = DescriptorChunk::new(
            self.descriptor_heaps.gpu_view_heap(),
            desc.num_blocks * num_resources,
        )?;

        let samplers_size = num_samplers as usize * size_of::<Option<GPUDescriptorHandle>>();
        let array_pool_size = samplers_size;

        let pool = Box::new(DescriptorPool {
            _device: self.this.upgrade().unwrap(),
            _layout: layout.this.upgrade().unwrap(),
            resource_arena,
            set_pool: ParameterBlockPool::new(desc.num_blocks),
            set_array_pool: BlinkAlloc::with_chunk_size(array_pool_size),
            descriptor_bump_index: 0,
        });

        Ok(pool)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError> {
        match desc.arena_type {
            DescriptorArenaType::Linear => {
                let resource_arena = DescriptorChunk::new(
                    self.descriptor_heaps.gpu_view_heap(),
                    desc.num_blocks * 16,
                )?
                .unwrap();

                let set_size = DescriptorArenaLinear::descriptor_set_allocation_layout(1)
                    .unwrap()
                    .size();
                let set_pool_capacity = set_size * desc.num_blocks as usize;

                let pool = Box::new(DescriptorArenaLinear {
                    _device: self.this.upgrade().unwrap(),
                    resource_arena,
                    set_pool: BlinkAlloc::with_chunk_size(set_pool_capacity),
                    descriptor_bump_index: Cell::new(0),
                    num_blocks: Cell::new(0),
                    set_capacity: desc.num_blocks,
                });

                Ok(pool)
            }
            DescriptorArenaType::Heap => {
                let resource_block = DescriptorChunk::new(
                    self.descriptor_heaps.gpu_view_heap(),
                    desc.num_blocks * 16,
                )?
                .unwrap();

                let resource_pool =
                    OffsetAllocator::new(resource_block.num_descriptors, desc.num_blocks * 2);
                let resource_pool = Box::new(resource_pool);

                let pool = Box::new(DescriptorArenaHeap {
                    _device: self.this.upgrade().unwrap(),
                    resource_block,
                    resource_pool: Cell::new(Some(resource_pool)),
                    set_pool: ParameterBlockPool::new(desc.num_blocks),
                    live_handles: Cell::new(Vec::with_capacity(128)),
                });

                Ok(pool)
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError> {
        let mut resource_desc = D3D12_RESOURCE_DESC1 {
            // Fields that will be the same regardless of the requested buffer desc
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: 0,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: Default::default(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            SamplerFeedbackMipRegion: Default::default(),
            Flags: Default::default(),
        };

        resource_desc.Width = desc.size;

        if desc.usage.contains(ResourceUsageFlags::UNORDERED_ACCESS) {
            resource_desc.Flags |= D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS;
        }

        let heap_type = match desc.cpu_access {
            CpuAccessMode::None => D3D12_HEAP_TYPE_DEFAULT,
            CpuAccessMode::Read => D3D12_HEAP_TYPE_READBACK,
            CpuAccessMode::Write => D3D12_HEAP_TYPE_UPLOAD,
        };

        let allocation_desc = d3d12ma::ALLOCATION_DESC {
            Flags: d3d12ma::ALLOCATION_FLAGS::empty(),
            HeapType: heap_type,
            ExtraHeapFlags: Default::default(),
            Pool: std::ptr::null_mut(),
            pPrivateData: std::ptr::null_mut(),
        };
        let (allocation, resource) = unsafe {
            self.allocator
                .CreateResource3::<ID3D12Resource>(
                    &allocation_desc,
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT_UNDEFINED,
                    None,
                    &[],
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };
        let base_address =
            unsafe { GPUDescriptorHandle::try_from(resource.GetGPUVirtualAddress()).unwrap() };

        if let Some(name) = desc.name {
            set_name(&resource, name).unwrap();
        }

        let out = Buffer {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_buffer(),
            allocation: ManuallyDrop::new(allocation),
            resource: ManuallyDrop::new(resource),
            base_address,
            map_state: Mutex::new(Default::default()),
            desc: OwnedBufferDesc::new(desc.clone()),
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(BufferHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError> {
        let alloc_desc = d3d12ma::ALLOCATION_DESC {
            Flags: d3d12ma::ALLOCATION_FLAGS::empty(),
            HeapType: D3D12_HEAP_TYPE_DEFAULT,
            ExtraHeapFlags: Default::default(),
            Pool: std::ptr::null_mut(),
            pPrivateData: std::ptr::null_mut(),
        };
        let resource_desc = texture_create_desc_to_dx12(desc)?;
        let optimized_clear_value = texture_create_clear_value_to_dx12(desc, resource_desc.Format)?;

        let (allocation, resource) = unsafe {
            self.allocator
                .CreateResource3::<ID3D12Resource>(
                    &alloc_desc,
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT_UNDEFINED,
                    optimized_clear_value.as_ref(),
                    &[],
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        if let Some(name) = desc.name {
            set_name(&resource, name).unwrap();
        }

        let out = Texture {
            device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_texture(),
            allocation: Some(ManuallyDrop::new(allocation)),
            resource: ManuallyDrop::new(resource),
            desc: OwnedTextureDesc::new(desc.clone()),
            dxgi_format: resource_desc.Format,
            views: Default::default(),
            rtvs: Default::default(),
            dsvs: Default::default(),
            image_views: Mutex::new(Blink::with_chunk_size(size_of::<ImageViewObject>() * 8)),
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(TextureHandle::new(out)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError> {
        let gpu_handle = self
            .descriptor_heaps
            .gpu_sampler_cache()
            .get(desc)
            .ok_or(SamplerCreateError::OutOfSamplers)?;

        // TODO: we probably need to validate the sampler description to keep this API safe.

        let out = Sampler {
            _device: self.this.upgrade().unwrap(),
            id: self.object_counter.next_sampler(),
            desc: OwnedSamplerDesc::new(desc.clone()),
            gpu_handle,
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
        // First we try and grab a command list from the free list. This way we reuse an old
        // list before we try and make a new one. This can save a lot of performance even if the
        // free list is a bit slow.
        //
        // Some drivers will lazily allocate pages for the command list on first use. If we're
        // only using fresh allocators then we hit that (very) slow path every time. To avoid
        // this we front creating new command pools with a free list so we recycle old ones
        // first.
        if let Some(list) = self.command_list_pool.get_for_queue_type(desc.queue_type) {
            if let Some(name) = desc.name {
                set_name(&list.allocator, name)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
                set_name(&list.list, name).map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            }

            let FreeCommandList {
                allocator,
                list,
                descriptor_heaps,
                list_type,
            } = list;

            // It is assumed that only command lists that are safe to reuse are placed into the
            // free list.
            //
            // Typically, this will be done in 'garbage_collect'.
            let out: Box<dyn ICommandList> = Box::new(CommandList {
                _device: self.this.upgrade().unwrap(),
                allocator,
                list,
                descriptor_heaps,
                list_type,
                state: ListState::Empty,
            });
            return Ok(out);
        }

        log::warn!(
            "CommandList free-object-pool empty. Taking slow-path for creating a new object!"
        );

        let platform_list_type = queue_type_to_dx12(desc.queue_type);

        let allocator = unsafe {
            self.device
                .CreateCommandAllocator(platform_list_type)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        let list = unsafe {
            self.device
                .CreateCommandList1(0, platform_list_type, Default::default())
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        if let Some(name) = desc.name {
            set_name(&allocator, name).map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            set_name(&list, name).map_err(|v| log::error!("Platform Error: {:#?}", v))?;
        }

        let descriptor_heaps = [
            Some(self.descriptor_heaps.gpu_view_heap().heap().clone()),
            Some(self.descriptor_heaps.gpu_sampler_cache().heap().clone()),
        ];

        let command_list = CommandList {
            _device: self.this.upgrade().unwrap(),
            list_type: desc.queue_type,
            descriptor_heaps,
            allocator,
            list,
            state: ListState::Empty,
        };
        Ok(Box::new(command_list))
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
        layout: &dyn IParameterBlockLayout,
        block: ParameterBlockHandle,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        let layout = unwrap::parameter_block_layout(layout);
        let block = unsafe { ParameterBlock::ptr_from_handle(block).as_mut() };

        let visitor =
            ParameterBlockLayoutVisitor::new(layout.desc.get(), base as u64, writes).unwrap();
        for v in visitor {
            let param = &layout.compiled.mapping.params[v.binding as usize];

            if v.ty.is_sampler() {
                let base_offset = param.register_offset as usize;
                let base_offset = base_offset + v.element as usize;

                for (i, write) in v.writes.iter().enumerate() {
                    let final_offset = base_offset + i;

                    match write {
                        ParameterWrite::Sampler(write) => unsafe {
                            let src = Sampler::get(write.sampler);
                            let dst = block.samplers.as_mut();
                            dst[final_offset] = Some(src.gpu_handle);
                        },
                        _ => unreachable!(),
                    }
                }
            } else {
                let base_offset = param.storage_offset as usize;
                let base_offset = base_offset + v.element as usize;

                for (i, write) in v.writes.iter().enumerate() {
                    let final_offset = base_offset + i;
                    let (dst, _) = unsafe { block.assume_r_handle() };
                    let dst = dst.add_increments(
                        final_offset,
                        self.descriptor_heaps.gpu_view_heap().descriptor_increment() as usize,
                    );

                    match write {
                        ParameterWrite::Sampler(_) => unreachable!(),
                        ParameterWrite::Texture(write) => unsafe {
                            // SAFETY: It is the caller's responsibility to ensure that the view
                            //         points to a live and valid ImageViewObject. The objects are
                            //         immutable so parallel access is safe implicitly.
                            let src = ImageViewObject::handle_to_ref(&write.image_view);
                            let src = src.handle;

                            self.device.CopyDescriptorsSimple(
                                1,
                                dst.into(),
                                src.into(),
                                D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                            );
                        },
                        ParameterWrite::Buffer(write) => unsafe {
                            let buffer = Buffer::get(write.buffer);
                            match v.ty {
                                ParameterType::ConstantBuffer => {
                                    self.update_uniform_buffer_descriptor(buffer, write, dst);
                                }
                                ParameterType::StructuredBuffer => {
                                    self.update_structured_buffer_descriptor_srv(
                                        buffer, write, dst,
                                    );
                                }
                                ParameterType::RWStructuredBuffer => {
                                    self.update_structured_buffer_descriptor_uav(
                                        buffer, write, dst,
                                    );
                                }
                                ParameterType::ByteAddressBuffer => {
                                    self.update_byte_address_buffer_descriptor_srv(
                                        buffer, write, dst,
                                    );
                                }
                                ParameterType::RWByteAddressBuffer => {
                                    self.update_byte_address_buffer_descriptor_uav(
                                        buffer, write, dst,
                                    );
                                }
                                ParameterType::AccelerationStructure => unimplemented!(),
                                _ => unreachable!(),
                            }
                        },
                        ParameterWrite::TextureBuffer(write) => unsafe {
                            let buffer = Buffer::get(write.buffer);
                            match v.ty {
                                ParameterType::Buffer => {
                                    self.update_texel_buffer_descriptor_srv(buffer, write, dst)
                                }
                                ParameterType::RWBuffer => {
                                    self.update_texel_buffer_descriptor_uav(buffer, write, dst)
                                }
                                _ => unreachable!(),
                            }
                        },
                    }
                }
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError> {
        let initial_value = if signalled { 1 } else { 0 };
        let fence: ID3D12Fence = unsafe {
            self.device
                .CreateFence(initial_value, D3D12_FENCE_FLAG_NONE)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        let fence = Fence {
            _device: self.this.upgrade().unwrap(),
            fence,
            value: AtomicU64::new(2),
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError> {
        let fence: ID3D12Fence = unsafe {
            self.device
                .CreateFence(0, D3D12_FENCE_FLAG_NONE)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };
        let semaphore = Semaphore {
            _device: self.this.upgrade().unwrap(),
            fence,
            value: AtomicU64::new(0),
        };
        let semaphore = Object::new_arc_opaque(semaphore);
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

            match fences.len() {
                0 => {
                    // Do nothing on empty list,
                    FenceWaitResult::Complete
                }
                1 => {
                    // Special case a single fence with 'SetEventOnCompletion'
                    thread_local! {
                        pub static WAIT_HANDLE: HANDLE = unsafe {
                            CreateEventW(None, false, false, None).unwrap()
                        };
                    }
                    let fence = Fence::get(fences[0]);
                    let wait_value = fence.get_wait_value();

                    WAIT_HANDLE.with(|handle| unsafe {
                        fence
                            .fence
                            .SetEventOnCompletion(wait_value, *handle)
                            .unwrap();
                        if handle_wait_result(WaitForSingleObject(*handle, timeout)) {
                            FenceWaitResult::Complete
                        } else {
                            FenceWaitResult::Timeout
                        }
                    })
                }
                _ => {
                    // Handle the 'n' case with 'SetEventOnMultipleFenceCompletion'
                    thread_local! {

                        pub static MULTIPLE_WAIT_HANDLE: HANDLE = unsafe {
                            CreateEventW(None, false, false, None).unwrap()
                        };
                    }

                    // Unwrap the fences into the form accepted by D3D12, and produce a matching array
                    // of values filled with the expected value for a signalled fence.
                    let mut inner_fences: BVec<Option<ID3D12Fence>, _> =
                        BVec::with_capacity_in(fences.len(), bump.allocator());
                    let mut wait_values: BVec<u64, _> =
                        BVec::with_capacity_in(fences.len(), bump.allocator());
                    for fence in fences.iter().copied().map(Fence::get) {
                        inner_fences.push(Some(fence.fence.clone()));
                        wait_values.push(fence.get_wait_value());
                    }

                    MULTIPLE_WAIT_HANDLE.with(|handle| unsafe {
                        let flags = if wait_all {
                            D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL
                        } else {
                            D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY
                        };

                        self.device
                            .SetEventOnMultipleFenceCompletion(
                                inner_fences.as_ptr(),
                                wait_values.as_ptr(),
                                fences.len() as u32,
                                flags,
                                *handle,
                            )
                            .unwrap();

                        if handle_wait_result(WaitForSingleObject(*handle, timeout)) {
                            FenceWaitResult::Complete
                        } else {
                            FenceWaitResult::Timeout
                        }
                    })
                }
            }
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn poll_fence(&self, fence: &FenceHandle) -> bool {
        let fence = Fence::get(fence);
        unsafe {
            let v = fence.fence.GetCompletedValue();
            v < fence.get_wait_value()
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, _fences: &[&FenceHandle]) {
        // Fence reset is a no-op on dx12 as a fence is always ready to use. It uses a monotonic
        // counter to keep the signals and waits correct.
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_id(&self, buffer: &BufferHandle) -> std::num::NonZeroU64 {
        Buffer::get(buffer).get_id()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b> {
        Buffer::get(buffer).desc()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn map_buffer(&self, buffer: &BufferHandle) -> Result<NonNull<u8>, ResourceMapError> {
        Buffer::get(buffer).map()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError> {
        Buffer::get(buffer).unmap()
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn flush_buffer_range(&self, buffer: &BufferHandle, _offset: u64, _len: u64) {
        let _ = Buffer::get(buffer);
        // intentional no-op
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn invalidate_buffer_range(&self, buffer: &BufferHandle, _offset: u64, _len: u64) {
        let _ = Buffer::get(buffer);
        // intentional no-op
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
        Texture::get(texture).get_view(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_rtv(desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        Texture::get(texture).get_dsv(desc)
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
    /// Internal function for translating the list of [`IShaderCodeSource`] stages into the pipeline
    /// description
    fn translate_shader_stage_list<'a>(
        shader_stages: &'a [&'a dyn IShaderCodeSource],
        mut builder: GraphicsPipelineStateStreamBuilder<'a>,
    ) -> Result<GraphicsPipelineStateStreamBuilder<'a>, PipelineCreateError> {
        for shader_stage in shader_stages.iter() {
            let code = shader_stage.get_dxil();
            builder = match shader_stage.shader_type() {
                ShaderType::Vertex => builder.vertex_shader(code),
                ShaderType::Hull => builder.hull_shader(code),
                ShaderType::Domain => builder.domain_shader(code),
                ShaderType::Geometry => builder.geometry_shader(code),
                ShaderType::Fragment => builder.pixel_shader(code),
                ShaderType::Compute => {
                    panic!("Can't bind a compute shader to a graphics pipeline")
                }
                ShaderType::Amplification | ShaderType::Mesh => {
                    todo!("Missing implementation for amplification and mesh shaders")
                }
            }
        }
        Ok(builder)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Internal function for translating the [VertexInputStateDesc] field of a pipeline
    /// description
    fn translate_vertex_input_state_desc<'a, A: Allocator + 'a>(
        allocator: A,
        desc: &VertexInputStateDesc,
    ) -> ([u32; 16], BVec<D3D12_INPUT_ELEMENT_DESC, A>) {
        // Copy the input binding strides into a buffer the pipeline will hold on to so it can be
        // used in the command encoders. Vulkan bakes these in the pipeline, d3d12 gets the values
        // when the input bindings are bound
        let mut input_binding_strides = [0u32; 16];
        for (binding, stride) in desc.input_bindings.iter().zip(&mut input_binding_strides) {
            *stride = binding.stride;
        }

        // Translate the vertex input description
        let mut input_layout = BVec::with_capacity_in(desc.input_attributes.len(), allocator);
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
            let semantic_name = c"A";
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
                VertexInputRate::PerVertex => (D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0),
                VertexInputRate::PerInstance => (D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA, 1),
            };

            input_layout.push(D3D12_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR(semantic_name.as_ptr() as *const _),
                SemanticIndex: semantic_index,
                Format: texture_format_to_dxgi(attribute.format),
                InputSlot: input_slot,
                AlignedByteOffset: aligned_byte_offset,
                InputSlotClass: input_slot_class,
                InstanceDataStepRate: instance_data_step_rate,
            });
        }

        (input_binding_strides, input_layout)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Internal function for translating the [InputAssemblyStateDesc] field of a pipeline
    /// description
    fn translate_input_assembly_state_desc<'b>(
        desc: &GraphicsPipelineDesc,
        mut builder: GraphicsPipelineStateStreamBuilder<'b>,
    ) -> (
        GraphicsPipelineStateStreamBuilder<'b>,
        D3D_PRIMITIVE_TOPOLOGY,
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
        let (r#type, topo) =
            primitive_topology_to_dx12(desc.input_assembly_state.primitive_topology);
        builder = builder.primitive_topology_type(r#type);
        (builder, topo)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Internal function for translating the [RasterizerStateDesc] field of a pipeline
    /// description
    fn translate_rasterizer_state_desc(desc: &RasterizerStateDesc) -> D3D12_RASTERIZER_DESC {
        let fill_mode = polygon_mode_to_dx12(desc.polygon_mode);
        let cull_mode = cull_mode_to_dx12(desc.cull_mode);
        let front_counter_clockwise = front_face_order_to_dx12(desc.front_face);
        D3D12_RASTERIZER_DESC {
            FillMode: fill_mode,
            CullMode: cull_mode,
            FrontCounterClockwise: front_counter_clockwise,
            DepthBias: desc.depth_bias,
            DepthBiasClamp: desc.depth_bias_clamp,
            SlopeScaledDepthBias: desc.depth_bias_slope_factor,
            DepthClipEnable: BOOL::from(true),    // TODO: translate
            MultisampleEnable: BOOL::from(false), // TODO: translate
            AntialiasedLineEnable: BOOL::from(false),
            ForcedSampleCount: 0,
            ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Internal function for translating the [DepthStencilStateDesc] field of a pipeline
    /// description
    fn translate_depth_stencil_desc(
        desc: &DepthStencilStateDesc,
    ) -> (Option<(f32, f32)>, D3D12_DEPTH_STENCIL_DESC1) {
        /// Internal function for translating our [StencilOpState] into the D3D12 equivalent
        fn translate_depth_stencil_op_desc(desc: &StencilOpState) -> D3D12_DEPTH_STENCILOP_DESC {
            let stencil_fail_op = stencil_op_to_dx12(desc.fail_op);
            let stencil_depth_fail_op = stencil_op_to_dx12(desc.depth_fail_op);
            let stencil_pass_op = stencil_op_to_dx12(desc.pass_op);
            let stencil_func = compare_op_to_dx12(desc.compare_op);
            D3D12_DEPTH_STENCILOP_DESC {
                StencilFailOp: stencil_fail_op,
                StencilDepthFailOp: stencil_depth_fail_op,
                StencilPassOp: stencil_pass_op,
                StencilFunc: stencil_func,
            }
        }

        let depth_enable = BOOL::from(desc.depth_test);
        let depth_write_mask = if desc.depth_write {
            D3D12_DEPTH_WRITE_MASK_ALL
        } else {
            D3D12_DEPTH_WRITE_MASK_ZERO
        };
        let depth_func = compare_op_to_dx12(desc.depth_compare_op);
        let stencil_enable = BOOL::from(desc.stencil_test);
        let stencil_read_mask = desc.stencil_read_mask;
        let stencil_write_mask = desc.stencil_write_mask;

        let front_face = translate_depth_stencil_op_desc(&desc.stencil_front);
        let back_face = translate_depth_stencil_op_desc(&desc.stencil_back);

        let depth_bounds_test_enable = BOOL::from(desc.depth_bounds_enable);
        let bounds = if desc.depth_bounds_enable {
            Some((desc.min_depth_bounds, desc.max_depth_bounds))
        } else {
            None
        };

        let desc = D3D12_DEPTH_STENCIL_DESC1 {
            DepthEnable: depth_enable,
            DepthWriteMask: depth_write_mask,
            DepthFunc: depth_func,
            StencilEnable: stencil_enable,
            StencilReadMask: stencil_read_mask,
            StencilWriteMask: stencil_write_mask,
            FrontFace: front_face,
            BackFace: back_face,
            DepthBoundsTestEnable: depth_bounds_test_enable,
        };

        (bounds, desc)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn translate_blend_state_desc(desc: &BlendStateDesc) -> D3D12_BLEND_DESC {
        // TODO: Figure out if alpha to coverage is possible to expose
        let alpha_to_coverage_enable = BOOL::from(false);
        let independent_blend_enable = BOOL::from(true);

        // Use our default attachment to initialize the array dx12 needs. Only the first 'n' values
        // will be read, where 'n' is the number of render targets in the pipeline desc, all other
        // items in the array will be ignored so they don't need to be in a well defined state.
        //
        // Safety: Using mem::zeroed is safe here as all zeroes is a valid bit pattern for the C
        // structs. D3D12_RENDER_TARGET_BLEND_DESC::default() is implemented as mem::zeroed, but
        // isn't tagged with #[inline] so I suspect won't be inlined across the crate bounds.
        let mut render_targets = unsafe {
            [
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
                core::mem::zeroed(),
            ]
        };

        for (i, attachment) in desc.attachments.iter().enumerate() {
            let blend_enable = BOOL::from(attachment.blend_enabled);

            let logic_op_enable = BOOL::from(false);
            let logic_op = D3D12_LOGIC_OP::default();

            let src_blend = blend_factor_to_dx12(attachment.src_factor);
            let dest_blend = blend_factor_to_dx12(attachment.dst_factor);
            let blend_op = blend_op_to_dx12(attachment.blend_op);

            let src_blend_alpha = blend_factor_to_dx12(attachment.alpha_src_factor);
            let dest_blend_alpha = blend_factor_to_dx12(attachment.alpha_dst_factor);
            let blend_op_alpha = blend_op_to_dx12(attachment.alpha_blend_op);

            let render_target_write_mask = attachment.color_write_mask.bits();

            render_targets[i] = D3D12_RENDER_TARGET_BLEND_DESC {
                BlendEnable: blend_enable,
                LogicOpEnable: logic_op_enable,
                LogicOp: logic_op,
                SrcBlend: src_blend,
                DestBlend: dest_blend,
                BlendOp: blend_op,
                SrcBlendAlpha: src_blend_alpha,
                DestBlendAlpha: dest_blend_alpha,
                BlendOpAlpha: blend_op_alpha,
                RenderTargetWriteMask: render_target_write_mask,
            };
        }

        D3D12_BLEND_DESC {
            AlphaToCoverageEnable: alpha_to_coverage_enable,
            IndependentBlendEnable: independent_blend_enable,
            RenderTarget: render_targets,
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_uniform_buffer_descriptor(
        &self,
        buffer: &Buffer,
        write: &BufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            // Calculates the 'BufferLocation' value, as D3D12 takes a raw virtual address to
            // the start of the CBV, rather than a ID3D12Resource + offset.
            let location = buffer.base_address.add(write.offset).get_inner().get();

            let view = D3D12_CONSTANT_BUFFER_VIEW_DESC {
                BufferLocation: location,
                SizeInBytes: write.len,
            };
            self.device
                .CreateConstantBufferView(Some(&view), dst.into());
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_byte_address_buffer_descriptor_srv(
        &self,
        buffer: &Buffer,
        write: &BufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let view = D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R32_TYPELESS,
                ViewDimension: D3D12_SRV_DIMENSION_BUFFER,
                Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_SRV {
                        FirstElement: write.offset / 4,
                        NumElements: len / 4,
                        StructureByteStride: 0,
                        Flags: D3D12_BUFFER_SRV_FLAG_RAW,
                    },
                },
            };
            self.device
                .CreateShaderResourceView(buffer.resource.deref(), Some(&view), dst.into());
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_byte_address_buffer_descriptor_uav(
        &self,
        buffer: &Buffer,
        write: &BufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let view = D3D12_UNORDERED_ACCESS_VIEW_DESC {
                Format: DXGI_FORMAT_R32_TYPELESS,
                ViewDimension: D3D12_UAV_DIMENSION_BUFFER,
                Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_UAV {
                        FirstElement: write.offset / 4,
                        NumElements: len / 4,
                        StructureByteStride: 0,
                        CounterOffsetInBytes: 0,
                        Flags: D3D12_BUFFER_UAV_FLAG_RAW,
                    },
                },
            };
            self.device.CreateUnorderedAccessView(
                buffer.resource.deref(),
                None,
                Some(&view),
                dst.into(),
            );
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_structured_buffer_descriptor_srv(
        &self,
        buffer: &Buffer,
        write: &BufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let first_element = write.offset / write.structure_byte_stride as u64;
            let num_elements = len / write.structure_byte_stride;
            let view = D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_UNKNOWN,
                ViewDimension: D3D12_SRV_DIMENSION_BUFFER,
                Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_SRV {
                        FirstElement: first_element,
                        NumElements: num_elements,
                        StructureByteStride: write.structure_byte_stride,
                        Flags: Default::default(),
                    },
                },
            };
            self.device
                .CreateShaderResourceView(buffer.resource.deref(), Some(&view), dst.into());
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_structured_buffer_descriptor_uav(
        &self,
        buffer: &Buffer,
        write: &BufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let first_element = write.offset / write.structure_byte_stride as u64;
            let num_elements = len / write.structure_byte_stride;
            let view = D3D12_UNORDERED_ACCESS_VIEW_DESC {
                Format: DXGI_FORMAT_UNKNOWN,
                ViewDimension: D3D12_UAV_DIMENSION_BUFFER,
                Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_UAV {
                        FirstElement: first_element,
                        NumElements: num_elements,
                        StructureByteStride: write.structure_byte_stride,
                        CounterOffsetInBytes: 0,
                        Flags: Default::default(),
                    },
                },
            };
            self.device.CreateUnorderedAccessView(
                buffer.resource.deref(),
                None,
                Some(&view),
                dst.into(),
            );
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_texel_buffer_descriptor_srv(
        &self,
        buffer: &Buffer,
        write: &TextureBufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let format = texture_format_to_dxgi(write.format);
            let bytes_per_element = write.format.bytes_per_element();
            let first_element = write.offset / bytes_per_element as u64;
            let num_elements = len / bytes_per_element;
            let view = D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: format,
                ViewDimension: D3D12_SRV_DIMENSION_BUFFER,
                Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_SRV {
                        FirstElement: first_element,
                        NumElements: num_elements,
                        StructureByteStride: 0,
                        Flags: Default::default(),
                    },
                },
            };
            self.device
                .CreateShaderResourceView(buffer.resource.deref(), Some(&view), dst.into());
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_texel_buffer_descriptor_uav(
        &self,
        buffer: &Buffer,
        write: &TextureBufferWrite,
        dst: CPUDescriptorHandle,
    ) {
        unsafe {
            let len = buffer.clamp_max_size_for_view(write.len);

            let format = texture_format_to_dxgi(write.format);
            let bytes_per_element = write.format.bytes_per_element();
            let first_element = write.offset / bytes_per_element as u64;
            let num_elements = len / bytes_per_element;
            let view = D3D12_UNORDERED_ACCESS_VIEW_DESC {
                Format: format,
                ViewDimension: D3D12_UAV_DIMENSION_BUFFER,
                Anonymous: D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Buffer: D3D12_BUFFER_UAV {
                        FirstElement: first_element,
                        NumElements: num_elements,
                        StructureByteStride: 0,
                        CounterOffsetInBytes: 0,
                        Flags: Default::default(),
                    },
                },
            };
            self.device.CreateUnorderedAccessView(
                buffer.resource.deref(),
                None,
                Some(&view),
                dst.into(),
            );
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // SAFETY: This should be safe but I can't prove it
        unsafe {
            if let Some(cookie) = self.debug_message_cookie {
                let _sink = device_unregister_message_callback(&self.device, cookie);
            }
        }
    }
}

thread_local! {
    pub static DEVICE_BUMP: BlinkCell = BlinkCell::new();
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
}

pub struct FreeCommandList {
    pub allocator: ID3D12CommandAllocator,
    pub list: ID3D12GraphicsCommandList7,
    pub descriptor_heaps: [Option<ID3D12DescriptorHeap>; 2],
    pub list_type: QueueType,
}
