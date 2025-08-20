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

use crate::{ArchivedParameterBlockDesc, IParameterBlockDesc, ParameterBlockDesc, ShaderType};

pub trait IShaderEntry {
    type PlatformEntry: IShaderPlatformEntry;
    fn get_shader_type(&self) -> ShaderType;
    fn get_spirv(&self) -> &Self::PlatformEntry;
    fn get_dxil(&self) -> &Self::PlatformEntry;
    fn get_msl(&self) -> &Self::PlatformEntry;
}

pub trait IShaderPlatformEntry {
    type ParamBlockDesc: IParameterBlockDesc;
    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc];
    fn get_push_constant_block(&self) -> Option<u64>;
    fn get_bytes(&self) -> &[u8];
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ShaderEntry {
    pub shader_type: ShaderType,
    pub spirv: Option<ShaderPlatformEntry>,
    pub dxil: Option<ShaderPlatformEntry>,
    pub msl: Option<ShaderPlatformEntry>,
}

impl IShaderEntry for ShaderEntry {
    type PlatformEntry = ShaderPlatformEntry;

    #[inline]
    fn get_shader_type(&self) -> ShaderType {
        self.shader_type
    }

    fn get_spirv(&self) -> &Self::PlatformEntry {
        self.spirv.as_ref().unwrap()
    }

    fn get_dxil(&self) -> &Self::PlatformEntry {
        self.dxil.as_ref().unwrap()
    }

    fn get_msl(&self) -> &Self::PlatformEntry {
        self.msl.as_ref().unwrap()
    }
}

impl IShaderEntry for ArchivedShaderEntry {
    type PlatformEntry = ArchivedShaderPlatformEntry;

    #[inline]
    fn get_shader_type(&self) -> ShaderType {
        self.shader_type.into()
    }

    fn get_spirv(&self) -> &Self::PlatformEntry {
        self.spirv.as_ref().unwrap()
    }

    fn get_dxil(&self) -> &Self::PlatformEntry {
        self.dxil.as_ref().unwrap()
    }

    fn get_msl(&self) -> &Self::PlatformEntry {
        self.msl.as_ref().unwrap()
    }
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ShaderPlatformEntry {
    pub parameter_blocks: Vec<ParameterBlockDesc>,
    pub push_constants: Option<u64>,
    pub bytes: Vec<u8>,
}

impl IShaderPlatformEntry for ShaderPlatformEntry {
    type ParamBlockDesc = ParameterBlockDesc;

    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc] {
        self.parameter_blocks.as_slice()
    }

    fn get_push_constant_block(&self) -> Option<u64> {
        self.push_constants
    }

    fn get_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

impl IShaderPlatformEntry for ArchivedShaderPlatformEntry {
    type ParamBlockDesc = ArchivedParameterBlockDesc;

    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc] {
        self.parameter_blocks.as_slice()
    }

    fn get_push_constant_block(&self) -> Option<u64> {
        self.push_constants.as_ref().map(|v| v.to_native())
    }

    fn get_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}
