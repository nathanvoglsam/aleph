//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::reflect::structure::ScalarType;
use crate::reflect::{FloatType, IntegerType, MatrixInfo, MatrixLayout, MemberType, VectorInfo};
use spirv_reflect::types::{
    ReflectDecorationFlags, ReflectNumericTraits, ReflectNumericTraitsScalar,
    ReflectTypeDescription, ReflectTypeFlags,
};

///
/// Represents the set of errors that can be thrown when reflecting a member variable
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MemberResolutionError {
    ///
    /// The type decorations did not specify the layout of the matrix so we could not resolve a
    /// layout.
    ///
    /// TODO: This error could be solved by assuming a default from the source language but wouldn't
    ///       be very robust against custom shader languages
    ///
    UnknownMatrixLayout,

    ///
    /// The scalar width for a type (bit width) is not of a number we understand. This bundles the
    /// bit width that caused the error.
    ///
    UnknownScalarWidth(u32),

    ///
    /// Type type of a member variable is not of a type supported by block or interface variables.
    /// For example, this error will be thrown if somehow a texture and/or sampler ended up in a
    /// uniform buffer layout
    ///
    UnsupportedMemberType,
}

///
/// Internal util for getting a member type from the given reflection daata
///
pub(crate) fn resolve_member_type(
    desc: &ReflectTypeDescription,
    decoration_flags: ReflectDecorationFlags,
    numeric: &ReflectNumericTraits,
) -> Result<MemberType, MemberResolutionError> {
    let int = desc.type_flags.contains(ReflectTypeFlags::INT);
    let float = desc.type_flags.contains(ReflectTypeFlags::FLOAT);
    let vector = desc.type_flags.contains(ReflectTypeFlags::VECTOR);
    let matrix = desc.type_flags.contains(ReflectTypeFlags::MATRIX);

    if matrix && vector && float {
        let fp_type = resolve_float_width(numeric.scalar)?;
        let layout = resolve_matrix_layout(decoration_flags)?;
        let info = MatrixInfo {
            elem_type: fp_type,
            layout,
            rows: numeric.matrix.row_count as _,
            cols: numeric.matrix.column_count as _,
            stride: numeric.matrix.stride,
        };
        Ok(MemberType::Matrix(info))
    } else if vector && float {
        let fp_type = resolve_float_width(numeric.scalar)?;
        let info = VectorInfo {
            elem_type: ScalarType::Float(fp_type),
            elements: numeric.vector.component_count as _,
        };
        Ok(MemberType::Vector(info))
    } else if vector && int {
        let int_type = resolve_int_width(numeric.scalar)?;
        let info = VectorInfo {
            elem_type: ScalarType::Integer(int_type),
            elements: numeric.vector.component_count as _,
        };
        Ok(MemberType::Vector(info))
    } else if float {
        Ok(MemberType::Float(resolve_float_width(numeric.scalar)?))
    } else if int {
        Ok(MemberType::Integer(resolve_int_width(numeric.scalar)?))
    } else {
        Err(MemberResolutionError::UnsupportedMemberType)
    }
}

///
/// Reduce the scalar width to an enum for a floating point type
///
pub(crate) fn resolve_float_width(
    scalar: ReflectNumericTraitsScalar,
) -> Result<FloatType, MemberResolutionError> {
    match scalar.width {
        8 => Ok(FloatType::Quarter),
        16 => Ok(FloatType::Half),
        32 => Ok(FloatType::Single),
        64 => Ok(FloatType::Double),
        _ => Err(MemberResolutionError::UnknownScalarWidth(scalar.width)),
    }
}

///
/// Reduce the scalar width to an enum for an integer type
///
pub(crate) fn resolve_int_width(
    scalar: ReflectNumericTraitsScalar,
) -> Result<IntegerType, MemberResolutionError> {
    match scalar.width {
        8 => {
            if scalar.signedness != 0 {
                Ok(IntegerType::I8)
            } else {
                Ok(IntegerType::U8)
            }
        }
        16 => {
            if scalar.signedness != 0 {
                Ok(IntegerType::I16)
            } else {
                Ok(IntegerType::U16)
            }
        }
        32 => {
            if scalar.signedness != 0 {
                Ok(IntegerType::I32)
            } else {
                Ok(IntegerType::U32)
            }
        }
        64 => {
            if scalar.signedness != 0 {
                Ok(IntegerType::I64)
            } else {
                Ok(IntegerType::U64)
            }
        }
        _ => Err(MemberResolutionError::UnknownScalarWidth(scalar.width)),
    }
}

///
/// Get whether matrix is column or row major from decoration flags
///
pub(crate) fn resolve_matrix_layout(
    flags: ReflectDecorationFlags,
) -> Result<MatrixLayout, MemberResolutionError> {
    if flags.contains(ReflectDecorationFlags::COLUMN_MAJOR) {
        Ok(MatrixLayout::ColumnMajor)
    } else if flags.contains(ReflectDecorationFlags::ROW_MAJOR) {
        Ok(MatrixLayout::RowMajor)
    } else {
        Err(MemberResolutionError::UnknownMatrixLayout)
    }
}
