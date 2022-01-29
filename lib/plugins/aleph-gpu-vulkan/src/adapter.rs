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

use crate::context::Context;
use crate::device::Device;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::gpu::{AdapterDescription, IAdapter, IDevice, RequestDeviceError};

pub struct Adapter {
    pub(crate) name: String,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) context: AnyArc<Context>,
}

impl IAdapter for Adapter {
    fn description(&mut self) -> AdapterDescription {
        AdapterDescription { name: &self.name }
    }

    fn request_device(&mut self) -> Result<Box<dyn IDevice>, RequestDeviceError> {
        todo!()
    }
}

pub trait IAdapterExt: IAdapter {}

impl IAdapterExt for Adapter {}

declare_interfaces!(Adapter, [IAdapter, IAdapterExt]);
