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

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;

use crate::ValidationSwapChain;

pub struct ValidationSwapImage {
    pub(crate) _swap_chain: AnyArc<ValidationSwapChain>,
    pub(crate) inner: Option<AnyArc<dyn ISwapImage>>,
    pub(crate) texture: Option<TextureHandle>,
}

declare_interfaces!(ValidationSwapImage, [ISwapImage]);

impl IGetPlatformInterface for ValidationSwapImage {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl ISwapImage for ValidationSwapImage {
    fn texture(&self) -> &TextureHandle {
        self.texture.as_ref().unwrap();
        if let Some(v) = &self.texture {
            v
        } else {
            panic!()
        }
    }

    fn texture_desc(&self) -> &TextureDesc {
        let v = self.inner.as_ref().unwrap();
        v.texture_desc()
    }
}
