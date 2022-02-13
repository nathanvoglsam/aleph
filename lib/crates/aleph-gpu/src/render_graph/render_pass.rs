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

use crate::command_buffer::{CommandBuffer, Open};
use crate::render_graph::{ResourceCreateDesc, ResourceReadDesc, ResourceSlot, ResourceWriteDesc};
use crossbeam::atomic::AtomicCell;
use std::collections::HashMap;
use std::convert::Into;
use std::sync::Arc;

/// The generic interface expected of a render pass.
pub trait IRenderPass {
    /// This will be called exactly once during the graph construction phase to declare what
    /// resources the pass will access.
    ///
    /// This information is used to bake resource barriers that the graph will automatically record
    /// when recording commands.
    fn declare_access(&mut self, builder: &mut RenderPassAccesses);

    /// This will be called once for every time the owning [`crate::RenderGraph`] has its
    /// [`crate::RenderGraph::record`] function called. Each pass will be recorded single-threaded
    /// onto a single command list.
    ///
    /// Parallel command recording is up to the [`IRenderPass`] implementation to perform using
    /// bundles. Command list submission must be handled outside of the graph.
    fn record(&mut self, command_list: CommandBuffer<Open>);
}

/// This object is used by [`IRenderPass`] implementations to record resource accesses for the pass.
#[derive(Default)]
pub struct RenderPassAccesses {
    pub(crate) creates: HashMap<String, ResourceCreateDesc>,
    pub(crate) reads: HashMap<String, ResourceReadDesc>,
    pub(crate) writes: HashMap<String, ResourceWrite>,
}

impl RenderPassAccesses {
    /// Declare for the current pass that we wish to create a new transient resource with the
    /// provided name and description
    #[inline]
    pub fn create_resource(
        &mut self,
        name: impl Into<String>,
        desc: impl Into<ResourceCreateDesc>,
    ) -> ResourceSlot {
        assert!(self.creates.insert(name.into(), desc.into()).is_none());
        ResourceSlot {
            inner: Arc::new(AtomicCell::new(None)),
        }
    }

    /// Declare that the current pass would like to read the resource with the name `input` with
    /// the provided access description
    #[inline]
    pub fn read_resource(
        &mut self,
        input: impl Into<String>,
        access: impl Into<ResourceReadDesc>,
    ) -> ResourceSlot {
        assert!(self.reads.insert(input.into(), access.into()).is_none());
        ResourceSlot {
            inner: Arc::new(AtomicCell::new(None)),
        }
    }

    /// Declare that the current pass would like to perform a write operation on `source` and
    /// produce a new handle `result` that refers to the result of the pass's write operation. The
    /// `access` argument provides how the resource will be used.
    #[inline]
    pub fn write_resource(
        &mut self,
        source: impl Into<String>,
        result: impl Into<String>,
        access: impl Into<ResourceWriteDesc>,
    ) -> ResourceSlot {
        let source = source.into();
        let result = result.into();
        let access = access.into();
        assert_ne!(source, result);
        assert!(self
            .writes
            .insert(source, ResourceWrite { result, access })
            .is_none());
        ResourceSlot {
            inner: Arc::new(AtomicCell::new(None)),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ResourceWrite {
    pub(crate) result: String,
    pub(crate) access: ResourceWriteDesc,
}