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

use crate::{Blend, BlendOp, Bool, ColorWriteEnable, LogicOp};

pub struct RenderTargetBlendDescBuilder {
    inner: RenderTargetBlendDesc,
}

impl RenderTargetBlendDescBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: RenderTargetBlendDesc::default(),
        }
    }

    #[inline]
    pub fn blend_enable(mut self, blend_enable: bool) -> Self {
        self.inner.blend_enable = blend_enable.into();
        self
    }

    #[inline]
    pub fn logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.inner.logic_op_enable = logic_op_enable.into();
        self
    }

    #[inline]
    pub fn src_blend(mut self, src_blend: Blend) -> Self {
        self.inner.src_blend = src_blend;
        self
    }

    #[inline]
    pub fn dest_blend(mut self, dest_blend: Blend) -> Self {
        self.inner.dest_blend = dest_blend;
        self
    }

    #[inline]
    pub fn blend_op(mut self, blend_op: BlendOp) -> Self {
        self.inner.blend_op = blend_op;
        self
    }

    #[inline]
    pub fn src_blend_alpha(mut self, src_blend_alpha: Blend) -> Self {
        self.inner.src_blend_alpha = src_blend_alpha;
        self
    }

    #[inline]
    pub fn dest_blend_alpha(mut self, dest_blend_alpha: Blend) -> Self {
        self.inner.dest_blend_alpha = dest_blend_alpha;
        self
    }

    #[inline]
    pub fn blend_op_alpha(mut self, blend_op_alpha: BlendOp) -> Self {
        self.inner.blend_op_alpha = blend_op_alpha;
        self
    }

    #[inline]
    pub fn logic_op(mut self, logic_op: LogicOp) -> Self {
        self.inner.logic_op = logic_op;
        self
    }

    #[inline]
    pub fn render_target_write_mask(mut self, render_target_write_mask: ColorWriteEnable) -> Self {
        self.inner.render_target_write_mask = render_target_write_mask;
        self
    }

    #[inline]
    pub fn build(self) -> RenderTargetBlendDesc {
        self.inner
    }
}

#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: Bool,
    pub logic_op_enable: Bool,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub render_target_write_mask: ColorWriteEnable,
}

impl RenderTargetBlendDesc {
    #[inline]
    pub fn builder() -> RenderTargetBlendDescBuilder {
        RenderTargetBlendDescBuilder::new()
    }
}

impl Default for RenderTargetBlendDesc {
    #[inline]
    fn default() -> Self {
        Self {
            blend_enable: false.into(),
            logic_op_enable: false.into(),
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: BlendOp::Add,
            src_blend_alpha: Blend::One,
            dest_blend_alpha: Blend::Zero,
            blend_op_alpha: BlendOp::Add,
            logic_op: LogicOp::Noop,
            render_target_write_mask: ColorWriteEnable::all(),
        }
    }
}
