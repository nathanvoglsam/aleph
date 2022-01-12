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
use crate::render_graph::{IRenderPass, RenderPassAccesses};

/// Generic implementation of [`IRenderPass`] based around closures
pub struct CallbackPass<
    T: Sized,
    D: FnOnce(&mut RenderPassAccesses) -> T,
    R: Fn(&mut T, CommandBuffer<Open>),
> {
    data: Option<T>,
    declare_access: Option<D>,
    record: R,
}

impl<T, D, R> CallbackPass<T, D, R>
where
    T: Sized,
    D: FnOnce(&mut RenderPassAccesses) -> T,
    R: Fn(&mut T, CommandBuffer<Open>),
{
    pub fn new(declare_access: D, record: R) -> Self {
        Self {
            data: None,
            declare_access: Some(declare_access),
            record,
        }
    }
}

impl<T, D, R> IRenderPass for CallbackPass<T, D, R>
where
    T: Sized,
    D: FnOnce(&mut RenderPassAccesses) -> T,
    R: Fn(&mut T, CommandBuffer<Open>),
{
    fn declare_access(&mut self, builder: &mut RenderPassAccesses) {
        let result = (self.declare_access.take().unwrap())(builder);
        self.data = Some(result);
    }

    fn record(&mut self, command_list: CommandBuffer<Open>) {
        (self.record)(self.data.as_mut().unwrap(), command_list);
    }
}
