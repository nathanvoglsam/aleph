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

use crate::{IRenderPass, RenderPassAccesses};

/// Generic implementation of [`IRenderPass`] based around closures
pub struct CallbackPass<
    T: Sized,
    D: FnOnce(&mut T, &mut RenderPassAccesses),
    R: Fn(&T, &mut dx12::GraphicsCommandList),
> {
    data: T,
    declare_access: Option<D>,
    record: R,
}

impl<T, D, R> CallbackPass<T, D, R>
where
    T: Sized,
    D: FnOnce(&mut T, &mut RenderPassAccesses),
    R: Fn(&T, &mut dx12::GraphicsCommandList),
{
    pub fn new(data: T, declare_access: D, record: R) -> Self {
        Self {
            data,
            declare_access: Some(declare_access),
            record,
        }
    }
}

impl<T, D, R> IRenderPass for CallbackPass<T, D, R>
where
    T: Sized,
    D: FnOnce(&mut T, &mut RenderPassAccesses),
    R: Fn(&T, &mut dx12::GraphicsCommandList),
{
    fn declare_access(&mut self, builder: &mut RenderPassAccesses) {
        (self.declare_access.take().unwrap())(&mut self.data, builder);
    }

    fn record(&self, command_list: &mut dx12::GraphicsCommandList) {
        (self.record)(&self.data, command_list);
    }
}
