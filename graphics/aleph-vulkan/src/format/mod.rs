//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gltf::json::accessor::{ComponentType, Type};
use aleph_vulkan_core::erupt::vk1_0::Format;

///
/// Represents the set of formats that could in theory be requested by a gltf file but have no
/// matching format available under Vulkan.
///
/// Mostly these are just UNORM/SNORM integer types with bit widths greater than 16. Mapping a 32
/// or 64 bit normalized value onto a single precision float will actually reduce the precision so
/// a raw floating point value is actually more efficient (no wasted extra precision) and more
/// flexible (can range outside of 0-1).
///
/// # Info
///
/// We allow non camel case types for this enum so the case matches the case of what the format's
/// name would be if Vulkan specified it. This is just for consistency with Vulkan as I feel that
/// is more important than consistency with Rust in this case.
///
#[allow(non_camel_case_types)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum MissingFormat {
    R32_SNORM,
    R32_UNORM,
    R32G32_SNORM,
    R32G32_UNORM,
    R32G32B32_SNORM,
    R32G32B32_UNORM,
    R32G32B32A32_SNORM,
    R32G32B32A32_UNORM,
}

///
/// Represents the set of errors that can be produced when trying to deduce a vertex format from a
/// gltf accessor object
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum AccessorFormatError {
    ///
    /// If the attribute type is not supported (i.e a matrix)
    ///
    UnsupportedAttributeType(Type),

    ///
    /// There is no format available that matches the requirements.
    ///
    NoFormatAvailable(MissingFormat),
}

///
/// Takes a gltf accessor and returns a vertex format that matches the accessor's specification
///
pub fn format_from_gltf_accessor(
    accessor: &crate::gltf::Accessor,
) -> Result<Format, AccessorFormatError> {
    match accessor.dimensions() {
        Type::Scalar => format_scalar(accessor),
        Type::Vec2 => format_vector2(accessor),
        Type::Vec3 => format_vector3(accessor),
        Type::Vec4 => format_vector4(accessor),
        v @ Type::Mat2 => Err(AccessorFormatError::UnsupportedAttributeType(v)),
        v @ Type::Mat3 => Err(AccessorFormatError::UnsupportedAttributeType(v)),
        v @ Type::Mat4 => Err(AccessorFormatError::UnsupportedAttributeType(v)),
    }
}

fn format_scalar(accessor: &crate::gltf::Accessor) -> Result<Format, AccessorFormatError> {
    match accessor.data_type() {
        ComponentType::I8 => {
            if accessor.normalized() {
                Ok(Format::R8_SNORM)
            } else {
                Ok(Format::R8_SINT)
            }
        }
        ComponentType::U8 => {
            if accessor.normalized() {
                Ok(Format::R8_UNORM)
            } else {
                Ok(Format::R8_UINT)
            }
        }
        ComponentType::I16 => {
            if accessor.normalized() {
                Ok(Format::R16_SNORM)
            } else {
                Ok(Format::R16_SINT)
            }
        }
        ComponentType::U16 => {
            if accessor.normalized() {
                Ok(Format::R16_UNORM)
            } else {
                Ok(Format::R16_UINT)
            }
        }
        ComponentType::U32 => {
            if accessor.normalized() {
                Err(AccessorFormatError::NoFormatAvailable(
                    MissingFormat::R32_UNORM,
                ))
            } else {
                Ok(Format::R32_UINT)
            }
        }
        ComponentType::F32 => Ok(Format::R32_SFLOAT),
    }
}

fn format_vector2(accessor: &crate::gltf::Accessor) -> Result<Format, AccessorFormatError> {
    match accessor.data_type() {
        ComponentType::I8 => {
            if accessor.normalized() {
                Ok(Format::R8G8_SNORM)
            } else {
                Ok(Format::R8G8_SINT)
            }
        }
        ComponentType::U8 => {
            if accessor.normalized() {
                Ok(Format::R8G8_UNORM)
            } else {
                Ok(Format::R8G8_UINT)
            }
        }
        ComponentType::I16 => {
            if accessor.normalized() {
                Ok(Format::R16G16_SNORM)
            } else {
                Ok(Format::R16G16_SINT)
            }
        }
        ComponentType::U16 => {
            if accessor.normalized() {
                Ok(Format::R16G16_UNORM)
            } else {
                Ok(Format::R16G16_UINT)
            }
        }
        ComponentType::U32 => {
            if accessor.normalized() {
                Err(AccessorFormatError::NoFormatAvailable(
                    MissingFormat::R32G32_UNORM,
                ))
            } else {
                Ok(Format::R32G32_UINT)
            }
        }
        ComponentType::F32 => Ok(Format::R32G32_SFLOAT),
    }
}

fn format_vector3(accessor: &crate::gltf::Accessor) -> Result<Format, AccessorFormatError> {
    match accessor.data_type() {
        ComponentType::I8 => {
            if accessor.normalized() {
                Ok(Format::R8G8B8_SNORM)
            } else {
                Ok(Format::R8G8B8_SINT)
            }
        }
        ComponentType::U8 => {
            if accessor.normalized() {
                Ok(Format::R8G8B8_UNORM)
            } else {
                Ok(Format::R8G8B8_UINT)
            }
        }
        ComponentType::I16 => {
            if accessor.normalized() {
                Ok(Format::R16G16B16_SNORM)
            } else {
                Ok(Format::R16G16B16_SINT)
            }
        }
        ComponentType::U16 => {
            if accessor.normalized() {
                Ok(Format::R16G16B16_UNORM)
            } else {
                Ok(Format::R16G16B16_UINT)
            }
        }
        ComponentType::U32 => {
            if accessor.normalized() {
                Err(AccessorFormatError::NoFormatAvailable(
                    MissingFormat::R32G32B32_UNORM,
                ))
            } else {
                Ok(Format::R32G32B32_UINT)
            }
        }
        ComponentType::F32 => Ok(Format::R32G32B32_SFLOAT),
    }
}

fn format_vector4(accessor: &crate::gltf::Accessor) -> Result<Format, AccessorFormatError> {
    match accessor.data_type() {
        ComponentType::I8 => {
            if accessor.normalized() {
                Ok(Format::R8G8B8A8_SNORM)
            } else {
                Ok(Format::R8G8B8A8_SINT)
            }
        }
        ComponentType::U8 => {
            if accessor.normalized() {
                Ok(Format::R8G8B8A8_UNORM)
            } else {
                Ok(Format::R8G8B8A8_UINT)
            }
        }
        ComponentType::I16 => {
            if accessor.normalized() {
                Ok(Format::R16G16B16A16_SNORM)
            } else {
                Ok(Format::R16G16B16A16_SINT)
            }
        }
        ComponentType::U16 => {
            if accessor.normalized() {
                Ok(Format::R16G16B16A16_UNORM)
            } else {
                Ok(Format::R16G16B16A16_UINT)
            }
        }
        ComponentType::U32 => {
            if accessor.normalized() {
                Err(AccessorFormatError::NoFormatAvailable(
                    MissingFormat::R32G32B32A32_UNORM,
                ))
            } else {
                Ok(Format::R32G32B32A32_UINT)
            }
        }
        ComponentType::F32 => Ok(Format::R32G32B32A32_SFLOAT),
    }
}
