//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use erupt::vk1_0::{
    CompareOp, PipelineDepthStencilStateCreateInfo, PipelineDepthStencilStateCreateInfoBuilder,
};

///
/// Namespace struct for depth state
///
pub struct DepthState {}

impl DepthState {
    ///
    /// Depth write and depth test disabled
    ///
    pub fn disabled() -> PipelineDepthStencilStateCreateInfo {
        *PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_write_enable(false)
            .depth_test_enable(false)
    }

    ///
    /// Depth testing enabled, with depth writes enabled chosen by `write` and the compare op chosen
    /// by `compare_op`.
    ///
    pub fn enabled(write: bool, compare_op: CompareOp) -> PipelineDepthStencilStateCreateInfo {
        *PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_write_enable(write)
            .depth_test_enable(true)
            .depth_bounds_test_enable(false)
            .depth_compare_op(compare_op)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false)
    }
}
