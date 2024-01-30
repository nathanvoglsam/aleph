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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;

use crate::{NullDevice, NullSurface};

pub struct NullSwapChain {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) _surface: AnyArc<NullSurface>,
    pub(crate) config: SwapChainConfiguration,
}

declare_interfaces!(NullSwapChain, [ISwapChain]);

crate::impl_platform_interface_passthrough!(NullSwapChain);

impl ISwapChain for NullSwapChain {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain> {
        AnyArc::map::<dyn ISwapChain, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn present_supported_on_queue(&self, _queue: QueueType) -> bool {
        true
    }

    fn get_config(&self) -> SwapChainConfiguration {
        self.config.clone()
    }

    fn rebuild(
        &self,
        _new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        unimplemented!()
    }

    fn get_images(&self, _images: &mut [Option<AnyArc<dyn ITexture>>]) {
        unimplemented!()
    }

    unsafe fn acquire_next_image(&self, _desc: &AcquireDesc) -> Result<u32, ImageAcquireError> {
        unimplemented!()
    }
}
