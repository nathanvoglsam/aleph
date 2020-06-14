//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::reflect::structure::resolve_struct_interface;
use crate::gpu::vk::reflect::Struct;
use spirv_reflect::types::{ReflectEntryPoint, ReflectShaderStageFlags};
use std::ops::Deref;

///
/// Represents a vertex input layout reflected from a shader module.
///
#[derive(Debug)]
pub struct VertexLayout {
    layout: Struct,
}

impl VertexLayout {
    ///
    /// Reflect the vertex layout from the shader entry point.
    ///
    /// # Errors
    ///
    /// Returns `None` if the given entry point is not a vertex shader
    ///
    pub fn reflect(entry_point: &mut ReflectEntryPoint) -> Option<Self> {
        if entry_point.shader_stage.contains(ReflectShaderStageFlags::VERTEX) {
            let layout = resolve_struct_interface(&mut entry_point.input_variables);
            Some(Self { layout })
        } else {
            None
        }
    }
}

impl Deref for VertexLayout {
    type Target = Struct;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
