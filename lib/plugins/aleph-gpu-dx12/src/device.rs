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

use crate::acquired_texture::AcquiredTexture;
use crate::adapter::Adapter;
use crate::buffer::Buffer;
use crate::command_pool::CommandPool;
use crate::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::general_command_list::GeneralCommandList;
use crate::internal::conv::{
    resource_state_to_dx12, texture_create_clear_value_to_dx12, texture_create_desc_to_dx12,
};
use crate::internal::in_flight_command_list::InFlightCommandList;
use crate::internal::queue::Queue;
use crate::shader::Shader;
use crate::texture::Texture;
use crossbeam::queue::SegQueue;
use dx12::{dxgi, AsWeakRef, D3D12Object};
use interfaces::any::QueryInterfaceBox;
use interfaces::anyhow;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    BackendAPI, BufferCreateError, BufferDesc, CommandListSubmitError, CommandPoolCreateError,
    CpuAccessMode, IAcquiredTexture, IBuffer, ICommandPool, IDevice, IGeneralCommandList,
    INamedObject, IShader, ISwapChain, ITexture, QueuePresentError, QueueType, ShaderBinary,
    ShaderCreateError, ShaderOptions, TextureCreateError, TextureDesc,
};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

ref_ptr_object! {
    pub struct Device: IDevice, IDeviceExt {
        pub(crate) device: dx12::Device,
        pub(crate) rtv_heap: DescriptorAllocatorCPU,
        pub(crate) dsv_heap: DescriptorAllocatorCPU,
        pub(crate) queues: Queues,
        pub(crate) adapter: RefPtr<Adapter>,
    }
}

impl IDevice for Device {
    fn garbage_collect(&self) {
        if let Some(queue) = &self.queues.general {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.queues.compute {
            queue.clear_completed_lists();
        }
        if let Some(queue) = &self.queues.transfer {
            queue.clear_completed_lists();
        }
    }

    fn wait_idle(&self) {
        if let Some(queue) = &self.queues.general {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.queues.compute {
            queue.wait_all_lists_completed();
        }
        if let Some(queue) = &self.queues.transfer {
            queue.wait_all_lists_completed();
        }
    }

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<RefPtr<dyn IShader>, ShaderCreateError> {
        if let ShaderBinary::Dxil(data) = options.data {
            // Empty shader binary is invalid
            if data.is_empty() {
                return Err(ShaderCreateError::InvalidInputSize(0));
            }

            let shader = ref_ptr_init! {
                Shader {
                    shader_type: options.shader_type,
                    data: data.to_vec(),
                    entry_point: options.entry_point.to_string(),
                }
            };
            let shader: RefPtr<Shader> = RefPtr::new(shader);

            Ok(shader.query_interface().unwrap())
        } else {
            Err(ShaderCreateError::UnsupportedShaderFormat)
        }
    }

    fn create_buffer(&self, desc: &BufferDesc) -> Result<RefPtr<dyn IBuffer>, BufferCreateError> {
        let mut resource_desc = dx12::ResourceDesc {
            // Fields that will be the same regardless of the requested buffer desc
            dimension: dx12::ResourceDimension::Buffer,
            layout: dx12::TextureLayout::RowMajor,
            format: dxgi::Format::Unknown,
            alignment: 0,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 1,
            sample_desc: dxgi::SampleDesc {
                count: 1,
                quality: 0,
            },

            // Fields based on the description
            width: 0,
            flags: dx12::ResourceFlags::NONE,
        };

        resource_desc.width = desc.size;

        if desc.allow_unordered_access {
            resource_desc.flags |= dx12::ResourceFlags::ALLOW_UNORDERED_ACCESS;
        }

        let (heap_type, initial_state) = match desc.cpu_access {
            CpuAccessMode::None => {
                (dx12::HeapType::Default, dx12::ResourceStates::COMMON) // TODO: Figure this out
            }
            CpuAccessMode::Read => (dx12::HeapType::ReadBack, dx12::ResourceStates::COPY_DEST),
            CpuAccessMode::Write => (dx12::HeapType::Upload, dx12::ResourceStates::GENERIC_READ),
        };

        let heap_properites = dx12::HeapProperties {
            r#type: heap_type,
            cpu_page_property: Default::default(),
            memory_pool_preference: Default::default(),
            creation_node_mask: 0,
            visible_node_mask: 0,
        };
        let resource = unsafe {
            self.device
                .create_committed_resource(
                    &heap_properites,
                    dx12::HeapFlags::NONE,
                    &resource_desc,
                    initial_state,
                    None,
                )
                .map_err(|v| anyhow!(v))?
        };

        let buffer = ref_ptr_init! {
            Buffer {
                resource: resource,
                desc: desc.clone(),
            }
        };
        let buffer: RefPtr<Buffer> = RefPtr::new(buffer);
        Ok(buffer.query_interface().unwrap())
    }

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<RefPtr<dyn ITexture>, TextureCreateError> {
        let heap_properties = dx12::HeapProperties {
            r#type: dx12::HeapType::Default,
            ..Default::default()
        };
        let heap_flags = dx12::HeapFlags::NONE;
        let resource_desc = texture_create_desc_to_dx12(desc)?;
        let initial_state = resource_state_to_dx12(desc.initial_state);
        let optimized_clear_value = texture_create_clear_value_to_dx12(desc, resource_desc.format)?;

        let resource = unsafe {
            self.device
                .create_committed_resource(
                    &heap_properties,
                    heap_flags,
                    &resource_desc,
                    initial_state,
                    optimized_clear_value,
                )
                .map_err(|v| anyhow!(v))?
        };

        let texture = ref_ptr_init! {
            Texture {
                device: self.as_ref_ptr(),
                resource: resource,
                desc: desc.clone(),
                dxgi_format: resource_desc.format,
                rtv_cache: RwLock::new(HashMap::new()),
                dsv_cache: RwLock::new(HashMap::new()),
            }
        };
        let texture: RefPtr<Texture> = RefPtr::new(texture);
        Ok(texture.query_interface().unwrap())
    }

    fn create_command_pool(&self) -> Result<RefPtr<dyn ICommandPool>, CommandPoolCreateError> {
        let pool = ref_ptr_init! {
            CommandPool {
                device: self.as_ref_ptr(),
                general_free_list: SegQueue::new(),
                compute_free_list: SegQueue::new(),
                transfer_free_list: SegQueue::new(),
            }
        };
        let pool: RefPtr<CommandPool> = RefPtr::new(pool);
        Ok(pool.query_interface().unwrap())
    }

    unsafe fn general_queue_submit_list(
        &self,
        command_list: Box<dyn IGeneralCommandList>,
    ) -> Result<(), CommandListSubmitError> {
        let queue =
            self.queues
                .general
                .as_ref()
                .ok_or(CommandListSubmitError::QueueNotAvailable(
                    QueueType::General,
                ))?;

        let command_list: Box<GeneralCommandList> = command_list
            .query_interface::<GeneralCommandList>()
            .ok()
            .unwrap();

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = queue.submit_lock.lock();
            queue
                .handle
                .execute_command_lists(&[command_list.list.as_weak()]);

            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        queue.in_flight.push(InFlightCommandList {
            index,
            list: command_list,
        });

        Ok(())
    }

    unsafe fn general_queue_submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn IGeneralCommandList>>,
    ) -> Result<(), CommandListSubmitError> {
        let queue =
            self.queues
                .general
                .as_ref()
                .ok_or(CommandListSubmitError::QueueNotAvailable(
                    QueueType::General,
                ))?;

        // Perform the actual submit operation
        let lists: Vec<Box<GeneralCommandList>> = command_lists
            .map(|v| v.query_interface::<GeneralCommandList>().ok().unwrap())
            .collect();

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let index = {
            let _lock = queue.submit_lock.lock();

            let handles: Vec<dx12::GraphicsCommandList> =
                lists.iter().map(|v| v.list.clone()).collect();

            queue.handle.execute_command_lists_strong(&handles);

            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        for list in lists {
            queue.in_flight.push(InFlightCommandList { index, list });
        }

        Ok(())
    }

    unsafe fn general_queue_present(
        &self,
        texture: Box<dyn IAcquiredTexture>,
    ) -> Result<(), QueuePresentError> {
        let image = texture.query_interface::<AcquiredTexture>().ok().unwrap();

        if !image
            .swap_chain
            .present_supported_on_queue(QueueType::General)
        {
            return Err(QueuePresentError::QueuePresentationNotSupported(
                QueueType::General,
            ));
        }

        let queue = self
            .queues
            .general
            .as_ref()
            .ok_or(QueuePresentError::QueueNotAvailable(QueueType::General))?;

        // Grab the submit lock to prevent concurrent submits. I'm not sure if d3d12 allows
        // concurrent submits from multiple threads but vulkan doesn't so I'll assume d3d12 doesn't
        // either.
        let _index = {
            let _lock = queue.submit_lock.lock();

            image
                .swap_chain
                .swap_chain
                .present(0, 0)
                .map_err(|v| anyhow!(v))?;
            let index = queue.last_submitted_index.fetch_add(1, Ordering::Relaxed);
            queue
                .handle
                .signal(&queue.fence, index)
                .map_err(|v| anyhow!(v))?;

            index
        };

        Ok(())
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }
}

impl Device {
    pub unsafe fn create_views_for_swap_images(
        &self,
        swap_chain: &dxgi::SwapChain,
        format: dxgi::Format,
        count: u32,
    ) -> anyhow::Result<Vec<(dx12::Resource, dx12::CPUDescriptorHandle)>> {
        let mut images = Vec::new();
        for i in 0..count {
            let buffer = swap_chain.get_buffer(i).map_err(|e| anyhow!(e))?;
            let view = self.rtv_heap.allocate().unwrap();

            let desc = dx12::RenderTargetViewDesc::Texture2D {
                format,
                texture_2d: dx12::Tex2DRtv {
                    mip_slice: 0,
                    plane_slice: 0,
                },
            };
            self.device.create_render_target_view(&buffer, &desc, view);

            images.push((buffer, view));
        }
        Ok(images)
    }
}

pub trait IDeviceExt: IDevice {
    fn get_raw_handle(&self) -> dx12::Device;
    fn get_raw_general_queue(&self) -> Option<dx12::CommandQueue>;
    fn get_raw_compute_queue(&self) -> Option<dx12::CommandQueue>;
    fn get_raw_transfer_queue(&self) -> Option<dx12::CommandQueue>;
}

impl IDeviceExt for Device {
    fn get_raw_handle(&self) -> dx12::Device {
        self.device.clone()
    }

    fn get_raw_general_queue(&self) -> Option<dx12::CommandQueue> {
        self.queues.general.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_compute_queue(&self) -> Option<dx12::CommandQueue> {
        self.queues.compute.as_ref().map(|v| v.handle.clone())
    }

    fn get_raw_transfer_queue(&self) -> Option<dx12::CommandQueue> {
        self.queues.transfer.as_ref().map(|v| v.handle.clone())
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl INamedObject for Device {
    fn set_name(&self, name: &str) {
        self.device.set_name(name).unwrap()
    }
}

/// Internal struct that logically associates all queues into a single block
///
/// # Info
///
/// I'm not sure if I need a mutex on D3D12, but vkQueue requires external synchronization so I am
/// just going to be safe for now and lock for the D3D12 backend too for now.
///
/// I can just remove them later
pub struct Queues {
    pub general: Option<Queue<GeneralCommandList>>,
    pub compute: Option<Queue<()>>,
    pub transfer: Option<Queue<()>>,
}
