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
    CullModeFlags, FrontFace, PipelineRasterizationStateCreateInfoBuilder, PolygonMode,
};

///
/// A namespace struct for rasterization state
///
pub struct RasterizationState {}

impl RasterizationState {
    ///
    /// A rasterization state with culling disabled and the given polygon mode and front face
    ///
    pub fn unculled(
        mode: PolygonMode,
        front_face: FrontFace,
    ) -> PipelineRasterizationStateCreateInfoBuilder<'static> {
        PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(mode)
            .cull_mode(CullModeFlags::NONE)
            .front_face(front_face)
            .line_width(1.0)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(false)
    }

    ///
    /// A rasterization state with the given polygon mode and front face. Back face culling
    /// is enabled.
    ///
    pub fn backface_culled(
        mode: PolygonMode,
        front_face: FrontFace,
    ) -> PipelineRasterizationStateCreateInfoBuilder<'static> {
        PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(mode)
            .cull_mode(CullModeFlags::BACK)
            .front_face(front_face)
            .line_width(1.0)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(false)
    }

    ///
    /// A rasterization state with the given polygon mode and front face. Back face culling
    /// is enabled as well as a depth bias with the given constant, slop and clamp parameters
    ///
    pub fn backface_culled_depth_bias(
        mode: PolygonMode,
        front_face: FrontFace,
        constant: f32,
        slope: f32,
        clamp: f32,
    ) -> PipelineRasterizationStateCreateInfoBuilder<'static> {
        PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(mode)
            .cull_mode(CullModeFlags::BACK)
            .front_face(front_face)
            .line_width(1.0)
            .rasterizer_discard_enable(false)
            .depth_bias_enable(true)
            .depth_bias_constant_factor(constant)
            .depth_bias_slope_factor(slope)
            .depth_bias_clamp(clamp)
    }
}
