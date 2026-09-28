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

use std::sync::Arc;

use aleph_object_system::unsafe_impl_iobject;
use aleph_vfs::IRouter;
use aleph_vfs::path::VPath;
use crossbeam::channel::SendError;
use mg::material_instance::MaterialInstanceHandle;

use crate::core::async_io::worker::AsyncLoaderQueue;
use crate::gltf::internal::GltfLoadPayload;
use crate::render::async_loader::systems::async_load_resolver::AsyncLoadResolverQueue;

#[derive(Clone)]
pub struct GltfLoader {
    pub loader_queue: AsyncLoaderQueue,
    pub dest: AsyncLoadResolverQueue,
    pub vfs: Arc<dyn IRouter>,
    pub default_material: MaterialInstanceHandle,
}

unsafe_impl_iobject!(GltfLoader, "01a0e23b-9013-7572-822f-8abf61cb6c77");

impl GltfLoader {
    pub fn load<P: AsRef<VPath>>(&self, path: P) -> Result<(), SendError<()>> {
        let path = path.as_ref();
        let vfs = self.vfs.clone();
        let payload = GltfLoadPayload {
            path: Box::from(path),
            dest: self.dest.clone(),
            default_material: self.default_material,
        };
        self.loader_queue
            .spawn(async move |io| crate::gltf::internal::task(vfs, io, payload).await)
    }
}
