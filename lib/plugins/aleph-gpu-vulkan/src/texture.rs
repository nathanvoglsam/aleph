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

use crate::device::Device;
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use std::any::TypeId;

pub struct Texture {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) image: vk::Image,
    pub(crate) vk_format: vk::Format,
    pub(crate) is_owned: bool,
    pub(crate) desc: TextureDesc<'static>,
    pub(crate) name: Option<String>,
}

declare_interfaces!(Texture, [ITexture]);

impl IGetPlatformInterface for Texture {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<vk::Image>(&self.image, out, target)
    }
}

impl ITexture for Texture {
    fn upgrade(&self) -> AnyArc<dyn ITexture> {
        AnyArc::map::<dyn ITexture, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> TextureDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            // Some images we don't own, like swap chain images, so we shouldn't destroy them
            if self.is_owned {
                self.device.device_loader.destroy_image(self.image, None);
            }
        }
    }
}
