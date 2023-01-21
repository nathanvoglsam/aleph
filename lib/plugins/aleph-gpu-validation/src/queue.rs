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

use crate::{ValidationCommandList, ValidationSwapChain};
use interfaces::any::{box_downcast, AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::{
    Color, ICommandList, INamedObject, IQueue, ISwapChain, QueuePresentError, QueueProperties,
    QueueSubmitError, QueueType,
};

pub struct ValidationQueue {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) inner: AnyArc<dyn IQueue>,
    pub(crate) queue_type: QueueType,
}

crate::validation_declare_interfaces!(ValidationQueue, [IQueue]);

impl IQueue for ValidationQueue {
    fn upgrade(&self) -> AnyArc<dyn IQueue> {
        AnyArc::map::<dyn IQueue, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        self.inner.queue_properties()
    }

    unsafe fn submit_list(
        &self,
        command_list: Box<dyn ICommandList>,
    ) -> Result<(), QueueSubmitError> {
        let list = box_downcast::<_, ValidationCommandList>(command_list)
            .ok()
            .expect("Unknown ICommandList implementation")
            .inner;

        self.inner.submit_list(list)
    }

    unsafe fn submit_lists(
        &self,
        command_lists: &mut dyn Iterator<Item = Box<dyn ICommandList>>,
    ) -> Result<(), QueueSubmitError> {
        let mut command_lists = command_lists.map(|v| {
            box_downcast::<_, ValidationCommandList>(v)
                .ok()
                .expect("Unknown ICommandList implementation")
                .inner
        });

        self.inner.submit_lists(&mut command_lists)
    }

    unsafe fn present(&self, swap_chain: &dyn ISwapChain) -> Result<(), QueuePresentError> {
        let swap_chain = swap_chain
            .query_interface::<ValidationSwapChain>()
            .expect("Unknown ISwapChain implementation");

        self.inner.present(swap_chain.inner.as_ref())
    }

    unsafe fn set_marker(&mut self, _color: Color, _message: &str) {
        unimplemented!();
        // self.inner.set_marker(color, message);
    }

    unsafe fn begin_event(&mut self, _color: Color, _message: &str) {
        unimplemented!();
        // self.inner.begin_event(color, message);
    }

    unsafe fn end_event(&mut self) {
        unimplemented!();
        // self.inner.end_event();
    }
}

impl INamedObject for ValidationQueue {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
