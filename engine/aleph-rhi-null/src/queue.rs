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

use std::sync::{Arc, Weak};

use aleph_rhi_api::*;

use crate::NullDevice;

pub struct NullQueue {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Weak<NullDevice>,
}

crate::impl_platform_interface_passthrough!(NullQueue);

impl IQueue for NullQueue {
    fn upgrade(&self) -> Arc<dyn IQueue> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn queue_properties(&self) -> QueueProperties {
        QueueProperties {
            min_image_transfer_granularity: Default::default(),
        }
    }

    fn garbage_collect(&self) -> Result<(), QueueGarbageCollectError> {
        Ok(())
    }

    fn wait_idle(&self) -> Result<(), QueueWaitError> {
        Ok(())
    }

    unsafe fn submit(&self, _desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError> {
        Ok(())
    }

    unsafe fn present(&self, _swap_image: Arc<dyn ISwapImage>) -> Result<(), QueuePresentError> {
        Ok(())
    }
}
