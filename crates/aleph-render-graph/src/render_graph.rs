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

use crate::{IRenderPass, RenderGraphBuilder};
use std::collections::HashSet;

pub struct RenderGraph<'passes> {
    pub(crate) pass_storages: Vec<RenderPass<'passes>>,
    pub(crate) pass_entry_barriers: Vec<Vec<dx12::ResourceBarrier>>,
    pub(crate) pass_exit_barriers: Vec<Vec<dx12::ResourceBarrier>>,
    pub(crate) recording_order: Vec<usize>,
    pub(crate) final_barriers: Vec<dx12::ResourceBarrier>,
}

impl<'passes> RenderGraph<'passes> {
    pub fn builder() -> RenderGraphBuilder<'passes> {
        RenderGraphBuilder::new()
    }

    pub fn record(&mut self, command_list: &mut dx12::GraphicsCommandList) {
        for i in self.recording_order.iter().copied() {
            unsafe {
                command_list.resource_barrier_dynamic(self.pass_entry_barriers[i].iter());
            }

            self.pass_storages[i].pass.record(command_list);

            unsafe {
                command_list.resource_barrier_dynamic(self.pass_exit_barriers[i].iter());
            }
        }

        unsafe {
            command_list.resource_barrier_dynamic(self.final_barriers.iter());
        }
    }
}

///
/// Internal struct for storing a render pass with its execution dependencies
///
pub(crate) struct RenderPass<'passes> {
    pub pass: Box<dyn IRenderPass + 'passes>,
    pub predecessors: HashSet<usize>,
    pub successors: HashSet<usize>,
}
