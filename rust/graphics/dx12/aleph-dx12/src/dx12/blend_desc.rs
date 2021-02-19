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

pub struct BlendDescBuilder {
    inner: BlendDesc,
}

impl BlendDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alpha_to_coverage_enable = alpha_to_coverage_enable;
        self
    }
    pub fn independent_blend_enable(mut self, independent_blend_enable: bool) -> Self {
        self.inner.independent_blend_enable = independent_blend_enable;
        self
    }
    pub fn render_targets(mut self, render_targets: &[RenderTargetBlendDesc]) -> Self {
        assert!(render_targets.len() <= 8);
        render_targets
            .iter()
            .cloned()
            .enumerate()
            .for_each(|(i, desc)| {
                self.inner.render_targets[i] = desc;
            });
        self
    }

    pub fn build(self) -> BlendDesc {
        self.inner
    }
}

#[derive(Clone, Debug, Hash)]
pub struct BlendDesc {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_targets: [RenderTargetBlendDesc; 8],
}

impl BlendDesc {
    pub fn builder() -> BlendDescBuilder {
        BlendDescBuilder::new()
    }
}

impl Default for BlendDesc {
    fn default() -> Self {
        Self {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_targets: [
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
                RenderTargetBlendDesc::default(),
            ],
        }
    }
}

impl Into<D3D12_BLEND_DESC> for BlendDesc {
    fn into(self) -> D3D12_BLEND_DESC {
        let mut render_target: [D3D12_RENDER_TARGET_BLEND_DESC; 8] = [
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
            D3D12_RENDER_TARGET_BLEND_DESC::default(),
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
