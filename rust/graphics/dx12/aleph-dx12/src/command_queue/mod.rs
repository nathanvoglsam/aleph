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

use crate::command_list_type::CommandListType;
use crate::raw::windows::win32::direct3d12::{
    ID3D12CommandQueue, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAGS,
};
use crate::raw::windows::{Abi, Interface};
use crate::Device;

pub struct CommandQueueBuilder {
    priority: i32,
    queue_type: Option<CommandListType>,
    flags: D3D12_COMMAND_QUEUE_FLAGS,
}

impl CommandQueueBuilder {
    pub fn new() -> Self {
        Self {
            priority: 0,
            queue_type: None,
            flags: Default::default(),
        }
    }

    pub fn queue_type(mut self, queue_type: CommandListType) -> Self {
        self.queue_type = Some(queue_type);
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn disable_gpu_timeout(mut self) -> Self {
        self.flags.0 |= D3D12_COMMAND_QUEUE_FLAGS::D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT.0;
        self
    }

    pub unsafe fn build(self, device: &Device) -> raw::windows::Result<CommandQueue> {
        let desc = D3D12_COMMAND_QUEUE_DESC {
            r#type: self.queue_type.unwrap().into(),
            priority: self.priority,
            flags: self.flags,
            node_mask: 0,
        };
        let mut queue: Option<ID3D12CommandQueue> = None;
        device
            .raw()
            .CreateCommandQueue(&desc, &ID3D12CommandQueue::IID, queue.set_abi())
            .and_some(queue)
            .map(|v| CommandQueue { queue: v })
    }
}

pub struct CommandQueue {
    queue: ID3D12CommandQueue,
}

impl CommandQueue {
    pub fn builder() -> CommandQueueBuilder {
        CommandQueueBuilder::new()
    }

    pub fn raw(&self) -> &ID3D12CommandQueue {
        &self.queue
    }
}
