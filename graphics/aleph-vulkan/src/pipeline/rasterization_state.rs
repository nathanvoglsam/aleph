//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
