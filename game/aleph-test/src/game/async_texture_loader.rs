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

use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::object_system::unsafe_impl_iobject;
use aleph_engine::interfaces::renderer::{
    TextureLoader, TextureMipUploadDesc, TextureStreamingRequest, TextureUploadSource,
};
use aleph_rhi_api::*;
use image::DynamicImage;

use crate::game::async_loader::{AsyncLoader, AsyncLoaderHandle};

pub struct TextureLoaderContext {
    pub device: AnyArc<dyn IDevice>,
    pub loader: Arc<TextureLoader>,
}

pub struct AsyncTextureLoader {
    loader: AsyncLoader<TextureLoaderContext, AsyncTextureLoadCommand>,
}

unsafe_impl_iobject!(AsyncTextureLoader, "0192663a-8d56-7bd0-b7b9-495d4590d8a9");

impl AsyncTextureLoader {
    pub fn new(context: TextureLoaderContext) -> Self {
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
    inner: AsyncLoaderHandle<TextureLoaderContext, AsyncTextureLoadCommand>,
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

fn handler(context: &TextureLoaderContext, request: &AsyncTextureLoadCommand) {
    let _ = load(context, request);
}

#[aleph_profile::function]
fn load(context: &TextureLoaderContext, request: &AsyncTextureLoadCommand) -> Option<()> {
    let extension = request.v.path.extension()?;
    let format = image::ImageFormat::from_extension(extension)?;
    let data = std::fs::read(&request.v.path).ok()?;

    let device = context.device.clone();
    let loader = context.loader.clone();
    let reqeust = request.req.clone();
    let srgb = request.v.srgb;
    rayon::spawn(move || {
        let _ = load_on_threadpool(device, loader, reqeust, srgb, data, format);
    });

    Some(())
}

#[aleph_profile::function]
fn load_on_threadpool(
    device: AnyArc<dyn IDevice>,
    loader: Arc<TextureLoader>,
    request: TextureStreamingRequest,
    srgb: bool,
    data: Vec<u8>,
    format: image::ImageFormat,
) -> Option<()> {
    let image = image::load_from_memory_with_format(&data, format).ok()?;

    let desc = TextureMipUploadDesc {
        width: image.width(),
        height: image.height(),
        depth: 1,
        format: if srgb {
            Format::Rgba8UnormSrgb
        } else {
            Format::Rgba8Unorm
        },
    };

    let data = match image {
        DynamicImage::ImageLuma8(_i) => return None,
        DynamicImage::ImageLumaA8(_i) => return None,
        DynamicImage::ImageRgb8(i) => {
            let data = unsafe {
                TextureUploadSource::new_owned(
                    device.as_ref(),
                    desc,
                    ResourceUsageFlags::SHADER_RESOURCE,
                )
                .ok()?
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

    loader.enqueue_new_upload(request, data).ok()?;

    Some(())
}
