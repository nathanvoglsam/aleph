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

use crate::{
    GenerateMips, TextureAllocMode, TextureHandle, TextureLoader, TexturePool, TextureUploadSource,
};

pub unsafe fn create_1x1_colour_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    texture_loader: &TextureLoader,
    payload: u32,
) -> TextureHandle {
    let mut data = TextureUploadSource::new_owned(
        device,
        crate::TextureMipUploadDesc {
            width: 1,
            height: 1,
            depth: 1,
            format: Format::Rgba8Unorm,
        },
        ResourceUsageFlags::SHADER_RESOURCE,
    )
    .unwrap();
    let dst = &mut data.data_mut()[0..4];
    dst.copy_from_slice(bytemuck::bytes_of(&payload));
    let handle = texture_pool.create_texture(None);
    texture_loader
        .immediate_upload(
            None,
            handle,
            data,
            TextureAllocMode::Deferred,
            GenerateMips::No,
        )
        .ok()
        .unwrap();
    handle
}

pub unsafe fn create_smaa_area_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    texture_loader: &TextureLoader,
) -> TextureHandle {
    let mut data = TextureUploadSource::new_owned(
        device,
        crate::TextureMipUploadDesc {
            width: smaa_area::WIDTH,
            height: smaa_area::HEIGHT,
            depth: 1,
            format: Format::Rg8Unorm,
        },
        ResourceUsageFlags::SHADER_RESOURCE,
    )
    .unwrap();

    for (i, row) in data.row_iter_mut().enumerate() {
        let src_base = smaa_area::ROW_STRIDE as usize * i;
        let src_end = smaa_area::ROW_STRIDE as usize * (i + 1);
        let src = &smaa_area::BYTES[src_base..src_end];
        row.copy_from_slice(src);
    }

    let handle = texture_pool.create_texture(None);
    texture_loader
        .immediate_upload(
            None,
            handle,
            data,
            TextureAllocMode::Deferred,
            GenerateMips::No,
        )
        .ok()
        .unwrap();
    handle
}

pub unsafe fn create_smaa_search_texture(
    device: &dyn IDevice,
    texture_pool: &mut TexturePool,
    texture_loader: &TextureLoader,
) -> TextureHandle {
    let mut data = TextureUploadSource::new_owned(
        device,
        crate::TextureMipUploadDesc {
            width: smaa_search::WIDTH,
            height: smaa_search::HEIGHT,
            depth: 1,
            format: Format::R8Unorm,
        },
        ResourceUsageFlags::SHADER_RESOURCE,
    )
    .unwrap();

    for (i, row) in data.row_iter_mut().enumerate() {
        let src_base = smaa_search::ROW_STRIDE as usize * i;
        let src_end = smaa_search::ROW_STRIDE as usize * (i + 1);
        let src = &smaa_search::BYTES[src_base..src_end];
        row.copy_from_slice(src);
    }

    let handle = texture_pool.create_texture(None);
    texture_loader
        .immediate_upload(
            None,
            handle,
            data,
            TextureAllocMode::Deferred,
            GenerateMips::No,
        )
        .ok()
        .unwrap();
    handle
}
