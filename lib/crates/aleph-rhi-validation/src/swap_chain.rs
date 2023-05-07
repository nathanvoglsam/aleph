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

use crate::internal::unwrap;
use crate::{ValidationDevice, ValidationSurface, ValidationTexture};
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::*;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ValidationSwapChain {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _surface: AnyArc<ValidationSurface>,
    pub(crate) inner: AnyArc<dyn ISwapChain>,
    pub(crate) queue_support: QueueType,
    pub(crate) textures: Mutex<Vec<AnyArc<dyn ITexture>>>,
    pub(crate) acquired: AtomicBool,
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

    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        // We create an array with space for as many swap chain textures as we'd ever need. This
        // avoids a heap alloc
        let mut scratch = Self::make_scratch();

        // Acquire the textures lock and grab the number of swap images. We need to know this number
        // for refilling the list after we clear it...
        let mut textures = self.textures.lock();
        let num_textures = textures.len();

        // Validate that we have exclusive ownership of the textures. This is an API requirement.
        for x in textures.iter() {
            assert!(
                x.weak_count() == 1 && x.strong_count() == 1,
                "It is invalid to resize a swap chain while still holding references to its images"
            );
        }

        // We have to clear the textures array to drop what should be the last remaining reference
        // to the swap images owned by something outside of the root RHI implementation.
        textures.clear();

        // We have to block and flush all GPU work before we rebuild to ensure that none of the
        // images can be in use on the GPU timeline.
        self._device.wait_idle();

        // Finally, we can actually do the real resize operation
        let result = self.inner.rebuild(new_size);

        // All swap images are immediately un-acquired after a rebuild
        self.acquired.store(false, Ordering::SeqCst);

        // We now need to re poll the images from the inner layer so we can hand out images
        // correctly in subsequent calls to 'get_images'
        let images = &mut scratch[0..num_textures];
        self.inner.get_images(images);

        // The returned images are from the inner implementation, wrap them in ValidationTexture.
        Self::wrap_images(self._device.as_ref(), &mut textures, images);

        result
    }

    fn get_images(&self, images: &mut [Option<AnyArc<dyn ITexture>>]) {
        let textures = self.textures.lock();

        for (out, v) in images.iter_mut().zip(textures.iter()) {
            *out = Some(v.clone());
        }
    }

    unsafe fn acquire_next_image(&self, desc: &AcquireDesc) -> Result<u32, ImageAcquireError> {
        if self
            .acquired
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("Attempted to acquire an image while one is already acquired");
        }

        let new_desc = AcquireDesc {
            signal_semaphore: unwrap::semaphore(desc.signal_semaphore).inner.as_ref(),
        };

        self.inner.acquire_next_image(&new_desc)
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

impl ValidationSwapChain {
    pub(crate) fn wrap_images(
        device: &ValidationDevice,
        textures: &mut Vec<AnyArc<dyn ITexture>>,
        images: &mut [Option<AnyArc<dyn ITexture>>],
    ) {
        // The returned images are from the inner implementation, wrap them in ValidationTexture.
        for image in images {
            let image = image.take().unwrap();
            let image = AnyArc::new_cyclic(move |v| ValidationTexture {
                _this: v.clone(),
                _device: device._this.upgrade().unwrap(),
                inner: image,
                views: Default::default(),
                rtvs: Default::default(),
                dsvs: Default::default(),
            });
            let image = AnyArc::map::<dyn ITexture, _>(image, |v| v);

            textures.push(image);
        }
    }

    pub(crate) fn make_scratch() -> [Option<AnyArc<dyn ITexture>>; 16] {
        [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ]
    }
}
