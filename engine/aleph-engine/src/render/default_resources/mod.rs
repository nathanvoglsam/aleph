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

use aleph_object_system::unsafe_impl_iobject;
use mg::material::binding::MaterialBinding;
use mg::material::{Material, StandardMaterial, StandardMaterialLayout};
use mg::material_instance::{MaterialInstanceDesc, MaterialInstanceHandle};
use mg::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use mg::renderer::{BufferOptions, Renderer, SimpleTextureOptions};
use mg::resource::texture::TextureHandle;
use mg::resource::texture::simple::SimpleTextureLayout;
use mg::resource_loader::mip_upload::MipUploadDesc;
use mg::resource_loader::upload_buffer::{IUploadBuffer, UploadBuffer};

#[derive(Clone)]
pub struct DefaultResources {
    pub white_texture: TextureHandle,
    pub black_texture: TextureHandle,
    pub normal_texture: TextureHandle,
    pub standard_material: Arc<Material>,
    pub default_material: MaterialInstanceHandle,
}

unsafe_impl_iobject!(DefaultResources, "01a0e236-e161-7b21-89fb-8c32e8f27603");

impl DefaultResources {
    pub fn new(renderer: &mut Renderer) -> Self {
        let standard_material = StandardMaterial::new();

        let mut resource_builder = renderer.immediate_resource_builder();
        let white_texture = create_1x1_colour_texture(&mut resource_builder, 0xFFFFFFFF);
        let black_texture = create_1x1_colour_texture(&mut resource_builder, 0x00000000);
        let normal_texture = create_1x1_colour_texture(&mut resource_builder, 0xFFFF8080);

        let colour = [1.0, 1.0, 1.0, 1.0];
        let metal = 0.0;
        let roughness = 0.5;
        let layout = StandardMaterialLayout {
            colour,
            metal_roughness: [metal, roughness, 0.0, 0.0],
            _padding1: [0; 128],
            _padding2: [0; 96],
        };

        // Create material data buffer
        let mut upload = UploadBuffer::new_owned(renderer.device(), 256).unwrap();
        upload
            .bytes_mut()
            .copy_from_slice(bytemuck::bytes_of(&layout));
        let buffer = renderer
            .create_buffer_immediate(256, Some(upload.into_smallbox()), &BufferOptions::default())
            .unwrap();
        let inst_bindings = [
            MaterialBinding::Buffer(Some(buffer)),
            MaterialBinding::Texture(Some(white_texture)),
            MaterialBinding::Texture(Some(white_texture)),
            MaterialBinding::Texture(Some(normal_texture)),
        ];
        let inst_desc = MaterialInstanceDesc {
            double_sided: false,
            bindings: &inst_bindings,
        };
        let default_material = renderer
            .create_material_instance(&standard_material, &inst_desc)
            .unwrap();

        Self {
            white_texture,
            black_texture,
            normal_texture,
            standard_material,
            default_material,
        }
    }
}

fn create_1x1_colour_texture(
    resource_builder: &mut ImmediateResourceBuilder,
    payload: u32,
) -> TextureHandle {
    let mut desc = SimpleTextureLayout::new();
    // desc.usage(rhi::ResourceUsageFlags::SHADER_RESOURCE);
    desc.with_format(rhi::Format::Rgba8Unorm);
    desc.image_2d(1, 1);

    let mut data = MipUploadDesc::new_owned(resource_builder.device, &desc, 0, 0, 1).unwrap();

    let dst = &mut data.buffer.bytes_mut()[0..4];
    dst.copy_from_slice(bytemuck::bytes_of(&payload));

    let handle = resource_builder
        .create_simple_texture_immediate(&desc, data, &SimpleTextureOptions::default())
        .unwrap();

    handle
}
