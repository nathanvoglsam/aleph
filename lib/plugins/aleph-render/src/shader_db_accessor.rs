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

use aleph_rhi_api::*;
use aleph_shader_db::*;

// A wrapper struct
pub struct ShaderDatabaseAccessor<'a> {
    loader: fn(shader_db: &'a dyn IShaderDatabase, &str) -> Option<ApiShaderEntry<'a>>,
    shader_db: &'a dyn IShaderDatabase,
}

impl<'a> ShaderDatabaseAccessor<'a> {
    pub fn new(device: &dyn IDevice, shader_db: &'a dyn IShaderDatabase) -> Self {
        let backend = device.get_backend_api();
        let loader = match backend {
            BackendAPI::Vulkan => vulkan_loader,
            BackendAPI::D3D12 => d3d12_loader,
            BackendAPI::Null => vulkan_loader, // Just give it spirv because it doesn't matter
        };
        Self { loader, shader_db }
    }

    pub fn load_stage<'b: 'a, S: aleph_shader_db::ShaderStage>(
        &self,
        name: ShaderName<'b, S>,
    ) -> Option<aleph_rhi_api::ShaderStage<'a>> {
        let loaded = (self.loader)(self.shader_db, name.into());
        if let Some(loaded) = loaded {
            if S::SHADER_TYPE == loaded.shader_type {
                let out = aleph_rhi_api::ShaderStage {
                    stage: map_shader_type(S::SHADER_TYPE),
                    data: loaded.data,
                };
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn load_data<'b: 'a, S: aleph_shader_db::ShaderStage>(
        &self,
        name: ShaderName<'b, S>,
    ) -> Option<ShaderBinary<'a>> {
        let loaded = (self.loader)(self.shader_db, name.into());
        if let Some(loaded) = loaded {
            if S::SHADER_TYPE == loaded.shader_type {
                Some(loaded.data)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> IShaderDatabase for ShaderDatabaseAccessor<'a> {
    fn get_by_name(&self, name: &str) -> Option<ShaderEntryRef> {
        self.shader_db.get_by_name(name)
    }
}

struct ApiShaderEntry<'a> {
    pub shader_type: aleph_shader_db::ShaderType,
    pub data: ShaderBinary<'a>,
}

fn vulkan_loader<'a>(shader_db: &'a dyn IShaderDatabase, name: &str) -> Option<ApiShaderEntry<'a>> {
    shader_db.get_by_name(name).map(|v| ApiShaderEntry {
        shader_type: v.shader_type,
        data: ShaderBinary::Spirv(v.spirv),
    })
}

fn d3d12_loader<'a>(shader_db: &'a dyn IShaderDatabase, name: &str) -> Option<ApiShaderEntry<'a>> {
    shader_db.get_by_name(name).map(|v| ApiShaderEntry {
        shader_type: v.shader_type,
        data: ShaderBinary::Dxil(v.dxil),
    })
}

const fn map_shader_type(from: aleph_shader_db::ShaderType) -> aleph_rhi_api::ShaderType {
    match from {
        aleph_shader_db::ShaderType::Compute => aleph_rhi_api::ShaderType::Compute,
        aleph_shader_db::ShaderType::Vertex => aleph_rhi_api::ShaderType::Vertex,
        aleph_shader_db::ShaderType::Hull => aleph_rhi_api::ShaderType::Hull,
        aleph_shader_db::ShaderType::Domain => aleph_rhi_api::ShaderType::Domain,
        aleph_shader_db::ShaderType::Geometry => aleph_rhi_api::ShaderType::Geometry,
        aleph_shader_db::ShaderType::Fragment => aleph_rhi_api::ShaderType::Fragment,
        aleph_shader_db::ShaderType::Amplification => aleph_rhi_api::ShaderType::Amplification,
        aleph_shader_db::ShaderType::Mesh => aleph_rhi_api::ShaderType::Mesh,
    }
}
