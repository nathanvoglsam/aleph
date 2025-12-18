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

use std::any::Any;

use aleph_frame_graph::{FrameGraphBuilder, ResourceRef};
use aleph_pin_board::PinBoard;

use crate::internal::built_in_textures::{create_smaa_area_texture, create_smaa_search_texture};
use crate::renderer::frame_graph::GraphArgs;
use crate::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use crate::renderer::state_cache::StateCache;
use crate::resource::texture::TextureHandle;

pub trait IRenderPlane: Any + Send + Sync {
    fn init_resources(&mut self, resource_builder: &mut ImmediateResourceBuilder);

    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn rhi::IDevice,
        pin_board: &PinBoard,
        state_cache: &mut StateCache,
    ) -> RenderPlaneOutput;
}

pub struct RenderPlaneOutput {
    /// Readable handle to the final (texture) resource that can be considered the output of this
    /// render plane. This will be composited together with the other planes before being presented
    /// to the screen.
    pub id: ResourceRef,

    /// A description of the texture resource. We need this so we know the format/size/etc of the
    /// image so we can correctly handle and validate when performing the merge pass.
    pub desc: rhi::TextureDesc<'static>,
}

#[derive(Default)]
pub struct DefaultRenderPlane {
    smaa_area_tex: Option<TextureHandle>,
    smaa_search_tex: Option<TextureHandle>,
}

impl IRenderPlane for DefaultRenderPlane {
    fn init_resources(&mut self, resource_builder: &mut ImmediateResourceBuilder) {
        self.smaa_area_tex = Some(create_smaa_area_texture(resource_builder));
        self.smaa_search_tex = Some(create_smaa_search_texture(resource_builder));
    }

    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn rhi::IDevice,
        pin_board: &PinBoard,
        state_cache: &mut StateCache,
    ) -> RenderPlaneOutput {
        use super::pass;

        pass::main_gbuffer::pass(frame_graph, device, pin_board, state_cache);
        pass::lighting_resolve::pass(frame_graph, device, pin_board, state_cache);
        let result = pass::tone_map::pass(frame_graph, device, pin_board, state_cache);

        let result = pass::smaa::pass(
            frame_graph,
            device,
            pin_board,
            state_cache,
            &result,
            self.smaa_area_tex.unwrap(),
            self.smaa_search_tex.unwrap(),
        );

        // let _result = pass::fxaa::pass(frame_graph, device, pin_board, state_cache, &result);

        result
    }
}
