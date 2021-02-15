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

use crate::RenderTargetBlendDesc;
use raw::windows::win32::direct3d12::{D3D12_BLEND_DESC, D3D12_RENDER_TARGET_BLEND_DESC};

#[derive(Copy, Clone, Debug, Hash)]
pub struct BlendDesc<'a> {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_targets: &'a [RenderTargetBlendDesc],
}

impl<'a> Into<D3D12_BLEND_DESC> for BlendDesc<'a> {
    fn into(self) -> D3D12_BLEND_DESC {
        let mut render_target: [D3D12_RENDER_TARGET_BLEND_DESC; 8] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];
        for i in 0..self.render_targets.len() {
            render_target[i] = self.render_targets[i].clone().into();
        }

        D3D12_BLEND_DESC {
            alpha_to_coverage_enable: self.alpha_to_coverage_enable.into(),
            independent_blend_enable: self.independent_blend_enable.into(),
            render_target,
        }
    }
}
