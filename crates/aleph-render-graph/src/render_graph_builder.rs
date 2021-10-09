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

use crate::{IRenderPass, RenderGraph, RenderPassAccesses};
use std::collections::HashMap;

pub struct RenderGraphBuilder<'passes> {
    pass_names: HashMap<String, usize>,
    pass_storage: Vec<Box<dyn IRenderPass + 'passes>>,
    pass_accesses: Vec<RenderPassAccesses>,
    inputs: HashMap<String, ()>,
    outputs: HashMap<String, ()>,
}

impl<'passes> RenderGraphBuilder<'passes> {
    pub fn new() -> Self {
        Self {
            pass_names: HashMap::with_capacity(32),
            pass_storage: Vec::with_capacity(32),
            pass_accesses: Vec::with_capacity(32),
            inputs: HashMap::with_capacity(4),
            outputs: HashMap::with_capacity(4),
        }
    }

    pub fn add_pass<T: IRenderPass + 'passes>(
        &mut self,
        name: impl Into<String>,
        pass: impl Into<T>,
    ) {
        // Capture the current pass index
        let index = self.pass_storage.len();

        // Box and push the pass
        self.pass_storage.push(Box::new(pass.into()));
        self.pass_accesses.push(RenderPassAccesses::default());

        // Insert the name mapping
        assert!(self.pass_names.insert(name.into(), index).is_none());
    }

    pub fn input_resource(&mut self, name: impl Into<String>) {
        assert!(self.inputs.insert(name.into(), ()).is_none())
    }

    pub fn output_resource(&mut self, name: impl Into<String>) {
        assert!(self.outputs.insert(name.into(), ()).is_none())
    }

    pub fn build(mut self) -> RenderGraph<'passes> {
        self.pass_storage
            .iter_mut()
            .zip(self.pass_accesses.iter_mut())
            .for_each(|v| {
                v.0.declare_access(v.1);
            });

        RenderGraph {
            pass_storages: self.pass_storage,
            pass_entry_barriers: todo!(),
            pass_exit_barriers: todo!(),
            recording_order: todo!(),
            final_barriers: todo!(),
        }
    }
}
