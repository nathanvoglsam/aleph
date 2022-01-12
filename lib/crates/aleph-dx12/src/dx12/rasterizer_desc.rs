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

use crate::{Bool, ConservativeRasterizationMode, CullMode, FillMode};

pub struct RasterizerDescBuilder {
    inner: RasterizerDesc,
}

impl RasterizerDescBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    #[inline]
    pub fn fill_mode(mut self, fill_mode: FillMode) -> Self {
        self.inner.fill_mode = fill_mode;
        self
    }

    #[inline]
    pub fn cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.inner.cull_mode = cull_mode;
        self
    }

    #[inline]
    pub fn front_counter_clockwise(mut self, front_counter_clockwise: bool) -> Self {
        self.inner.front_counter_clockwise = front_counter_clockwise.into();
        self
    }

    #[inline]
    pub fn depth_bias(mut self, depth_bias: i32) -> Self {
        self.inner.depth_bias = depth_bias;
        self
    }

    #[inline]
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }

    #[inline]
    pub fn slope_scaled_depth_bias(mut self, slope_scaled_depth_bias: f32) -> Self {
        self.inner.slope_scaled_depth_bias = slope_scaled_depth_bias;
        self
    }

    #[inline]
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = depth_clip_enable.into();
        self
    }

    #[inline]
    pub fn multisample_enable(mut self, multisample_enable: bool) -> Self {
        self.inner.multisample_enable = multisample_enable.into();
        self
    }

    #[inline]
    pub fn antialiased_line_enable(mut self, antialiased_line_enable: bool) -> Self {
        self.inner.antialiased_line_enable = antialiased_line_enable.into();
        self
    }

    #[inline]
    pub fn forced_sample_count(mut self, forced_sample_count: u32) -> Self {
        self.inner.forced_sample_count = forced_sample_count;
        self
    }

    #[inline]
    pub fn conservative_raster(
        mut self,
        conservative_raster: ConservativeRasterizationMode,
    ) -> Self {
        self.inner.conservative_raster = conservative_raster;
        self
    }

    #[inline]
    pub fn build(self) -> RasterizerDesc {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: Bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: Bool,
    pub multisample_enable: Bool,
    pub antialiased_line_enable: Bool,
    pub forced_sample_count: u32,
    pub conservative_raster: ConservativeRasterizationMode,
}

impl RasterizerDesc {
    #[inline]
    pub fn builder() -> RasterizerDescBuilder {
        RasterizerDescBuilder::new()
    }
}

impl Default for RasterizerDesc {
    fn default() -> Self {
        Self {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            front_counter_clockwise: false.into(),
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: true.into(),
            multisample_enable: false.into(),
            antialiased_line_enable: false.into(),
            forced_sample_count: 0,
            conservative_raster: ConservativeRasterizationMode::Off,
        }
    }
}
