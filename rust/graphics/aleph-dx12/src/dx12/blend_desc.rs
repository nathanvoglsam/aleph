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

use crate::{Bool, RenderTargetBlendDesc};

pub struct BlendDescBuilder {
    inner: BlendDesc,
}

impl BlendDescBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    #[inline]
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alpha_to_coverage_enable = alpha_to_coverage_enable.into();
        self
    }

    #[inline]
    pub fn independent_blend_enable(mut self, independent_blend_enable: bool) -> Self {
        self.inner.independent_blend_enable = independent_blend_enable.into();
        self
    }

    #[inline]
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

    #[inline]
    pub fn build(self) -> BlendDesc {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct BlendDesc {
    pub alpha_to_coverage_enable: Bool,
    pub independent_blend_enable: Bool,
    pub render_targets: [RenderTargetBlendDesc; 8],
}

impl BlendDesc {
    #[inline]
    pub fn builder() -> BlendDescBuilder {
        BlendDescBuilder::new()
    }
}

impl Default for BlendDesc {
    #[inline]
    fn default() -> Self {
        Self {
            alpha_to_coverage_enable: false.into(),
            independent_blend_enable: false.into(),
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
