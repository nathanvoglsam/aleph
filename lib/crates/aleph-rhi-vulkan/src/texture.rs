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
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::vk;
use parking_lot::Mutex;
use std::any::TypeId;
use std::collections::HashMap;
use vulkan_alloc::vma;

pub struct Texture {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) image: vk::Image,
    pub(crate) creation_flags: vk::ImageCreateFlags,
    pub(crate) created_usage: vk::ImageUsageFlags,
    pub(crate) allocation: Option<vma::Allocation>,
    pub(crate) is_owned: bool,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, vk::ImageView>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, Box<RenderTargetView>>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, Box<RenderTargetView>>>,
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

            let view = self.create_view_for_usage(desc, usage)?;

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_rtv_or_dsv(desc, vk::ImageUsageFlags::COLOR_ATTACHMENT)
    }

    fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_rtv_or_dsv(desc, vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
    }
}

impl Texture {
    fn get_rtv_or_dsv(
        &self,
        desc: &ImageViewDesc,
        usage: vk::ImageUsageFlags,
    ) -> Result<ImageView, ()> {
        let mut views = self.dsvs.lock();

        let view = if let Some(view) = views.get(desc) {
            view.as_ref() as *const RenderTargetView
        } else {
            let view = self.create_view_for_usage(desc, usage)?;

            // Wrap into our RTV wrapper object
            let view = Box::new(RenderTargetView {
                _texture: self._this.clone(),
                image_view: view,
                format: texture_format_to_vk(desc.format),
                creation_flags: self.creation_flags,
                usage,
            });
            let view_ptr = view.as_ref() as *const RenderTargetView;

            views.insert(desc.clone(), view);

            view_ptr
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn create_view_for_usage(
        &self,
        desc: &ImageViewDesc,
        usage: vk::ImageUsageFlags,
    ) -> Result<vk::ImageView, ()> {
        let mut usage_info = vk::ImageViewUsageCreateInfo::builder().usage(usage);

        let create_info = vk::ImageViewCreateInfo::builder()
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
            self._device
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
                self._device
                    .device
                    .destroy_image_view(view.image_view, None);
            }

            for (_desc, view) in self.dsvs.get_mut().drain() {
                self._device
                    .device
                    .destroy_image_view(view.image_view, None);
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

/// Unfortunately, to sometimes need the format of an RTV. We create a proxy object so we can fetch
/// it in our implementation.
pub struct RenderTargetView {
    pub _texture: AnyWeak<Texture>,
    pub image_view: vk::ImageView,
    pub format: vk::Format,
    pub creation_flags: vk::ImageCreateFlags,
    pub usage: vk::ImageUsageFlags,
}

impl RenderTargetView {
    pub unsafe fn from_view(v: ImageView) -> *const RenderTargetView {
        std::mem::transmute(v)
    }
}
