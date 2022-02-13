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

use crate::reflect::{
    Binding, BindingType, FloatType, IntegerType, MatrixLayout, MemberType, ScalarType, Struct,
};
use bytemuck::Pod;
use ultraviolet::{DMat4, DVec2, DVec3, DVec4, Mat4, Vec2, Vec3, Vec4};

///
/// Represents the set of errors that creating a new `UniformBufferWriter` object could throw
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WriterCreateError {
    BufferTooSmall,
    WrongBindingType,
}

///
/// The set of errors that can be produced when trying to write a UBO member
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MemberWriteError {
    WrongType,
    MemberNotFound,
}

///
/// The set of errors that can be produced when trying to finalize a writer
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WriterFinalizeError {
    NotAllMembersWritten,
}

///
/// Represents the set of supported UBO member variable types that can be written
///
#[derive(Clone, Debug)]
pub enum Member {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    DVec2(DVec2),
    DVec3(DVec3),
    DVec4(DVec4),
    Mat4x4(Mat4),
    DMat4x4(DMat4),
}

impl Member {
    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_integer<T: Into<IntegerType> + Pod>(
        member_type: &MemberType,
        src: T,
        dest: &mut [u8],
    ) -> bool {
        if let MemberType::Integer(v) = member_type {
            if *v == src.into() {
                Self::write_scalars(&[src], dest)
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_float<T: Into<FloatType> + Pod>(
        member_type: &MemberType,
        src: T,
        dest: &mut [u8],
    ) -> bool {
        if let MemberType::Float(v) = member_type {
            if *v == src.into() {
                Self::write_scalars(&[src], dest)
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_vector<T: Into<FloatType> + Pod>(
        member_type: &MemberType,
        src: &[T],
        dest: &mut [u8],
    ) -> bool {
        if let MemberType::Vector(info) = member_type {
            let elems = src.len() as u8;
            if info.elements == elems && info.elem_type == ScalarType::Float(T::zeroed().into()) {
                Self::write_scalars(src, dest);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_mat4x4(member_type: &MemberType, src: &Mat4, dest: &mut [u8]) -> bool {
        if let MemberType::Matrix(info) = member_type {
            if info.cols == 4 && info.rows == 4 && info.elem_type == FloatType::Single {
                match info.layout {
                    MatrixLayout::ColumnMajor => Self::write_scalars(src.as_slice(), dest),
                    MatrixLayout::RowMajor => {
                        let src = src.transposed();
                        Self::write_scalars(src.as_slice(), dest)
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_dmat4x4(member_type: &MemberType, src: &DMat4, dest: &mut [u8]) -> bool {
        if let MemberType::Matrix(info) = member_type {
            if info.cols == 4 && info.rows == 4 && info.elem_type == FloatType::Double {
                match info.layout {
                    MatrixLayout::ColumnMajor => Self::write_scalars(src.as_slice(), dest),
                    MatrixLayout::RowMajor => {
                        let src = src.transposed();
                        Self::write_scalars(src.as_slice(), dest)
                    }
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Writes an N component vector to the destination buffer, if there's enough space to do so
    ///
    #[inline(always)]
    fn write_scalars<T: Pod>(src: &[T], dest: &mut [u8]) -> bool {
        // Get the vector as an array of bytes
        let bytes = bytemuck::cast_slice::<T, u8>(src);

        // Copy the data if there is enough space
        if let Some(sub_dest) = dest.get_mut(0..bytes.len()) {
            sub_dest.copy_from_slice(bytes);
            true
        } else {
            false
        }
    }

    ///
    /// Internal function for writing
    ///
    #[inline(always)]
    fn write_member_to_memory(
        &self,
        member_type: &MemberType,
        buffer: &mut [u8],
    ) -> Result<(), ()> {
        let success = match self {
            Member::I8(v) => Self::handle_integer(member_type, *v, buffer),
            Member::I16(v) => Self::handle_integer(member_type, *v, buffer),
            Member::I32(v) => Self::handle_integer(member_type, *v, buffer),
            Member::I64(v) => Self::handle_integer(member_type, *v, buffer),
            Member::U8(v) => Self::handle_integer(member_type, *v, buffer),
            Member::U16(v) => Self::handle_integer(member_type, *v, buffer),
            Member::U32(v) => Self::handle_integer(member_type, *v, buffer),
            Member::U64(v) => Self::handle_integer(member_type, *v, buffer),
            Member::F32(v) => Self::handle_float(member_type, *v, buffer),
            Member::F64(v) => Self::handle_float(member_type, *v, buffer),
            Member::Vec2(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::Vec3(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::Vec4(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::DVec2(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::DVec3(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::DVec4(src) => Self::handle_vector(member_type, src.as_slice(), buffer),
            Member::Mat4x4(src) => Self::handle_mat4x4(member_type, src, buffer),
            Member::DMat4x4(src) => Self::handle_dmat4x4(member_type, src, buffer),
        };

        // Map bool to error
        if success {
            Ok(())
        } else {
            Err(())
        }
    }
}

///
/// Utility wrapper for writing a uniform buffer's expected data to a memory buffer that safely
/// handles writing to the buffer, manages offsets and ensures that all resources are written.
///
pub struct UniformBufferWriter<'binding, 'buffer> {
    binding: &'binding Struct,
    buffer: &'buffer mut [u8],
    member_written: Vec<bool>,
}

impl<'binding, 'buffer> UniformBufferWriter<'binding, 'buffer> {
    ///
    /// Creates a new uniform buffer writer from the given binding reflection and a buffer to write
    /// to
    ///
    pub fn new_for_binding(
        binding: &'binding Binding,
        buffer: &'buffer mut [u8],
    ) -> Result<Self, WriterCreateError> {
        match binding.binding_type() {
            BindingType::Sampler => Err(WriterCreateError::WrongBindingType),
            BindingType::CombinedImageSampler => Err(WriterCreateError::WrongBindingType),
            BindingType::SampledImage => Err(WriterCreateError::WrongBindingType),
            BindingType::StorageImage => Err(WriterCreateError::WrongBindingType),
            BindingType::UniformBuffer(binding) => Self::new_for_struct(binding, buffer),
            BindingType::InputAttachment => Err(WriterCreateError::WrongBindingType),
            BindingType::AccelerationStructureNV => Err(WriterCreateError::WrongBindingType),
        }
    }

    ///
    /// Creates a new uniform buffer writer from the given binding reflection and a buffer to write
    /// to
    ///
    pub fn new_for_struct(
        binding: &'binding Struct,
        buffer: &'buffer mut [u8],
    ) -> Result<Self, WriterCreateError> {
        if (binding.size() as usize) < buffer.len() {
            Err(WriterCreateError::BufferTooSmall)
        } else {
            let member_written = Self::member_written_vec(binding);
            Ok(Self {
                binding,
                buffer,
                member_written,
            })
        }
    }

    fn member_written_vec(binding: &Struct) -> Vec<bool> {
        let mut vec = Vec::new();
        vec.resize(binding.members().len(), false);
        vec
    }

    ///
    /// Writes the given value to a member variable of the given name, if it exists.
    ///
    pub fn write_member(&mut self, name: &str, member: Member) -> Result<(), MemberWriteError> {
        // Find the member by name
        let (i, binding_member) = self
            .binding
            .members()
            .iter()
            .enumerate()
            .find(|(_, v)| v.name() == name)
            .ok_or(MemberWriteError::MemberNotFound)?;

        // Get the range of bytes that we'll be writing to
        let offset = binding_member.offset() as usize;
        let offset_end = offset + binding_member.size_padded() as usize;

        //
        let buffer = &mut self.buffer[offset..offset_end];
        let member_type = binding_member.member_type().clone();
        if member.write_member_to_memory(&member_type, buffer).is_err() {
            return Err(MemberWriteError::WrongType);
        }
        self.member_written[i] = true;

        Ok(())
    }

    pub fn write_member_index(
        &mut self,
        index: usize,
        member: Member,
    ) -> Result<(), MemberWriteError> {
        // Ensure that an index'th member exists
        if index >= self.binding.members().len() {
            return Err(MemberWriteError::MemberNotFound);
        }

        // Find the member by index
        let binding_member = &self.binding.members()[index];

        // Get the range of bytes that we'll be writing to
        let offset = binding_member.offset() as usize;
        let offset_end = offset + binding_member.size_padded() as usize;

        //
        let buffer = &mut self.buffer[offset..offset_end];
        let member_type = binding_member.member_type().clone();
        if member.write_member_to_memory(&member_type, buffer).is_err() {
            return Err(MemberWriteError::WrongType);
        }
        self.member_written[index] = true;

        Ok(())
    }

    ///
    /// Finalize the uniform buffer producing errors if writing was invalid.
    ///
    /// This function is important to call to prevent an implicit panic. The invariant of checking
    /// that all members have been written is still upheld by the `Drop` implementation by causing
    /// a panic if the invariant is not met. This function moves that panic to a Result so it can
    /// be checked for and handled manually (you can also just panic explicitly).
    ///
    pub fn finalize(mut self) -> Result<(), WriterFinalizeError> {
        for b in self.member_written.iter() {
            if !(*b) {
                return Err(WriterFinalizeError::NotAllMembersWritten);
            }
        }

        // Empty the vec so the drop implementation wont panic
        self.member_written.clear();

        Ok(())
    }
}

impl<'binding, 'buffer> Drop for UniformBufferWriter<'binding, 'buffer> {
    fn drop(&mut self) {
        for b in self.member_written.iter() {
            if !(*b) {
                panic!("Didn't write all uniform buffer members");
            }
        }
    }
}