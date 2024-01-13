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
mod global;
mod params;

use crate::renderer::egui_pass::EguiPassContext;
use crate::renderer::params::BackBufferInfo;
use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use egui::RenderData;
pub(crate) use frame::PerFrameObjects;
pub(crate) use global::GlobalObjects;
use interfaces::any::AnyArc;
use std::ops::{Deref, DerefMut};

pub struct EguiRenderer {
    pub device: AnyArc<dyn IDevice>,
    pub frames: Vec<PerFrameObjects>,
    pub global: GlobalObjects,
    pub frame_graph: FrameGraph,
    pub back_buffer_id: ResourceMut,
    pub graph_build_pin_board: PinBoard,
    pub execute_context: PinBoard,
}

impl EguiRenderer {
    pub fn new(
        device: AnyArc<dyn IDevice>,
        back_buffer_desc: &TextureDesc,
        pixels_per_point: f32,
    ) -> Self {
        log::trace!("Initializing Egui Renderer");

        let global = GlobalObjects::new(device.deref());

        let pin_board = PinBoard::new();
        pin_board.publish(BackBufferInfo {
            desc: back_buffer_desc.clone().strip_name(),
            pixels_per_point,
        });

        let frame_graph = Self::create_frame_graph(&global, &pin_board);

        let output: &egui_pass::EguiPassOutput = pin_board.get().unwrap();
        let back_buffer_id = output.id;

        let frames = (0..2)
            .into_iter()
            .map(|_| PerFrameObjects::new(device.deref(), &global, &frame_graph))
            .collect();

        Self {
            device,
            frames,
            global,
            frame_graph,
            back_buffer_id,
            graph_build_pin_board: pin_board,
            execute_context: PinBoard::new(),
        }
    }

    pub fn rebuild_after_resize(&mut self, back_buffer_desc: &TextureDesc, pixels_per_point: f32) {
        self.graph_build_pin_board.clear();
        let pin_board = &self.graph_build_pin_board;

        pin_board.publish(BackBufferInfo {
            desc: back_buffer_desc.clone().strip_name(),
            pixels_per_point,
        });

        let frame_graph = Self::create_frame_graph(&self.global, &pin_board);

        let output: &egui_pass::EguiPassOutput = pin_board.get().unwrap();
        let back_buffer_id = output.id;

        let frames: Vec<_> = (0..2)
            .into_iter()
            .map(|_| PerFrameObjects::new(self.device.as_ref(), &self.global, &frame_graph))
            .collect();

        self.frames = frames;
        self.frame_graph = frame_graph;
        self.back_buffer_id = back_buffer_id;
    }

    pub fn create_frame_graph(global: &GlobalObjects, pin_board: &PinBoard) -> FrameGraph {
        let mut frame_graph = FrameGraph::builder();
        egui_pass::egui_pass(&mut frame_graph, &pin_board, global);
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
                    self.global.update_font_texture(delta);
                }
            }

            // If the versions do not match then we should re-upload the texture to the GPU
            if self.frames[index].font_version != self.global.font_texture.version {
                self.frames[index].update_texture_data(
                    self.device.deref(),
                    self.global.sampler.deref(),
                    &self.global.font_texture,
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
}
