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

use aleph_frame_graph::{FrameGraph, FrameGraphBuilder, ImportBundle, ResourceMut};
use aleph_pin_board::PinBoard;

use crate::internal::renderer::swap_manager::SwapManager;
use crate::renderer::config::RendererConfig;
use crate::renderer::frame_graph::{GraphArgs, GraphArgsLayout, GraphSwapImageInfo};
use crate::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use crate::renderer::pass;
use crate::renderer::render_plane::IRenderPlane;
use crate::renderer::state_cache::StateCache;

/// Manager type that handles the creation/recreation of a [`FrameGraph`] instance by pulling
/// together a list of passes from the set of registered [`IRenderPlanes`] objects.
pub struct GraphManager {
    /// The frame graph object itself. Can be `None` if it has not yet been built for the first
    /// time.
    pub frame_graph: Option<FrameGraph<GraphArgs>>,

    /// Pin-board used (and re-used) for both building the frame graph, and when executing the frame
    /// graph.
    pub pin_board: PinBoard,

    /// Ordered list of render plane objects that make up the frame graph we'll be building and
    /// executing
    pub render_planes: Vec<Box<dyn IRenderPlane>>,

    /// The ID of the swap chain image resource inside the frame graph. This is needed so that you
    /// can import the swap chain image into the frame graph.
    pub swap_image_id: Option<ResourceMut>,
}

impl GraphManager {
    //#[aleph_profile::function]
    pub unsafe fn build_graph(
        &mut self,
        config: &RendererConfig,
        state_cache: &mut StateCache,
        device: &dyn rhi::IDevice,
        swap_manager: &SwapManager,
    ) {
        self.pin_board.clear();

        self.pin_board.publish(GraphSwapImageInfo {
            desc: swap_manager.desc.clone(),
            aspect: swap_manager.extent.width as f32 / swap_manager.extent.height as f32,
        });

        unsafe {
            let NewFrameGraph {
                builder,
                swap_image_id,
            } = self.register_graph_passes(state_cache, device);

            // TODO: what is keeping the transients from a previous frame alive?
            let mut frame_graph = builder.build(device);
            frame_graph.allocate_transients(config.render_ahead_frames + 1);

            self.frame_graph = Some(frame_graph);
            self.swap_image_id = Some(swap_image_id);
        }
    }

    /// This function should be called by the renderer exactly once, at startup, to allow each
    /// render plane implementation to create any texture and buffer objects that are _not_
    /// transient.
    ///
    /// For example:
    ///
    /// - A pass that uses a lookup table texture can create the texture once in here.
    /// - A pass that uses the same vertex buffer every time can create it here.
    #[aleph_profile::function]
    pub fn init_graph_resources(&mut self, resource_builder: &mut ImmediateResourceBuilder) {
        for plane in self.render_planes.iter_mut() {
            plane.init_resources(resource_builder);
        }
    }

    /// This function should be called to create a new frame buffer, and must be called after
    /// [`GraphManager::init_graph_resource`] has been called before.
    ///
    /// This function creates a new FrameGraph instance and drives the [`IRenderPlane`] objects to
    /// provide their list of passes. The plane's output texture is then merged in an internally
    /// managed 'composite planes' pass that combines the planes directly into the swap texture.
    ///
    /// This function ultimately returns the new [`FrameGraph`] instances, and a graph resource id
    /// that represents the imported swap chain texture. The renderer must provide the swap texture
    /// when executing the graph in an import bundle associated with the returned ID.
    #[aleph_profile::function]
    pub unsafe fn register_graph_passes(
        &mut self,
        state_cache: &mut StateCache,
        device: &dyn rhi::IDevice,
    ) -> NewFrameGraph {
        let mut frame_graph = FrameGraph::<GraphArgs>::builder();

        let mut outputs = Vec::with_capacity(self.render_planes.capacity());
        for plane in self.render_planes.iter_mut() {
            let output =
                plane.register_passes(&mut frame_graph, device, &self.pin_board, state_cache);
            outputs.push(output);
        }

        let swap_id = pass::composite_planes::pass(
            &mut frame_graph,
            device,
            &self.pin_board,
            state_cache,
            &outputs,
        );

        NewFrameGraph {
            builder: frame_graph,
            swap_image_id: swap_id,
        }
    }

    /// Simple wrapper for executing the inner [`FrameGraph`] this type manages. This must be
    /// called after the graph has been constructed with [`GraphManager::build_graph`].
    ///
    /// # Panics
    ///
    /// This will panic if the graph hasn't been built yet.
    ///
    /// # Safety
    ///
    /// Lol. Lmao. This records work into a GPU command encoder. This is only as safe as the
    /// commands the passes record.
    pub unsafe fn execute_graph(
        &mut self,
        config: &RendererConfig,
        frame_index: usize,
        import_bundle: &ImportBundle,
        encoder: &mut rhi::CommandEncoder,
        args: &GraphArgsLayout,
    ) {
        unsafe {
            let index = frame_index % (config.render_ahead_frames + 1);

            let fg = self.frame_graph.as_mut().unwrap();
            fg.execute(index, import_bundle, encoder, args)
        }
    }

    /// Destroys all the internally managed resources.
    ///
    /// Intended for use when dropping a renderer instance to ensure all handles are released.
    pub fn clean_up(&mut self) {
        self.frame_graph = None;
        self.pin_board.clear();
        self.render_planes.clear();
        self.swap_image_id = None;
    }
}

/// Result of [`GraphManager::register_graph_passes`].
pub struct NewFrameGraph {
    /// The new frame graph builder that we have created. Clients 'register graph passes' are
    /// expected to decide the correct time to actually _build_ the graph.
    pub builder: FrameGraphBuilder<GraphArgs>,

    /// A resource ID that must be provided in an import bundle when executing the returned frame
    /// graph instance. This ID represents the swap chain image, and expects to be directly given
    /// the current frame's back buffer texture.
    pub swap_image_id: ResourceMut,
}
