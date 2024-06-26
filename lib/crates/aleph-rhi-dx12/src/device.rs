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
use std::collections::HashMap;
use std::mem::{size_of, transmute_copy};
use std::ptr::NonNull;
use std::sync::atomic::AtomicU64;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::bump_cell::BumpCell;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use aleph_rhi_impl_utils::{cstr, try_clone_value_into_slot};
use blink_alloc::BlinkAlloc;
use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use parking_lot::Mutex;
use windows::core::PCSTR;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::System::Threading::*;

use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_list::CommandList;
use crate::context::Context;
use crate::descriptor_arena::{DescriptorArenaHeap, DescriptorArenaLinear};
use crate::descriptor_pool::DescriptorPool;
use crate::descriptor_set_layout::{
    DescriptorBindingInfo, DescriptorBindingLayout, DescriptorSetLayout,
};
use crate::fence::Fence;
use crate::internal::conv::{
    blend_factor_to_dx12, blend_op_to_dx12, border_color_to_dx12_static, compare_op_to_dx12,
    cull_mode_to_dx12, descriptor_type_to_dx12, front_face_order_to_dx12, polygon_mode_to_dx12,
    primitive_topology_to_dx12, queue_type_to_dx12, sampler_address_mode_to_dx12,
    sampler_filters_to_dx12, shader_visibility_to_dx12, stencil_op_to_dx12,
    texture_create_clear_value_to_dx12, texture_create_desc_to_dx12, texture_format_to_dxgi,
};
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::descriptor_heap_info::DescriptorHeapInfo;
use crate::internal::descriptor_heaps::DescriptorHeaps;
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::descriptor_set_pool::DescriptorSetPool;
use crate::internal::graphics_pipeline_state_stream::{
    GraphicsPipelineStateStream, GraphicsPipelineStateStreamBuilder,
};
use crate::internal::register_message_callback::device_unregister_message_callback;
use crate::internal::root_signature_blob::RootSignatureBlob;
use crate::internal::set_name::set_name;
use crate::internal::{handle_wait_result, unwrap};
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::pipeline_layout::{PipelineLayout, PushConstantBlockInfo};
use crate::queue::Queue;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::texture::{ImageViewObject, Texture};

pub struct Device {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _adapter: AnyArc<Adapter>,
    pub(crate) device: ID3D12Device10,
    pub(crate) debug_message_cookie: Option<u32>,
    pub(crate) descriptor_heap_info: DescriptorHeapInfo,
    pub(crate) descriptor_heaps: DescriptorHeaps,
    pub(crate) general_queue: Option<AnyArc<Queue>>,
    pub(crate) compute_queue: Option<AnyArc<Queue>>,
    pub(crate) transfer_queue: Option<AnyArc<Queue>>,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

declare_interfaces!(Device, [IDevice]);

impl IGetPlatformInterface for Device {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<ID3D12Device10>(&self.device, out, target)
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

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, PipelineCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            // Unwrap the pipeline layout trait object into the concrete implementation
            let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout)
                .this
                .upgrade()
                .unwrap();

            let builder = GraphicsPipelineStateStreamBuilder::new();

            // Add all shaders in the list to their corresponding slot
            let builder = Self::translate_shader_stage_list(desc.shader_stages, builder)?;

            let builder = builder.root_signature(pipeline_layout.root_signature.clone());

            let (input_binding_strides, input_layout) =
                Self::translate_vertex_input_state_desc(&bump, desc.vertex_layout);
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
            let mut rtv_formats = BVec::with_capacity_in(desc.render_target_formats.len(), &bump);
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

            let pipeline = AnyArc::new_cyclic(move |v| GraphicsPipeline {
                this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                pipeline,
                pipeline_layout,
                primitive_topology,
                input_binding_strides,
                depth_bounds,
            });
            Ok(AnyArc::map::<dyn IGraphicsPipeline, _>(pipeline, |v| v))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, PipelineCreateError> {
        // Unwrap the pipeline layout trait object into the concrete implementation
        let pipeline_layout = unwrap::pipeline_layout(desc.pipeline_layout)
            .this
            .upgrade()
            .unwrap();

        let shader = match desc.shader_module {
            ShaderBinary::Dxil(v) => v,
            _ => return Err(PipelineCreateError::UnsupportedShaderFormat(0)),
        };

        let pipeline_desc = D3D12_COMPUTE_PIPELINE_STATE_DESC {
            pRootSignature: unsafe { transmute_copy(&pipeline_layout.root_signature) },
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

        let pipeline = AnyArc::new_cyclic(move |v| ComputePipeline {
            this: v.clone(),
            pipeline,
            pipeline_layout,
        });
        Ok(AnyArc::map::<dyn IComputePipeline, _>(pipeline, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError> {
        // TODO: Currently we always create a descriptor table. In the future we could use some
        //       optimization heuristics to detect when a root descriptor is better.
        let visibility = shader_visibility_to_dx12(desc.visibility);

        // First we produce a descriptor table for the non-sampler descriptors. Samplers have to go
        // in their own descriptor heap and so we can't emit a single descriptor table for the
        // layout.
        //
        // Any non-immutable samplers require a second descriptor table.
        let mut binding_info = HashMap::with_capacity(desc.items.len());
        let dynamic_constant_buffers = self.build_dynamic_buffer_views(desc, &mut binding_info);
        let resource_table = self.build_resource_table_layout(desc, &mut binding_info);
        let (sampler_tables, static_samplers) = self.build_sampler_tables(desc, &mut binding_info);
        let resource_num = Self::calculate_descriptor_num(&resource_table);

        let descriptor_set_layout = AnyArc::new_cyclic(move |v| DescriptorSetLayout {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            binding_info,
            visibility,
            dynamic_constant_buffers,
            resource_table,
            resource_num,
            sampler_tables,
            static_samplers,
        });
        Ok(AnyArc::map::<dyn IDescriptorSetLayout, _>(
            descriptor_set_layout,
            |v| v,
        ))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::descriptor_set_layout(desc.layout)
            .this
            .upgrade()
            .unwrap();

        let resource_arena = DescriptorChunk::new(
            self.descriptor_heaps.gpu_view_heap(),
            desc.num_sets * layout.resource_num,
        )?;

        let dynamic_cbs_size = layout.dynamic_constant_buffers.len() * size_of::<u64>();
        let samplers_size = layout.sampler_tables.len() * size_of::<Option<GPUDescriptorHandle>>();
        let array_pool_size = dynamic_cbs_size + samplers_size;

        let pool = Box::new(DescriptorPool {
            _device: self.this.upgrade().unwrap(),
            _layout: layout,
            resource_arena,
            set_pool: DescriptorSetPool::new(desc.num_sets),
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
                    desc.num_sets * 16,
                )?
                .unwrap();

                let set_size = DescriptorArenaLinear::descriptor_set_allocation_layout(1, 1)
                    .unwrap()
                    .size();
                let set_pool_capacity = set_size * desc.num_sets as usize;

                let pool = Box::new(DescriptorArenaLinear {
                    _device: self.this.upgrade().unwrap(),
                    resource_arena,
                    set_pool: BlinkAlloc::with_chunk_size(set_pool_capacity),
                    descriptor_bump_index: Cell::new(0),
                    num_sets: Cell::new(0),
                    set_capacity: desc.num_sets,
                });

                Ok(pool)
            }
            DescriptorArenaType::Heap => {
                let resource_block = DescriptorChunk::new(
                    self.descriptor_heaps.gpu_view_heap(),
                    desc.num_sets * 16,
                )?
                .unwrap();

                let resource_pool =
                    OffsetAllocator::new(resource_block.num_descriptors, desc.num_sets * 2);
                let resource_pool = Box::new(resource_pool);

                let pool = Box::new(DescriptorArenaHeap {
                    _device: self.this.upgrade().unwrap(),
                    resource_block,
                    resource_pool: Cell::new(Some(resource_pool)),
                    set_pool: DescriptorSetPool::new(desc.num_sets),
                    live_handles: Cell::new(Vec::with_capacity(128)),
                });

                Ok(pool)
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError> {
        DEVICE_BUMP.with(|bump_cell| {
            let bump = bump_cell.scope();

            // Bundle up all the table layouts after we patch them for use in this layout as we need to
            // extend the lifetime for the call to create the root signature
            let mut resource_tables = BVec::with_capacity_in(desc.set_layouts.len(), &bump);
            let mut static_samplers = BVec::new_in(&bump);
            let mut parameters = BVec::with_capacity_in(
                desc.set_layouts.len() + desc.push_constant_blocks.len(),
                &bump,
            );
            let mut set_root_param_indices = Vec::with_capacity(desc.set_layouts.len());

            let mut root_param_index = 0;
            for (i, layout) in desc.set_layouts.iter().enumerate() {
                let layout = unwrap::descriptor_set_layout_d(layout);

                // First thing is we store the base root parameter index into our set->param_index
                // lookup table.
                set_root_param_indices.push(root_param_index);

                for dynamic_cb in layout.dynamic_constant_buffers.iter() {
                    let mut dynamic_cb = *dynamic_cb;
                    dynamic_cb.RegisterSpace = i as u32;
                    let dynamic_cb =
                        root_param_for_cbv_root_descriptor(dynamic_cb, layout.visibility);

                    parameters.push(dynamic_cb);
                    root_param_index += 1;
                }

                // Create a parameter for the resource table only if this set has resources in its
                // layout.
                if !layout.resource_table.is_empty() {
                    // Take a copy of the pre-calculated layout and patch the register space to match the
                    // set index that it is being used for
                    let table =
                        create_descriptor_range_list(&bump, &layout.resource_table, i as u32);

                    // Create the root parameter referencing the list in 'table', the lifetime of 'table'
                    // will be extended so the reference remains valid
                    let param = root_param_for_range_list(&table, layout.visibility);
                    parameters.push(param);
                    root_param_index += 1; // Advance the counter as we added a root param

                    // Extend the lifetime of 'table' so it remains alive for the CreateRootSignature
                    // call.
                    resource_tables.push(table);
                }

                // Create a table for each sampler binding in the set, only if there are samplers in the
                // set.
                if !layout.sampler_tables.is_empty() {
                    let table =
                        create_descriptor_range_list(&bump, &layout.sampler_tables, i as u32);

                    // We create a single table for _each_ individual sampler.
                    for range in table.iter() {
                        let range = std::slice::from_ref(range);
                        // Create the root parameter referencing the list in 'table', the lifetime of 'table'
                        // will be extended so the reference remains valid
                        let param = root_param_for_range_list(range, layout.visibility);
                        parameters.push(param);
                        root_param_index += 1; // Advance the counter as we added a root param
                    }

                    resource_tables.push(table);
                }

                // Extend our list of static samplers based on the provided list for this binding
                static_samplers.extend(layout.static_samplers.iter().map(|v| {
                    let mut out = *v;
                    out.RegisterSpace = i as u32;
                    out
                }));
            }

            // TODO: Putting root constants after all descriptors may have performance implications.
            //       D3D12 requires priority to lower root parameter indices so, (on AMD) having push
            //       constants after descriptors means the constants are more likely to spill into
            //       memory instead of being in the registers.
            let mut push_constant_blocks = Vec::with_capacity(desc.push_constant_blocks.len());
            for block in desc.push_constant_blocks {
                if (block.size % 4) != 0 {
                    return Err(PipelineLayoutCreateError::InvalidPushConstantBlockSize);
                }
                let num32_bit_values = (block.size / 4) as u32;
                let range = D3D12_ROOT_PARAMETER1 {
                    ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
                    Anonymous: D3D12_ROOT_PARAMETER1_0 {
                        Constants: D3D12_ROOT_CONSTANTS {
                            ShaderRegister: block.binding,
                            RegisterSpace: 1024, // A reserved space for root/push constants
                            Num32BitValues: num32_bit_values,
                        },
                    },
                    ShaderVisibility: shader_visibility_to_dx12(block.visibility),
                };
                push_constant_blocks.push(PushConstantBlockInfo {
                    size: num32_bit_values * 4,
                    root_parameter_index: parameters.len() as u32,
                });
                parameters.push(range);
            }

            let root_signature = unsafe {
                let desc = D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
                    Version: D3D_ROOT_SIGNATURE_VERSION_1_1,
                    Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
                        Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1 {
                            NumParameters: parameters.len() as _,
                            pParameters: parameters.as_ptr(),
                            NumStaticSamplers: static_samplers.len() as _,
                            pStaticSamplers: static_samplers.as_ptr(),
                            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
                        },
                    },
                };
                let blob = RootSignatureBlob::new(&desc)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
                self.device
                    .CreateRootSignature::<ID3D12RootSignature>(0, &blob)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?
            };

            if let Some(name) = desc.name {
                set_name(&root_signature, name).unwrap();
            }

            let pipeline_layout = AnyArc::new_cyclic(move |v| PipelineLayout {
                this: v.clone(),
                _device: self.this.upgrade().unwrap(),
                root_signature,
                push_constant_blocks,
                set_root_param_indices,
            });
            Ok(AnyArc::map::<dyn IPipelineLayout, _>(
                pipeline_layout,
                |v| v,
            ))
        })
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError> {
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

        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: heap_type,
            CPUPageProperty: Default::default(),
            MemoryPoolPreference: Default::default(),
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource = unsafe {
            let mut resource: Option<ID3D12Resource> = None;
            self.device
                .CreateCommittedResource3::<_, ID3D12Resource>(
                    &heap_properties,
                    Default::default(),
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT_UNDEFINED,
                    None,
                    None,
                    None,
                    &mut resource,
                )
                .map(|_| resource.unwrap())
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };
        let base_address =
            unsafe { GPUDescriptorHandle::try_from(resource.GetGPUVirtualAddress()).unwrap() };

        if let Some(name) = desc.name {
            set_name(&resource, name).unwrap();
        }

        let name = desc.name.map(str::to_string);
        let desc = desc.clone().strip_name();

        let buffer = AnyArc::new_cyclic(move |v| Buffer {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            resource,
            base_address,
            desc,
            name,
        });
        Ok(AnyArc::map::<dyn IBuffer, _>(buffer, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            CPUPageProperty: Default::default(),
            MemoryPoolPreference: Default::default(),
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource_desc = texture_create_desc_to_dx12(desc)?;
        let optimized_clear_value = texture_create_clear_value_to_dx12(desc, resource_desc.Format)?;

        let resource = unsafe {
            let optimized_clear_value = optimized_clear_value.map(D3D12_CLEAR_VALUE::from);
            let optimized_clear_value_ref = optimized_clear_value
                .as_ref()
                .map(|v| v as *const D3D12_CLEAR_VALUE);

            let mut resource: Option<ID3D12Resource> = None;
            self.device
                .CreateCommittedResource3::<_, ID3D12Resource>(
                    &heap_properties,
                    Default::default(),
                    &resource_desc,
                    D3D12_BARRIER_LAYOUT_UNDEFINED,
                    optimized_clear_value_ref,
                    None,
                    None, // TODO: We could use this maybe?
                    &mut resource,
                )
                .map(|_| resource.unwrap())
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };

        if let Some(name) = desc.name {
            set_name(&resource, name).unwrap();
        }

        let name = desc.name.map(str::to_string);
        let desc = desc.clone().strip_name();

        let texture = AnyArc::new_cyclic(move |v| Texture {
            this: v.clone(),
            device: self.this.upgrade().unwrap(),
            resource,
            desc,
            name,
            dxgi_format: resource_desc.Format,
            views: Default::default(),
            rtvs: Default::default(),
            dsvs: Default::default(),
            image_views: Mutex::new(Bump::with_capacity(size_of::<ImageViewObject>() * 8)),
        });
        Ok(AnyArc::map::<dyn ITexture, _>(texture, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError> {
        let gpu_handle = self
            .descriptor_heaps
            .gpu_sampler_cache()
            .get(desc)
            .ok_or(SamplerCreateError::OutOfSamplers)?;

        // TODO: we probably need to validate the sampler description to keep this API safe.

        let static_desc = D3D12_STATIC_SAMPLER_DESC {
            Filter: sampler_filters_to_dx12(
                desc.min_filter,
                desc.mag_filter,
                desc.mip_filter,
                desc.compare_op.is_some(),
                desc.enable_anisotropy,
            ),
            AddressU: sampler_address_mode_to_dx12(desc.address_mode_u),
            AddressV: sampler_address_mode_to_dx12(desc.address_mode_v),
            AddressW: sampler_address_mode_to_dx12(desc.address_mode_w),
            MipLODBias: desc.lod_bias,
            MaxAnisotropy: desc.max_anisotropy,
            ComparisonFunc: desc
                .compare_op
                .map(compare_op_to_dx12)
                .unwrap_or(D3D12_COMPARISON_FUNC(0)),
            BorderColor: border_color_to_dx12_static(desc.border_color),
            MinLOD: desc.min_lod,
            MaxLOD: desc.max_lod,
            ShaderRegister: 0,
            RegisterSpace: 0,
            ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
        };

        let name = desc.name.map(str::to_string);
        let desc = desc.clone().strip_name();

        let sampler = AnyArc::new_cyclic(move |v| Sampler {
            this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            desc,
            name,
            gpu_handle,
            static_desc,
        });
        Ok(AnyArc::map::<dyn ISampler, _>(sampler, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        // TODO: Can probably get easy gains by maintaining an object pool to reuse from
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

    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]) {
        for set_write in writes {
            self.update_descriptor_set(set_write);
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_fence(&self, signalled: bool) -> Result<AnyArc<dyn IFence>, FenceCreateError> {
        let initial_value = if signalled { 1 } else { 0 };
        let fence: ID3D12Fence = unsafe {
            self.device
                .CreateFence(initial_value, D3D12_FENCE_FLAG_NONE)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };
        let fence = AnyArc::new_cyclic(move |v| Fence {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            fence,
            value: AtomicU64::new(2),
        });
        Ok(AnyArc::map::<dyn IFence, _>(fence, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn create_semaphore(&self) -> Result<AnyArc<dyn ISemaphore>, SemaphoreCreateError> {
        let fence: ID3D12Fence = unsafe {
            self.device
                .CreateFence(0, D3D12_FENCE_FLAG_NONE)
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?
        };
        let semaphore = AnyArc::new_cyclic(move |v| Semaphore {
            _this: v.clone(),
            _device: self.this.upgrade().unwrap(),
            fence,
            value: AtomicU64::new(0),
        });
        Ok(AnyArc::map::<dyn ISemaphore, _>(semaphore, |v| v))
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn wait_fences(&self, fences: &[&dyn IFence], wait_all: bool, timeout: u32) -> FenceWaitResult {
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
                    let fence = unwrap::fence(fences[0]);
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
                    let mut inner_fences: BVec<Option<ID3D12Fence>> =
                        BVec::with_capacity_in(fences.len(), &bump);
                    let mut wait_values: BVec<u64> = BVec::with_capacity_in(fences.len(), &bump);
                    for fence in unwrap::fence_iter(fences) {
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

    fn poll_fence(&self, fence: &dyn IFence) -> bool {
        let fence = unwrap::fence(fence);
        unsafe {
            let v = fence.fence.GetCompletedValue();
            v < fence.get_wait_value()
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn reset_fences(&self, _fences: &[&dyn IFence]) {
        // Fence reset is a no-op on dx12 as a fence is always ready to use. It uses a monotonic
        // counter to keep the signals and waits correct.
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }
}

impl Device {
    /// Internal function for translating the list of [ShaderStage] stages into the pipeline
    /// description
    fn translate_shader_stage_list<'a>(
        shader_stages: &'a [ShaderStage<'a>],
        mut builder: GraphicsPipelineStateStreamBuilder<'a>,
    ) -> Result<GraphicsPipelineStateStreamBuilder<'a>, PipelineCreateError> {
        for (i, shader_stage) in shader_stages.iter().enumerate() {
            builder = match shader_stage.data {
                ShaderBinary::Dxil(shader) => match shader_stage.stage {
                    ShaderType::Vertex => builder.vertex_shader(shader),
                    ShaderType::Hull => builder.hull_shader(shader),
                    ShaderType::Domain => builder.domain_shader(shader),
                    ShaderType::Geometry => builder.geometry_shader(shader),
                    ShaderType::Fragment => builder.pixel_shader(shader),
                    ShaderType::Compute => {
                        panic!("Can't bind a compute shader to a graphics pipeline")
                    }
                    ShaderType::Amplification | ShaderType::Mesh => {
                        todo!("Missing implementation for amplification and mesh shaders")
                    }
                },
                _ => return Err(PipelineCreateError::UnsupportedShaderFormat(i)),
            }
        }
        Ok(builder)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Internal function for translating the [VertexInputStateDesc] field of a pipeline
    /// description
    fn translate_vertex_input_state_desc<'a>(
        bump: &'a Bump,
        desc: &VertexInputStateDesc,
    ) -> ([u32; 16], BVec<'a, D3D12_INPUT_ELEMENT_DESC>) {
        // Copy the input binding strides into a buffer the pipeline will hold on to so it can be
        // used in the command encoders. Vulkan bakes these in the pipeline, d3d12 gets the values
        // when the input bindings are bound
        let mut input_binding_strides = [0u32; 16];
        for (binding, stride) in desc.input_bindings.iter().zip(&mut input_binding_strides) {
            *stride = binding.stride;
        }

        // Translate the vertex input description
        let mut input_layout = BVec::with_capacity_in(desc.input_attributes.len(), bump);
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
            let semantic_name = cstr!("A");
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

    fn build_dynamic_buffer_views(
        &self,
        desc: &DescriptorSetLayoutDesc,
        binding_info: &mut HashMap<u32, DescriptorBindingInfo>,
    ) -> Vec<D3D12_ROOT_DESCRIPTOR1> {
        let mut table = Vec::with_capacity(desc.items.len());
        for item in desc
            .items
            .iter()
            .filter(|v| v.binding_type == DescriptorType::UniformBufferDynamic)
        {
            if item.binding_count.is_some() {
                // Descriptor arrays are currently unimplemented pending a solution for mapping
                // how they surface in SPIR-V vs D3D12.
                //
                // - Vulkan uses a single binding for the whole array.
                // - D3D12 uses a register per element.
                //
                // We currently map binding_num directly to register number. Arrays break this
                // mapping, Vulkan will work but D3D12 will not. We either have to force asinine
                // D3D12 behavior on Vulkan or
                //
                unimplemented!("Currently descriptor arrays are unimplemented");
            }

            let num_descriptors = match item.binding_count {
                None => 1,
                Some(v) => v.get(),
            };

            let base_shader_register = item.binding_num;

            let info = DescriptorBindingInfo {
                r#type: item.binding_type,
                is_static_sampler: false,
                layout: DescriptorBindingLayout {
                    base: base_shader_register,
                    num_descriptors,
                },
            };
            binding_info.insert(item.binding_num, info);

            let item = D3D12_ROOT_DESCRIPTOR1 {
                ShaderRegister: base_shader_register,
                RegisterSpace: 0,
                Flags: D3D12_ROOT_DESCRIPTOR_FLAG_NONE,
            };
            table.push(item);
        }
        table
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn build_resource_table_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
        binding_info: &mut HashMap<u32, DescriptorBindingInfo>,
    ) -> Vec<D3D12_DESCRIPTOR_RANGE1> {
        fn not_sampler_or_dynamic_ubo(v: DescriptorType) -> bool {
            !matches!(
                v,
                DescriptorType::Sampler | DescriptorType::UniformBufferDynamic
            )
        }

        let mut offset = 0;
        let mut table = Vec::with_capacity(desc.items.len());
        for item in desc
            .items
            .iter()
            .filter(|v| not_sampler_or_dynamic_ubo(v.binding_type))
        {
            if item.binding_count.is_some() {
                // Descriptor arrays are currently unimplemented pending a solution for mapping
                // how they surface in SPIR-V vs D3D12.
                //
                // - Vulkan uses a single binding for the whole array.
                // - D3D12 uses a register per element.
                //
                // We currently map binding_num directly to register number. Arrays break this
                // mapping, Vulkan will work but D3D12 will not. We either have to force asinine
                // D3D12 behavior on Vulkan or
                //
                unimplemented!("Currently descriptor arrays are unimplemented");
            }

            let range_type = descriptor_type_to_dx12(item.binding_type);

            let num_descriptors = match item.binding_count {
                None => 1,
                Some(v) => v.get(),
            };

            let base_shader_register = item.binding_num;

            let info = DescriptorBindingInfo {
                r#type: item.binding_type,
                is_static_sampler: item.static_samplers.is_some(),
                layout: DescriptorBindingLayout {
                    base: offset,
                    num_descriptors,
                },
            };
            binding_info.insert(item.binding_num, info);

            let item = D3D12_DESCRIPTOR_RANGE1 {
                RangeType: range_type,
                NumDescriptors: num_descriptors,
                BaseShaderRegister: base_shader_register,
                RegisterSpace: 0,
                Flags: D3D12_DESCRIPTOR_RANGE_FLAG_NONE,
                OffsetInDescriptorsFromTableStart: offset,
            };
            table.push(item);
            offset += num_descriptors;
        }
        table
    }

    // ========================================================================================== //
    // ========================================================================================== //

    fn build_sampler_tables(
        &self,
        desc: &DescriptorSetLayoutDesc,
        binding_info: &mut HashMap<u32, DescriptorBindingInfo>,
    ) -> (Vec<D3D12_DESCRIPTOR_RANGE1>, Vec<D3D12_STATIC_SAMPLER_DESC>) {
        let mut sampler_tables = Vec::new();
        let mut static_samplers = Vec::new();
        for item in desc
            .items
            .iter()
            .filter(|v| v.binding_type == DescriptorType::Sampler)
        {
            if item.binding_count.is_some() {
                // we don't support sampler array bindings due to strict limits imposed on D3D12.
                // - (Tier 1) max 16 samplers in a single root signature
                // - (Tier 2+) max 2048 samplers in a single root signature
                // - max 2048 samplers in a single device-visible descriptor heap
                //
                // Only 2048 samplers can ever be addressed at once, making bindless difficult as
                // the limit is very small, and non-bindless capable hardware can only have 16
                // samplers in a root signature meaning static sized arrays will typically be so
                // small it makes using an array redundant.
                unimplemented!("Sampler Arrays are currently un-implemented");
            }

            // Dynamic samplers require a descriptor table as they're dynamic. There is a separate
            // part of a root signature that handles static samplers.
            //
            // We switch how we output the binding based on the presence of static samplers
            if let Some(samplers) = item.static_samplers {
                for sampler in samplers {
                    let sampler = unwrap::sampler_d(sampler);
                    let mut desc = sampler.static_desc;
                    desc.ShaderRegister = item.binding_num;

                    static_samplers.push(desc);
                }
            } else {
                // Handle dynamic samplers by inserting them into a descriptor table.
                let num_descriptors = match item.binding_count {
                    None => 1,
                    Some(v) => v.get(),
                };
                let base_shader_register = item.binding_num;

                let info = DescriptorBindingInfo {
                    r#type: item.binding_type,
                    is_static_sampler: item.static_samplers.is_some(),
                    layout: DescriptorBindingLayout {
                        base: base_shader_register,
                        num_descriptors,
                    },
                };
                binding_info.insert(item.binding_num, info);

                let item = D3D12_DESCRIPTOR_RANGE1 {
                    RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
                    NumDescriptors: num_descriptors,
                    BaseShaderRegister: base_shader_register,
                    RegisterSpace: 0,
                    Flags: D3D12_DESCRIPTOR_RANGE_FLAG_NONE,
                    OffsetInDescriptorsFromTableStart: 0,
                };
                sampler_tables.push(item);
            }
        }
        (sampler_tables, static_samplers)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_descriptor_set(&self, set_write: &DescriptorWriteDesc) {
        let mut set = DescriptorSet::ptr_from_handle(set_write.set);
        let set_layout = {
            let set = set.as_ref();
            let layout = set._layout.as_ref();
            layout
        };
        let binding_layout = set_layout
            .get_binding_info(set_write.binding)
            .unwrap()
            .layout;

        match set_write.writes {
            DescriptorWrites::Sampler(writes) => {
                self.update_sampler_descriptors(set_write.binding, set, &binding_layout, writes)
            }
            DescriptorWrites::TexelBufferRW(writes) | DescriptorWrites::TexelBuffer(writes) => self
                .update_texel_buffer_descriptors(
                    set_write.array_element,
                    set,
                    binding_layout,
                    writes,
                    set_write.writes.descriptor_type(),
                ),
            DescriptorWrites::TextureRW(writes) | DescriptorWrites::Texture(writes) => {
                self.update_image_descriptors(set_write.array_element, set, binding_layout, writes)
            }
            DescriptorWrites::ByteAddressBufferRW(writes)
            | DescriptorWrites::ByteAddressBuffer(writes)
            | DescriptorWrites::StructuredBufferRW(writes)
            | DescriptorWrites::StructuredBuffer(writes)
            | DescriptorWrites::UniformBuffer(writes) => self.update_buffer_descriptors(
                set_write.array_element,
                set,
                binding_layout,
                writes,
                set_write.writes.descriptor_type(),
            ),
            DescriptorWrites::UniformBufferDynamic(writes) => {
                let dynamic_cb_index = set_layout
                    .dynamic_constant_buffers
                    .iter()
                    .enumerate()
                    .find(|v| v.1.ShaderRegister == set_write.binding)
                    .map(|v| v.0)
                    .unwrap();
                let set = set.as_mut();
                let dynamic_cbs = set.dynamic_constant_buffers.as_mut();
                let dynamic_cbs = &mut dynamic_cbs[dynamic_cb_index..];

                for (i, v) in writes.iter().enumerate() {
                    let buffer = unwrap::buffer(v.buffer);
                    let buffer = buffer.base_address.add(v.offset);
                    dynamic_cbs[i] = buffer.get_inner().get();
                }
            }
            DescriptorWrites::InputAttachment(_) => {
                unimplemented!()
            }
        };
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_sampler_descriptors(
        &self,
        binding: u32,
        mut set: NonNull<DescriptorSet>,
        _binding_layout: &DescriptorBindingLayout,
        writes: &[SamplerDescriptorWrite],
    ) {
        let set = set.as_mut();

        // Find which index in the set's sampler array maps to the binding we're writing to.
        //
        // Linear search is best here as the 'sampler_tables' array is rarely if _ever_ going to
        // have more than a few elements. Linear search is best for tiny search spaces on modern
        // GPUs.
        let sampler_index = set
            ._layout
            .as_ref()
            .sampler_tables
            .iter()
            .enumerate()
            .find_map(|(i, v)| {
                if v.BaseShaderRegister == binding {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap();

        // We don't support sampler arrays so we assert if someone tries to write one
        debug_assert_eq!(writes.len(), 1, "No support for sampler arrays currently");

        for (_i, v) in writes.iter().enumerate() {
            let target = set.samplers.as_mut();
            let sampler = unwrap::sampler(v.sampler);

            // Copy the gpu descriptor handle into the descriptor set's internal list of samplers.
            // This internal list will be consumed at bind time to set each sampler as samplers are
            // always encoded as single entry descriptor tables to work around tight sampler limit
            // foot guns.
            //
            // This makes adopting Vulkan's binding style friendlier to the developer at the cost of
            // not being able to support 'update after bind' for sampler descriptors.
            let src = sampler.gpu_handle;
            target[sampler_index] = Some(src);
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_image_descriptors(
        &self,
        array_base: u32,
        set: NonNull<DescriptorSet>,
        binding_layout: DescriptorBindingLayout,
        writes: &[ImageDescriptorWrite],
    ) {
        let set = set.as_ref();

        for (i, v) in writes.iter().enumerate() {
            // SAFETY: It is the caller's responsibility to ensure that the view points to a live
            //         and valid ImageViewObject. The objects are immutable so parallel access is
            //         safe implicitly.
            let src = std::mem::transmute::<_, *const ImageViewObject>(v.image_view);
            let src = (*src).handle;

            let (dst, _) = set.assume_r_handle();
            let dst = Self::calculate_dst_handle(
                dst,
                self.descriptor_heap_info.resource_inc,
                binding_layout.base,
                array_base,
                i,
            );

            self.device.CopyDescriptorsSimple(
                1,
                dst.into(),
                src.into(),
                D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            );
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_buffer_descriptors(
        &self,
        array_base: u32,
        set: NonNull<DescriptorSet>,
        binding_layout: DescriptorBindingLayout,
        writes: &[BufferDescriptorWrite],
        d_type: DescriptorType,
    ) {
        let set = set.as_ref();

        for (i, v) in writes.iter().enumerate() {
            let (dst, _) = set.assume_r_handle();

            let buffer = unwrap::buffer(v.buffer);

            let dst = Self::calculate_dst_handle(
                dst,
                self.descriptor_heap_info.resource_inc,
                binding_layout.base,
                array_base,
                i,
            );

            match d_type {
                DescriptorType::UniformBuffer => {
                    self.update_uniform_buffer_descriptor(v, buffer, dst);
                }
                DescriptorType::ByteAddressBuffer => {
                    self.update_byte_address_buffer_descriptor_srv(v, buffer, dst);
                }
                DescriptorType::ByteAddressBufferRW => {
                    self.update_byte_address_buffer_descriptor_uav(v, buffer, dst);
                }
                DescriptorType::StructuredBuffer => {
                    self.update_structured_buffer_descriptor_srv(v, buffer, dst);
                }
                DescriptorType::StructuredBufferRW => {
                    self.update_structured_buffer_descriptor_uav(v, buffer, dst);
                }
                _ => {}
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_texel_buffer_descriptors(
        &self,
        array_base: u32,
        set: NonNull<DescriptorSet>,
        binding_layout: DescriptorBindingLayout,
        writes: &[TexelBufferDescriptorWrite],
        d_type: DescriptorType,
    ) {
        let set = set.as_ref();

        for (i, v) in writes.iter().enumerate() {
            let (dst, _) = set.assume_r_handle();

            let buffer = unwrap::buffer(v.buffer);

            let dst = Self::calculate_dst_handle(
                dst,
                self.descriptor_heap_info.resource_inc,
                binding_layout.base,
                array_base,
                i,
            );

            match d_type {
                DescriptorType::TexelBuffer => {
                    self.update_texel_buffer_descriptor_srv(v, buffer, dst);
                }
                DescriptorType::TexelBufferRW => {
                    self.update_texel_buffer_descriptor_uav(v, buffer, dst);
                }
                _ => {}
            }
        }
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_uniform_buffer_descriptor(
        &self,
        write: &BufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_byte_address_buffer_descriptor_srv(
        &self,
        write: &BufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
            .CreateShaderResourceView(&buffer.resource, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_byte_address_buffer_descriptor_uav(
        &self,
        write: &BufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
        self.device
            .CreateUnorderedAccessView(&buffer.resource, None, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_structured_buffer_descriptor_srv(
        &self,
        write: &BufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
            .CreateShaderResourceView(&buffer.resource, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_structured_buffer_descriptor_uav(
        &self,
        write: &BufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
        self.device
            .CreateUnorderedAccessView(&buffer.resource, None, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_texel_buffer_descriptor_srv(
        &self,
        write: &TexelBufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
            .CreateShaderResourceView(&buffer.resource, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    unsafe fn update_texel_buffer_descriptor_uav(
        &self,
        write: &TexelBufferDescriptorWrite,
        buffer: &Buffer,
        dst: CPUDescriptorHandle,
    ) {
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
        self.device
            .CreateUnorderedAccessView(&buffer.resource, None, Some(&view), dst.into());
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Calculates the destination descriptor handle based on the given increment size and a set of
    /// descriptor offsets.
    ///
    /// This function is intended to be used to calculate [CPUDescriptorHandle] values for writing
    /// descriptors into descriptor sets. The expected usage pattern can be described as:
    ///
    /// - I have the address of a descriptor set 'handle'
    /// - I want the address of the beginning of some binding that starts 'binding_base' descriptors
    ///   after 'handle'
    /// - Assuming I'm working with an array binding, I want the address of the 'array_base'th
    ///   element in that array. This could be thought of taking a sub-slice of the larger array
    ///   binding.
    /// - Assuming I want to index the sub-slice I just got the beginning of, I want the 'i'th
    ///   element in the sub-array. This could be thought of as indexing the sub-array.
    ///
    /// All of this assumes a common descriptor increment 'increment'.
    ///
    /// # Arguments
    ///
    /// - 'handle': The descriptor handle for the beginning of the descriptor set we're offsetting
    ///   into.
    /// - 'increment': The descriptor increment for the descriptor type we're working with.
    /// - 'set_base': The offset in descriptors from 'handle' the target binding begins at.
    /// - 'array_base': The offset in descriptors from the combined 'handle + binding' where the
    ///   target base array element beings.
    /// - 'i': The offset in descriptors from the combined 'handle + binding + array_base' where the
    ///   target array element begins.
    const fn calculate_dst_handle(
        handle: CPUDescriptorHandle,
        increment: u32,
        binding_base: u32,
        array_base_element: u32,
        i: usize,
    ) -> CPUDescriptorHandle {
        // The offset from the start of the descriptor set where the target binding begins
        let binding_base_offset = binding_base as usize * increment as usize;

        // The offset from the start of the binding where the target array element begins
        let binding_array_offset = array_base_element as usize * increment as usize;

        // The offset from the start of the array where the target write element begins
        let binding_element_offset = i * increment as usize;

        handle.add(binding_base_offset + binding_array_offset + binding_element_offset)
    }

    // ========================================================================================== //
    // ========================================================================================== //

    /// Calculate the number of descriptors by finding the highest offset from the base of
    /// the table that any of the ranges requires.
    fn calculate_descriptor_num(ranges: &[D3D12_DESCRIPTOR_RANGE1]) -> u32 {
        let mut highest_offset = 0;
        for table in ranges {
            highest_offset =
                highest_offset.max(table.OffsetInDescriptorsFromTableStart + table.NumDescriptors);
        }
        highest_offset
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

fn create_descriptor_range_list<'a>(
    bump: &'a Bump,
    v: &[D3D12_DESCRIPTOR_RANGE1],
    i: u32,
) -> BVec<'a, D3D12_DESCRIPTOR_RANGE1> {
    let mut table = BVec::from_iter_in(v.iter().copied(), bump);
    for binding in table.iter_mut() {
        binding.RegisterSpace = i;
    }
    table
}

fn root_param_for_range_list(
    v: &[D3D12_DESCRIPTOR_RANGE1],
    visibility: D3D12_SHADER_VISIBILITY,
) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
        Anonymous: D3D12_ROOT_PARAMETER1_0 {
            DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1 {
                NumDescriptorRanges: v.len() as _,
                pDescriptorRanges: v.as_ptr(),
            },
        },
        ShaderVisibility: visibility,
    }
}

fn root_param_for_cbv_root_descriptor(
    v: D3D12_ROOT_DESCRIPTOR1,
    visibility: D3D12_SHADER_VISIBILITY,
) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_CBV,
        Anonymous: D3D12_ROOT_PARAMETER1_0 { Descriptor: v },
        ShaderVisibility: visibility,
    }
}

thread_local! {
    pub static DEVICE_BUMP: BumpCell = BumpCell::with_capacity(8192 * 2);
}
