//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::reflect::{MatrixInfo, MatrixLayout, MemberType, ScalarType, VectorInfo};
use spirv_reflect::types::{
    ReflectDecorationFlags, ReflectNumericTraits, ReflectNumericTraitsScalar,
    ReflectTypeDescription, ReflectTypeFlags,
};

///
/// Internal util for getting a member type from the given reflection daata
///
pub(crate) fn resolve_member_type(
    desc: &ReflectTypeDescription,
    decoration_flags: ReflectDecorationFlags,
    numeric: &ReflectNumericTraits,
) -> MemberType {
    let float = desc.type_flags.contains(ReflectTypeFlags::FLOAT);
    let vector = desc.type_flags.contains(ReflectTypeFlags::VECTOR);
    let matrix = desc.type_flags.contains(ReflectTypeFlags::MATRIX);

    if matrix && vector && float {
        let fp_type = resolve_scalar_width(numeric.scalar);
        let layout = resolve_matrix_layout(decoration_flags);
        let info = MatrixInfo {
            fp_type,
            layout,
            rows: numeric.matrix.row_count as _,
            cols: numeric.matrix.column_count as _,
            stride: numeric.matrix.stride,
        };
        MemberType::Matrix(info)
    } else if vector && float {
        let fp_type = resolve_scalar_width(numeric.scalar);
        let info = VectorInfo {
            fp_type,
            elements: numeric.vector.component_count as _,
        };
        MemberType::Vector(info)
    } else if float {
        MemberType::Scalar(resolve_scalar_width(numeric.scalar))
    } else {
        panic!("Unsupported member type");
    }
}

///
/// Reduce the scalar width integer to an enum
///
pub(crate) fn resolve_scalar_width(scalar: ReflectNumericTraitsScalar) -> ScalarType {
    match scalar.width {
        32 => ScalarType::Float,
        64 => ScalarType::Double,
        _ => panic!("Unsupported floating point member size"),
    }
}

///
/// Get whether matrix is column or row major from decoration flags
///
pub(crate) fn resolve_matrix_layout(flags: ReflectDecorationFlags) -> MatrixLayout {
    if flags.contains(ReflectDecorationFlags::COLUMN_MAJOR) {
        MatrixLayout::ColumnMajor
    } else if flags.contains(ReflectDecorationFlags::ROW_MAJOR) {
        MatrixLayout::RowMajor
    } else {
        panic!("Unknown matrix layout");
    }
}
