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
use egui::epaint::ImageDelta;
use egui::ImageData;
use interfaces::any::AnyArc;

use crate::render::FontTexture;
use crate::render::{TextureHandle, TextureLoader, TextureMipUploadDesc, TextureUploadSource};

pub struct EguiRenderer {
    pub device: AnyArc<dyn IDevice>,

    pub texture_loader: TextureLoader,

    pub font_texture: FontTexture,
    pub font_handle: TextureHandle,
}

impl EguiRenderer {
    pub fn update_font_texture<'a>(&mut self, deltas: impl Iterator<Item = &'a ImageDelta>) {
        let mut updated = false;
        for delta in deltas {
            updated = true;
            self.apply_delta_to_font_texture(delta);
        }

        if updated {
            unsafe {
                let dimensions = (
                    self.font_texture.width as u32,
                    self.font_texture.height as u32,
                );

                let desc =
                    TextureMipUploadDesc::new(dimensions.0, dimensions.1, 1, Format::R8Unorm);
                let staging_buffer = TextureUploadSource::new_owned(
                    self.device.as_ref(),
                    desc.clone(),
                    ResourceUsageFlags::SHADER_RESOURCE,
                )
                .unwrap();

                assert_eq!(
                    staging_buffer.desc.aligned_width(),
                    staging_buffer.desc.width,
                    "Currently we don't handle row pitch here"
                );

                let data = staging_buffer.data_ptr().cast::<u8>();
                data.as_ptr().copy_from_nonoverlapping(
                    self.font_texture.bytes.as_ptr(),
                    desc.size_requirement(),
                );

                staging_buffer.unmap();

                self.texture_loader
                    .immediate_upload(None, self.font_handle, staging_buffer)
                    .unwrap();
            }
        }
    }

    pub fn apply_delta_to_font_texture(&mut self, delta: &ImageDelta) {
        // We only support font images here so we panic if we get something else
        match &delta.image {
            ImageData::Font(font) => {
                // In the event of a whole update we need to re-allocate the texture as the size may have
                // increased.
                //
                // Partial updates patch the data in place
                if let Some(position) = &delta.pos {
                    let pos = (position[0], position[1]);
                    self.font_texture.apply_patch_to_font_texture(font, pos);
                } else {
                    self.font_texture.apply_whole_to_font_texture(font);
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
