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

use aleph_rhi_api::*;
use interfaces::any::AnyArc;

use crate::render::LoaderDeletionPool;

pub struct PerFrameObjects {
    pub acquire_semaphore: AnyArc<dyn ISemaphore>,
    pub present_semaphore: AnyArc<dyn ISemaphore>,

    pub deletion_pool: LoaderDeletionPool,

    pub uniform_buffer: AnyArc<dyn IBuffer>,

    pub done_fence: AnyArc<dyn IFence>,
}

impl PerFrameObjects {
    pub fn new(device: &dyn IDevice) -> Self {
        let uniform_buffer = device
            .create_buffer(&BufferDesc {
                size: 1024,
                cpu_access: CpuAccessMode::Write,
                usage: ResourceUsageFlags::CONSTANT_BUFFER,
                name: Some("egui::ConstantBuffer"),
            })
            .unwrap();

        Self {
            acquire_semaphore: device.create_semaphore().unwrap(),
            present_semaphore: device.create_semaphore().unwrap(),
            deletion_pool: Default::default(),
            uniform_buffer,
            done_fence: device.create_fence(true).unwrap(),
        }
    }
}
