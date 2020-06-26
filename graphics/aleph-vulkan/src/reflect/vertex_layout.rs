//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::reflect::member_resolution::resolve_member_type;
use crate::reflect::{MemberResolutionError, MemberType, ScalarType, VectorInfo};
use spirv_reflect::types::ReflectInterfaceVariable;
use vulkan_core::erupt::vk1_0::{
    Format, PipelineVertexInputStateCreateInfo, VertexInputAttributeDescription,
};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AttributeType {
    Scalar(ScalarType),
    Vector(VectorInfo),
}

///
/// Represents the set of errors that can be emitted from checking if a reflected vertex layout is
/// compatible with a given `PipelineVertexInputStateCreateInfo`
///
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AttributeCompatibilityError {
    ///
    /// The create info object was expecting an attribute at a given location but none was found in
    /// the reflected shader data.
    ///
    /// The location where an attribute was expected is provided by this error variant
    ///
    AttributeNotFoundAtLocation(u32),

    ///
    /// If the create info is expecting a different attribute location than what is found.
    ///
    /// The location found in the VertexAttributeReflection and in the create info object are
    /// returned in the form (VertexAttributeReflection location, CreateInfo location)
    ///
    IncorrectAttributeLocation(u32, u32),

    ///
    /// Attribute is not compatible with the format provided
    ///
    /// Returns the expected format and the type it was supposed to match
    ///
    IncompatibleFormat(AttributeType, Format),
}

///
/// This represents a vertex attribute reflected from a vertex shader
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VertexAttributeReflection {
    name: String,
    location: u32,
    attribute_type: AttributeType,
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
    pub fn attribute_type(&self) -> &AttributeType {
        &self.attribute_type
    }

    pub fn is_attribute_compatible(
        &self,
        attr: &VertexInputAttributeDescription,
    ) -> Result<(), AttributeCompatibilityError> {
        // Check if the location is correct
        if self.location != attr.location {
            return Err(AttributeCompatibilityError::IncorrectAttributeLocation(
                self.location,
                attr.location,
            ));
        }

        // Check if the format is compatible
        //
        // This is a big fuck off pile of match statements. There's really no way to make this any
        // neater but at least it can be split into different functions
        //
        match &self.attribute_type {
            AttributeType::Scalar(scalar) => {
                VertexAttributeReflection::check_scalar_compatibility(attr, scalar)
            }
            AttributeType::Vector(vector) => {
                VertexAttributeReflection::check_vector_compatibility(attr, vector)
            }
        }
    }

    ///
    /// Internal function for checking for format compatibility
    ///
    fn check_vector_compatibility(
        attr: &VertexInputAttributeDescription,
        vector: &VectorInfo,
    ) -> Result<(), AttributeCompatibilityError> {
        match vector.elements {
            2 => VertexAttributeReflection::check_vector2_compatibility(attr, vector),
            3 => VertexAttributeReflection::check_vector3_compatibility(attr, vector),
            4 => VertexAttributeReflection::check_vector4_compatibility(attr, vector),
            _ => panic!("Unsupported number of vector components"),
        }
    }

    ///
    /// Internal function for checking for format compatibility
    ///
    fn check_vector4_compatibility(
        attr: &VertexInputAttributeDescription,
        vector: &VectorInfo,
    ) -> Result<(), AttributeCompatibilityError> {
        match attr.format {
            Format::R8G8B8A8_SINT
            | Format::R8G8B8A8_UINT
            | Format::R8G8B8A8_SNORM
            | Format::R8G8B8A8_UNORM
            | Format::R8G8B8A8_SSCALED
            | Format::R8G8B8A8_USCALED
            | Format::R16G16B16A16_SFLOAT
            | Format::R16G16B16A16_SINT
            | Format::R16G16B16A16_UINT
            | Format::R16G16B16A16_SNORM
            | Format::R16G16B16A16_UNORM
            | Format::R16G16B16A16_SSCALED
            | Format::R16G16B16A16_USCALED
            | Format::R32G32B32A32_SFLOAT
            | Format::R32G32B32A32_SINT
            | Format::R32G32B32A32_UINT
            | Format::R64G64B64A64_SFLOAT
            | Format::R64G64B64A64_SINT
            | Format::R64G64B64A64_UINT => Ok(()),
            _ => Err(AttributeCompatibilityError::IncompatibleFormat(
                AttributeType::Vector(vector.clone()),
                attr.format,
            )),
        }
    }

    ///
    /// Internal function for checking for format compatibility
    ///
    fn check_vector3_compatibility(
        attr: &VertexInputAttributeDescription,
        vector: &VectorInfo,
    ) -> Result<(), AttributeCompatibilityError> {
        match attr.format {
            Format::R8G8B8_SINT
            | Format::R8G8B8_UINT
            | Format::R8G8B8_SNORM
            | Format::R8G8B8_UNORM
            | Format::R8G8B8_SSCALED
            | Format::R8G8B8_USCALED
            | Format::R16G16B16_SFLOAT
            | Format::R16G16B16_SINT
            | Format::R16G16B16_UINT
            | Format::R16G16B16_SNORM
            | Format::R16G16B16_UNORM
            | Format::R16G16B16_SSCALED
            | Format::R16G16B16_USCALED
            | Format::R32G32B32_SFLOAT
            | Format::R32G32B32_SINT
            | Format::R32G32B32_UINT
            | Format::R64G64B64_SFLOAT
            | Format::R64G64B64_SINT
            | Format::R64G64B64_UINT => Ok(()),
            _ => Err(AttributeCompatibilityError::IncompatibleFormat(
                AttributeType::Vector(vector.clone()),
                attr.format,
            )),
        }
    }

    ///
    /// Internal function for checking for format compatibility
    ///
    fn check_vector2_compatibility(
        attr: &VertexInputAttributeDescription,
        vector: &VectorInfo,
    ) -> Result<(), AttributeCompatibilityError> {
        match attr.format {
            Format::R8G8_SINT
            | Format::R8G8_UINT
            | Format::R8G8_SNORM
            | Format::R8G8_UNORM
            | Format::R8G8_SSCALED
            | Format::R8G8_USCALED
            | Format::R16G16_SFLOAT
            | Format::R16G16_SINT
            | Format::R16G16_UINT
            | Format::R16G16_SNORM
            | Format::R16G16_UNORM
            | Format::R16G16_SSCALED
            | Format::R16G16_USCALED
            | Format::R32G32_SFLOAT
            | Format::R32G32_SINT
            | Format::R32G32_UINT
            | Format::R64G64_SFLOAT
            | Format::R64G64_SINT
            | Format::R64G64_UINT => Ok(()),
            _ => Err(AttributeCompatibilityError::IncompatibleFormat(
                AttributeType::Vector(vector.clone()),
                attr.format,
            )),
        }
    }

    ///
    /// Internal function for checking for format compatibility
    ///
    fn check_scalar_compatibility(
        attr: &VertexInputAttributeDescription,
        scalar: &ScalarType,
    ) -> Result<(), AttributeCompatibilityError> {
        match attr.format {
            Format::R8_SINT
            | Format::R8_UINT
            | Format::R8_SNORM
            | Format::R8_UNORM
            | Format::R8_SSCALED
            | Format::R8_USCALED
            | Format::R16_SFLOAT
            | Format::R16_SINT
            | Format::R16_UINT
            | Format::R16_SNORM
            | Format::R16_UNORM
            | Format::R16_SSCALED
            | Format::R16_USCALED
            | Format::R32_SFLOAT
            | Format::R32_SINT
            | Format::R32_UINT
            | Format::R64_SFLOAT
            | Format::R64_SINT
            | Format::R64_UINT => Ok(()),
            _ => Err(AttributeCompatibilityError::IncompatibleFormat(
                AttributeType::Scalar(scalar.clone()),
                attr.format,
            )),
        }
    }
}

///
/// Represents the set of errors that can be produced when reflecting a vertex layout
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VertexLayoutResolutionError {
    ///
    /// An error occurred resolving one of the interface variables
    ///
    MemberResolutionError(MemberResolutionError),

    ///
    /// If one of the members tried specifying a type we don't support as a vertex input attribute.
    ///
    /// For example, matrices as vertex input types are not supported. There isn't a format big
    /// enough to hold one so a matrix would have to be passed as multiple attributes. The matrix
    /// layout (row/column major) also can't be deduced from the spirv alone so allowing them in
    /// vertex layouts would be full of foot guns.
    ///
    /// Matrices as input attributes are better handled explicitly by packing the data across,
    /// multiple vector types and reconstructing the matrix in the shader.
    ///
    UnsupportedType,
}

impl From<MemberResolutionError> for VertexLayoutResolutionError {
    fn from(other: MemberResolutionError) -> Self {
        match other {
            MemberResolutionError::UnknownMatrixLayout => {
                VertexLayoutResolutionError::UnsupportedType
            }
            v @ MemberResolutionError::UnknownScalarWidth(_) => {
                VertexLayoutResolutionError::MemberResolutionError(v)
            }
            v @ MemberResolutionError::UnsupportedMemberType => {
                VertexLayoutResolutionError::MemberResolutionError(v)
            }
        }
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
    pub fn reflect(
        interface: impl Iterator<Item = ReflectInterfaceVariable>,
    ) -> Result<Self, VertexLayoutResolutionError> {
        let attributes = resolve_vertex_attributes(interface)?;
        Ok(Self { attributes })
    }

    ///
    /// Gets the list of vertex attributes defined in the shader
    ///
    pub fn attributes(&self) -> &[VertexAttributeReflection] {
        &self.attributes
    }

    ///
    /// Takes a vertex
    ///
    pub fn is_layout_compatible(
        &self,
        layout: &PipelineVertexInputStateCreateInfo,
    ) -> Result<(), AttributeCompatibilityError> {
        let attributes = unsafe {
            std::slice::from_raw_parts(
                layout.p_vertex_attribute_descriptions,
                layout.vertex_attribute_description_count as _,
            )
        };

        for attr in attributes.iter() {
            match self
                .attributes
                .iter()
                .find(|v| v.location() == attr.location)
            {
                None => {
                    // The provided
                    return Err(AttributeCompatibilityError::AttributeNotFoundAtLocation(
                        attr.location,
                    ));
                }
                Some(other) => {
                    other.is_attribute_compatible(attr)?;
                }
            }
        }

        Ok(())
    }
}

///
/// Internal function for building list of vertex attributes from a `ReflectInterfaceVariable`
///
pub(crate) fn resolve_vertex_attributes(
    interface: impl Iterator<Item = ReflectInterfaceVariable>,
) -> Result<Vec<VertexAttributeReflection>, VertexLayoutResolutionError> {
    let mut attrs = Vec::with_capacity(interface.size_hint().0);
    for m in interface {
        let attribute_type = resolve_interface_member_type(&m)?;

        // Make sure we haven't passed a matrix in as one of the vertex input attachments as we
        // don't support this
        let attribute_type = match attribute_type {
            MemberType::Scalar(scalar) => AttributeType::Scalar(scalar),
            MemberType::Vector(vector) => AttributeType::Vector(vector),
            MemberType::Matrix(_) => return Err(VertexLayoutResolutionError::UnsupportedType),
        };

        let name = m.name;
        let location = m.location;
        let attr = VertexAttributeReflection {
            name,
            location,
            attribute_type,
        };
        attrs.push(attr);
    }
    Ok(attrs)
}

///
/// Internal wrapper for extracting required information for calling `resolve_member_type` from a
/// `ReflectInterfaceVariable`
///
pub(crate) fn resolve_interface_member_type(
    interface: &ReflectInterfaceVariable,
) -> Result<MemberType, MemberResolutionError> {
    let desc = interface.type_description.as_ref().unwrap();
    let decoration_flags = interface.decoration_flags;
    let numeric = &interface.numeric;
    resolve_member_type(desc, decoration_flags, numeric)
}
