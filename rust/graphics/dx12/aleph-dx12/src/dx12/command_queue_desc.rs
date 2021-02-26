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

use crate::{CommandListType, CommandQueueFlags};
use raw::windows::win32::direct3d12::D3D12_COMMAND_QUEUE_DESC;
use std::mem::transmute;

pub struct CommandQueueDescBuilder {
    inner: CommandQueueDesc,
}

impl CommandQueueDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: CommandQueueDesc {
                queue_type: CommandListType::Direct,
                priority: 0,
                flags: CommandQueueFlags::NONE,
                node_mask: 0,
            },
        }
    }

    pub fn queue_type(mut self, queue_type: CommandListType) -> Self {
        self.inner.queue_type = queue_type;
        self
    }

    pub fn priority(mut self, priority: i32) -> Self {
        self.inner.priority = priority;
        self
    }

    pub fn flags(mut self, flags: CommandQueueFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    pub fn build(self) -> CommandQueueDesc {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct CommandQueueDesc {
    pub queue_type: CommandListType,
    pub priority: i32,
    pub flags: CommandQueueFlags,
    pub node_mask: u32,
}

impl CommandQueueDesc {
    pub fn builder() -> CommandQueueDescBuilder {
        CommandQueueDescBuilder::new()
    }
}

impl Into<D3D12_COMMAND_QUEUE_DESC> for CommandQueueDesc {
    fn into(self) -> D3D12_COMMAND_QUEUE_DESC {
        unsafe { transmute(self) }
    }
}
