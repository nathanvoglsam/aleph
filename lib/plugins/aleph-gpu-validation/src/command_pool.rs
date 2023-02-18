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

use crate::device::ValidationDevice;
use crate::ValidationCommandList;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{CommandListCreateError, ICommandList, ICommandPool, QueueType};

pub struct ValidationCommandPool {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn ICommandPool>,
}

interfaces::any::declare_interfaces!(ValidationCommandPool, [ICommandPool]);

crate::impl_platform_interface_passthrough!(ValidationCommandPool);

impl ICommandPool for ValidationCommandPool {
    fn upgrade(&self) -> AnyArc<dyn ICommandPool> {
        AnyArc::map::<dyn ICommandPool, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn create_command_list(&self) -> Result<Box<dyn ICommandList>, CommandListCreateError> {
        let inner = self.inner.create_command_list()?;
        let list = Box::new(ValidationCommandList {
            _pool: self._this.upgrade().unwrap(),
            inner,
            list_type: QueueType::General,
        });
        Ok(list)
    }
}
