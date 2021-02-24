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
use crate::raw::windows::win32::dxgi::DXGI_SWAP_CHAIN_DESC1;
use std::convert::{TryFrom, TryInto};

pub struct SwapChainDesc1Builder {
    inner: SwapChainDesc1,
}

impl SwapChainDesc1Builder {
    pub fn new() -> Self {
        Self {
            inner: SwapChainDesc1 {
                width: 0,
                height: 0,
                format: Default::default(),
                stereo: false,
                sample_desc: Default::default(),
                buffer_usage: UsageFlags::NONE,
                buffer_count: 0,
                scaling: Scaling::Stretch,
                swap_effect: SwapEffect::FlipSequential,
                alpha_mode: AlphaMode::Ignore,
                flags: SwapChainFlags::NONE,
            },
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }

    pub fn format(mut self, format: Format) -> Self {
        self.inner.format = format;
        self
    }

    pub fn stereo(mut self, stereo: bool) -> Self {
        self.inner.stereo = stereo;
        self
    }

    pub fn sample_desc(mut self, sample_desc: SampleDesc) -> Self {
        self.inner.sample_desc = sample_desc;
        self
    }

    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.inner.buffer_count = buffer_count;
        self
    }

    pub fn scaling(mut self, scaling: Scaling) -> Self {
        self.inner.scaling = scaling;
        self
    }

    pub fn swap_effect(mut self, swap_effect: SwapEffect) -> Self {
        self.inner.swap_effect = swap_effect;
        self
    }

    pub fn alpha_mode(mut self, alpha_mode: AlphaMode) -> Self {
        self.inner.alpha_mode = alpha_mode;
        self
    }

    pub fn usage_flags(mut self, usage_flags: UsageFlags) -> Self {
        self.inner.buffer_usage |= usage_flags;
        self
    }

    pub fn flags(mut self, flags: SwapChainFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    pub fn build(self) -> SwapChainDesc1 {
        self.inner
    }
}

#[derive(Clone, Debug)]
pub struct SwapChainDesc1 {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub stereo: bool,
    pub sample_desc: SampleDesc,
    pub buffer_usage: UsageFlags,
    pub buffer_count: u32,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: SwapChainFlags,
}

impl SwapChainDesc1 {
    pub fn builder() -> SwapChainDesc1Builder {
        SwapChainDesc1Builder::new()
    }
}

impl TryFrom<DXGI_SWAP_CHAIN_DESC1> for SwapChainDesc1 {
    type Error = ();

    fn try_from(v: DXGI_SWAP_CHAIN_DESC1) -> Result<Self, Self::Error> {
        let out = Self {
            width: v.width,
            height: v.height,
            format: v.format.try_into()?,
            stereo: v.stereo.as_bool(),
            sample_desc: v.sample_desc.into(),
            buffer_usage: UsageFlags(v.buffer_usage),
            buffer_count: v.buffer_count,
            scaling: v.scaling.try_into()?,
            swap_effect: v.swap_effect.try_into()?,
            alpha_mode: v.alpha_mode.try_into()?,
            flags: SwapChainFlags(v.flags),
        };
        Ok(out)
    }
}

impl Into<DXGI_SWAP_CHAIN_DESC1> for SwapChainDesc1 {
    fn into(self) -> DXGI_SWAP_CHAIN_DESC1 {
        DXGI_SWAP_CHAIN_DESC1 {
            width: self.width,
            height: self.height,
            format: self.format.into(),
            stereo: self.stereo.into(),
            sample_desc: self.sample_desc.into(),
            buffer_usage: self.buffer_usage.0,
            buffer_count: self.buffer_count,
            scaling: self.scaling.into(),
            swap_effect: self.swap_effect.into(),
            alpha_mode: self.alpha_mode.into(),
            flags: self.flags.0,
        }
    }
}
