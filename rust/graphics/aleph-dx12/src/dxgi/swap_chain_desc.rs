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

use crate::dxgi::{AlphaMode, Format, SampleDesc, Scaling, SwapChainFlags, SwapEffect, UsageFlags};
use crate::Bool;

pub struct SwapChainDesc1Builder {
    inner: SwapChainDesc1,
}

impl SwapChainDesc1Builder {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: SwapChainDesc1 {
                width: 0,
                height: 0,
                format: Default::default(),
                stereo: false.into(),
                sample_desc: SampleDesc {
                    count: 1,
                    quality: 0,
                },
                buffer_usage: UsageFlags::NONE,
                buffer_count: 0,
                scaling: Scaling::Stretch,
                swap_effect: SwapEffect::FlipSequential,
                alpha_mode: AlphaMode::Ignore,
                flags: SwapChainFlags::NONE,
            },
        }
    }

    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }

    #[inline]
    pub fn format(mut self, format: Format) -> Self {
        self.inner.format = format;
        self
    }

    #[inline]
    pub fn stereo(mut self, stereo: bool) -> Self {
        self.inner.stereo = stereo.into();
        self
    }

    #[inline]
    pub fn sample_desc(mut self, sample_desc: SampleDesc) -> Self {
        self.inner.sample_desc = sample_desc;
        self
    }

    #[inline]
    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.inner.buffer_count = buffer_count;
        self
    }

    #[inline]
    pub fn scaling(mut self, scaling: Scaling) -> Self {
        self.inner.scaling = scaling;
        self
    }

    #[inline]
    pub fn swap_effect(mut self, swap_effect: SwapEffect) -> Self {
        self.inner.swap_effect = swap_effect;
        self
    }

    #[inline]
    pub fn alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.inner.alpha_mode = alpha_mode;
        self
    }

    #[inline]
    pub fn usage_flags(mut self, usage_flags: UsageFlags) -> Self {
        self.inner.buffer_usage |= usage_flags;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: SwapChainFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    #[inline]
    pub fn build(self) -> SwapChainDesc1 {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SwapChainDesc1 {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub stereo: Bool,
    pub sample_desc: SampleDesc,
    pub buffer_usage: UsageFlags,
    pub buffer_count: u32,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: SwapChainFlags,
}

impl SwapChainDesc1 {
    #[inline]
    pub fn builder() -> SwapChainDesc1Builder {
        SwapChainDesc1Builder::new()
    }
}
