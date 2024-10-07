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
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;

use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::object_system::unsafe_impl_iobject;
use aleph_engine::interfaces::renderer::{
    Renderer, TextureLoader, TextureMipUploadDesc, TextureStreamingRequest, TextureUploadSource,
};
use aleph_rhi_api::*;
use image::DynamicImage;

pub struct AsyncTextureLoader {
    queue: AsyncTextureLoaderHandle,
    worker: Option<JoinHandle<()>>,
}

unsafe_impl_iobject!(AsyncTextureLoader, "0192663a-8d56-7bd0-b7b9-495d4590d8a9");

impl AsyncTextureLoader {
    pub fn new(renderer: &Renderer) -> Self {
        let (sender, receiver) = mpsc::channel();

        let mut worker = LoaderWorker::new(
            receiver,
            renderer.device().upgrade(),
            renderer.get_texture_loader_handle(),
        );
        let worker = std::thread::Builder::new()
            .name("AsyncTextureLoader".to_string())
            .spawn(move || {
                aleph_profile::register_thread!("AsyncTextureLoader");
                worker.run()
            })
            .unwrap();

        Self {
            queue: AsyncTextureLoaderHandle::new(sender),
            worker: Some(worker),
        }
    }

    pub fn load(&self, path: PathBuf, srgb: bool) -> TextureStreamingRequest {
        self.queue.load(path, srgb)
    }
}

impl Drop for AsyncTextureLoader {
    fn drop(&mut self) {
        self.queue.queue.send(WorkerCommand::Exit).unwrap();
        self.worker.take().unwrap().join().unwrap();
    }
}

pub struct AsyncTextureLoaderHandle {
    queue: mpsc::Sender<WorkerCommand>,
}

impl AsyncTextureLoaderHandle {
    fn new(v: mpsc::Sender<WorkerCommand>) -> Self {
        Self { queue: v }
    }

    pub fn load(&self, path: PathBuf, srgb: bool) -> TextureStreamingRequest {
        let request = TextureStreamingRequest::new();
        let message = WorkerCommand::Load(LoadCommand {
            path,
            request: request.clone(),
            srgb,
        });
        self.queue.send(message).ok().unwrap();
        request
    }
}

struct LoaderWorker {
    queue: mpsc::Receiver<WorkerCommand>,
    device: AnyArc<dyn IDevice>,
    loader: Arc<TextureLoader>,
}

impl LoaderWorker {
    fn new(
        receiver: mpsc::Receiver<WorkerCommand>,
        device: AnyArc<dyn IDevice>,
        loader: Arc<TextureLoader>,
    ) -> Self {
        Self {
            queue: receiver,
            device,
            loader,
        }
    }

    fn run(&mut self) {
        loop {
            match self.queue.recv() {
                Ok(WorkerCommand::Exit) => {
                    return;
                }
                Ok(WorkerCommand::Load(cmd)) => match self.load(&cmd) {
                    Some(_) => {}
                    None => {
                        let _ = cmd.request.mark_failed();
                    }
                },
                Err(_e) => {
                    // TODO: log the error then return
                    return;
                }
            }
        }
    }

    #[aleph_profile::function]
    fn load(&mut self, cmd: &LoadCommand) -> Option<()> {
        let extension = cmd.path.extension()?;
        let format = image::ImageFormat::from_extension(extension)?;

        let data = std::fs::read(&cmd.path).ok()?;
        let image = image::load_from_memory_with_format(&data, format).ok()?;

        let desc = TextureMipUploadDesc {
            width: image.width(),
            height: image.height(),
            depth: 1,
            format: if cmd.srgb {
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
                        self.device.as_ref(),
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
                        self.device.as_ref(),
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

        self.loader
            .enqueue_new_upload(cmd.request.clone(), data)
            .ok()?;
        Some(())
    }
}

enum WorkerCommand {
    Exit,
    Load(LoadCommand),
}

struct LoadCommand {
    path: PathBuf,
    request: TextureStreamingRequest,
    srgb: bool,
}
