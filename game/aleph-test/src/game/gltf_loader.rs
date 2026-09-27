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

use aleph_device_allocators::UploadBumpAllocator;
use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::components::{StaticMesh, Transform};
use aleph_engine::interfaces::ecs::World;
use aleph_engine::interfaces::math::{Mat4, Rotor3, ToDouble, Vec3, Vec4};
use aleph_engine::interfaces::renderer::{
    BufferHandle, BufferObject, BufferObjectDesc, BufferUploadDesc, Material, MaterialBinding,
    MaterialInstanceHandle, MaterialInstanceObject, PollCompleteError, Renderer, ResourceCommand,
    StandardMaterialLayout, TextureStreamingRequest,
};
use aleph_rhi_api::*;
use gltf::accessor::{DataType, Dimensions};
use gltf::buffer::Data;
use gltf::material::AlphaMode;
use gltf::{Accessor, Primitive};
use rayon::prelude::*;

use crate::game::async_texture_loader::{AsyncTextureLoadRequest, AsyncTextureLoader};
use crate::game::cube_mesh::Vertex;


#[aleph_profile::function]
pub fn load_scene(
    world: &mut World,
    renderer: &mut Renderer,
    arena: &mut BumpThingy,
    thinkers: &mut Vec<TextureLoadThinker>,
    standard_material: &Arc<Material>,
    loader: &AsyncTextureLoader,
    path: &std::path::Path,
) {
    let (document, buffers) = import_path(path).unwrap();

    let base = path.parent().unwrap();
    let mut tex_table: Vec<Option<TextureStreamingRequest>> = Vec::new();
    for image in document.images() {
        match image.source() {
            gltf::image::Source::Uri { uri, .. } => {
                let path = base.join(uri);
                let request = loader.load(AsyncTextureLoadRequest { path });
                tex_table.push(Some(request));
            }
            _ => tex_table.push(None),
        }
    }

    let mut mat_table = Vec::new();
    for (i, mat) in document.materials().enumerate() {
        let _i = mat.index().unwrap();
        debug_assert_eq!(i, _i);

        let pbr_mat = mat.pbr_metallic_roughness();

        let white_tex = renderer.default_resources().white_texture_rgba8();
        let norm_tex = renderer.default_resources().normal_texture_rgba8();
        let layout = StandardMaterialLayout {
            colour: pbr_mat.base_color_factor(),
            metal_roughness: [
                pbr_mat.metallic_factor(),
                pbr_mat.roughness_factor(),
                0.0,
                0.0,
            ],
            _padding1: [0; 128],
            _padding2: [0; 96],
        };

        let mut desc = BufferObjectDesc::new();
        desc.size(256);
        desc.usage(ResourceUsageFlags::CONSTANT_BUFFER);
        let mut upload = BufferUploadDesc::new_owned(renderer.device(), &desc).unwrap();
        upload
            .buffer
            .bytes_mut()
            .copy_from_slice(bytemuck::bytes_of(&layout));
        let object = BufferObject::new_for_desc(renderer.device(), desc).unwrap();
        let buffer = renderer.create_buffer(object).unwrap();
        renderer.submit_resource_command(ResourceCommand::BufferUpload(buffer, upload));

        let mut material_instance = MaterialInstanceObject::new(standard_material.clone());
        material_instance.set_double_sided(mat.double_sided());
        material_instance.update_binding(0, MaterialBinding::Buffer(Some(buffer)));
        material_instance.update_binding(1, MaterialBinding::Texture(Some(white_tex)));
        material_instance.update_binding(2, MaterialBinding::Texture(Some(white_tex)));
        material_instance.update_binding(3, MaterialBinding::Texture(Some(norm_tex)));
        let material_instance = renderer
            .create_material_instance(material_instance)
            .unwrap();

        if let Some(tex) = pbr_mat.base_color_texture() {
            if let Some(req) = &tex_table[tex.texture().source().index()] {
                thinkers.push(TextureLoadThinker {
                    target: material_instance,
                    request: Some(req.clone()),
                    target_tex: TargetTex::Colour,
                });
            }
        }

        if let Some(tex) = pbr_mat.metallic_roughness_texture() {
            if let Some(req) = &tex_table[tex.texture().source().index()] {
                thinkers.push(TextureLoadThinker {
                    target: material_instance,
                    request: Some(req.clone()),
                    target_tex: TargetTex::MetalRoughness,
                });
            }
        }

        if let Some(tex) = mat.normal_texture() {
            if let Some(req) = &tex_table[tex.texture().source().index()] {
                thinkers.push(TextureLoadThinker {
                    target: material_instance,
                    request: Some(req.clone()),
                    target_tex: TargetTex::Normal,
                });
            }
        }

        mat_table.push(material_instance);
    }
}
