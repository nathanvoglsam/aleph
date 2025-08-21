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

pub trait IParameterDesc {
    fn get_type(&self) -> ParameterType;
    fn get_array_size(&self) -> u64;
}

pub trait IParameterBlockDesc {
    type ParamDesc: IParameterDesc;
    fn get_parameters(&self) -> &[Self::ParamDesc];
}

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

impl Into<ParameterType> for ArchivedParameterType {
    fn into(self) -> ParameterType {
        match self {
            Self::ConstantBuffer => ParameterType::ConstantBuffer,
            Self::StructuredBuffer => ParameterType::StructuredBuffer,
            Self::RWStructuredBuffer => ParameterType::RWStructuredBuffer,
            Self::ByteAddressBuffer => ParameterType::ByteAddressBuffer,
            Self::RWByteAddressBuffer => ParameterType::RWByteAddressBuffer,
            Self::Buffer => ParameterType::Buffer,
            Self::RWBuffer => ParameterType::RWBuffer,
            Self::Texture1D => ParameterType::Texture1D,
            Self::RWTexture1D => ParameterType::RWTexture1D,
            Self::Texture2D => ParameterType::Texture2D,
            Self::RWTexture2D => ParameterType::RWTexture2D,
            Self::Texture3D => ParameterType::Texture3D,
            Self::RWTexture3D => ParameterType::RWTexture3D,
            Self::Texture1DArray => ParameterType::Texture1DArray,
            Self::RWTexture1DArray => ParameterType::RWTexture1DArray,
            Self::Texture2DArray => ParameterType::Texture2DArray,
            Self::RWTexture2DArray => ParameterType::RWTexture2DArray,
            Self::Texture3DArray => ParameterType::Texture3DArray,
            Self::RWTexture3DArray => ParameterType::RWTexture3DArray,
            Self::Texture2DMS => ParameterType::Texture2DMS,
            Self::RWTexture2DMS => ParameterType::RWTexture2DMS,
            Self::Texture2DMSArray => ParameterType::Texture2DMSArray,
            Self::RWTexture2DMSArray => ParameterType::RWTexture2DMSArray,
            Self::TextureCube => ParameterType::TextureCube,
            Self::TextureCubeArray => ParameterType::TextureCubeArray,
            Self::SamplerState => ParameterType::SamplerState,
            Self::AccelerationStructure => ParameterType::AccelerationStructure,
        }
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

impl IParameterDesc for ParameterDesc {
    fn get_type(&self) -> ParameterType {
        self.ty
    }

    fn get_array_size(&self) -> u64 {
        self.array_size
    }
}

impl IParameterDesc for ArchivedParameterDesc {
    fn get_type(&self) -> ParameterType {
        self.ty.into()
    }

    fn get_array_size(&self) -> u64 {
        self.array_size.to_native()
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ParameterBlockDesc {
    /// Ordered list of parameters that make up the parameter block.
    pub parameters: Vec<ParameterDesc>,
}

impl IParameterBlockDesc for ParameterBlockDesc {
    type ParamDesc = ParameterDesc;

    fn get_parameters(&self) -> &[Self::ParamDesc] {
        self.parameters.as_slice()
    }
}

impl IParameterBlockDesc for ArchivedParameterBlockDesc {
    type ParamDesc = ArchivedParameterDesc;

    fn get_parameters(&self) -> &[Self::ParamDesc] {
        self.parameters.as_slice()
    }
}
