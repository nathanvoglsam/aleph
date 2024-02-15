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

use std::mem::size_of_val;
use std::ptr::NonNull;

use aleph_rhi_api::*;

use crate::{FrameGraphResources, Payload};

pub trait IRenderPass: Send + 'static {
    fn execute(&mut self, encoder: &mut dyn IGeneralEncoder, resources: &FrameGraphResources);
}

pub(crate) struct CallbackRenderPass<
    T: Send + 'static,
    ExecFn: FnMut(Option<&T>, &mut dyn IGeneralEncoder, &FrameGraphResources) + Send + 'static,
> {
    /// A type-erased pointer to the payload object of type 'T'.
    payload: NonNull<Payload<T>>,

    /// The function that will be called on execute
    exec_fn: ExecFn,
}

impl<T, ExecFn> CallbackRenderPass<T, ExecFn>
where
    T: Send + 'static,
    ExecFn: FnMut(Option<&T>, &mut dyn IGeneralEncoder, &FrameGraphResources) + Send + 'static,
{
    pub fn new(payload: NonNull<Payload<T>>, exec_fn: ExecFn) -> Self {
        assert!(
            size_of_val(&exec_fn) < 1024,
            "Size limit for ExecFn closure exceeded"
        );
        Self {
            payload: payload.cast(),
            exec_fn,
        }
    }
}

impl<T, ExecFn> IRenderPass for CallbackRenderPass<T, ExecFn>
where
    T: Send + 'static,
    ExecFn: FnMut(Option<&T>, &mut dyn IGeneralEncoder, &FrameGraphResources) + Send + 'static,
{
    fn execute(&mut self, encoder: &mut dyn IGeneralEncoder, resources: &FrameGraphResources) {
        // Safety: It is the responsibility of the frame graph implementation to ensure that this
        //         is safe to do. So, it's the responsibility of whoever constructs the callback
        //         pass.
        //
        //         We require sound access to the payload struct. This shouldn't be too hard as we
        //         can simply store it in the arena and give the callback pass the only pointer
        //         to it, making it safe to access here.
        let payload = unsafe { self.payload.as_ref() };
        let payload = if payload.written {
            let r = unsafe { payload.payload.assume_init_ref() };
            Some(r)
        } else {
            None
        };
        (self.exec_fn)(payload, encoder, resources)
    }
}

unsafe impl<T, ExecFn> Send for CallbackRenderPass<T, ExecFn>
where
    T: Send + 'static,
    ExecFn: FnMut(Option<&T>, &mut dyn IGeneralEncoder, &FrameGraphResources) + Send + 'static,
{
}
