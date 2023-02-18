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

use crate::command_list::CommandList;
use crate::device::Device;
use crossbeam::queue::SegQueue;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow;
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use std::any::TypeId;
use windows::Win32::Graphics::Direct3D12::*;

pub struct CommandPool {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) descriptor_heaps: [Option<ID3D12DescriptorHeap>; 2],
    pub(crate) general_free_list: SegQueue<CommandPoolFreeListItem>,
    pub(crate) _compute_free_list: SegQueue<CommandPoolFreeListItem>,
    pub(crate) _transfer_free_list: SegQueue<CommandPoolFreeListItem>,
}

declare_interfaces!(CommandPool, [ICommandPool]);

pub type CommandPoolFreeListItem = (ID3D12CommandAllocator, ID3D12GraphicsCommandList7);

impl CommandPool {
    fn new_list(
        &self,
        list_type: D3D12_COMMAND_LIST_TYPE,
    ) -> anyhow::Result<CommandPoolFreeListItem> {
        let allocator = unsafe {
            self.device
                .device
                .CreateCommandAllocator(list_type)
                .map_err(|v| anyhow!(v))?
        };

        let list = unsafe {
            self.device
                .device
                .CreateCommandList1(0, list_type, Default::default())
                .map_err(|v| anyhow!(v))?
        };

        Ok((allocator, list))
    }
}

// SAFETY: Correct access to the CommandAllocator is enforced with an atomic lock
unsafe impl Send for CommandPool {}
unsafe impl Sync for CommandPool {}

impl ICommandPool for CommandPool {
    fn upgrade(&self) -> AnyArc<dyn ICommandPool> {
        AnyArc::map::<dyn ICommandPool, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn create_command_list(&self) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let (allocator, list) = if let Some(v) = self.general_free_list.pop() {
            unsafe {
                v.0.Reset().map_err(|v| anyhow!(v))?;
                v
            }
        } else {
            self.new_list(D3D12_COMMAND_LIST_TYPE_DIRECT)?
        };

        let command_list = CommandList {
            pool: self.this.upgrade().unwrap(),
            list_type: QueueType::General,
            allocator,
            list,
        };
        Ok(Box::new(command_list))
    }
}

impl IGetPlatformInterface for CommandPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}
