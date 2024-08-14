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

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;

use crate::{NullDevice, NullEncoder};

pub struct NullCommandList {
    pub(crate) _device: AnyArc<NullDevice>,
}

declare_interfaces!(NullCommandList, [ICommandList]);

crate::impl_platform_interface_passthrough!(NullCommandList);

unsafe impl Send for NullCommandList {}

impl ICommandList for NullCommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        let encoder = Box::new(NullEncoder {});
        let encoder: Box<dyn IGeneralEncoder + 'a> = encoder;
        Ok(encoder)
    }

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError> {
        let encoder = Box::new(NullEncoder {});
        let encoder: Box<dyn IComputeEncoder + 'a> = encoder;
        Ok(encoder)
    }

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError> {
        let encoder = Box::new(NullEncoder {});
        let encoder: Box<dyn ITransferEncoder + 'a> = encoder;
        Ok(encoder)
    }
}
