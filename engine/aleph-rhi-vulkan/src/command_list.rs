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

use aleph_alloc::{Blink, BlinkAlloc};
use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::{RhiSystem, try_clone_value_into_slot};
use ash::vk;

use crate::device::Device;
use crate::encoder::Encoder;
use crate::internal::allocation_callbacks::GLOBAL;

pub struct CommandList {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) pool: vk::CommandPool,
    pub(crate) buffer: vk::CommandBuffer,
    pub(crate) list_type: QueueType,
    pub(crate) state: ListState,
}

declare_interfaces!(CommandList, [ICommandList]);

impl IGetPlatformInterface for CommandList {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe {
            if try_clone_value_into_slot(&self.buffer, out, target).is_some() {
                return Some(());
            };
            if try_clone_value_into_slot(&self.pool, out, target).is_some() {
                return Some(());
            };
            None
        }
    }
}

unsafe impl Send for CommandList {}

impl ICommandList for CommandList {
    fn begin_general(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError> {
        if matches!(self.list_type, QueueType::General) {
            self.begin()
        } else {
            Err(CommandListBeginError::InvalidEncoderType(
                QueueType::General,
            ))
        }
    }

    fn begin_compute(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError> {
        if matches!(self.list_type, QueueType::General | QueueType::Compute) {
            self.begin()
        } else {
            Err(CommandListBeginError::InvalidEncoderType(
                QueueType::Compute,
            ))
        }
    }

    fn begin_transfer(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError> {
        self.begin()
    }
}

impl CommandList {
    fn begin(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError> {
        match self.state {
            ListState::Empty => {
                // Open the command list for recording with no bound pipeline so we can attach it to
                // the command allocator
                unsafe {
                    self._device
                        .device
                        .reset_command_pool(self.pool, Default::default())
                        .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                        .map_err(|_| CommandListBeginError::Platform)?;

                    let begin_info = vk::CommandBufferBeginInfo::default()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
                    self._device
                        .device
                        .begin_command_buffer(self.buffer, &begin_info)
                        .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                        .map_err(|_| CommandListBeginError::Platform)?;
                }

                self.state = ListState::Open;

                let _buffer = self.buffer;
                let _context = self._device.context.clone();
                let _device = self._device.clone();
                let encoder = Encoder::<'_> {
                    _parent: self,
                    _buffer,
                    _context,
                    _device,
                    bound_graphics_pipeline: None,
                    bound_compute_pipeline: None,
                    arena: Blink::new_in(BlinkAlloc::new_in(RhiSystem::default())),
                };
                let encoder = Box::new(encoder);

                // Safety: This isn't unsound/unsafe
                unsafe { Ok(CommandEncoder::from_abi(encoder)) }
            }
            ListState::Open => Err(CommandListBeginError::InvalidCommandListState),
            ListState::Closed => Err(CommandListBeginError::InvalidCommandListState),
        }
    }
}

impl Drop for CommandList {
    fn drop(&mut self) {
        unsafe {
            if self.pool == vk::CommandPool::null() {
                // The list is destroyed with the pool
                self._device.device.destroy_command_pool(self.pool, GLOBAL);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum ListState {
    Empty,
    Open,
    Closed,
}
