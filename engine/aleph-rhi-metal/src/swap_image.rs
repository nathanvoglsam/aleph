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
use std::sync::Arc;

use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use objc2_quartz_core::CAMetalDrawable;

use crate::swap_chain::SwapChain;

pub struct SwapImage {
    pub(crate) _swap_chain: Arc<SwapChain>,
    pub(crate) objects: SwapImageObjects,
    pub(crate) texture: TextureHandle,
}

impl IGetPlatformInterface for SwapImage {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl ISwapImage for SwapImage {
    fn texture(&self) -> &TextureHandle {
        &self.texture
    }

    fn texture_desc(&self) -> &TextureDesc<'_> {
        self._swap_chain.device.get_texture_desc(self.texture())
    }
}

/// Wrapper to scope our 'unsafe impl Send+Sync'
pub struct SwapImageObjects {
    pub list: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
    pub drawable: Retained<ProtocolObject<dyn CAMetalDrawable>>,
}

// Safety: Needed because of 'MTLTexture'
unsafe impl Send for SwapImage {}
unsafe impl Sync for SwapImage {}
