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

use crate::{BufferCreateDesc, RenderTargetCreateDesc, ResourceAccessDesc, ResourceCreateDesc};
use std::collections::HashMap;
use std::convert::Into;

pub trait IRenderPass {
    fn declare_access(&mut self, builder: &mut RenderPassAccesses);

    fn record(&self, command_list: &mut dx12::GraphicsCommandList);
}

#[derive(Default)]
pub struct RenderPassAccesses {
    pub(crate) creates: HashMap<String, ResourceCreateDesc>,
    pub(crate) reads: HashMap<String, ResourceAccessDesc>,
    pub(crate) writes: HashMap<String, ResourceWrite>,
}

impl RenderPassAccesses {
    /// Declare for the current pass that we wish to create a new transient render target with the
    /// provided name and description
    pub fn create_render_target(&mut self, name: impl Into<String>, desc: RenderTargetCreateDesc) {
        assert!(self.creates.insert(name.into(), desc.into()).is_none())
    }

    /// Declare for the current pass that we wish to create a new transient buffer with the provided
    /// name and description
    pub fn create_buffer(&mut self, name: impl Into<String>, desc: BufferCreateDesc) {
        assert!(self.creates.insert(name.into(), desc.into()).is_none())
    }

    /// Declare that the current pass would like to read the resource with the name `input` with
    /// the provided access description
    pub fn read(&mut self, input: impl Into<String>, access: ResourceAccessDesc) {
        assert!(self.reads.insert(input.into(), access).is_none());
    }

    /// Declare that the current pass would like to perform a write operation on `source` and
    /// produce a new handle `result` that refers to the result of the pass's write operation. The
    /// `access` argument provides how the resource will be used.
    pub fn write(
        &mut self,
        source: impl Into<String>,
        result: impl Into<String>,
        access: ResourceAccessDesc,
    ) {
        assert!(self
            .writes
            .insert(
                source.into(),
                ResourceWrite {
                    result: result.into(),
                    access,
                }
            )
            .is_none());
    }
}

pub(crate) struct ResourceWrite {
    pub(crate) result: String,
    pub(crate) access: ResourceAccessDesc,
}
