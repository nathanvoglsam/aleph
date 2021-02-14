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

use crate::{ConservativeRasterizationMode, CullMode, FillMode};
use raw::windows::win32::direct3d12::D3D12_RASTERIZER_DESC;

#[derive(Copy, Clone, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: bool,
    pub multisample_enable: bool,
    pub antialiased_line_enable: bool,
    pub forced_sample_count: u32,
    pub conservative_raster: ConservativeRasterizationMode,
}

impl Into<D3D12_RASTERIZER_DESC> for RasterizerDesc {
    fn into(self) -> D3D12_RASTERIZER_DESC {
        D3D12_RASTERIZER_DESC {
            fill_mode: self.fill_mode.into(),
            cull_mode: self.cull_mode.into(),
            front_counter_clockwise: self.front_counter_clockwise.into(),
            depth_bias: self.depth_bias.into(),
            depth_bias_clamp: self.depth_bias_clamp.into(),
            slope_scaled_depth_bias: self.slope_scaled_depth_bias.into(),
            depth_clip_enable: self.depth_clip_enable.into(),
            multisample_enable: self.multisample_enable.into(),
            antialiased_line_enable: self.antialiased_line_enable.into(),
            forced_sample_count: self.forced_sample_count.into(),
            conservative_raster: self.conservative_raster.into(),
        }
    }
}
