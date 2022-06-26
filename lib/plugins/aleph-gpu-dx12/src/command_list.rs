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
use crate::internal::in_flight_command_list::ReturnToPool;
use dx12::D3D12Object;
use interfaces::any::{declare_interfaces, AnyArc, IAny};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    CommandListBeginError, ICommandList, IComputeEncoder, IGeneralEncoder, INamedObject,
    ITransferEncoder, QueueType,
};

pub struct CommandList {
    pub(crate) pool: AnyArc<CommandPool>,
    pub(crate) tracker: CommandListTracker,
    pub(crate) list_type: QueueType,
    pub(crate) allocator: dx12::CommandAllocator,
    pub(crate) list: dx12::GraphicsCommandList,
}

declare_interfaces!(CommandList, [ICommandList, ICommandListExt]);

unsafe impl Send for CommandList {}

impl ICommandList for CommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        if matches!(self.list_type, QueueType::General) {
            // Open the command list for recording with no bound pipeline so we can attach it to
            // the command allocator
            unsafe {
                self.list
                    .reset::<(), _>(&self.allocator, None)
                    .map_err(|v| anyhow!(v))?;
            }

            // Clear the resource tracker data. The list can't be in flight when we do this so it's all
            // good to do this here. If the tracker is tracking anything it's because of the command
            // list having already been recorded
            self.tracker.clear();

            let encoder = Encoder::<'a> {
                list: self.list.clone(),
                parent: self,
                bound_graphics_pipeline: None,
                input_binding_strides: [0; 16],
            };
            Ok(Box::new(encoder))
        } else {
            Err(CommandListBeginError::InvalidEncoderType(
                QueueType::General,
            ))
        }
    }

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError> {
        Err(CommandListBeginError::InvalidEncoderType(
            QueueType::Compute,
        ))
    }

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError> {
        Err(CommandListBeginError::InvalidEncoderType(
            QueueType::Transfer,
        ))
    }
}

impl ReturnToPool for CommandList {
    fn return_to_pool(&mut self) {
        self.tracker.images.clear();
        self.tracker.binding_sets.clear();
        self.tracker.buffers.clear();
        self.pool
            .general_free_list
            .push((self.allocator.clone(), self.list.clone()))
    }
}

pub trait ICommandListExt: IAny {
    fn get_raw_allocator(&self) -> dx12::CommandAllocator;

    fn get_raw_list(&self) -> dx12::GraphicsCommandList;
}

impl ICommandListExt for CommandList {
    fn get_raw_allocator(&self) -> dx12::CommandAllocator {
        self.allocator.clone()
    }

    fn get_raw_list(&self) -> dx12::GraphicsCommandList {
        self.list.clone()
    }
}

impl INamedObject for CommandList {
    fn set_name(&self, name: &str) {
        self.allocator.set_name(name).unwrap();
        self.list.set_name(name).unwrap();
    }
}
