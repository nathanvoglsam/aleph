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
use aleph_math::matrix::TMat4x4;
use aleph_math::traits::{Real, Transpose};

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
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    DVec2([f64; 2]),
    DVec3([f64; 3]),
    DVec4([f64; 4]),
    Mat4x4(aleph_math::types::Mat4x4),
    DMat4x4(aleph_math::types::DMat4x4),
    Quat([f32; 4]),
    DQuat([f64; 4]),
}

impl Member {
    ///
    /// Internal function for handling checking for matching type and handing off to another
    /// function to write the actual data
    ///
    #[inline(always)]
    fn handle_integer<T: Into<IntegerType> + Copy>(
        member_type: &MemberType,
        src: T,
        dest: &mut [u8],
    ) -> bool {
        if let MemberType::Integer(v) = member_type {
            if *v == src.into() {
                Self::write_scalar(src, dest);
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
    fn handle_float<T: Into<FloatType> + Copy>(
        member_type: &MemberType,
        src: T,
        dest: &mut [u8],
    ) -> bool {
        if let MemberType::Float(v) = member_type {
            if *v == src.into() {
                Self::write_scalar(src, dest);
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
    fn handle_vector<T>(member_type: &MemberType, src: &[T], dest: &mut [u8]) -> bool {
        if let MemberType::Vector(info) = member_type {
            let len = src.len() as u8;
            if info.elements == len && info.elem_type == ScalarType::Float(FloatType::Single) {
                Self::write_vector(src, dest);
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
    fn handle_mat4x4<T: Real>(member_type: &MemberType, src: &TMat4x4<T>, dest: &mut [u8]) -> bool {
        if let MemberType::Matrix(info) = member_type {
            if info.cols == 4 && info.rows == 4 && info.elem_type == FloatType::Single {
                match info.layout {
                    MatrixLayout::ColumnMajor => {
                        Self::write_mat4x4(src, dest);
                    }
                    MatrixLayout::RowMajor => {
                        let src = src.clone().transpose();
                        Self::write_mat4x4(&src, dest);
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    /// Writes a single scalar value to the destination buffer, if there's enough space to do so
    ///
    #[inline(always)]
    fn write_scalar<T>(src: T, dest: &mut [u8]) {
        let count = core::mem::size_of::<T>();
        let src = &src as *const T;
        assert!(dest.len() >= count);
        unsafe {
            dest.as_mut_ptr().copy_from(src as *const u8, count);
        }
    }

    ///
    /// Writes an N component vector to the destination buffer, if there's enough space to do so
    ///
    #[inline(always)]
    fn write_vector<T>(src: &[T], dest: &mut [u8]) {
        let count = core::mem::size_of::<T>() * src.len();
        let src = src.as_ptr();
        assert!(dest.len() >= count);
        unsafe {
            dest.as_mut_ptr().copy_from(src as *const u8, count);
        }
    }

    ///
    /// Writes an 4x4 matrix to the destination buffer, if there's enough space to do so
    ///
    #[inline(always)]
    fn write_mat4x4<T: Real>(src: &TMat4x4<T>, dest: &mut [u8]) {
        let src: &[T; 16] = src.as_slice();

        let count = core::mem::size_of::<T>() * src.len();
        let src = src.as_ptr();
        assert!(dest.len() >= count);
        unsafe {
            dest.as_mut_ptr().copy_from(src as *const u8, count);
        }
    }

    ///
    /// Internal function for writing
    ///
    #[inline(always)]
    fn write_member_to_memory(&self, member_type: &MemberType, buffer: &mut [u8]) -> bool {
        match self {
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
            Member::Vec2(src) => Self::handle_vector(member_type, src, buffer),
            Member::Vec3(src) => Self::handle_vector(member_type, src, buffer),
            Member::Vec4(src) => Self::handle_vector(member_type, src, buffer),
            Member::Quat(src) => Self::handle_vector(member_type, src, buffer),
            Member::DVec2(src) => Self::handle_vector(member_type, src, buffer),
            Member::DVec3(src) => Self::handle_vector(member_type, src, buffer),
            Member::DQuat(src) => Self::handle_vector(member_type, src, buffer),
            Member::DVec4(src) => Self::handle_vector(member_type, src, buffer),
            Member::Mat4x4(src) => Self::handle_mat4x4(member_type, src, buffer),
            Member::DMat4x4(src) => Self::handle_mat4x4(member_type, src, buffer),
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
        if !member.write_member_to_memory(&member_type, buffer) {
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
        if !member.write_member_to_memory(&member_type, buffer) {
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
            if *b == false {
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
            if *b == false {
                panic!("Didn't write all uniform buffer members");
            }
        }
    }
}
