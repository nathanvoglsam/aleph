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

use crate::device::Device;
use crate::surface::Surface;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{
    AcquireImageError, INamedObject, ISwapChain, ITexture, QueueType, SwapChainConfiguration,
};

pub struct SwapChain {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _surface: AnyArc<Surface>,
    pub(crate) inner: AnyArc<dyn ISwapChain>,
    pub(crate) queue_support: QueueType,
}

crate::validation_declare_interfaces!(SwapChain, [ISwapChain]);

impl ISwapChain for SwapChain {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain> {
        AnyArc::map::<dyn ISwapChain, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn present_supported_on_queue(&self, queue: QueueType) -> bool {
        self.inner.present_supported_on_queue(queue)
    }

    fn get_config(&self) -> SwapChainConfiguration {
        self.inner.get_config()
    }

    fn queue_resize(&self, width: u32, height: u32) {
        self.inner.queue_resize(width, height)
    }

    unsafe fn acquire_image(&self) -> Result<AnyArc<dyn ITexture>, AcquireImageError> {
        self.inner.acquire_image()
    }

    fn get_current_image(&self) -> Option<AnyArc<dyn ITexture>> {
        self.inner.get_current_image()
    }
}

impl INamedObject for SwapChain {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
