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

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::Instrumented;
use aleph_egui::Egui;
use aleph_magnesium::renderer::{Renderer, SimpleTextureOptions};
use aleph_magnesium::resource::texture::TextureHandle;
use aleph_magnesium::resource::texture::physical::PhysicalTextureDesc;
use aleph_magnesium::resource::texture::simple::SimpleTextureLayout;
use aleph_magnesium::resource_loader::mip_upload::MipUploadDesc;
use egui::epaint::ImageDelta;
use egui::{ColorImage, ImageData};
use mg::resource::texture::single::SingleTextureDesc;

pub struct EguiFontTexture {
    pub font_texture: FontTexture,
    pub font_handle: Option<TextureHandle>,
}

impl EguiFontTexture {
    pub fn new() -> Self {
        Self {
            font_texture: FontTexture::new(),
            font_handle: None,
        }
    }

    pub fn update_font_texture<'a>(
        &mut self,
        renderer: &mut Renderer,
        deltas: impl Iterator<Item = &'a ImageDelta>,
    ) {
        let mut updated = false;
        for delta in deltas {
            updated = true;
            self.apply_delta_to_font_texture(delta);
        }

        if updated {
            let dimensions = (
                self.font_texture.width as u32,
                self.font_texture.height as u32,
            );

            let mut desc = SimpleTextureLayout::new();
            desc.with_format(rhi::Format::Rgba8Unorm);
            desc.image_2d(dimensions.0, dimensions.1);

            let mut data = MipUploadDesc::new_owned(renderer.device(), &desc, 0, 0, 1).unwrap();

            let physical_desc = desc.with_pitch(0);
            let size = physical_desc.upload_bytes();
            let dst = &mut data.buffer.bytes_mut()[0..size];
            dst.copy_from_slice(&self.font_texture.bytes[0..size]);

            let handle = renderer
                .create_simple_texture_immediate(&desc, data, &SimpleTextureOptions::default())
                .unwrap();

            let mut old_texture = Some(handle);
            std::mem::swap(&mut old_texture, &mut self.font_handle);
            if let Some(old_texture) = old_texture {
                renderer.destroy_texture(old_texture);
            }
        }
    }

    pub fn apply_delta_to_font_texture(&mut self, delta: &ImageDelta) {
        // We only support font images here so we panic if we get something else
        match &delta.image {
            ImageData::Color(image) => {
                // In the event of a whole update we need to re-allocate the texture as the size may have
                // increased.
                //
                // Partial updates patch the data in place
                if let Some(position) = &delta.pos {
                    let pos = (position[0], position[1]);
                    self.font_texture.apply_patch_to_font_texture(image, pos);
                } else {
                    // Protect the texture from growing over the size we've allocated for it
                    assert!(image.width() <= 8192);
                    assert!(image.height() <= 8192);

                    // Update the texture's logical size (there is always an allocation for
                    // 8192x8192 pixels, we just reuse the buffer when we resize).
                    self.font_texture.width = image.width();
                    self.font_texture.height = image.height();

                    self.font_texture.apply_patch_to_font_texture(image, (0, 0));
                }
            }
        }
    }
}

pub struct FontTexture {
    /// Width in pixels of the texture
    pub width: usize,

    /// Height in pixels of the texture
    pub height: usize,

    /// Raw data for the texture
    pub bytes: BVec<u8, Instrumented<Font>>,
}

impl FontTexture {
    pub fn new() -> Self {
        Self {
            width: 8192,
            height: 8192,
            bytes: aleph_alloc::vec![in Default::default(); 0u8; 8192 * 8192 * 4],
        }
    }

    #[inline(never)]
    pub fn apply_patch_to_font_texture(&mut self, font: &ColorImage, pos: (usize, usize)) {
        let pix_stride = 4;
        let src_row_stride = font.width() * pix_stride;
        let dst_row_stride = self.width * pix_stride;

        let x = pos.0;
        let y = pos.1;
        let w = font.width();
        let h = font.height();

        // Assert that we can't access the texture out of bounds based on the input we
        // got.
        assert!(x < self.width);
        assert!(y < self.height);
        assert!(x + w <= self.width);
        assert!(y + h <= self.height);

        // Assert that the buffer is big enough for the reported size
        assert!(font.pixels.len() >= w * h);

        let mut row_y = 0;
        while row_y < h {
            // Get slices over the src and dst rows we're doing the copy for.
            let src_begin = row_y * src_row_stride;
            let src_end = src_begin + src_row_stride;

            let dst_begin = ((row_y + y) * dst_row_stride) + (x * pix_stride);
            let dst_end = dst_begin + (w * pix_stride);

            let src_row = &font.as_raw()[src_begin..src_end];
            let dst_row = &mut self.bytes[dst_begin..dst_end];

            dst_row.copy_from_slice(src_row);

            row_y += 1;
        }
    }
}

pub struct Font;
aleph_alloc::new_child_alloc_category!(Egui, Font, "01992ebc-f283-7f30-ac65-f3cd2e10ade6");
