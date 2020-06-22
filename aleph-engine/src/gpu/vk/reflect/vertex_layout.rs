//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::reflect::utils::resolve_member_type;
use crate::gpu::vk::reflect::MemberType;
use spirv_reflect::types::ReflectInterfaceVariable;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VertexAttributeReflection {
    name: String,
    location: u32,
    attribute_type: MemberType,
}

impl VertexAttributeReflection {
    ///
    /// Gets the name of the vertex attribute
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    ///
    /// Get the binding location for this vertex attribute
    ///
    pub fn location(&self) -> u32 {
        self.location
    }

    ///
    /// The data type this vertex attribute was declared as in the shader
    ///
    pub fn attribute_type(&self) -> &MemberType {
        &self.attribute_type
    }
}

///
/// Represents the expected vertex data to be provided to a vertex shader.
///
/// # Warning
///
/// This, unfortunately, can not be used to derive the memory layout for vertex buffers. The vertex
/// layout reflection can only be used to see the type each attribute was declared as in the shader.
///
/// The vertex layout declares each attribute as a float2, float3, etc. This does not have any say
/// over the actual format of the buffer. A float3 attribute could be a R32B32G32_SFLOAT or an
/// R8G8B8_UNORM or a number of other formats. The shader alone can not specify this so we cant use
/// the shader to build vertex buffer layouts.
///
/// Fortunately we can still use this information to sanity check the shader with a pipeline's
/// vertex layout. We can at least check a format has the correct number of vector components and
/// maybe do some heuristics based on the name of the attribute in the shader.
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VertexLayoutReflection {
    attributes: Vec<VertexAttributeReflection>,
}

impl VertexLayoutReflection {
    ///
    /// Reflect a push constant layout from the given block variable
    ///
    pub fn reflect(interface: impl Iterator<Item = ReflectInterfaceVariable>) -> Self {
        let attributes = resolve_vertex_attributes(interface);
        Self { attributes }
    }

    ///
    /// Gets the list of vertex attributes defined in the shader
    ///
    pub fn attributes(&self) -> &[VertexAttributeReflection] {
        &self.attributes
    }
}

///
/// Internal function for building list of vertex attributes from a `ReflectInterfaceVariable`
///
pub(crate) fn resolve_vertex_attributes(
    interface: impl Iterator<Item = ReflectInterfaceVariable>,
) -> Vec<VertexAttributeReflection> {
    interface
        .map(|m| {
            let attribute_type = resolve_interface_member_type(&m);
            let name = m.name;
            let location = m.location;
            let attr = VertexAttributeReflection {
                name,
                location,
                attribute_type,
            };
            attr
        })
        .collect()
}

///
/// Internal wrapper for extracting required information for calling `resolve_member_type` from a
/// `ReflectInterfaceVariable`
///
pub(crate) fn resolve_interface_member_type(interface: &ReflectInterfaceVariable) -> MemberType {
    let desc = interface.type_description.as_ref().unwrap();
    let decoration_flags = interface.decoration_flags;
    let numeric = &interface.numeric;
    resolve_member_type(desc, decoration_flags, numeric)
}
