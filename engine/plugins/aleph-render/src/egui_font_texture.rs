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

use std::ops::BitAnd;

use aleph_rhi_api::*;
use egui::epaint::ImageDelta;
use egui::{FontImage, ImageData};
use wide::{f32x4, f32x8, i32x4, i32x8, CmpEq};

use aleph_renderer::{
    GenerateMips, Renderer, TextureAllocMode, TextureHandle, TextureMipUploadDesc,
    TextureUploadSource,
};

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
            unsafe {
                let dimensions = (
                    self.font_texture.width as u32,
                    self.font_texture.height as u32,
                );

                let desc =
                    TextureMipUploadDesc::new(dimensions.0, dimensions.1, 1, Format::R8Unorm);
                let staging_buffer = TextureUploadSource::new_owned(
                    renderer.device(),
                    desc.clone(),
                    ResourceUsageFlags::SHADER_RESOURCE,
                )
                .unwrap();

                assert_eq!(
                    desc.aligned_width(),
                    desc.width,
                    "Currently we don't handle row pitch here"
                );

                let data = staging_buffer.data_ptr().cast::<u8>();
                data.as_ptr().copy_from_nonoverlapping(
                    self.font_texture.bytes.as_ptr(),
                    desc.size_requirement(),
                );

                staging_buffer.unmap().unwrap();

                if let Some(handle) = self.font_handle {
                    renderer
                        .get_texture_loader()
                        .immediate_upload(
                            None,
                            handle,
                            staging_buffer,
                            TextureAllocMode::Deferred,
                            GenerateMips::No,
                        )
                        .unwrap();
                } else {
                    let result = renderer.create_texture(
                        staging_buffer,
                        TextureAllocMode::Deferred,
                        GenerateMips::No,
                    );
                    self.font_handle = Some(result.unwrap());
                }
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

pub struct FontTexture {
    /// Width in pixels of the texture
    pub width: usize,

    /// Height in pixels of the texture
    pub height: usize,

    /// Raw data for the texture
    pub bytes: Vec<u8>,
}

impl FontTexture {
    pub fn new() -> Self {
        Self {
            width: 8192,
            height: 8192,
            bytes: vec![0u8; 8192 * 8192],
        }
    }

    #[inline(never)]
    pub fn apply_patch_to_font_texture(&mut self, font: &FontImage, pos: (usize, usize)) {
        // Handle a partial update
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

        // We need to know how many blocks we can handle with simd, and how many
        // trailing pixels in a row have to take the scalar path.
        let blocks = w / 8;
        let non_block_pixels = w % 8;

        let mut row_y = 0;
        while row_y < h {
            unsafe {
                // Get slices over the src and dst rows we're doing the copy for.
                let src_i = row_y * w;
                let dst_y = row_y + y;
                let dst_i = (dst_y * self.width) + x;
                let src_row = &font.pixels[src_i..src_i + w];
                let dst_row = &mut self.bytes[dst_i..dst_i + w];

                // First we copy the contiguous blocks of 4 using the simd path
                let mut src_ptr = src_row.as_ptr() as *const f32x8;
                let mut dst_ptr = dst_row.as_mut_ptr() as *mut [u8; 8];
                let end_ptr = src_ptr.add(blocks);
                while src_ptr != end_ptr {
                    // Load and do the gamma mapping in a SIMD register (hopefully)
                    //
                    // We have to read unaligned as we aren't guaranteed for each row
                    // to be 16 byte aligned.
                    let block_data = src_ptr.read_unaligned();
                    let mapped = coverage_mapper_simd_256(block_data);

                    // Store the mapped result into the destination block
                    *dst_ptr = mapped;

                    // Advance to the next block
                    src_ptr = src_ptr.add(1);
                    dst_ptr = dst_ptr.add(1);
                }

                // Then we copy the trailing bytes of the row using a scalar path
                let mut src_ptr = src_ptr as *const f32;
                let mut dst_ptr = dst_ptr as *mut u8;
                let end_ptr = src_ptr.add(non_block_pixels);
                while src_ptr != end_ptr {
                    // Load and do the gamma mapping
                    let pixel_data = *src_ptr;
                    let mapped = coverage_mapper(pixel_data);

                    // Store the mapped result into the destination block
                    *dst_ptr = mapped;

                    // Advance to the next block
                    src_ptr = src_ptr.add(1);
                    dst_ptr = dst_ptr.add(1);
                }
            }

            row_y += 1;
        }
    }

    #[inline(never)]
    pub fn apply_whole_to_font_texture(&mut self, font: &FontImage) {
        assert_eq!(font.width(), self.width);

        let new_width = font.width();
        let new_height = font.height();
        let pixels = new_width * new_height;
        self.width = new_width;
        self.height = new_height;

        assert_eq!(
            pixels.next_multiple_of(8),
            pixels,
            "Must be a multiple of 8 pixels"
        );

        unsafe {
            if font.pixels.as_ptr().align_offset(32) == 0 {
                self.copy_map_256(pixels, font);
            } else if font.pixels.as_ptr().align_offset(16) == 0 {
                self.copy_map_128(pixels, font);
            } else {
                unimplemented!("'font.pixels' must be 16 or 32 byte aligned");
            }
        }
    }

    unsafe fn copy_map_128(&mut self, pixels: usize, font: &FontImage) {
        let blocks = pixels / 4;
        let mut block_ptr = font.pixels.as_ptr() as *const f32x4;
        let mut dst_ptr = self.bytes.as_mut_ptr() as *mut [u8; 4];
        let end_ptr = block_ptr.add(blocks);
        while block_ptr != end_ptr {
            // Load and do the gamma mapping in a SIMD register (hopefully)
            let block_data = *block_ptr;
            let mapped = coverage_mapper_simd_128(block_data);

            // Store the mapped result into the destination block
            *dst_ptr = mapped;

            // Advance to the next block
            block_ptr = block_ptr.add(1);
            dst_ptr = dst_ptr.add(1);
        }
    }

    unsafe fn copy_map_256(&mut self, pixels: usize, font: &FontImage) {
        let blocks = pixels / 8;
        let mut block_ptr = font.pixels.as_ptr() as *const f32x8;
        let mut dst_ptr = self.bytes.as_mut_ptr() as *mut [u8; 8];

        let end_ptr = block_ptr.add(blocks);
        while block_ptr != end_ptr {
            // Load and do the gamma mapping in a SIMD register (hopefully)
            let block_data = *block_ptr;
            let mapped = coverage_mapper_simd_256(block_data);

            // Store the mapped result into the destination block
            *dst_ptr = mapped;

            // Advance to the next block
            block_ptr = block_ptr.add(1);
            dst_ptr = dst_ptr.add(1);
        }
    }
}

#[inline(always)]
fn coverage_mapper(v: f32) -> u8 {
    // Function jigged from egui
    #[inline(always)]
    fn fast_round(r: f32) -> u32 {
        // Mask so we only get the lower 8 bits to guarantee an output in the 0-255 range
        ((r + 0.5).floor() as u32) & 0xFF // rust does a saturating cast since 1.45
    }

    // Safety: Index is guaranteed to be in bounds due to masking the output to 8 bits. 8 bits
    //         can't encode an invalid index so this is safe.
    unsafe { *GAMMA_LUT.get_unchecked(fast_round(v * 255.0) as usize) }
}

#[inline(always)]
fn coverage_mapper_simd_128(v: f32x4) -> [u8; 4] {
    if v.cmp_eq(f32x4::splat(0.0)).all() {
        // Special case for all zeroes as we can guarantee the output. This lets us skip the math
        // below cheaply.
        [0, 0, 0, 0]
    } else {
        // The reference implementation from egui uses powf to do the gamma correction in the FPU.
        // powf is ___slow___. The actual range of values we output though, 0-255, is tiny. Instead,
        // this implementation performs the conversion to unorm, cast to int, and bitmask all in
        // simd registers (AVX on x86) before applying egui's gamma correction using a LUT.
        //
        // The output range is 0-255, so we only need a 256 byte LUT.
        //
        // This does lose precision though as we effectively do the powf after rounding to the
        // values representable by an 8-bit unorm. We find the quality loss acceptable, considering
        // that this makes the function ~5x faster on average.
        //
        // TODO: could this be improved (do we need to?)
        //
        // We _could_ improve the quality by doing linear interpolation of the LUT to get a better
        // approximation of the gamma curve. What would it cost? Is the quality needed?
        let v = v * f32x4::splat(255.0);
        let v = v.fast_round_int();
        let v = v.bitand(i32x4::splat(0xFF));
        let v = v.to_array();

        // Safety: Index is guaranteed to be in bounds due to masking the output to 8 bits. 8 bits
        //         can't encode an invalid index so this is safe.
        unsafe {
            [
                *GAMMA_LUT.get_unchecked(v[0] as usize),
                *GAMMA_LUT.get_unchecked(v[1] as usize),
                *GAMMA_LUT.get_unchecked(v[2] as usize),
                *GAMMA_LUT.get_unchecked(v[3] as usize),
            ]
        }
    }
}

#[inline(always)]
fn coverage_mapper_simd_256(v: f32x8) -> [u8; 8] {
    if v.cmp_eq(f32x8::splat(0.0)).all() {
        // Special case for all zeroes as we can guarantee the output. This lets us skip the math
        // below cheaply.
        [0, 0, 0, 0, 0, 0, 0, 0]
    } else {
        // The reference implementation from egui uses powf to do the gamma correction in the FPU.
        // powf is ___slow___. The actual range of values we output though, 0-255, is tiny. Instead,
        // this implementation performs the conversion to unorm, cast to int, and bitmask all in
        // simd registers (AVX on x86) before applying egui's gamma correction using a LUT.
        //
        // The output range is 0-255, so we only need a 256 byte LUT.
        //
        // This does lose precision though as we effectively do the powf after rounding to the
        // values representable by an 8-bit unorm. We find the quality loss acceptable, considering
        // that this makes the function ~5x faster on average.
        //
        // TODO: could this be improved (do we need to?)
        //
        // We _could_ improve the quality by doing linear interpolation of the LUT to get a better
        // approximation of the gamma curve. What would it cost? Is the quality needed?
        let v = v * f32x8::splat(255.0);
        let v = v.fast_round_int();
        let v = v.bitand(i32x8::splat(0xFF));
        let v = v.to_array();

        // Safety: Index is guaranteed to be in bounds due to masking the output to 8 bits. 8 bits
        //         can't encode an invalid index so this is safe.
        unsafe {
            [
                *GAMMA_LUT.get_unchecked(v[0] as usize),
                *GAMMA_LUT.get_unchecked(v[1] as usize),
                *GAMMA_LUT.get_unchecked(v[2] as usize),
                *GAMMA_LUT.get_unchecked(v[3] as usize),
                *GAMMA_LUT.get_unchecked(v[4] as usize),
                *GAMMA_LUT.get_unchecked(v[5] as usize),
                *GAMMA_LUT.get_unchecked(v[6] as usize),
                *GAMMA_LUT.get_unchecked(v[7] as usize),
            ]
        }
    }
}

/// A lookup table that maps linear coverage to gamma coverage based on egui's reference gamma
/// conversion of int((v ^ 0.55) * 255).
///
/// This is pre-calculated using the following python script
///
/// ```ignore
/// for i in range(256):
///     v1 = float(i) / 255 # normalize from unorm range
///     v2 = int(pow(v1, 0.55) * 255) # do the actual conversion, including going back to unorm
///     print(f'{v2},',)
/// ```
const GAMMA_LUT: [u8; 256] = [
    0, 12, 17, 22, 25, 29, 32, 35, 37, 40, 42, 45, 47, 49, 51, 53, 55, 57, 59, 61, 62, 64, 66, 67,
    69, 71, 72, 74, 75, 77, 78, 80, 81, 82, 84, 85, 86, 88, 89, 90, 92, 93, 94, 95, 97, 98, 99,
    100, 101, 102, 104, 105, 106, 107, 108, 109, 110, 111, 112, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 131, 132, 133, 134, 135, 136, 137, 138,
    139, 140, 141, 142, 142, 143, 144, 145, 146, 147, 148, 149, 149, 150, 151, 152, 153, 154, 154,
    155, 156, 157, 158, 158, 159, 160, 161, 162, 162, 163, 164, 165, 166, 166, 167, 168, 169, 169,
    170, 171, 172, 173, 173, 174, 175, 176, 176, 177, 178, 178, 179, 180, 181, 181, 182, 183, 184,
    184, 185, 186, 186, 187, 188, 189, 189, 190, 191, 191, 192, 193, 193, 194, 195, 195, 196, 197,
    198, 198, 199, 200, 200, 201, 202, 202, 203, 204, 204, 205, 206, 206, 207, 207, 208, 209, 209,
    210, 211, 211, 212, 213, 213, 214, 215, 215, 216, 216, 217, 218, 218, 219, 220, 220, 221, 221,
    222, 223, 223, 224, 224, 225, 226, 226, 227, 227, 228, 229, 229, 230, 230, 231, 232, 232, 233,
    233, 234, 235, 235, 236, 236, 237, 238, 238, 239, 239, 240, 240, 241, 242, 242, 243, 243, 244,
    244, 245, 246, 246, 247, 247, 248, 248, 249, 250, 250, 251, 251, 252, 252, 253, 253, 254, 255,
];
