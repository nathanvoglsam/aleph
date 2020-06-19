//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::cstr;
use erupt::vk1_0::{PipelineShaderStageCreateInfoBuilder, ShaderModule, ShaderStageFlagBits};

///
/// A namespace struct for specifying pipeline shader stages
///
pub struct ShaderStage {}

impl ShaderStage {
    ///
    /// Specifies a fragment shader module
    ///
    pub fn fragment(module: ShaderModule) -> PipelineShaderStageCreateInfoBuilder<'static> {
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::FRAGMENT)
            .module(module)
            .name(cstr!("main"))
    }

    ///
    /// Specifies a vertex shader module
    ///
    pub fn vertex(module: ShaderModule) -> PipelineShaderStageCreateInfoBuilder<'static> {
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::VERTEX)
            .module(module)
            .name(cstr!("main"))
    }

    ///
    /// Specifies a geometry shader module
    ///
    pub fn geometry(module: ShaderModule) -> PipelineShaderStageCreateInfoBuilder<'static> {
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::GEOMETRY)
            .module(module)
            .name(cstr!("main"))
    }

    ///
    /// Specifies a compute shader module
    ///
    pub fn compute(module: ShaderModule) -> PipelineShaderStageCreateInfoBuilder<'static> {
        PipelineShaderStageCreateInfoBuilder::new()
            .stage(ShaderStageFlagBits::COMPUTE)
            .module(module)
            .name(cstr!("main"))
    }
}
