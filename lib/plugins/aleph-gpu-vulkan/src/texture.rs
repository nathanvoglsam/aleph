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
use crate::internal::conv::{image_view_type_to_vk, subresource_range_to_vk, texture_format_to_vk};
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use erupt::{vk, ExtendableFrom};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use parking_lot::Mutex;
use std::any::TypeId;
use std::collections::HashMap;
use vulkan_alloc::vma;

pub struct Texture {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) image: vk::Image,
    pub(crate) allocation: Option<vma::Allocation>,
    pub(crate) is_owned: bool,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
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
        AnyArc::map::<dyn ITexture, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn desc(&self) -> TextureDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }

    fn get_view(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.views.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let usage = if desc.writable {
                vk::ImageUsageFlags::STORAGE
            } else {
                vk::ImageUsageFlags::SAMPLED
            };

            let mut usage_info = vk::ImageViewUsageCreateInfoBuilder::new().usage(usage);

            let create_info = vk::ImageViewCreateInfoBuilder::new()
                .image(self.image)
                .view_type(image_view_type_to_vk(desc.view_type))
                .format(texture_format_to_vk(desc.format))
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(subresource_range_to_vk(&desc.sub_resources))
                .extend_from(&mut usage_info);

            let view = unsafe {
                self._device
                    .device_loader
                    .create_image_view(&create_info, None)
                    .map_err(|_| ())? // TODO: error handling
            };

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.rtvs.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let mut usage_info = vk::ImageViewUsageCreateInfoBuilder::new()
                .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

            let create_info = vk::ImageViewCreateInfoBuilder::new()
                .image(self.image)
                .view_type(image_view_type_to_vk(desc.view_type))
                .format(texture_format_to_vk(desc.format))
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(subresource_range_to_vk(&desc.sub_resources))
                .extend_from(&mut usage_info);

            let view = unsafe {
                self._device
                    .device_loader
                    .create_image_view(&create_info, None)
                    .map_err(|_| ())? // TODO: error handling
            };

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.dsvs.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let mut usage_info = vk::ImageViewUsageCreateInfoBuilder::new()
                .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);

            let create_info = vk::ImageViewCreateInfoBuilder::new()
                .image(self.image)
                .view_type(image_view_type_to_vk(desc.view_type))
                .format(texture_format_to_vk(desc.format))
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(subresource_range_to_vk(&desc.sub_resources))
                .extend_from(&mut usage_info);

            let view = unsafe {
                self._device
                    .device_loader
                    .create_image_view(&create_info, None)
                    .map_err(|_| ())? // TODO: error handling
            };

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            for (_desc, view) in self.views.get_mut().drain() {
                self._device.device_loader.destroy_image_view(view, None);
            }

            for (_desc, view) in self.rtvs.get_mut().drain() {
                self._device.device_loader.destroy_image_view(view, None);
            }

            for (_desc, view) in self.dsvs.get_mut().drain() {
                self._device.device_loader.destroy_image_view(view, None);
            }

            // Some images we don't own, like swap chain images, so we shouldn't destroy them
            if self.is_owned {
                if let Some(allocation) = self.allocation {
                    self._device.allocator.destroy_image(self.image, allocation);
                }
            }
        }
    }
}
