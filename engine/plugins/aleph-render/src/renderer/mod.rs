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

mod frame;
mod pass;

use std::num::NonZeroU8;
use std::ops::{BitAnd, Deref};

use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::ShaderDatabase;
use egui::epaint::ImageDelta;
use egui::{FontImage, ImageData, RenderData};
pub(crate) use frame::PerFrameObjects;
use interfaces::any::AnyArc;
use wide::{f32x8, i32x8, CmpEq};

use crate::render::ShaderDatabaseAccessor;
use crate::render::{
    TextureHandle, TextureLoader, TextureMipUploadDesc, TexturePool, TextureStreamingRequest,
    TextureUploadSource,
};
use crate::renderer::pass::backbuffer_import::BackBufferHandle;
use crate::renderer::pass::egui_draw::EguiPassContext;
use crate::renderer::pass::BackBufferInfo;

pub struct EguiRenderer {
    pub device: AnyArc<dyn IDevice>,
    pub frames: Vec<PerFrameObjects>,

    pub texture_pool: TexturePool,
    pub texture_loader: TextureLoader,

    pub frame_graph: FrameGraph,
    pub back_buffer_id: ResourceMut,
    pub graph_build_pin_board: PinBoard,
    pub execute_context: PinBoard,

    pub font_texture: FontTexture,
    pub font_handle: TextureHandle,

    pub shader_db_bin: Vec<u8>,
}

impl EguiRenderer {
    pub fn new(
        device: AnyArc<dyn IDevice>,
        back_buffer_desc: &TextureDesc,
        pixels_per_point: f32,
    ) -> Self {
        log::info!("Initializing Egui Renderer");

        // Try load the shader db, first from the immediate working directory and then from the
        // potential aleph project's .aleph/shaders directory.
        // TODO: we need a better way of managing and configuring where we get our shader db from
        let shader_db_bin = std::fs::read("shaders.shaderdb")
            .or_else(|_| std::fs::read(".aleph/shaders/shaders.shaderdb"))
            .unwrap();

        let shader_db = unsafe { rkyv::archived_root::<ShaderDatabase>(&shader_db_bin) };
        shader_db.validate_header();

        let shader_db = ShaderDatabaseAccessor::new(device.as_ref(), shader_db);

        let pin_board = PinBoard::new();
        pin_board.publish(BackBufferInfo {
            desc: back_buffer_desc.clone().strip_name(),
            pixels_per_point,
        });

        let frame_graph = Self::create_frame_graph(device.as_ref(), &pin_board, &shader_db);

        let BackBufferHandle { back_buffer } = pin_board.get().unwrap();

        let mut frames = Vec::new();
        frames.resize_with(2, || PerFrameObjects::new(device.deref()));

        let mut texture_pool = TexturePool::new(NonZeroU8::new(1).unwrap());
        let texture_loader = TextureLoader::new();

        let font_handle = texture_pool.reserve_handle();

        Self {
            device,
            frames,
            texture_pool,
            texture_loader,
            frame_graph,
            back_buffer_id: *back_buffer,
            graph_build_pin_board: pin_board,
            execute_context: PinBoard::new(),
            font_texture: FontTexture::new(),
            font_handle,
            shader_db_bin,
        }
    }

    pub fn rebuild_after_resize(&mut self, back_buffer_desc: &TextureDesc, pixels_per_point: f32) {
        self.graph_build_pin_board.clear();
        let pin_board = &self.graph_build_pin_board;

        pin_board.publish(BackBufferInfo {
            desc: back_buffer_desc.clone().strip_name(),
            pixels_per_point,
        });

        let shader_db = unsafe { rkyv::archived_root::<ShaderDatabase>(&self.shader_db_bin) };
        let shader_db = ShaderDatabaseAccessor::new(self.device.as_ref(), shader_db);
        let frame_graph = Self::create_frame_graph(self.device.as_ref(), pin_board, &shader_db);

        let BackBufferHandle { back_buffer } = pin_board.get().unwrap();

        let mut frames = Vec::new();
        frames.resize_with(2, || PerFrameObjects::new(self.device.deref()));

        self.frames = frames;
        self.frame_graph = frame_graph;
        self.back_buffer_id = *back_buffer;
    }

    pub fn create_frame_graph(
        device: &dyn IDevice,
        pin_board: &PinBoard,
        shader_db: &ShaderDatabaseAccessor,
    ) -> FrameGraph {
        let mut frame_graph = FrameGraph::builder();
        pass::backbuffer_import::pass(&mut frame_graph, device, pin_board, shader_db);
        pass::main_gbuffer::pass(&mut frame_graph, device, pin_board, shader_db);
        pass::lighting_resolve::pass(&mut frame_graph, device, pin_board, shader_db);
        pass::tone_map::pass(&mut frame_graph, device, pin_board, shader_db);
        pass::copy_texture::pass(&mut frame_graph, device, pin_board, shader_db);
        pass::egui_draw::pass(&mut frame_graph, device, pin_board, shader_db);
        let mut frame_graph = frame_graph.build(device);

        // Safety: We _just_ created this graph. There's no way any transient allocations exist
        //         yet.
        unsafe {
            frame_graph.allocate_transients(2);
        }

        frame_graph
    }

    pub unsafe fn record_frame(
        &mut self,
        index: usize,
        texture: &dyn ITexture,
        render_data: RenderData,
    ) -> Box<dyn ICommandList> {
        // Recording frame 'index' means 'index' must have completed on the GPU time so this should
        // be safe
        self.frames[index].texture_deletion_pool.purge();

        // Begin recording commands into the command list
        let mut list = self
            .device
            .create_command_list(&CommandListDesc {
                queue_type: QueueType::General,
                name: None,
            })
            .unwrap();

        {
            let mut encoder = list.begin_general().unwrap();

            self.texture_loader.upload_requests(
                &mut self.texture_pool,
                &mut self.frames[index].texture_deletion_pool,
                self.device.as_ref(),
                encoder.as_mut(),
                usize::MAX,
            );

            let mut import_bundle = ImportBundle::default();
            import_bundle.add_resource(self.back_buffer_id, texture);

            self.execute_context.clear();
            self.execute_context.publish(EguiPassContext {
                buffer: self.frames[index].uniform_buffer.clone(),
                font_view: self
                    .texture_pool
                    .get_default_view(self.font_handle)
                    .unwrap(),
                render_data,
            });

            self.frame_graph.execute(
                index,
                &import_bundle,
                encoder.as_mut(),
                &self.execute_context,
            );
        }

        list
    }

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
                let staging_buffer =
                    TextureUploadSource::new_owned(self.device.as_ref(), desc.clone()).unwrap();

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
                    .immediate_upload(
                        TextureStreamingRequest::new(),
                        self.font_handle,
                        staging_buffer,
                    )
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
    fn apply_patch_to_font_texture(&mut self, font: &FontImage, pos: (usize, usize)) {
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
    fn apply_whole_to_font_texture(&mut self, font: &FontImage) {
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
        assert_eq!(
            font.pixels.as_ptr().align_offset(32),
            0,
            "Src data must be aligned to 32 bytes"
        );

        unsafe {
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
