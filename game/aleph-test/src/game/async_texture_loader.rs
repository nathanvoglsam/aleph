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

use std::path::PathBuf;
use std::sync::Arc;

use aleph_device_allocators::UploadBumpAllocator;
use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::object_system::unsafe_impl_iobject;
use aleph_ktx::{DocumentType, KtxDocument, VkFormat};
use aleph_rhi_api::*;
use crossbeam::queue::SegQueue;

use crate::game::async_loader::AsyncLoader;

pub struct AsyncTextureLoader {
    loader: AsyncLoader<AsyncTextureLoadCommand>,
    channel: Arc<SegQueue<WorkerOutput>>,
}

unsafe_impl_iobject!(AsyncTextureLoader, "0192663a-8d56-7bd0-b7b9-495d4590d8a9");

impl AsyncTextureLoader {
    pub fn new(device: AnyArc<dyn IDevice>) -> Self {
        let upload_buffer = UploadBumpAllocator::new_upload_buffer(
            device.as_ref(),
            256 * 1024 * 1024,
            Some("AsyncTextureLoaderUploadBlock"),
        )
        .unwrap();

        let channel = Arc::new(SegQueue::new());

        let context = TextureLoaderContext {
            device,
            channel: channel.clone(),
            upload_buffer,
        };
        let loader = AsyncLoader::new(context, handler);

        Self { loader, channel }
    }

    pub fn load(&self, request: AsyncTextureLoadRequest) -> TextureStreamingRequest {
        let req = TextureStreamingRequest::new();

        self.loader.load(AsyncTextureLoadCommand {
            req: req.clone(),
            v: request,
        });

        req
    }

    pub fn think(&self, renderer: &mut Renderer) {
        while let Some(output) = self.channel.pop() {
            let handle = renderer.create_texture(output.object).unwrap();
            renderer.submit_resource_command(ResourceCommand::TextureUpload(
                handle,
                GenerateMips::No,
                output.data,
            ));
            output.request.mark_complete(handle).unwrap();
        }
    }
}

pub struct AsyncTextureLoadRequest {
    pub path: PathBuf,
}

struct AsyncTextureLoadCommand {
    req: TextureStreamingRequest,
    v: AsyncTextureLoadRequest,
}

struct TextureLoaderContext {
    device: AnyArc<dyn IDevice>,
    channel: Arc<SegQueue<WorkerOutput>>,
    upload_buffer: UploadBumpAllocator,
}

fn handler(context: &mut TextureLoaderContext, request: &AsyncTextureLoadCommand) {
    let _ = load(context, request);
}

#[aleph_profile::function]
fn load(context: &mut TextureLoaderContext, request: &AsyncTextureLoadCommand) -> Option<()> {
    let data = std::fs::read(&request.v.path).ok()?;
    let doc = KtxDocument::from_slice(&data).ok()?;

    if doc.document_type() != DocumentType::Image2D {
        return None;
    }

    let mut desc = TextureObjectDesc::new();
    desc.image_2d(doc.width(), doc.height());
    desc.num_levels_full();
    desc.format(Format::Rgba8Unorm);
    desc.usage(ResourceUsageFlags::SHADER_RESOURCE);

    let new_source = {
        let new_source = TextureUploadDesc::new_in_bump_arena(
            &context.upload_buffer,
            &desc,
            0,
            desc.num_levels.get(),
        );
        match new_source {
            Ok(v) => v,
            Err(BufferCreateError::OutOfMemory) => {
                aleph_profile::scope_named!("AsyncTextureLoader::NewBlock");
                let mut new_buffer = UploadBumpAllocator::new_upload_buffer(
                    context.device.as_ref(),
                    256 * 1024 * 1024,
                    Some("AsyncTextureLoaderUploadBlock"),
                )
                .unwrap();
                std::mem::swap(&mut new_buffer, &mut context.upload_buffer);

                TextureUploadDesc::new_in_bump_arena(
                    &context.upload_buffer,
                    &desc,
                    0,
                    desc.num_levels.get(),
                )
                .unwrap()
            }
            Err(_) => {
                panic!()
            }
        }
    };

    let send = context.channel.clone();
    let device = context.device.clone();
    let reqeust = request.req.clone();
    rayon::spawn(move || {
        let result = load_on_threadpool(&send, device, reqeust, data, desc, new_source);
        match result {
            Some(_) => {}
            None => {
                log::error!("Texture Upload Failed on Worker!")
            }
        }
    });

    Some(())
}

#[aleph_profile::function]
fn load_on_threadpool(
    send: &SegQueue<WorkerOutput>,
    device: AnyArc<dyn IDevice>,
    request: TextureStreamingRequest,
    data: Vec<u8>,
    desc: TextureObjectDesc,
    mut upload: TextureUploadDesc,
) -> Option<()> {
    // If the request has been moved into a terminal state (cancelled) we should bail.
    if !request.poll_state().is_open() {
        return Some(());
    }

    let doc = match KtxDocument::from_slice(&data) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to parse KTX on Worker: {e:?}");
            return None;
        }
    };

    let data = match doc.format() {
        VkFormat::R8G8B8A8_UNORM => {
            for level in 0..desc.num_levels.get() {
                let num_rows = desc.num_rows_for_level(level);
                let row_bytes = desc.row_bytes_for_level(level);
                let row_bytes_padded = desc.upload_row_bytes_for_level(level);
                let src = doc
                    .get_level_info(level)
                    .inspect_err(|e| log::error!("Failed to get level {level} in KTX doc: {e:?}"))
                    .ok()?;
                let mut src = &data[src.to_slice_range()];
                let dst = upload.data.offset_for_level(level)?;
                let mut dst = &mut upload.buffer.bytes_mut()[dst..];
                for _ in 0..num_rows {
                    let copy_src = &src[0..row_bytes];
                    let copy_dst = &mut dst[0..row_bytes];
                    copy_dst.copy_from_slice(copy_src);
                    src = &src[row_bytes..];
                    dst = &mut dst[row_bytes_padded..];
                }
            }

            upload
        }
        _ => return None,
    };

    // We want to allocate the texture on the worker thread, even at the cost of contention, so we
    // don't block the render thread and so we can distribute the work over multiple cores.
    {
        aleph_profile::scope_named!("EnqueueAndAllocate");
        let mut object = TextureObject::new_for_desc(device.as_ref(), desc).unwrap();
        object.recreate_default_view(device.as_ref());
        send.push(WorkerOutput {
            request,
            data,
            object,
        });
    }

    Some(())
}

struct WorkerOutput {
    request: TextureStreamingRequest,
    data: TextureUploadDesc,
    object: TextureObject,
}
