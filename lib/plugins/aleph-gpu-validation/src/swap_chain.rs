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
use crate::surface::ValidationSurface;
use crate::ValidationTexture;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{AcquireImageError, ISwapChain, ITexture, QueueType, SwapChainConfiguration};
use parking_lot::Mutex;
use std::ops::DerefMut;

pub struct ValidationSwapChain {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _surface: AnyArc<ValidationSurface>,
    pub(crate) inner: AnyArc<dyn ISwapChain>,
    pub(crate) queue_support: QueueType,
    pub(crate) current_image: Mutex<Option<AnyArc<ValidationTexture>>>,
}

interfaces::any::declare_interfaces!(ValidationSwapChain, [ISwapChain]);

crate::impl_platform_interface_passthrough!(ValidationSwapChain);

impl ISwapChain for ValidationSwapChain {
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
        // Acquire and wrap the inner image
        let inner = self.inner.acquire_image()?;
        let image = AnyArc::new_cyclic(move |v| ValidationTexture {
            _this: v.clone(),
            _device: self._device.clone(),
            inner,
        });

        // Cache the newly fetched image so we can hand the object out from 'get_current_image'
        let mut lock = self.current_image.lock();
        let current = lock.deref_mut();
        *current = Some(image.clone());

        Ok(AnyArc::map::<dyn ITexture, _>(image, |v| v))
    }

    fn get_current_image(&self) -> Option<AnyArc<dyn ITexture>> {
        let image = self.current_image.lock().clone();
        image.map(|v| AnyArc::map::<dyn ITexture, _>(v, |v| v))
    }
}
