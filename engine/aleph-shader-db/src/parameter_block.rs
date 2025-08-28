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

#[derive(
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
)]
#[rkyv(compare(PartialEq), derive(Copy, Clone))]
pub enum ParameterType {
    ConstantBuffer,
    StructuredBuffer,
    RWStructuredBuffer,
    ByteAddressBuffer,
    RWByteAddressBuffer,
    Buffer,
    RWBuffer,
    Texture1D,
    RWTexture1D,
    Texture2D,
    RWTexture2D,
    Texture3D,
    RWTexture3D,
    Texture1DArray,
    RWTexture1DArray,
    Texture2DArray,
    RWTexture2DArray,
    Texture3DArray,
    RWTexture3DArray,
    Texture2DMS,
    RWTexture2DMS,
    Texture2DMSArray,
    RWTexture2DMSArray,
    TextureCube,
    TextureCubeArray,
    SamplerState,
    AccelerationStructure,
}

macro_rules! convert_parameter_type_match {
    ($sself: ident, $dst_t: path) => {{
        type T = $dst_t;
        match $sself {
            Self::ConstantBuffer => T::ConstantBuffer,
            Self::StructuredBuffer => T::StructuredBuffer,
            Self::RWStructuredBuffer => T::RWStructuredBuffer,
            Self::ByteAddressBuffer => T::ByteAddressBuffer,
            Self::RWByteAddressBuffer => T::RWByteAddressBuffer,
            Self::Buffer => T::Buffer,
            Self::RWBuffer => T::RWBuffer,
            Self::Texture1D => T::Texture1D,
            Self::RWTexture1D => T::RWTexture1D,
            Self::Texture2D => T::Texture2D,
            Self::RWTexture2D => T::RWTexture2D,
            Self::Texture3D => T::Texture3D,
            Self::RWTexture3D => T::RWTexture3D,
            Self::Texture1DArray => T::Texture1DArray,
            Self::RWTexture1DArray => T::RWTexture1DArray,
            Self::Texture2DArray => T::Texture2DArray,
            Self::RWTexture2DArray => T::RWTexture2DArray,
            Self::Texture3DArray => T::Texture3DArray,
            Self::RWTexture3DArray => T::RWTexture3DArray,
            Self::Texture2DMS => T::Texture2DMS,
            Self::RWTexture2DMS => T::RWTexture2DMS,
            Self::Texture2DMSArray => T::Texture2DMSArray,
            Self::RWTexture2DMSArray => T::RWTexture2DMSArray,
            Self::TextureCube => T::TextureCube,
            Self::TextureCubeArray => T::TextureCubeArray,
            Self::SamplerState => T::SamplerState,
            Self::AccelerationStructure => T::AccelerationStructure,
        }
    }};
}

impl Into<ParameterType> for ArchivedParameterType {
    #[inline]
    fn into(self) -> ParameterType {
        convert_parameter_type_match!(self, ParameterType)
    }
}

impl Into<aleph_rhi_api::ParameterType> for ParameterType {
    #[inline]
    fn into(self) -> aleph_rhi_api::ParameterType {
        convert_parameter_type_match!(self, aleph_rhi_api::ParameterType)
    }
}

impl Into<aleph_rhi_api::ParameterType> for ArchivedParameterType {
    #[inline]
    fn into(self) -> aleph_rhi_api::ParameterType {
        convert_parameter_type_match!(self, aleph_rhi_api::ParameterType)
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParameterDesc {
    /// The type of the parameter
    pub ty: ParameterType,

    /// If this is a descriptor array, this is the size of the descriptor array.
    ///
    /// If 'array_size = 0' then this encodes a singular non-array parameter.
    pub array_size: u64,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParameterBlockDesc {
    /// Ordered list of parameters that make up the parameter block.
    pub parameters: Vec<ParameterDesc>,
}
