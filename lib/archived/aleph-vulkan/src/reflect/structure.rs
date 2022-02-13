//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use crate::reflect::member_resolution::resolve_member_type;
use crate::reflect::MemberResolutionError;
use spirv_reflect::types::ReflectBlockVariable;

///
/// An enum to represent the different widths of integer values supported
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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

impl From<i8> for IntegerType {
    #[inline(always)]
    fn from(_: i8) -> Self {
        IntegerType::I8
    }
}

impl From<i16> for IntegerType {
    #[inline(always)]
    fn from(_: i16) -> Self {
        IntegerType::I16
    }
}

impl From<i32> for IntegerType {
    #[inline(always)]
    fn from(_: i32) -> Self {
        IntegerType::I32
    }
}

impl From<i64> for IntegerType {
    #[inline(always)]
    fn from(_: i64) -> Self {
        IntegerType::I64
    }
}

impl From<u8> for IntegerType {
    #[inline(always)]
    fn from(_: u8) -> Self {
        IntegerType::U8
    }
}

impl From<u16> for IntegerType {
    #[inline(always)]
    fn from(_: u16) -> Self {
        IntegerType::U16
    }
}

impl From<u32> for IntegerType {
    #[inline(always)]
    fn from(_: u32) -> Self {
        IntegerType::U32
    }
}

impl From<u64> for IntegerType {
    #[inline(always)]
    fn from(_: u64) -> Self {
        IntegerType::U64
    }
}

///
/// An enum to represent the different widths of floating point values supported
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FloatType {
    /// 8 bit type
    Quarter,

    /// 16 bit type
    Half,

    /// 32 bit type
    Single,

    /// 64 bit ype
    Double,
}

impl From<f32> for FloatType {
    #[inline(always)]
    fn from(_: f32) -> Self {
        FloatType::Single
    }
}

impl From<f64> for FloatType {
    #[inline(always)]
    fn from(_: f64) -> Self {
        FloatType::Double
    }
}

///
///
/// An enum to represent a union between integers and floats as a singular "scalar" concept
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ScalarType {
    ///
    /// An integer
    ///
    Integer(IntegerType),

    ///
    /// A floating point value
    ///
    Float(FloatType),
}

///
/// A struct to represent a vector type in a uniform buffer
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct VectorInfo {
    /// The type of floating point value this vector is constructed of
    pub elem_type: ScalarType,

    /// The number of elements in the vector
    pub elements: u8,
}

///
/// An enum to represent the possible ways of laying out a matrix in a uniform buffer
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum MatrixLayout {
    /// The matrix is expected to be laid out in column major form
    ColumnMajor,

    /// The matrix is expected to be laid out in row major form
    RowMajor,
}

///
/// A struct to represent a matrix type in a uniform buffer
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct MatrixInfo {
    /// The type of floating point value this matrix is constructed of
    pub elem_type: FloatType,

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
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum MemberType {
    /// An integer value
    Integer(IntegerType),

    /// A floating point value (i.e a single float)
    Float(FloatType),

    /// A vector value (i.e a float3 or float4)
    Vector(VectorInfo),

    /// A matrix value (i.e a float3x3 or float4x3)
    Matrix(MatrixInfo),
}

///
/// A struct that represents a member variable for a uniform buffer struct
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Member {
    pub(crate) name: String,
    pub(crate) size: u32,
    pub(crate) size_padded: u32,
    pub(crate) offset: u32,
    pub(crate) offset_absolute: u32,
    pub(crate) member_type: MemberType,
}

impl Member {
    ///
    /// The name of the member variable
    ///
    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }

    ///
    /// The size of this member
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the member was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.size
    }

    ///
    /// The size of this member including padding bytes for alignment
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the member was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn size_padded(&self) -> u32 {
        self.size_padded
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the member was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.offset
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the member was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn offset_absolute(&self) -> u32 {
        self.offset_absolute
    }

    ///
    /// The type of value this member represents
    ///
    #[inline(always)]
    pub fn member_type(&self) -> &MemberType {
        &self.member_type
    }
}

///
/// Represents the set of errors that can be produced when resolving a shader struct layout
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum StructResolutionError {
    ///
    /// An error was encountered resolving a member variable
    ///
    MemberResolutionError(MemberResolutionError),
}

impl From<MemberResolutionError> for StructResolutionError {
    #[inline(always)]
    fn from(other: MemberResolutionError) -> Self {
        StructResolutionError::MemberResolutionError(other)
    }
}

///
/// A struct that represents a uniform buffer's struct layout
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Struct {
    pub(crate) members: Vec<Member>,
    pub(crate) size: u32,
    pub(crate) size_padded: u32,
    pub(crate) offset: u32,
    pub(crate) offset_absolute: u32,
}

impl Struct {
    ///
    /// Slice of the members of this struct
    ///
    #[inline(always)]
    pub fn members(&self) -> &[Member] {
        &self.members
    }

    ///
    /// The size of the struct
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the struct was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.size
    }

    ///
    /// The size of the struct including padding bytes for alignment
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the struct was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn size_padded(&self) -> u32 {
        self.size_padded
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the struct was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn offset(&self) -> u32 {
        self.offset
    }

    ///
    /// The offset from the beginning of the struct of this member
    ///
    /// # Warning
    ///
    /// May return zero when size has no real meaning in the context the struct was reflected from,
    /// such as from a vertex input layout. Vertex layout is not defined by the shader's input
    /// description so size has no meaning. The vertex input's memory layout is defined by the API
    /// and not the shader.
    ///
    #[inline(always)]
    pub fn offset_absolute(&self) -> u32 {
        self.offset_absolute
    }
}

///
/// Internal function for resolving a `ReflectBlockVariable` into a `Struct` description
///
pub(crate) fn resolve_struct_block(
    mut block: ReflectBlockVariable,
) -> Result<Struct, StructResolutionError> {
    let mut members = Vec::with_capacity(block.members.len());
    for m in block.members.drain(..) {
        let member_type = resolve_block_member_type(&m)?;
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
        members.push(member);
    }

    let item = Struct {
        members,
        size: block.size,
        size_padded: block.padded_size,
        offset: block.offset,
        offset_absolute: block.absolute_offset,
    };

    Ok(item)
}

///
/// Internal wrapper for extracting required information for calling `resolve_member_type` from a
/// `ReflectBlockVariable`
///
pub(crate) fn resolve_block_member_type(
    block: &ReflectBlockVariable,
) -> Result<MemberType, MemberResolutionError> {
    let desc = block.type_description.as_ref().unwrap();
    let decoration_flags = block.decoration_flags;
    let numeric = &block.numeric;
    resolve_member_type(desc, decoration_flags, numeric)
}