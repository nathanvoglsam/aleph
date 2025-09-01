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

use aleph_rhi_api::{IShaderCodeSource, ParameterArraySize, ParameterDesc, PushConstantBlock};

use crate::{ParameterBlockDesc, ShaderType};

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Default)]
pub struct ShaderEntry {
    pub shader_type: ShaderType,
    pub parameter_blocks: Vec<ParameterBlockDesc>,
    pub push_constants: Option<u64>,
    pub spirv: Option<Vec<u8>>,
    pub dxil: Option<Vec<u8>>,
    pub msl: Option<Vec<u8>>,
}

unsafe impl IShaderCodeSource for ShaderEntry {
    fn shader_type(&self) -> aleph_rhi_api::ShaderType {
        self.shader_type.into()
    }

    fn shader_name(&self) -> &str {
        todo!()
    }

    fn get_spirv(&self) -> &[u8] {
        self.spirv.as_ref().unwrap().as_slice()
    }

    fn get_dxil(&self) -> &[u8] {
        self.dxil.as_ref().unwrap().as_slice()
    }

    fn get_msl(&self) -> &[u8] {
        self.msl.as_ref().unwrap().as_slice()
    }

    fn get_parameter_block_count(&self) -> usize {
        self.parameter_blocks.len()
    }

    fn get_parameter_count_for_block(&self, block: usize) -> usize {
        self.parameter_blocks[block].parameters.len()
    }

    fn get_parameters_for_block(&self, block: usize, dst: &mut [ParameterDesc]) {
        let src = self.parameter_blocks[block].parameters.iter();
        for (i, s) in src.enumerate() {
            dst[i] = ParameterDesc {
                ty: s.ty.into(),
                array_size: ParameterArraySize::new(s.array_size),
                structured_buffer_stride: 0,
            };
        }
    }

    fn get_push_constant_block(&self) -> Option<PushConstantBlock> {
        self.push_constants
            .as_ref()
            .map(|v| {
                let v: u8 = (*v).try_into().ok()?;
                PushConstantBlock::new(v)
            })
            .flatten()
    }
}

unsafe impl IShaderCodeSource for ArchivedShaderEntry {
    fn shader_type(&self) -> aleph_rhi_api::ShaderType {
        self.shader_type.into()
    }

    fn shader_name(&self) -> &str {
        todo!()
    }

    fn get_spirv(&self) -> &[u8] {
        self.spirv.as_ref().unwrap().as_slice()
    }

    fn get_dxil(&self) -> &[u8] {
        self.dxil.as_ref().unwrap().as_slice()
    }

    fn get_msl(&self) -> &[u8] {
        self.msl.as_ref().unwrap().as_slice()
    }

    fn get_parameter_block_count(&self) -> usize {
        self.parameter_blocks.len()
    }

    fn get_parameter_count_for_block(&self, block: usize) -> usize {
        self.parameter_blocks[block].parameters.len()
    }

    fn get_parameters_for_block(&self, block: usize, dst: &mut [ParameterDesc]) {
        let src = self.parameter_blocks[block].parameters.iter();
        for (i, s) in src.enumerate() {
            dst[i] = ParameterDesc {
                ty: s.ty.into(),
                array_size: ParameterArraySize::new(s.array_size.to_native()),
                structured_buffer_stride: 0,
            };
        }
    }

    fn get_push_constant_block(&self) -> Option<PushConstantBlock> {
        self.push_constants
            .as_ref()
            .map(|v| {
                let v: u8 = v.to_native().try_into().ok()?;
                PushConstantBlock::new(v)
            })
            .flatten()
    }
}
