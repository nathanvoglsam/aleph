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

mod backbuffer_import_pass;
mod copy_texture_pass;
mod egui_pass;
mod frame;
mod lighting_resolve_pass;
mod main_gbuffer_pass;
mod params;
mod tone_map_pass;

use crate::render::{
    TextureHandle, TextureLoader, TextureMipUploadDesc, TexturePool, TextureUploadSource,
};
use crate::renderer::backbuffer_import_pass::BackBufferHandle;
use crate::renderer::egui_pass::EguiPassContext;
use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;

use std::num::NonZeroU8;
use std::ops::Deref;

use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::ShaderDatabase;
use egui::epaint::ImageDelta;
use egui::{ImageData, RenderData};
use interfaces::any::AnyArc;

pub(crate) use frame::PerFrameObjects;

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

        let frames = (0..2)
            .map(|_| PerFrameObjects::new(device.deref()))
            .collect();

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
            font_texture: FontTexture {
                width: 256,
                height: 1,
                bytes: vec![255; 256],
            },
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

        let frames: Vec<_> = (0..2)
            .map(|_| PerFrameObjects::new(self.device.as_ref()))
            .collect();

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
        backbuffer_import_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        main_gbuffer_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        lighting_resolve_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        tone_map_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        copy_texture_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        egui_pass::pass(&mut frame_graph, device, pin_board, shader_db);
        let mut frame_graph = frame_graph.build(device);

        // Safety: We _just_ created this graph. There's no way any transient allocations exist
        //         yet.
        unsafe {
            frame_graph.allocate_transients(3);
        }

        frame_graph
    }

    pub unsafe fn record_frame(
        &mut self,
        index: usize,
        texture: &dyn ITexture,
        render_data: RenderData,
    ) -> Box<dyn ICommandList> {
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
                    TextureUploadSource::new_owned(self.device.as_ref(), desc).unwrap();

                assert_eq!(
                    staging_buffer.desc.aligned_width(),
                    staging_buffer.desc.width,
                    "Currently we don't handle row pitch here"
                );

                staging_buffer
                    .data
                    .cast::<u8>()
                    .as_ptr()
                    .copy_from_nonoverlapping(
                        self.font_texture.bytes.as_ptr(),
                        self.font_texture.bytes.len(),
                    );

                self.texture_loader
                    .immediate_upload(self.font_handle, staging_buffer);
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
                    // Handle a partial update
                    let x = position[0];
                    let y = position[1];
                    let w = font.size[0];
                    let h = font.size[1];

                    // Assert that we can't access the texture out of bounds based on the input we
                    // got.
                    assert!(x < self.font_texture.width);
                    assert!(y < self.font_texture.height);
                    assert!(x + w <= self.font_texture.width);
                    assert!(y + h <= self.font_texture.height);

                    // Assert that the buffers are big enough.
                    //
                    // We're trying to convince the optimizer that it can elide the bounds checks
                    // on array indexing.
                    assert!(
                        self.font_texture.bytes.len()
                            >= self.font_texture.width * self.font_texture.height
                    );
                    assert!(font.pixels.len() >= w * h);

                    // Iterate over each row
                    for d_row in 0..w {
                        // Transform our row in the delta pixels to our texture's pixel
                        let f_row = d_row + x;

                        // Iterate over all the columns in the current row
                        for d_col in 0..h {
                            // Transform our column in the delta pixels to our texture's pixels
                            let f_col = d_col + y;

                            // Calculate indices
                            let d_idx = d_row + d_col * w; // In delta tex
                            let f_idx = f_row + f_col * self.font_texture.width; // In our tex

                            // Copy and map our coverage sample into our font texture
                            self.font_texture.bytes[f_idx] = coverage_mapper(font.pixels[d_idx]);
                        }
                    }
                } else {
                    // Handle a full update

                    // Just replace the old texture with the new data, mapped to u8
                    self.font_texture.width = delta.image.width();
                    self.font_texture.height = delta.image.height();
                    self.font_texture.bytes =
                        font.pixels.iter().copied().map(coverage_mapper).collect();
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

fn coverage_mapper(v: f32) -> u8 {
    // Function jigged from egui
    fn fast_round(r: f32) -> u8 {
        (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
    }

    // 0.55 from egui srgba_pixels conversion on FontImage
    fast_round(v.powf(0.55) * 255.0)
}
