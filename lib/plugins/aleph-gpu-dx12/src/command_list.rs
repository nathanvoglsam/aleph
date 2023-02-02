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
use crate::internal::in_flight_command_list::ReturnToPool;
use crate::internal::set_name::set_name;
use crate::internal::try_clone_value_into_slot;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    CommandListBeginError, ICommandList, IComputeEncoder, IGeneralEncoder, IGetPlatformInterface,
    INamedObject, ITransferEncoder, QueueType,
};
use std::any::TypeId;
use windows::Win32::Graphics::Direct3D12::*;

pub struct CommandList {
    pub(crate) pool: AnyArc<CommandPool>,
    pub(crate) list_type: QueueType,
    pub(crate) allocator: ID3D12CommandAllocator,
    pub(crate) list: ID3D12GraphicsCommandList7,
}

declare_interfaces!(CommandList, [ICommandList]);

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
                    .Reset(&self.allocator, None)
                    .map_err(|v| anyhow!(v))?;

                self.list.SetDescriptorHeaps(&self.pool.descriptor_heaps);
            }

            let encoder = Encoder::<'a> {
                list: self.list.clone(),
                _parent: self,
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
        self.pool
            .general_free_list
            .push((self.allocator.clone(), self.list.clone()))
    }
}

impl IGetPlatformInterface for CommandList {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        if try_clone_value_into_slot(&self.list, out, target).is_some() {
            return Some(());
        };
        if try_clone_value_into_slot(&self.allocator, out, target).is_some() {
            return Some(());
        };
        None
    }
}

impl INamedObject for CommandList {
    fn set_name(&self, name: &str) {
        set_name(&self.allocator, name).unwrap();
        set_name(&self.list, name).unwrap();
    }
}
