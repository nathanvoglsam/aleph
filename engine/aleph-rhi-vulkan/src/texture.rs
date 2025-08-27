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

use std::collections::HashMap;
use std::num::NonZeroU64;

use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::owned_desc::OwnedTextureDesc;
use ash::vk;
use parking_lot::Mutex;
use vulkan_alloc::vma;

use crate::device::Device;
use crate::internal::conv::{image_view_type_to_vk, subresource_range_to_vk, texture_format_to_vk};

pub struct Texture {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) image: vk::Image,
    // pub(crate) creation_flags: vk::ImageCreateFlags,
    // pub(crate) created_usage: vk::ImageUsageFlags,
    pub(crate) allocation: Option<vma::Allocation>,
    pub(crate) is_owned: bool,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) desc: OwnedTextureDesc,
}

unsafe_impl_iobject!(Texture, "01944ef4-bac0-7310-aec2-60edef2587bb");

impl Texture {
    pub(crate) fn get(v: &TextureHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Texture implementation!")
    }

    pub(crate) fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &TextureDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn get_view(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.views.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let usage = if desc.writable {
                vk::ImageUsageFlags::STORAGE
            } else {
                vk::ImageUsageFlags::SAMPLED
            };

            let view = self.create_view_for_usage(device, desc, usage)?;

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    pub(crate) fn get_rtv(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_rtv_or_dsv(device, desc, vk::ImageUsageFlags::COLOR_ATTACHMENT)
    }

    pub(crate) fn get_dsv(&self, device: &Device, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_rtv_or_dsv(device, desc, vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
    }
}

impl Texture {
    fn get_rtv_or_dsv(
        &self,
        device: &Device,
        desc: &ImageViewDesc,
        usage: vk::ImageUsageFlags,
    ) -> Result<ImageView, ()> {
        let mut views = self.dsvs.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let view = self.create_view_for_usage(device, desc, usage)?;

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn create_view_for_usage(
        &self,
        device: &Device,
        desc: &ImageViewDesc,
        usage: vk::ImageUsageFlags,
    ) -> Result<vk::ImageView, ()> {
        let mut usage_info = vk::ImageViewUsageCreateInfo::default().usage(usage);

        let create_info = vk::ImageViewCreateInfo::default()
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
            .push_next(&mut usage_info);

        let view = unsafe {
            device
                .device
                .create_image_view(&create_info, None)
                .map_err(|_| ())? // TODO: error handling
        };

        Ok(view)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            for (_desc, view) in self.views.get_mut().drain() {
                self._device.device.destroy_image_view(view, None);
            }

            for (_desc, view) in self.rtvs.get_mut().drain() {
                self._device.device.destroy_image_view(view, None);
            }

            for (_desc, view) in self.dsvs.get_mut().drain() {
                self._device.device.destroy_image_view(view, None);
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
