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

pub mod pass;

pub(crate) mod built_in_textures;

mod camera;
mod deletion_pool;
mod enqueue_error;
mod format;
mod mesh_layout;
mod object_pool;
mod objects;
mod render_scene;
mod render_scene_objects;
mod renderer;
mod resource_commands;
mod shader_db_accessor;
mod shaders;
mod state_cache;
mod streaming_request;
mod upload_buffer;

pub use camera::{CameraInfo, PerspectiveInfo};
pub use deletion_pool::DeletionPool;
pub use enqueue_error::{EnqueueError, EnqueueErrorKind};
pub use format::vk_to_rhi_format;
pub use mesh_layout::{
    MeshLayoutDesc, MeshLayoutDescError, MeshLayoutId, MeshLayoutIdFields, VertexStream,
};
pub use object_pool::{
    BufferHandle, BufferPool, Handle, HandleFreeError, HandlePool, IntoHandle, MeshHandle,
    ObjectPool, TextureHandle, TexturePool,
};
pub use objects::{BufferObject, TextureObject};
pub use render_scene::{RenderScene, RenderSceneParam, RenderTransform, StorageMut, StorageRef};
pub use render_scene_objects::StaticMesh;
pub use renderer::{
    DefaultRenderPlane, DefaultResources, DrawOptions, IRenderPlane, IRenderSurface,
    RenderPlaneOutput, Renderer, RendererBuilder,
};
pub use resource_commands::{ResourceCommand, ResourceCommandBuffer};
pub use shader_db_accessor::ShaderDatabaseAccessor;
pub use state_cache::{IStateCacheKey, StateCache};
pub use streaming_request::{
    BufferStreamingRequest, ConsumeStreamingRequest, MeshStreamingRequest, PollCompleteError,
    PollFailError, RequestState, StreamingRequest, TextureStreamingRequest,
};
pub use upload_buffer::{
    BufferObjectDesc, BufferUploadDesc, IUploadBuffer, SharedUploadBuffer, TextureObjectDesc,
    TextureUploadDataDesc, TextureUploadDesc,
};

#[cfg(test)]
mod test_utils {
    //!
    //! Utils used by numerous tests in a shared module to reduce code duplication
    //!

    use std::rc::Rc;

    #[derive(Clone)]
    pub struct DropCanary(Rc<()>);

    impl DropCanary {
        pub fn new() -> Self {
            Self(Rc::new(()))
        }

        pub fn strong_count(&self) -> usize {
            Rc::strong_count(&self.0)
        }
    }
}
