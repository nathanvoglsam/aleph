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

use crate::command_pool::ValidationCommandPool;
use crate::encoder::ValidationEncoder;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::gpu::{
    CommandListBeginError, ICommandList, IComputeEncoder, IGeneralEncoder, INamedObject,
    ITransferEncoder, QueueType,
};

pub struct ValidationCommandList {
    pub(crate) _pool: AnyArc<ValidationCommandPool>,
    pub(crate) inner: Box<dyn ICommandList>,
    pub(crate) list_type: QueueType,
}

declare_interfaces!(ValidationCommandList, [ICommandList]);

unsafe impl Send for ValidationCommandList {}

impl ICommandList for ValidationCommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_general()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
        });
        Ok(encoder)
    }

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_general()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
        });
        Ok(encoder)
    }

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError> {
        let inner = self.inner.begin_general()?;
        let encoder = Box::new(ValidationEncoder {
            inner,
            bound_graphics_pipeline: None,
        });
        Ok(encoder)
    }
}

impl INamedObject for ValidationCommandList {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
