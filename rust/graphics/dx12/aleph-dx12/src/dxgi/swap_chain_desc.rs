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

    pub fn usage_back_buffer(mut self, back_buffer: bool) -> Self {
        if back_buffer {
            self.inner.buffer_usage.0 |= UsageFlags::BACK_BUFFER.0;
        }
        self
    }

    pub fn usage_discard_on_present(mut self, discard_on_present: bool) -> Self {
        if discard_on_present {
            self.inner.buffer_usage.0 |= UsageFlags::DISCARD_ON_PRESENT.0;
        }
        self
    }

    pub fn usage_read_only(mut self, read_only: bool) -> Self {
        if read_only {
            self.inner.buffer_usage.0 |= UsageFlags::READ_ONLY.0;
        }
        self
    }

    pub fn usage_render_target_output(mut self, render_target_output: bool) -> Self {
        if render_target_output {
            self.inner.buffer_usage.0 |= UsageFlags::RENDER_TARGET_OUTPUT.0;
        }
        self
    }

    pub fn usage_shader_input(mut self, shader_input: bool) -> Self {
        if shader_input {
            self.inner.buffer_usage.0 |= UsageFlags::SHADER_INPUT.0;
        }
        self
    }

    pub fn usage_shared(mut self, shared: bool) -> Self {
        if shared {
            self.inner.buffer_usage.0 |= UsageFlags::SHARED.0;
        }
        self
    }

    pub fn usage_unordered_access(mut self, unordered_access: bool) -> Self {
        if unordered_access {
            self.inner.buffer_usage.0 |= UsageFlags::UNORDERED_ACCESS.0;
        }
        self
    }

    pub fn flag_non_pre_rotated(mut self, non_pre_rotated: bool) -> Self {
        if non_pre_rotated {
            self.inner.flags.0 |= SwapChainFlags::NON_PRE_ROTATED.0;
        }
        self
    }

    pub fn flag_allow_mode_switch(mut self, allow_mode_switch: bool) -> Self {
        if allow_mode_switch {
            self.inner.flags.0 |= SwapChainFlags::ALLOW_MODE_SWITCH.0;
        }
        self
    }

    pub fn flag_gdi_compatible(mut self, gdi_compatible: bool) -> Self {
        if gdi_compatible {
            self.inner.flags.0 |= SwapChainFlags::GDI_COMPATIBLE.0;
        }
        self
    }

    pub fn flag_restricted_content(mut self, restricted_content: bool) -> Self {
        if restricted_content {
            self.inner.flags.0 |= SwapChainFlags::RESTRICTED_CONTENT.0;
        }
        self
    }

    pub fn flag_restrict_shared_resource_driver(
        mut self,
        restrict_shared_resource_driver: bool,
    ) -> Self {
        if restrict_shared_resource_driver {
            self.inner.flags.0 |= SwapChainFlags::RESTRICT_SHARED_RESOURCE_DRIVER.0;
        }
        self
    }

    pub fn flag_display_only(mut self, display_only: bool) -> Self {
        if display_only {
            self.inner.flags.0 |= SwapChainFlags::DISPLAY_ONLY.0;
        }
        self
    }

    pub fn flag_frame_latency_waitable_object(
        mut self,
        frame_latency_waitable_object: bool,
    ) -> Self {
        if frame_latency_waitable_object {
            self.inner.flags.0 |= SwapChainFlags::FRAME_LATENCY_WAITABLE_OBJECT.0;
        }
        self
    }

    pub fn flag_foreground_layer(mut self, foreground_layer: bool) -> Self {
        if foreground_layer {
            self.inner.flags.0 |= SwapChainFlags::FOREGROUND_LAYER.0;
        }
        self
    }

    pub fn flag_fullscreen_video(mut self, fullscreen_video: bool) -> Self {
        if fullscreen_video {
            self.inner.flags.0 |= SwapChainFlags::FULLSCREEN_VIDEO.0;
        }
        self
    }

    pub fn flag_yuv_video(mut self, yuv_video: bool) -> Self {
        if yuv_video {
            self.inner.flags.0 |= SwapChainFlags::YUV_VIDEO.0;
        }
        self
    }

    pub fn flag_hw_protected(mut self, hw_protected: bool) -> Self {
        if hw_protected {
            self.inner.flags.0 |= SwapChainFlags::HW_PROTECTED.0;
        }
        self
    }

    pub fn flag_allow_tearing(mut self, allow_tearing: bool) -> Self {
        if allow_tearing {
            self.inner.flags.0 |= SwapChainFlags::ALLOW_TEARING.0;
        }
        self
    }

    pub fn flag_restricted_to_all_holographic_displays(
        mut self,
        restricted_to_all_holographic_displays: bool,
    ) -> Self {
        if restricted_to_all_holographic_displays {
            self.inner.flags.0 |= SwapChainFlags::RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS.0;
        }
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
