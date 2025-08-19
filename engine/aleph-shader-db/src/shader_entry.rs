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
    type ParamBlockDesc: IParameterBlockDesc;
    fn get_shader_type(&self) -> ShaderType;
    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc];
    fn get_spirv(&self) -> &[u8];
    fn get_dxil(&self) -> &[u8];
    fn get_msl(&self) -> &[u8];
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct ShaderEntry {
    pub shader_type: ShaderType,
    pub paramter_blocks: Vec<ParameterBlockDesc>,
    pub spirv: Vec<u8>,
    pub dxil: Vec<u8>,
    pub msl: Vec<u8>,
}

impl IShaderEntry for ShaderEntry {
    type ParamBlockDesc = ParameterBlockDesc;

    #[inline]
    fn get_shader_type(&self) -> ShaderType {
        self.shader_type
    }

    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc] {
        self.paramter_blocks.as_slice()
    }

    fn get_spirv(&self) -> &[u8] {
        self.spirv.as_slice()
    }

    fn get_dxil(&self) -> &[u8] {
        self.dxil.as_slice()
    }

    fn get_msl(&self) -> &[u8] {
        self.msl.as_slice()
    }
}

impl IShaderEntry for ArchivedShaderEntry {
    type ParamBlockDesc = ArchivedParameterBlockDesc;

    #[inline]
    fn get_shader_type(&self) -> ShaderType {
        self.shader_type.into()
    }

    fn get_parameter_blocks(&self) -> &[Self::ParamBlockDesc] {
        self.paramter_blocks.as_slice()
    }

    fn get_spirv(&self) -> &[u8] {
        self.spirv.as_slice()
    }

    fn get_dxil(&self) -> &[u8] {
        self.dxil.as_slice()
    }

    fn get_msl(&self) -> &[u8] {
        self.msl.as_slice()
    }
}
