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

mod smaa_area;
mod smaa_search;

use aleph_rhi_api::*;

use crate::pass::resource_processor::GenerateMips;
use crate::{
    ResourceCommand, ResourceCommandBuffer, TextureHandle, TextureObject, TextureObjectDesc,
    TexturePool, TextureUploadDesc,
};

pub unsafe fn create_1x1_colour_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    cmds: &mut ResourceCommandBuffer,
    payload: u32,
) -> TextureHandle {
    let mut desc = TextureObjectDesc::new();
    desc.format(Format::Rgba8Unorm);
    desc.usage(ResourceUsageFlags::SHADER_RESOURCE);
    desc.image_2d(1, 1);

    let mut data = TextureUploadDesc::new_owned(device, &desc, 0, 1).unwrap();

    let dst = &mut data.buffer.bytes_mut()[0..4];
    dst.copy_from_slice(bytemuck::bytes_of(&payload));

    let mut object = TextureObject::new_for_desc(device, desc).unwrap();
    object.recreate_default_view();
    let handle = texture_pool.alloc(object);

    cmds.push_command(ResourceCommand::TextureUpload(handle, GenerateMips::No, data));

    handle
}

pub unsafe fn create_smaa_area_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    cmds: &mut ResourceCommandBuffer,
) -> TextureHandle {
    let mut desc = TextureObjectDesc::new();
    desc.format(Format::Rg8Unorm);
    desc.usage(ResourceUsageFlags::SHADER_RESOURCE);
    desc.image_2d(smaa_area::WIDTH, smaa_area::HEIGHT);

    let mut data = TextureUploadDesc::new_owned(device, &desc, 0, 1).unwrap();

    let num_rows = desc.num_rows_for_level(0);
    let row_bytes = desc.row_bytes_for_level(0);
    let row_bytes_padded = desc.upload_row_bytes_for_level(0);
    let mut src = smaa_area::BYTES.as_slice();
    let mut dst = data.buffer.bytes_mut();
    for _ in 0..num_rows {
        assert_eq!(row_bytes, smaa_area::ROW_STRIDE as usize);

        dst[0..row_bytes].copy_from_slice(&src[0..row_bytes]);

        src = &src[row_bytes..];
        dst = &mut dst[row_bytes_padded..];
    }

    let mut object = TextureObject::new_for_desc(device, desc).unwrap();
    object.recreate_default_view();
    let handle = texture_pool.alloc(object);

    cmds.push_command(ResourceCommand::TextureUpload(handle, GenerateMips::No, data));

    handle
}

pub unsafe fn create_smaa_search_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    cmds: &mut ResourceCommandBuffer,
) -> TextureHandle {
    let mut desc = TextureObjectDesc::new();
    desc.format(Format::R8Unorm);
    desc.usage(ResourceUsageFlags::SHADER_RESOURCE);
    desc.image_2d(smaa_search::WIDTH, smaa_search::HEIGHT);

    let mut data = TextureUploadDesc::new_owned(device, &desc, 0, 1).unwrap();

    let num_rows = desc.num_rows_for_level(0);
    let row_bytes = desc.row_bytes_for_level(0);
    let row_bytes_padded = desc.upload_row_bytes_for_level(0);
    let mut src = smaa_search::BYTES.as_slice();
    let mut dst = data.buffer.bytes_mut();
    for _ in 0..num_rows {
        assert_eq!(row_bytes, smaa_search::ROW_STRIDE as usize);

        dst[0..row_bytes].copy_from_slice(&src[0..row_bytes]);

        src = &src[row_bytes..];
        dst = &mut dst[row_bytes_padded..];
    }

    let mut object = TextureObject::new_for_desc(device, desc.clone()).unwrap();
    object.recreate_default_view();
    let handle = texture_pool.alloc(object);

    cmds.push_command(ResourceCommand::TextureUpload(handle, GenerateMips::No, data));

    handle
}
