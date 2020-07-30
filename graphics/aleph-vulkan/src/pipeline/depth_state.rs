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

use aleph_vulkan_core::erupt::vk1_0::{CompareOp, PipelineDepthStencilStateCreateInfoBuilder};

///
/// Namespace struct for depth state
///
pub struct DepthState {}

impl DepthState {
    ///
    /// Depth write and depth test disabled
    ///
    pub fn disabled() -> PipelineDepthStencilStateCreateInfoBuilder<'static> {
        PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_write_enable(false)
            .depth_test_enable(false)
    }

    ///
    /// Depth testing enabled, with depth writes enabled chosen by `write` and the compare op chosen
    /// by `compare_op`.
    ///
    pub fn enabled(
        write: bool,
        compare_op: CompareOp,
    ) -> PipelineDepthStencilStateCreateInfoBuilder<'static> {
        PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_write_enable(write)
            .depth_test_enable(true)
            .depth_bounds_test_enable(false)
            .depth_compare_op(compare_op)
            .stencil_test_enable(false)
    }
}
