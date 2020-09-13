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

use aleph_vulkan_core::erupt::vk1_0::{
    BlendFactor, BlendOp, ColorComponentFlags, PipelineColorBlendAttachmentStateBuilder,
};

///
/// A namespace struct for colour blend attachment state
///
pub struct ColorBlendAttachmentState {}

impl ColorBlendAttachmentState {
    ///
    /// Color blending disabled
    ///
    pub fn disabled() -> PipelineColorBlendAttachmentStateBuilder<'static> {
        PipelineColorBlendAttachmentStateBuilder::new().blend_enable(false)
    }

    ///
    /// Standard alpha blending
    ///
    /// OUT_COLOR = (SRC_C * SRC_A) + (DST_C * (1 - SRC_A))
    /// OUT_ALPHA = (SRC_A * 1) + (DST_A * 0)
    ///
    pub fn alpha_blending() -> PipelineColorBlendAttachmentStateBuilder<'static> {
        PipelineColorBlendAttachmentStateBuilder::new()
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .color_write_mask(Self::write_all())
    }

    fn write_all() -> ColorComponentFlags {
        ColorComponentFlags::R
            | ColorComponentFlags::G
            | ColorComponentFlags::B
            | ColorComponentFlags::A
    }
}
