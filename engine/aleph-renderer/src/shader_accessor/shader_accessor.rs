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

use crate::shader_accessor::map_shader_type;

pub trait IShaderAccessor {
    fn get_stage_by_name(&self, name: &str) -> Option<aleph_rhi_api::ShaderStage>;
}

#[derive(Clone)]
pub struct ShaderAccessor<'a, T: IShaderDatabase> {
    loader: fn(&'a T, &str) -> Option<aleph_rhi_api::ShaderStage<'a>>,
    db: &'a T,
}

impl<'a, T: IShaderDatabase> ShaderAccessor<'a, T> {
    pub fn new(device: &dyn IDevice, db: &'a T) -> Self {
        let backend = device.get_backend_api();
        let loader = match backend {
            BackendAPI::Vulkan => vulkan_loader,
            BackendAPI::D3D12 => d3d12_loader,
            BackendAPI::Metal => metal_loader,
            BackendAPI::Null => vulkan_loader, // Just give it spirv because it doesn't matter
        };
        Self { loader, db }
    }
}

impl<'a, T: IShaderDatabase> IShaderAccessor for ShaderAccessor<'a, T> {
    fn get_stage_by_name(&self, name: &str) -> Option<aleph_rhi_api::ShaderStage> {
        (self.loader)(self.db, name)
    }
}

fn vulkan_loader<'a, T: IShaderDatabase>(
    db: &'a T,
    name: &str,
) -> Option<aleph_rhi_api::ShaderStage<'a>> {
    let v = db.get_by_name(name)?;
    Some(aleph_rhi_api::ShaderStage {
        stage: map_shader_type(v.get_shader_type()),
        data: ShaderBinary::Spirv(v.get_spirv().get_bytes()),
    })
}

fn d3d12_loader<'a, T: IShaderDatabase>(
    db: &'a T,
    name: &str,
) -> Option<aleph_rhi_api::ShaderStage<'a>> {
    let v = db.get_by_name(name)?;
    Some(aleph_rhi_api::ShaderStage {
        stage: map_shader_type(v.get_shader_type()),
        data: ShaderBinary::Dxil(v.get_dxil().get_bytes()),
    })
}

fn metal_loader<'a, T: IShaderDatabase>(
    db: &'a T,
    name: &str,
) -> Option<aleph_rhi_api::ShaderStage<'a>> {
    let v = db.get_by_name(name)?;
    Some(aleph_rhi_api::ShaderStage {
        stage: map_shader_type(v.get_shader_type()),
        data: ShaderBinary::Spirv(v.get_spirv().get_bytes()), // TODO: fix binary type
    })
}
