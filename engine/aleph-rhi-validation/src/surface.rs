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
use parking_lot::Mutex;

use crate::internal::unwrap;
use crate::{ValidationContext, ValidationSwapChain};

pub struct ValidationSurface {
    pub(crate) _this: Weak<Self>,
    pub(crate) _context: Arc<ValidationContext>,
    pub(crate) inner: Arc<dyn ISurface>,
    pub(crate) has_swap_chain: Mutex<bool>,
}

crate::impl_platform_interface_passthrough!(ValidationSurface);

impl ISurface for ValidationSurface {
    fn upgrade(&self) -> Arc<dyn ISurface> {
        self._this.upgrade().unwrap()
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
    ) -> Result<Arc<dyn ISwapChain>, SwapChainCreateError> {
        let device = unwrap::device(device);
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

        let swap_chain = Arc::new_cyclic(move |v| ValidationSwapChain {
            _this: v.clone(),
            _device: device._this.upgrade().unwrap(),
            _surface: self._this.upgrade().unwrap(),
            inner,
            queue_support: Default::default(),
            acquired: Default::default(),
        });
        Ok(swap_chain)
    }
}
