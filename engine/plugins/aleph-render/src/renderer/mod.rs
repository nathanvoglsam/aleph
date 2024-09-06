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

mod egui_font_texture;
mod frame;
mod pass;

use std::num::NonZeroU8;
use std::ops::Deref;

use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::ShaderDatabase;
use egui::epaint::ImageDelta;
use egui::{ImageData, RenderData};
pub(crate) use frame::PerFrameObjects;
use interfaces::any::AnyArc;

use crate::render::{
    BufferLoader, BufferPool, ShaderDatabaseAccessor, TextureHandle, TextureLoader,
    TextureMipUploadDesc, TexturePool, TextureStreamingRequest, TextureUploadSource,
};
use crate::renderer::egui_font_texture::FontTexture;
use crate::renderer::pass::backbuffer_import::BackBufferHandle;
use crate::renderer::pass::egui_draw::EguiPassContext;
use crate::renderer::pass::BackBufferInfo;

pub struct EguiRenderer {
    pub device: AnyArc<dyn IDevice>,
    pub frames: Vec<PerFrameObjects>,

    pub buffer_pool: BufferPool,
    pub buffer_loader: BufferLoader,

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

        let buffer_pool = BufferPool::new(NonZeroU8::new(2).unwrap());
        let buffer_loader = BufferLoader::new();

        let mut texture_pool = TexturePool::new(NonZeroU8::new(1).unwrap());
        let texture_loader = TextureLoader::new();

        let font_handle = texture_pool.reserve_handle();

        Self {
            device,
            frames,
            buffer_pool,
            buffer_loader,
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
        self.frames[index].deletion_pool.purge();

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

            {
                encoder.begin_event(Color::BLUE, nstr!("Upload Streaming Requests"));

                // TODO: we want to batch all of these, so we need a better interface so we can bundle
                //       the barriers and copy commands for all our loaders into a single batch.
                //
                //       either that or we unify to a single loader type.
                self.buffer_loader.upload_requests(
                    &mut self.buffer_pool,
                    &mut self.frames[index].deletion_pool,
                    self.device.as_ref(),
                    encoder.as_mut(),
                    usize::MAX,
                );
                self.texture_loader.upload_requests(
                    &mut self.texture_pool,
                    &mut self.frames[index].deletion_pool,
                    self.device.as_ref(),
                    encoder.as_mut(),
                    usize::MAX,
                );

                encoder.end_event();
            }

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
