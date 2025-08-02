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

use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

/// The struct layout we use for render target views. [ImageView] handles are actually pointers to
/// this struct for RTV/DSV views.
pub struct RenderTargetView {
    /// Unretained reference to the metal texture. The views can't outlive the parent texture so
    /// this is fine. If this was retained we'd generate more retain traffic than we otherwise need.
    pub texture: Retained<ProtocolObject<dyn MTLTexture>>,
}

// Safety: none, up to the caller to use this correctly.
unsafe impl Send for RenderTargetView {}
unsafe impl Sync for RenderTargetView {}

impl RenderTargetView {
    pub unsafe fn from_view(view: &ImageView) -> &RenderTargetView {
        let view = *view;
        // Safety: none, lmao. it's up to the caller to use this correctly.
        unsafe { std::mem::transmute::<ImageView, &RenderTargetView>(view) }
    }
}
