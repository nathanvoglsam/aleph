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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use aleph_rhi_api::*;
use parking_lot::Mutex;

use crate::{ValidationContext, ValidationDevice, ValidationSwapChain};

pub struct ValidationSurface {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<ValidationContext>,
    pub(crate) inner: AnyArc<dyn ISurface>,
    pub(crate) has_swap_chain: Mutex<bool>,
}

declare_interfaces!(ValidationSurface, [ISurface]);

crate::impl_platform_interface_passthrough!(ValidationSurface);

impl ISurface for ValidationSurface {
    fn upgrade(&self) -> AnyArc<dyn ISurface> {
        AnyArc::map::<dyn ISurface, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError> {
        let device = device
            .query_interface::<ValidationDevice>()
            .expect("Unknown IDevice implementation");
        let inner_device = device.inner.as_ref();

        let inner = {
            // Check if a swapchain that owns this surface already exists
            let mut has_swap_chain = self.has_swap_chain.lock();
            if *has_swap_chain {
                return Err(SwapChainCreateError::SurfaceAlreadyOwned);
            }

            let result = self.inner.create_swap_chain(inner_device, config);

            // Update the owned flag if we have successfully created a new swap chain
            if result.is_ok() {
                *has_swap_chain = true
            }

            result
        }?;

        // Prefill the 'textures' array in the swap chain.
        let inner_config = inner.get_config();
        let mut scratch = ValidationSwapChain::make_scratch();
        let images = &mut scratch[0..inner_config.buffer_count as usize];
        inner.get_images(images);
        let mut textures = Vec::with_capacity(inner_config.buffer_count as usize);
        ValidationSwapChain::wrap_images(device, &mut textures, images);

        let swap_chain = AnyArc::new_cyclic(move |v| ValidationSwapChain {
            _this: v.clone(),
            _device: device._this.upgrade().unwrap(),
            _surface: self._this.upgrade().unwrap(),
            inner,
            queue_support: Default::default(),
            textures: Mutex::new(textures),
            acquired: Default::default(),
        });
        Ok(AnyArc::map::<dyn ISwapChain, _>(swap_chain, |v| v))
    }
}
