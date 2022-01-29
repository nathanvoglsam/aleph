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
    BlendFactor, BlendOp, ColorComponentFlagBits, PipelineColorBlendAttachmentState,
    PipelineColorBlendAttachmentStateBuilder, PipelineColorBlendStateCreateInfoBuilder, FALSE,
};

///
/// Namespace struct for color blend state
///
pub struct ColorBlendState {}

impl ColorBlendState {
    ///
    /// Creates a state from the given list of attachments with logic ops disabled and default blend
    /// constants
    ///
    pub fn attachments<'a>(
        attachments: &'a [PipelineColorBlendAttachmentStateBuilder],
    ) -> PipelineColorBlendStateCreateInfoBuilder<'a> {
        PipelineColorBlendStateCreateInfoBuilder::new()
            .attachments(attachments)
            .logic_op_enable(false)
    }

    ///
    /// Creates a state description for disabling blending on all attachments, with num specifying
    /// the number of attachments.
    ///
    /// This supports a `num` of up to 16, any more will trigger a panic as we have a static
    /// array we slice from for handing the `PipelineColorBlendAttachmentStateBuilder` instance
    ///
    pub fn disabled(num: usize) -> PipelineColorBlendStateCreateInfoBuilder<'static> {
        static DESCRIPTIONS: [PipelineColorBlendAttachmentState; 16] = [blend_disabled(); 16];

        let descriptions: &[PipelineColorBlendAttachmentStateBuilder<'static>; 16] =
            unsafe { std::mem::transmute(&DESCRIPTIONS) };

        PipelineColorBlendStateCreateInfoBuilder::new().attachments(&descriptions[0..num])
    }
}

///
/// Internal const function for producing a `PipelineColorBlendAttachmentState` object at compile
/// time
///
const fn blend_disabled() -> PipelineColorBlendAttachmentState {
    PipelineColorBlendAttachmentState {
        blend_enable: FALSE,
        src_color_blend_factor: BlendFactor::ONE,
        dst_color_blend_factor: BlendFactor::ZERO,
        color_blend_op: BlendOp::ADD,
        src_alpha_blend_factor: BlendFactor::ONE,
        dst_alpha_blend_factor: BlendFactor::ZERO,
        alpha_blend_op: BlendOp::ADD,
        color_write_mask: ColorComponentFlagBits(0b1111).bitmask(),
    }
}
