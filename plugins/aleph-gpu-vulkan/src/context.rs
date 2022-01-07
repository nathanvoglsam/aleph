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

use crate::adapter::Adapter;
use crate::surface::Surface;
use erupt::vk;
use interfaces::any::declare_interfaces;
use interfaces::gpu::{
    AdapterPowerClass, AdapterRequestOptions, IGpuAdapter, IGpuContext, IGpuSurface,
};
use interfaces::platform::HasRawWindowHandle;

pub struct Context {
    pub(crate) instance_loader: erupt::InstanceLoader,
    pub(crate) messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl IGpuContext for Context {
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<Box<dyn IGpuAdapter>> {
        todo!()
    }

    fn create_surface(&self, window: &dyn HasRawWindowHandle) -> Box<dyn IGpuSurface> {
        todo!()
    }
}

pub trait IGpuContextExt: IGpuContext {}

impl IGpuContextExt for Context {}

declare_interfaces!(Context, [IGpuContext, IGpuContextExt]);