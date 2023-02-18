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

use crate::context::ValidationContext;
use crate::{ValidationDevice, ValidationSwapChain};
use interfaces::any::{AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ValidationSurface {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<ValidationContext>,
    pub(crate) inner: AnyArc<dyn ISurface>,
    pub(crate) has_swap_chain: AtomicBool,
}

interfaces::any::declare_interfaces!(ValidationSurface, [ISurface]);

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

        // Check if the surface is currently taken with an existing swap chain
        if self.has_swap_chain.swap(true, Ordering::Relaxed) {
            return Err(SwapChainCreateError::SurfaceAlreadyOwned);
        }

        let inner = match self.inner.create_swap_chain(inner_device, config) {
            v @ Ok(_) => v,
            v @ Err(_) => {
                // Release the surface if we failed to actually create the swap chain
                assert!(self.has_swap_chain.swap(false, Ordering::Relaxed));
                v
            }
        };
        let inner = inner?;

        let swap_chain = AnyArc::new_cyclic(move |v| ValidationSwapChain {
            _this: v.clone(),
            _device: device._this.upgrade().unwrap(),
            _surface: self._this.upgrade().unwrap(),
            inner,
            queue_support: Default::default(),
            current_image: Default::default(),
        });
        Ok(AnyArc::map::<dyn ISwapChain, _>(swap_chain, |v| v))
    }
}
