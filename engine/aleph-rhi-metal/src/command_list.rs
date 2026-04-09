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
use std::sync::Arc;

use aleph_rhi_api::*;
use blink_alloc::Blink;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::device::Device;
use crate::encoder::{ActiveEncoder, Encoder, EncoderObjects};

pub struct CommandList {
    pub(crate) _device: Arc<Device>,
    pub(crate) list_type: QueueType,
    pub(crate) state: ListState,
    pub(crate) objects: CommandListObjects,
}

impl IGetPlatformInterface for CommandList {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

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
        if matches!(self.list_type, QueueType::Compute | QueueType::General) {
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
    pub(crate) fn create(
        device: &Device,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let queue = match device.get_queue_internal(desc.queue_type) {
            Some(v) => v,
            None => return Err(CommandListCreateError::NoSuchQueue(desc.queue_type)),
        };

        let list = match queue.objects.queue.commandBuffer() {
            Some(v) => v,
            None => return Err(CommandListCreateError::Platform),
        };

        if let Some(name) = desc.name {
            let mtl_name = NSString::from_str(name);
            list.setLabel(Some(&mtl_name));
        }

        let out: Box<dyn ICommandList> = Box::new(CommandList {
            _device: device.this.upgrade().unwrap(),
            list_type: desc.queue_type,
            state: ListState::Empty,
            objects: CommandListObjects { list },
        });

        Ok(out)
    }

    fn begin(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError> {
        match self.state {
            ListState::Empty => {
                self.state = ListState::Open;

                let _context = self._device.context.clone();
                let _device = self._device.clone();
                let list = self.objects.list.clone();
                let encoder = Encoder::<'_> {
                    _parent: self,
                    _context,
                    _device,
                    objects: EncoderObjects { list },
                    active: ActiveEncoder::None,
                    bound_graphics_pipeline: None,
                    bound_compute_pipeline: None,
                    bound_index_buffer: None,
                    bound_graphics_pipeline_state: Default::default(),
                    bound_compute_pipeline_state: Default::default(),
                    arena: Blink::new(),
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum ListState {
    Empty,
    Open,
    Closed,
}

/// Wrapper to limit the scope of our 'unsafe impl Send'
pub struct CommandListObjects {
    pub list: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
}

unsafe impl Send for CommandListObjects {}
