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

use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_rhi_api::*;
use aleph_shader_db::{Fragment, ShaderName, Vertex};

use crate::{
    BufferPool, IMaterial, Material, MaterialBinding, MaterialBindingType, MaterialInstanceObject,
    TexturePool, shaders,
};

pub struct StandardMaterial();

impl StandardMaterial {
    pub fn new() -> Arc<Material> {
        Material::new(StandardMaterial())
    }
}

unsafe impl IMaterial for StandardMaterial {
    fn frag_name(&self) -> ShaderName<'static, Fragment> {
        shaders::deferred::main_gbuffer_frag()
    }

    fn vert_name(&self) -> ShaderName<'static, Vertex> {
        shaders::deferred::main_gbuffer_vert()
    }

    fn create_parameter_block_layout(
        &self,
        device: &dyn IDevice,
    ) -> AnyArc<dyn IParameterBlockLayout> {
        let desc = ParameterBlockDesc {
            params: &[
                ParameterType::ConstantBuffer.param(),
                ParameterType::Texture2D.param(),
                ParameterType::Texture2D.param(),
                ParameterType::Texture2D.param(),
            ],
            visibility: DescriptorShaderVisibility::All,
            flags: Default::default(),
            name: obj_name_opt!("StandardMaterialSetLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    fn check_binding_type(&self, binding: u32, binding_type: MaterialBindingType) -> bool {
        match (binding, binding_type) {
            (0, MaterialBindingType::Buffer) => true,
            (1, MaterialBindingType::Texture) => true,
            (2, MaterialBindingType::Texture) => true,
            (3, MaterialBindingType::Texture) => true,
            _ => false,
        }
    }

    fn instantiate_bindings(&self) -> Vec<MaterialBinding> {
        let mut out = Vec::with_capacity(4);
        out.push(MaterialBinding::Buffer(None));
        out.push(MaterialBinding::Texture(None));
        out.push(MaterialBinding::Texture(None));
        out.push(MaterialBinding::Texture(None));
        out
    }

    unsafe fn update_parameter_block(
        &self,
        block_layout: &dyn IParameterBlockLayout,
        buffer_pool: &BufferPool,
        texture_pool: &TexturePool,
        device: &dyn IDevice,
        instance: &MaterialInstanceObject,
        dst: ParameterBlockHandle,
    ) {
        let constants = instance.bindings[0].unwrap_buffer().unwrap();
        let constants = buffer_pool.get_ref(constants).unwrap();
        let constants = constants.get().unwrap();

        let image_view_c = instance.bindings[1].unwrap_texture().unwrap();
        let image_view_c = texture_pool.get_ref(image_view_c).unwrap();
        let image_view_c = image_view_c.get_default_view().unwrap();

        let image_view_mr = instance.bindings[2].unwrap_texture().unwrap();
        let image_view_mr = texture_pool.get_ref(image_view_mr).unwrap();
        let image_view_mr = image_view_mr.get_default_view().unwrap();

        let image_view_nrm = instance.bindings[3].unwrap_texture().unwrap();
        let image_view_nrm = texture_pool.get_ref(image_view_nrm).unwrap();
        let image_view_nrm = image_view_nrm.get_default_view().unwrap();

        unsafe {
            let params = [
                BufferWrite::cbv(constants, 256).into(),
                TextureWrite::srv(image_view_c).into(),
                TextureWrite::srv(image_view_mr).into(),
                TextureWrite::srv(image_view_nrm).into(),
            ];
            device.update_parameter_block(block_layout, dst, 0, &params);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::AnyBitPattern, bytemuck::NoUninit)]
pub struct StandardMaterialLayout {
    pub colour: [f32; 4],
    pub metal_roughness: [f32; 4],
    pub _padding1: [u8; 128],
    pub _padding2: [u8; 96],
}
