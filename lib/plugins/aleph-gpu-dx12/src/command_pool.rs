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

use crossbeam::queue::SegQueue;
use interfaces::anyhow;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{CommandListCreateError, ICommandPool, IGeneralCommandList, INamedObject};

use crate::device::Device;
use crate::general_command_list::GeneralCommandList;
use interfaces::ref_ptr::{ref_ptr_object, RefPtr, RefPtrObject};

ref_ptr_object! {
    pub struct CommandPool: ICommandPool {
        pub(crate) device: RefPtr<Device>,
        pub(crate) general_free_list: SegQueue<CommandPoolFreeListItem>,
        pub(crate) compute_free_list: SegQueue<CommandPoolFreeListItem>,
        pub(crate) transfer_free_list: SegQueue<CommandPoolFreeListItem>,
    }
}

pub type CommandPoolFreeListItem = (dx12::CommandAllocator, dx12::GraphicsCommandList);

impl CommandPool {
    fn new_list(
        &self,
        list_type: dx12::CommandListType,
    ) -> anyhow::Result<CommandPoolFreeListItem> {
        let allocator = self
            .device
            .device
            .create_command_allocator(list_type)
            .map_err(|v| anyhow!(v))?;

        let list = self
            .device
            .device
            .create_graphics_command_list(list_type)
            .map_err(|v| anyhow!(v))?;

        Ok((allocator, list))
    }
}

// SAFETY: Correct access to the CommandAllocator is enforced with an atomic lock
unsafe impl Send for CommandPool {}
unsafe impl Sync for CommandPool {}

impl ICommandPool for CommandPool {
    fn create_general_command_list(
        &self,
    ) -> Result<Box<dyn IGeneralCommandList>, CommandListCreateError> {
        let (allocator, list) = if let Some(v) = self.general_free_list.pop() {
            unsafe {
                v.0.reset().map_err(|v| anyhow!(v))?;
                v
            }
        } else {
            self.new_list(dx12::CommandListType::Direct)?
        };

        let command_list = GeneralCommandList {
            pool: self.as_ref_ptr(),
            tracker: Default::default(),
            allocator,
            list,
        };
        Ok(Box::new(command_list))
    }
}

impl INamedObject for CommandPool {
    fn set_name(&self, _name: &str) {
        // No matching object to name
    }
}
