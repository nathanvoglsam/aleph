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

use std::any::TypeId;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2_core_foundation::CGSize;
use objc2_quartz_core::CAMetalLayer;
use parking_lot::Mutex;

use crate::device::Device;
use crate::surface::Surface;

pub struct SwapChain {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) surface: AnyArc<Surface>,
    pub(crate) objects: SwapChainObjects,
    pub(crate) inner: Mutex<SwapChainState>,
}

declare_interfaces!(SwapChain, [ISwapChain]);

impl IGetPlatformInterface for SwapChain {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: We can probably expose a few objects from a swapchain, but they're behind a mutex
        //       so we'll wait before implementing this
        None
    }
}

impl ISwapChain for SwapChain {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain> {
        AnyArc::map::<dyn ISwapChain, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn present_supported_on_queue(&self, queue: QueueType) -> bool {
        true // TODO: is this right?
    }

    fn get_config(&self) -> SwapChainConfiguration {
        self.inner.lock().config.clone()
    }

    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        let new_size = if let Some(new_size) = new_size {
            CGSize {
                width: new_size.width as f64,
                height: new_size.height as f64,
            }
        } else {
            self.natural_drawable_size()
        };

        let out_config = unsafe {
            let mut state = self.inner.lock();

            log::debug!(
                "Setting CAMetalLayer 'drawableSize' to ({}, {})",
                new_size.width,
                new_size.height
            );
            self.objects.layer.setDrawableSize(new_size);

            state.config.width = new_size.width as u32;
            state.config.height = new_size.height as u32;

            state.config.clone()
        };

        // TODO: how do we invalidate the texture objects?

        Ok(out_config)
    }

    fn get_images(&self, images: &mut [Option<TextureHandle>]) {
        todo!()
    }

    unsafe fn acquire_next_image(&self, desc: &AcquireDesc) -> Result<u32, ImageAcquireError> {
        todo!()
    }
}

impl SwapChain {
    pub(crate) fn natural_drawable_size(&self) -> CGSize {
        let scale = self.objects.layer.contentsScale();
        let mut size = self.objects.layer.bounds().size;

        size.width *= scale;
        size.height *= scale;

        size
    }
}

/// Wrapper struct to limit the scope of our 'unsafe impl Send+Sync'
pub struct SwapChainObjects {
    pub layer: Retained<CAMetalLayer>,
}

// SAFETY: Needed for CAMetalLayer
unsafe impl Send for SwapChainObjects {}
unsafe impl Sync for SwapChainObjects {}

pub struct SwapChainState {
    pub config: SwapChainConfiguration,
}
