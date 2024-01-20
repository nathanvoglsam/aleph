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

mod egui_pass;
mod frame;
mod lighting_resolve_pass;
mod main_gbuffer_pass;
mod params;

use crate::renderer::egui_pass::EguiPassContext;
use crate::renderer::params::BackBufferInfo;
use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::IShaderDatabase;
use aleph_shader_db::ShaderDatabase;
use egui::ImageData;
use egui::RenderData;
pub(crate) use frame::PerFrameObjects;
use interfaces::any::AnyArc;
use std::ops::{Deref, DerefMut};

pub struct EguiRenderer {
    pub device: AnyArc<dyn IDevice>,
    pub frames: Vec<PerFrameObjects>,
    pub frame_graph: FrameGraph,
    pub back_buffer_id: ResourceMut,
    pub graph_build_pin_board: PinBoard,
    pub execute_context: PinBoard,
    pub sampler: AnyArc<dyn ISampler>,
    pub font_texture: FontTexture,
    pub shader_db_bin: Vec<u8>,
}

impl EguiRenderer {
    pub fn new(
        device: AnyArc<dyn IDevice>,
        back_buffer_desc: &TextureDesc,
        pixels_per_point: f32,
    ) -> Self {
        log::trace!("Initializing Egui Renderer");

        let shader_db_bin =
            std::fs::read("shaders.shaderdb").expect("Failed to load shader database");
        let shader_db = unsafe { rkyv::archived_root::<ShaderDatabase>(&shader_db_bin) };

        let sampler = Self::create_sampler(device.as_ref());

        let pin_board = PinBoard::new();
        pin_board.publish(BackBufferInfo {
            desc: back_buffer_desc.clone().strip_name(),
            pixels_per_point,
        });

        let frame_graph = Self::create_frame_graph(device.as_ref(), &pin_board, shader_db);

        let output: &egui_pass::EguiPassOutput = pin_board.get().unwrap();
        let back_buffer_id = output.id;

        let frames = (0..2)
            .into_iter()
            .map(|_| PerFrameObjects::new(device.deref(), output.set_layout.as_ref(), &frame_graph))
            .collect();

        Self {
            device,
            frames,
            frame_graph,
            back_buffer_id,
            graph_build_pin_board: pin_board,
            execute_context: PinBoard::new(),
            sampler,
            font_texture: FontTexture {
                width: 256,
                height: 1,
                bytes: vec![255; 256],
                version: 1,
            },
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
        let frame_graph = Self::create_frame_graph(self.device.as_ref(), &pin_board, shader_db);

        let output: &egui_pass::EguiPassOutput = pin_board.get().unwrap();
        let back_buffer_id = output.id;

        let frames: Vec<_> = (0..2)
            .into_iter()
            .map(|_| {
                PerFrameObjects::new(
                    self.device.as_ref(),
                    output.set_layout.as_ref(),
                    &frame_graph,
                )
            })
            .collect();

        self.frames = frames;
        self.frame_graph = frame_graph;
        self.back_buffer_id = back_buffer_id;
    }

    pub fn create_frame_graph(
        device: &dyn IDevice,
        pin_board: &PinBoard,
        shader_db: &dyn IShaderDatabase,
    ) -> FrameGraph {
        let mut frame_graph = FrameGraph::builder();
        egui_pass::egui_pass(&mut frame_graph, device, pin_board, shader_db);
        frame_graph.build()
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

            // If the font texture has changed then we need to update our copy and increment the
            // version to invalidate the per-frame font textures
            for (_, delta) in render_data.textures_delta.set.iter() {
                if let egui::epaint::ImageData::Font(_) = &delta.image {
                    self.update_font_texture(delta);
                }
            }

            // If the versions do not match then we should re-upload the texture to the GPU
            if self.frames[index].font_version != self.font_texture.version {
                self.frames[index].update_texture_data(
                    self.device.deref(),
                    self.sampler.deref(),
                    &self.font_texture,
                );
                self.frames[index].record_texture_upload(encoder.deref_mut());
            }

            let mut import_bundle = ImportBundle::default();
            import_bundle.add_resource(self.back_buffer_id, texture);

            self.execute_context.clear();
            self.execute_context.publish(EguiPassContext {
                descriptor_set: self.frames[index].descriptor_set.clone(),
                render_data,
            });

            self.frame_graph.execute(
                &self.frames[index].transient_bundle,
                &import_bundle,
                encoder.as_mut(),
                &self.execute_context,
            );
        }

        list
    }

    pub fn create_sampler(device: &dyn IDevice) -> AnyArc<dyn ISampler> {
        let desc = SamplerDesc {
            min_filter: SamplerFilter::Linear,
            mag_filter: SamplerFilter::Linear,
            mip_filter: SamplerMipFilter::Linear,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }

    pub fn update_font_texture(&mut self, delta: &egui::epaint::ImageDelta) {
        fn coverage_mapper(v: &f32) -> u8 {
            // Function jigged from egui
            fn fast_round(r: f32) -> u8 {
                (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
            }

            fast_round(v.powf(1.0 / 2.2) * 255.0)
        }

        // Increment the version to invalidate the cached textures on the GPU
        self.font_texture.version += 1;

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
                            self.font_texture.bytes[f_idx] = coverage_mapper(&font.pixels[d_idx]);
                        }
                    }
                } else {
                    // Handle a full update

                    // Just replace the old texture with the new data, mapped to u8
                    self.font_texture.width = delta.image.width();
                    self.font_texture.height = delta.image.height();
                    self.font_texture.bytes = font.pixels.iter().map(coverage_mapper).collect();
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

    /// Version index that should be incremented every time the texture is changed so the per-frame
    /// data can detect when it needs to update
    pub version: usize,
}
