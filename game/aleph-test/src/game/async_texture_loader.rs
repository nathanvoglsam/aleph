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

use aleph_device_allocators::{IUploadAllocator, RawDeviceAllocationResult, UploadBumpAllocator};
use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::object_system::unsafe_impl_iobject;
use aleph_engine::interfaces::renderer::{
    GenerateMips, TextureAllocMode, TextureLoader, TextureMipUploadDesc, TextureStreamingRequest,
    TextureUploadSource,
};
use aleph_rhi_api::*;
use image::{DynamicImage, ImageDecoder};

use crate::game::async_loader::{AsyncLoader, AsyncLoaderHandle};

pub struct AsyncTextureLoader {
    loader: AsyncLoader<AsyncTextureLoadCommand>,
}

unsafe_impl_iobject!(AsyncTextureLoader, "0192663a-8d56-7bd0-b7b9-495d4590d8a9");

impl AsyncTextureLoader {
    pub fn new(device: AnyArc<dyn IDevice>, loader: Arc<TextureLoader>) -> Self {
        let upload_buffer = UploadBumpAllocator::new_upload_buffer(
            device.as_ref(),
            256 * 1000 * 1000,
            Some("AsyncTextureLoaderUploadBlock"),
        )
        .unwrap();
        let context = TextureLoaderContext {
            device,
            loader,
            upload_buffer,
        };
        let loader = AsyncLoader::new(context, handler);
        Self { loader }
    }

    pub fn load(&self, request: AsyncTextureLoadRequest) -> TextureStreamingRequest {
        let req = TextureStreamingRequest::new();

        self.loader.load(AsyncTextureLoadCommand {
            req: req.clone(),
            v: request,
        });

        req
    }

    pub fn handle(&self) -> AsyncTextureLoader2Handle {
        AsyncTextureLoader2Handle {
            inner: self.loader.handle(),
        }
    }
}

pub struct AsyncTextureLoader2Handle {
    inner: AsyncLoaderHandle<AsyncTextureLoadCommand>,
}

impl AsyncTextureLoader2Handle {
    pub fn load(&self, request: AsyncTextureLoadRequest) -> TextureStreamingRequest {
        let req = TextureStreamingRequest::new();

        self.inner.load(AsyncTextureLoadCommand {
            req: req.clone(),
            v: request,
        });

        req
    }
}

pub struct AsyncTextureLoadRequest {
    pub path: PathBuf,
    pub srgb: bool,
}

struct AsyncTextureLoadCommand {
    req: TextureStreamingRequest,
    v: AsyncTextureLoadRequest,
}

struct TextureLoaderContext {
    device: AnyArc<dyn IDevice>,
    loader: Arc<TextureLoader>,
    upload_buffer: UploadBumpAllocator,
}

fn handler(context: &mut TextureLoaderContext, request: &AsyncTextureLoadCommand) {
    let _ = load(context, request);
}

#[aleph_profile::function]
fn load(context: &mut TextureLoaderContext, request: &AsyncTextureLoadCommand) -> Option<()> {
    let extension = request.v.path.extension()?;
    let format = image::ImageFormat::from_extension(extension)?;
    let data = std::fs::read(&request.v.path).ok()?;

    let reader = image::ImageReader::with_format(std::io::Cursor::new(&data), format)
        .into_decoder()
        .ok()?;
    let (width, height) = reader.dimensions();
    let desc = TextureMipUploadDesc {
        width,
        height,
        depth: 1,
        format: if request.v.srgb {
            Format::Rgba8UnormSrgb
        } else {
            Format::Rgba8Unorm
        },
    };
    let size = desc.size_requirement();
    let block = match context.upload_buffer.allocate_aligned(size, 256) {
        Some(block) => block,
        None => {
            aleph_profile::scope!("AsyncTextureLoader::NewBlock");
            let mut new_buffer = UploadBumpAllocator::new_upload_buffer(
                context.device.as_ref(),
                256 * 1024 * 1024,
                Some("AsyncTextureLoaderUploadBlock"),
            )
            .unwrap();
            std::mem::swap(&mut new_buffer, &mut context.upload_buffer);
            context.upload_buffer.allocate_aligned(size, 256).unwrap()
        }
    };
    let allocation = AllocationWithBuffer {
        allocation: block,
        buffer: context.upload_buffer.buffer().upgrade(),
    };
    drop(reader);

    let device = context.device.clone();
    let loader = context.loader.clone();
    let reqeust = request.req.clone();
    rayon::spawn(move || {
        let _ = load_on_threadpool(device, loader, reqeust, desc, data, format, allocation);
    });

    Some(())
}

#[aleph_profile::function]
fn load_on_threadpool(
    device: AnyArc<dyn IDevice>,
    loader: Arc<TextureLoader>,
    request: TextureStreamingRequest,
    desc: TextureMipUploadDesc,
    data: Vec<u8>,
    format: image::ImageFormat,
    allocation: AllocationWithBuffer,
) -> Option<()> {
    // If the request has been moved into a terminal state (cancelled) we should bail.
    if request.poll_state().is_terminal() {
        return Some(());
    }

    let image = image::load_from_memory_with_format(&data, format).ok()?;

    let data = match image {
        DynamicImage::ImageLuma8(_i) => return None,
        DynamicImage::ImageLumaA8(_i) => return None,
        DynamicImage::ImageRgb8(i) => {
            let data = unsafe {
                let data = std::ptr::NonNull::slice_from_raw_parts(
                    allocation.allocation.result,
                    desc.size_requirement(),
                );
                TextureUploadSource::new(
                    allocation.buffer,
                    desc,
                    allocation.allocation.device_offset as u64,
                    ResourceUsageFlags::SHADER_RESOURCE,
                    data,
                )
            };

            let row_width = i.width() as usize * 3;
            let src = i.as_raw().as_slice();
            for row in 0..i.height() {
                let dst = unsafe { data.row_ptr(row).as_mut() };

                let row_start = row as usize * row_width;
                let src = &src[row_start..row_start + row_width];
                for col in 0..i.width() {
                    let dst_base = col as usize * 4;
                    let dst = &mut dst[dst_base..dst_base + 3];

                    let src_base = col as usize * 3;
                    let src = &src[src_base..src_base + 3];

                    dst.copy_from_slice(src);
                }
            }

            data
        }
        DynamicImage::ImageRgba8(i) => {
            let data = unsafe {
                TextureUploadSource::new_owned(
                    device.as_ref(),
                    desc,
                    ResourceUsageFlags::SHADER_RESOURCE,
                )
                .ok()?
            };

            let row_width = i.width() as usize * 4;
            let src = i.as_raw().as_slice();
            for row in 0..i.height() {
                let dst = unsafe { data.row_ptr(row).as_mut() };

                let row_start = row as usize * row_width;
                let src = &src[row_start..row_start + row_width];
                dst.copy_from_slice(src);
            }

            data
        }
        DynamicImage::ImageLuma16(_i) => return None,
        DynamicImage::ImageLumaA16(_i) => return None,
        DynamicImage::ImageRgb16(_i) => return None,
        DynamicImage::ImageRgba16(_i) => return None,
        DynamicImage::ImageRgb32F(_i) => return None,
        DynamicImage::ImageRgba32F(_i) => return None,
        _ => todo!(),
    };

    // We want to allocate the texture on the worker thread, even at the cost of contention, so we
    // don't block the render thread and so we can distribute the work over multiple cores.
    {
        aleph_profile::scope!("load_on_threadpool::EnqueueAndAllocate");
        loader
            .enqueue_new_upload(
                request,
                data,
                TextureAllocMode::Immediate,
                GenerateMips::Yes,
            )
            .ok()?;
    }

    Some(())
}

struct AllocationWithBuffer {
    allocation: RawDeviceAllocationResult,
    buffer: AnyArc<dyn IBuffer>,
}

unsafe impl Send for AllocationWithBuffer {}
