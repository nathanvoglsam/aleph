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

use crate::swap_chain::SwapChain;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{INamedObject, ITexture, TextureDesc};
use std::ffi::CString;

pub struct SwapTexture {
    pub this: AnyWeak<Self>,
    pub swap_chain: AnyArc<SwapChain>,
    pub image: vk::Image,
    pub vk_format: vk::Format,
    pub desc: TextureDesc,
}

declare_interfaces!(SwapTexture, [ITexture, IImageResourceExt]);

impl ITexture for SwapTexture {
    fn upgrade(&self) -> AnyArc<dyn ITexture> {
        self.this.upgrade().unwrap().query_interface().unwrap()
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &TextureDesc {
        &self.desc
    }
}

impl Drop for SwapTexture {
    fn drop(&mut self) {
        self.swap_chain
            .inner
            .lock()
            .unwrap()
            .images_in_flight
            .checked_sub(1)
            .unwrap();
    }
}

pub trait IImageResourceExt: ITexture {
    fn get_raw_handle(&self) -> vk::Image;
}

impl IImageResourceExt for SwapTexture {
    fn get_raw_handle(&self) -> vk::Image {
        self.image
    }
}

impl INamedObject for SwapTexture {
    fn set_name(&self, name: &str) {
        let loader = &self.swap_chain.device.device_loader;
        if let Some(func) = loader.set_debug_utils_object_name_ext {
            let name = CString::new(name).unwrap();
            let info = vk::DebugUtilsObjectNameInfoEXTBuilder::new()
                .object_type(vk::ObjectType::IMAGE)
                .object_handle(self.image.object_handle())
                .object_name(&name);
            unsafe {
                (func)(loader.handle, &info.build());
            }
        }
    }
}
