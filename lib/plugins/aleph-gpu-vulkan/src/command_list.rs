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

use crate::device::Device;
use crate::encoder::Encoder;
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use bumpalo::Bump;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use std::any::TypeId;

pub struct CommandList {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) pool: vk::CommandPool,
    pub(crate) buffer: vk::CommandBuffer,
    pub(crate) list_type: QueueType,
}

declare_interfaces!(CommandList, [ICommandList]);

impl IGetPlatformInterface for CommandList {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        if try_clone_value_into_slot(&self.buffer, out, target).is_some() {
            return Some(());
        };
        if try_clone_value_into_slot(&self.pool, out, target).is_some() {
            return Some(());
        };
        None
    }
}

unsafe impl Send for CommandList {}

impl ICommandList for CommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        if matches!(self.list_type, QueueType::General) {
            // Open the command list for recording with no bound pipeline so we can attach it to
            // the command allocator
            unsafe {
                self._device
                    .device_loader
                    .reset_command_pool(self.pool, Default::default())
                    .map_err(|v| anyhow!(v))?;
            }

            let encoder = Encoder::<'a> {
                buffer: self.buffer.clone(),
                _parent: self,
                bound_graphics_pipeline: None,
                arena: Bump::with_capacity(1024 * 16),
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

impl Drop for CommandList {
    fn drop(&mut self) {
        unsafe {
            // The list is destroyed with the pool
            self._device
                .device_loader
                .destroy_command_pool(self.pool, None);
        }
    }
}