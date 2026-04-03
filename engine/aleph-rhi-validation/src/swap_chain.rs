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

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak};

use aleph_object_system::{ArcObject, Object};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::map_acquired_image;

use crate::{ValidationDevice, ValidationSurface, ValidationSwapImage, ValidationTexture};

pub struct ValidationSwapChain {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Arc<ValidationDevice>,
    pub(crate) _surface: Arc<ValidationSurface>,
    pub(crate) inner: Arc<dyn ISwapChain>,
    pub(crate) queue_support: QueueType,
    pub(crate) acquired: AtomicBool,
}

crate::impl_platform_interface_passthrough!(ValidationSwapChain);

impl ISwapChain for ValidationSwapChain {
    fn upgrade(&self) -> Arc<dyn ISwapChain> {
        self._this.upgrade().unwrap()
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

    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        // We have to block and flush all GPU work before we rebuild to ensure that none of the
        // images can be in use on the GPU timeline.
        match self._device.wait_idle() {
            Ok(_) => {}
            Err(QueueWaitError::DeviceLost) => return Err(SwapChainRebuildError::DeviceLost),
            Err(QueueWaitError::Platform) => return Err(SwapChainRebuildError::Platform),
        }

        // Finally, we can actually do the real resize operation
        let result = self.inner.rebuild(new_size);

        // All swap images are immediately un-acquired after a rebuild
        self.acquired.store(false, Ordering::SeqCst);

        result
    }

    unsafe fn acquire_next_image(&self) -> Result<AcquiredImage, ImageAcquireError> {
        if self
            .acquired
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("Attempted to acquire an image while one is already acquired");
        }

        let inner = unsafe { self.inner.acquire_next_image()? };
        let acquired = map_acquired_image(inner, |swap_image| {
            let texture = Arc::new_cyclic(|v| {
                let inner = swap_image.texture().clone();
                let desc = self._device.inner.get_texture_desc(&inner);
                let desc = desc.clone().strip_name();
                Object::new(ValidationTexture {
                    _this: v.clone(),
                    _device: self._device.clone(),
                    inner,
                    desc,
                    views: Default::default(),
                    rtvs: Default::default(),
                    dsvs: Default::default(),
                })
            });
            let texture = ArcObject::from_object(texture);
            let texture = unsafe { TextureHandle::new(texture) };

            let swap_image = Arc::new(ValidationSwapImage {
                _swap_chain: self._this.upgrade().unwrap(),
                inner: Some(swap_image),
                texture: Some(texture),
            });
            swap_image
        });
        Ok(acquired)
    }
}

impl Drop for ValidationSwapChain {
    fn drop(&mut self) {
        let mut has_swap_chain = self._surface.has_swap_chain.lock();

        // Release the surface as the swap chain no longer owns it
        assert!(*has_swap_chain);
        *has_swap_chain = false;
    }
}
