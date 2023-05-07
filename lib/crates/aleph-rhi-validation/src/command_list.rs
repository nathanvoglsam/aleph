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

use crate::{ValidationDevice, ValidationEncoder};
use interfaces::any::AnyArc;
use interfaces::gpu::*;

pub struct ValidationCommandList {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: Box<dyn ICommandList>,
    pub(crate) list_type: QueueType,
}

interfaces::any::declare_interfaces!(ValidationCommandList, [ICommandList]);

crate::impl_platform_interface_passthrough!(ValidationCommandList);

unsafe impl Send for ValidationCommandList {}

impl ICommandList for ValidationCommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_general()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
            list_type: QueueType::General,
            render_pass_open: false,
        });
        let encoder: Box<dyn IGeneralEncoder + 'a> = encoder;
        Ok(encoder)
    }

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_compute()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
            list_type: QueueType::Compute,
            render_pass_open: false,
        });
        let encoder: Box<dyn IComputeEncoder + 'a> = encoder;
        Ok(encoder)
    }

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_transfer()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
            list_type: QueueType::Transfer,
            render_pass_open: false,
        });
        let encoder: Box<dyn ITransferEncoder + 'a> = encoder;
        Ok(encoder)
    }
}
