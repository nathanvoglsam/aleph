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

pub mod smaa_area;
pub mod smaa_search;

use crate::renderer::SimpleTextureOptions;
use crate::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use crate::resource::texture::TextureHandle;
use crate::resource::texture::physical::PhysicalTextureDesc;
use crate::resource::texture::simple::SimpleTextureLayout;
use crate::resource::texture::single::SingleTextureDesc;
use crate::resource_loader::mip_upload::MipUploadDesc;

/// Constructs the built-in texture used for the SMAA area pass
pub fn create_smaa_area_texture(resource_builder: &mut ImmediateResourceBuilder) -> TextureHandle {
    let mut desc = SimpleTextureLayout::new();
    desc.with_format(rhi::Format::Rg8Unorm);
    desc.image_2d(smaa_area::WIDTH, smaa_area::HEIGHT);

    let mut data = MipUploadDesc::new_owned(resource_builder.device, &desc, 0, 0, 1).unwrap();

    let physical_desc = desc.with_stride(0);
    let num_rows = desc.num_rows();
    let row_bytes = desc.row_bytes();
    let row_bytes_padded = physical_desc.upload_row_bytes();
    assert_eq!(row_bytes, smaa_area::ROW_STRIDE as usize);
    copy_rows(
        smaa_area::BYTES.as_slice(),
        data.buffer.bytes_mut(),
        num_rows,
        row_bytes,
        row_bytes_padded,
    );

    let handle = resource_builder
        .create_simple_texture_immediate(&desc, data, &SimpleTextureOptions::default())
        .unwrap();

    handle
}

pub fn create_smaa_search_texture(
    resource_builder: &mut ImmediateResourceBuilder,
) -> TextureHandle {
    let mut desc = SimpleTextureLayout::new();
    desc.with_format(rhi::Format::R8Unorm);
    desc.image_2d(smaa_search::WIDTH, smaa_search::HEIGHT);

    let mut data = MipUploadDesc::new_owned(resource_builder.device, &desc, 0, 0, 1).unwrap();

    let physical_desc = desc.with_stride(0);
    let num_rows = desc.num_rows();
    let row_bytes = desc.row_bytes();
    let row_bytes_padded = physical_desc.upload_row_bytes();
    assert_eq!(row_bytes, smaa_search::ROW_STRIDE as usize);
    copy_rows(
        smaa_search::BYTES.as_slice(),
        data.buffer.bytes_mut(),
        num_rows,
        row_bytes,
        row_bytes_padded,
    );

    let handle = resource_builder
        .create_simple_texture_immediate(&desc, data, &SimpleTextureOptions::default())
        .unwrap();

    handle
}

fn copy_rows(
    mut src: &[u8],
    mut dst: &mut [u8],
    num_rows: usize,
    row_bytes: usize,
    row_bytes_padded: usize,
) {
    for _ in 0..num_rows {
        dst[0..row_bytes].copy_from_slice(&src[0..row_bytes]);
        src = &src[row_bytes..];
        dst = &mut dst[row_bytes_padded..];
    }
}
