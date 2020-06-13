//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use spirv_reflect::types::{
    ReflectBlockVariable, ReflectDecorationFlags, ReflectDescriptorSet, ReflectDescriptorType,
    ReflectNumericTraitsScalar, ReflectTypeFlags,
};

///
/// An enum to represent the different widths of integer values supported
///
#[derive(Debug)]
pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

///
/// An enum to represent the different widths of floating point values supported
///
#[derive(Debug)]
pub enum ScalarType {
    /// FP32 (single precision)
    Float,

    /// FP64 (double precision)
    Double,
}

///
/// A struct to represent a vector type in a uniform buffer
///
#[derive(Debug)]
pub struct VectorInfo {
    /// The type of floating point value this vector is constructed of
    pub fp_type: ScalarType,

    /// The number of elements in the vector
    pub elements: u8,
    // /// TODO: DOCUMENT
    // pub stride: u32,
}

///
/// An enum to represent the possible ways of laying out a matrix in a uniform buffer
///
#[derive(Debug)]
pub enum MatrixLayout {
    /// The matrix is expected to be laid out in column major form
    ColumnMajor,

    /// The matrix is expected to be laid out in row major form
    RowMajor,
}

///
/// A struct to represent a matrix type in a uniform buffer
///
#[derive(Debug)]
pub struct MatrixInfo {
    /// The type of floating point value this matrix is constructed of
    pub fp_type: ScalarType,

    /// The expected layout of the matrix data
    pub layout: MatrixLayout,

    /// The number of rows in the matrix
    pub rows: u8,

    /// The number of columns in the matrix
    pub cols: u8,

    /// The size of a single run of values for a row/column in bytes
    pub stride: u32,
}

///
/// An enum to represent the supported variable types in a uniform buffer
///
#[derive(Debug)]
pub enum MemberType {
    /// A scalar value (i.e a single float)
    Scalar(ScalarType),

    /// A vector value (i.e a float3 or float4)
    Vector(VectorInfo),

    /// A matrix value (i.e a float3x3 or float4x3)
    Matrix(MatrixInfo),
}

///
/// A struct that represents a member variable for a uniform buffer struct
///
#[derive(Debug)]
pub struct Member {
    name: String,
    size: u32,
    size_padded: u32,
    offset: u32,
    offset_absolute: u32,
    member_type: MemberType,
}

impl Member {
    ///
    /// The name of the member variable
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    ///
    /// The size of this member
    ///
    pub fn size(&self) -> u32 {
        self.size
    }

    ///
    /// The size of this member including padding bytes for alignment
    ///
    pub fn size_padded(&self) -> u32 {
        self.size_padded
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    pub fn offset(&self) -> u32 {
        self.offset
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    pub fn offset_absolute(&self) -> u32 {
        self.offset_absolute
    }

    ///
    /// The type of value this member represents
    ///
    pub fn member_type(&self) -> &MemberType {
        &self.member_type
    }
}

///
/// A struct that represents a uniform buffer's struct layout
///
#[derive(Debug)]
pub struct Struct {
    members: Vec<Member>,
    size: u32,
    size_padded: u32,
    offset: u32,
    offset_absolute: u32,
}

impl Struct {
    ///
    /// Slice of the members of this struct
    ///
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    ///
    /// The size of the struct
    ///
    pub fn size(&self) -> u32 {
        self.size
    }

    ///
    /// The size of the struct including padding bytes for alignment
    ///
    pub fn size_padded(&self) -> u32 {
        self.size_padded
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    pub fn offset(&self) -> u32 {
        self.offset
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    pub fn offset_absolute(&self) -> u32 {
        self.offset_absolute
    }
}

///
/// Type that represents the set of supported descriptor bindings that is currently supported
///
#[derive(Debug)]
pub enum BindingType {
    //Undefined,
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    //UniformTexelBuffer,
    //StorageTexelBuffer,
    UniformBuffer(Struct),
    //StorageBuffer(),
    UniformBufferDynamic(Struct),
    //StorageBufferDynamic(),
    InputAttachment,
    AccelerationStructureNV,
}

///
/// A struct that represents a descriptor binding
///
#[derive(Debug)]
pub struct Binding {
    binding: u32,
    name: String,
    binding_type: BindingType,
}

impl Binding {
    ///
    /// Returns the name of the binding
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    ///
    /// Returns the type of this binding
    ///
    pub fn binding_type(&self) -> &BindingType {
        &self.binding_type
    }
}

#[derive(Debug)]
pub struct Set {
    set: u32,
    bindings: Vec<Binding>,
}

impl Set {
    pub fn bindings(&self) -> &[Binding] {
        &self.bindings
    }

    pub fn reflect(mut set: ReflectDescriptorSet) -> Set {
        let bindings = set
            .bindings
            .drain(..)
            .map(|b| {
                let name = b.name;
                let binding_type = match b.descriptor_type {
                    ReflectDescriptorType::StorageBufferDynamic
                    | ReflectDescriptorType::StorageBuffer
                    | ReflectDescriptorType::UniformTexelBuffer
                    | ReflectDescriptorType::StorageTexelBuffer
                    | ReflectDescriptorType::Undefined => panic!("Unsupported descriptor type"),
                    ReflectDescriptorType::Sampler => BindingType::Sampler,
                    ReflectDescriptorType::CombinedImageSampler => {
                        BindingType::CombinedImageSampler
                    }
                    ReflectDescriptorType::SampledImage => BindingType::SampledImage,
                    ReflectDescriptorType::StorageImage => BindingType::StorageImage,
                    ReflectDescriptorType::UniformBuffer => {
                        BindingType::UniformBuffer(Self::resolve_struct(b.block))
                    }
                    ReflectDescriptorType::UniformBufferDynamic => {
                        BindingType::UniformBufferDynamic(Self::resolve_struct(b.block))
                    }
                    ReflectDescriptorType::InputAttachment => BindingType::InputAttachment,
                    ReflectDescriptorType::AccelerationStructureNV => {
                        BindingType::AccelerationStructureNV
                    }
                };
                let binding = b.binding;
                Binding {
                    binding,
                    name,
                    binding_type,
                }
            })
            .collect();

        let set = set.set;
        let out_set = Set { set, bindings };

        out_set
    }

    fn resolve_struct(mut block: ReflectBlockVariable) -> Struct {
        let members = block
            .members
            .drain(..)
            .map(|m| {
                let member_type = Self::resolve_member_type(&m);
                let name = m.name;
                let size = m.size;
                let size_padded = m.padded_size;
                let offset = m.offset;
                let offset_absolute = m.absolute_offset;
                let member = Member {
                    name,
                    size,
                    size_padded,
                    offset,
                    offset_absolute,
                    member_type,
                };
                member
            })
            .collect();

        let item = Struct {
            members,
            size: block.size,
            size_padded: block.padded_size,
            offset: block.offset,
            offset_absolute: block.absolute_offset,
        };

        item
    }

    fn resolve_member_type(block: &ReflectBlockVariable) -> MemberType {
        let desc = block.type_description.as_ref().unwrap();
        let float = desc.type_flags.contains(ReflectTypeFlags::FLOAT);
        let vector = desc.type_flags.contains(ReflectTypeFlags::VECTOR);
        let matrix = desc.type_flags.contains(ReflectTypeFlags::MATRIX);

        if matrix && vector && float {
            let fp_type = Self::resolve_scalar_width(block.numeric.scalar);
            let layout = Self::resolve_matrix_layout(block.decoration_flags);
            let info = MatrixInfo {
                fp_type,
                layout,
                rows: block.numeric.matrix.row_count as _,
                cols: block.numeric.matrix.column_count as _,
                stride: block.numeric.matrix.stride,
            };
            MemberType::Matrix(info)
        } else if vector && float {
            let fp_type = Self::resolve_scalar_width(block.numeric.scalar);
            let info = VectorInfo {
                fp_type,
                elements: block.numeric.vector.component_count as _,
            };
            MemberType::Vector(info)
        } else if float {
            MemberType::Scalar(Self::resolve_scalar_width(block.numeric.scalar))
        } else {
            panic!("Unsupported member type");
        }
    }

    fn resolve_scalar_width(scalar: ReflectNumericTraitsScalar) -> ScalarType {
        match scalar.width {
            32 => ScalarType::Float,
            64 => ScalarType::Double,
            _ => panic!("Unsupported floating point member size"),
        }
    }

    fn resolve_matrix_layout(flags: ReflectDecorationFlags) -> MatrixLayout {
        if flags.contains(ReflectDecorationFlags::COLUMN_MAJOR) {
            MatrixLayout::ColumnMajor
        } else if flags.contains(ReflectDecorationFlags::ROW_MAJOR) {
            MatrixLayout::RowMajor
        } else {
            panic!("Unknown matrix layout");
        }
    }
}
