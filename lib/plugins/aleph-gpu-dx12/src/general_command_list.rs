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

use crate::command_pool::CommandPool;
use crate::encoder::Encoder;
use crate::internal::command_list_tracker::CommandListTracker;
use dx12::D3D12Object;
use interfaces::any::{declare_interfaces, IAny};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{CommandListBeginError, IGeneralCommandList, IGeneralEncoder, INamedObject};
use interfaces::ref_ptr::RefPtr;

pub struct GeneralCommandList {
    pub(crate) pool: RefPtr<CommandPool>,
    pub(crate) allocator: dx12::CommandAllocator,
    pub(crate) list: dx12::GraphicsCommandList,
}

declare_interfaces!(GeneralCommandList, [IGeneralCommandList, ICommandListExt]);

unsafe impl Send for GeneralCommandList {}

impl IGeneralCommandList for GeneralCommandList {
    fn begin<'a>(&'a mut self) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        // Open the command list for recording with no bound pipeline so we can attach it to
        // the command allocator
        unsafe {
            self.list
                .reset::<(), _>(&self.allocator, None)
                .map_err(|v| anyhow!(v))?;
        }

        let encoder = Encoder::<'a> {
            list: self.list.clone(),
            tracker: CommandListTracker {
                images: Vec::new(),
                buffers: Vec::new(),
                binding_sets: Vec::new(),
            },
            _phantom: Default::default(),
        };
        Ok(Box::new(encoder))
    }
}

pub trait ICommandListExt: IAny {
    fn get_raw_allocator(&self) -> dx12::CommandAllocator;

    fn get_raw_list(&self) -> dx12::GraphicsCommandList;
}

impl ICommandListExt for GeneralCommandList {
    fn get_raw_allocator(&self) -> dx12::CommandAllocator {
        self.allocator.clone()
    }

    fn get_raw_list(&self) -> dx12::GraphicsCommandList {
        self.list.clone()
    }
}

impl INamedObject for GeneralCommandList {
    fn set_name(&self, name: &str) {
        self.allocator.set_name(name).unwrap();
        self.list.set_name(name).unwrap();
    }
}
