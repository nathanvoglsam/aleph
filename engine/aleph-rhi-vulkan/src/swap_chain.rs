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

use std::any::TypeId;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use aleph_alloc::BVec;
use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_nstr::{NStr, nstr};
use aleph_object_system::{ArcObject, Object};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::owned_desc::OwnedTextureDesc;
use ash::vk::{self, Handle};
use parking_lot::Mutex;

use crate::context::Context;
use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::conv::{present_mode_to_vk, texture_format_to_vk};
use crate::internal::queue_present_support::QueuePresentSupportFlags;
use crate::internal::semaphore_pool::SemaphorePool;
use crate::internal::set_name::set_name_nstr;
use crate::surface::Surface;
use crate::swap_image::SwapImage;
use crate::texture::Texture;

pub struct SwapChain {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) surface: AnyArc<Surface>,
    pub(crate) inner: Mutex<SwapChainState>,
    pub(crate) queue_support: QueuePresentSupportFlags,
}

declare_interfaces!(SwapChain, [ISwapChain]);

impl IGetPlatformInterface for SwapChain {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: We can probably expose a few objects from a swapchain, but they're behind a mutex
        //       so we'll wait before implementing this
        None
    }
}

impl ISwapChain for SwapChain {
    fn upgrade(&self) -> AnyArc<dyn ISwapChain> {
        AnyArc::map::<dyn ISwapChain, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn present_supported_on_queue(&self, queue: QueueType) -> bool {
        match queue {
            QueueType::General => self
                .queue_support
                .contains(QueuePresentSupportFlags::GENERAL),
            QueueType::Compute => self
                .queue_support
                .contains(QueuePresentSupportFlags::COMPUTE),
            QueueType::Transfer => self
                .queue_support
                .contains(QueuePresentSupportFlags::TRANSFER),
        }
    }

    fn get_config(&self) -> SwapChainConfiguration {
        let inner = self.inner.lock();
        inner.get_config(self.queue_support)
    }

    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError> {
        // Lock the swap chain immediately, it prevents acquiring any more images
        let mut inner = self.inner.lock();

        // Trigger a wait idle to flush the GPU of work. Once this returns no work can be in flight
        // on any swap chain image.
        self.device.wait_idle();

        // Grab a snapshot of the current 'SwapChainConfiguration' that we use as the base for
        // recreating the vulkan swap chain. Vulkan may change support for present modes or
        // resolutions depending on whether the window is fullscreen exclusive or windowed.
        let mut old_config = inner.get_config(self.queue_support);
        if let Some(new_size) = new_size {
            // Override the width/height if the user requests a specific extent. This is just a
            // suggestion and may be ignored (almost certainly will on Vulkan Windows)
            old_config.width = new_size.width;
            old_config.height = new_size.height;
        }

        unsafe {
            self.build(&mut inner, &old_config).unwrap();
        }

        // Return the config after 'build' which represents the actual state of the swap chain. The
        // build function takes the given config as more of a suggestion as Vulkan's support matrix
        // for swap chain stuff is stupidly complex and largely has safe fallbacks.
        //
        // When the exact config requested can't be matched the implementation will fall back to
        // values that will work.
        Ok(inner.get_config(self.queue_support))
    }

    unsafe fn acquire_next_image(&self) -> Result<AcquiredImage, ImageAcquireError> {
        let ready_semaphore = unsafe { self.device.swap_semaphore_pool.get(&self.device.device) };

        let loader = self.device.swapchain.as_ref().unwrap();

        let inner = self.inner.lock();
        let result = unsafe {
            loader.acquire_next_image(
                inner.swap_chain,
                u64::MAX,
                ready_semaphore,
                vk::Fence::null(),
            )
        };

        match result {
            Ok((i, sub_optimal)) => {
                let texture = inner.images[i as usize].clone();
                let texture = ArcObject::from_object(texture);
                let texture = unsafe { TextureHandle::new(texture) };

                let semaphore_pool = inner.semaphore_pools[i as usize].clone();

                let swap_image = AnyArc::new(SwapImage {
                    swap_chain: self.this.upgrade().unwrap(),
                    index: i,
                    texture,
                    ready_semaphore: AtomicU64::new(ready_semaphore.as_raw()),
                    work_semaphores: Mutex::new(BVec::new_in(Default::default())),
                    semaphore_pool,
                });
                let swap_image = AnyArc::map::<dyn ISwapImage, _>(swap_image, |v| v);
                if sub_optimal {
                    Ok(AcquiredImage::SubOptimal(swap_image))
                } else {
                    Ok(AcquiredImage::Ok(swap_image))
                }
            }
            Err(vk::Result::NOT_READY) => unimplemented!(),
            Err(vk::Result::TIMEOUT) => unimplemented!(),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Err(ImageAcquireError::OutOfDate),
            Err(vk::Result::ERROR_SURFACE_LOST_KHR) => Err(ImageAcquireError::SurfaceLost),
            Err(e) => {
                // Coerce everything we don't explicitly handle to an error.
                log::error!("Platform Error: {:#?}", e);
                Err(ImageAcquireError::Platform)
            }
        }
    }
}

impl SwapChain {
    pub(crate) unsafe fn build(
        &self,
        inner: &mut SwapChainState,
        config: &SwapChainConfiguration,
    ) -> Result<(), SwapChainCreateError> {
        let surface_loader = self.device.context.surface_loaders.base.as_ref().unwrap();
        let swapchain_loader = self.device.swapchain.as_ref().unwrap();
        let (capabilities, formats, present_modes) = Context::get_device_surface_support(
            surface_loader,
            self.device.adapter.physical_device,
            self.surface.surface,
        )
        .map_err(|e| log::error!("Platform Error: {:#?}", e))?;

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.current_extent.width == 0
            || capabilities.current_extent.height == 0
            || capabilities.max_image_extent.width == 0
            || capabilities.max_image_extent.height == 0
        {
            // TODO: this should not cause creation of ISwapChain to fail, this should be surfaced when trying to
            //       acquire images instead.
            return Err(SwapChainCreateError::SurfaceNotAvailable);
        }

        let extents = Self::select_extents(config, &capabilities)?;
        let present_mode = Self::select_presentation_mode(config, &present_modes);
        let (format, color_space) = Self::select_format_and_color_space(config, &formats)?;
        let buffer_count = Self::select_buffer_count(config.buffer_count, &capabilities);

        let image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
        if !capabilities.supported_usage_flags.contains(image_usage) {
            // TODO: Make this return an error
            panic!("swap chain doesn't support all required usage flags");
        }

        let old_swapchain = inner.swap_chain;

        // Select our set of view-compatible formats
        let format_list = config.format.compatible_view_formats();
        let format_list = Vec::from_iter(format_list.iter().copied().map(texture_format_to_vk));
        let mut format_flags = vk::SwapchainCreateFlagsKHR::empty();
        if format_list.len() > 1 {
            format_flags |= vk::SwapchainCreateFlagsKHR::MUTABLE_FORMAT
        }
        let mut format_list = vk::ImageFormatListCreateInfo::default().view_formats(&format_list);

        let swap_create_info = vk::SwapchainCreateInfoKHR::default()
            .flags(format_flags)
            .surface(self.surface.surface)
            .min_image_count(buffer_count)
            .present_mode(present_mode)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(extents)
            .image_array_layers(1)
            .image_usage(image_usage)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .old_swapchain(old_swapchain)
            .clipped(true);
        let swap_create_info = swap_create_info.push_next(&mut format_list);

        inner.swap_chain = unsafe {
            swapchain_loader
                .create_swapchain(&swap_create_info, GLOBAL)
                .map_err(|e| log::error!("Platform Error: {:#?}", e))?
        };

        if old_swapchain != vk::SwapchainKHR::null() {
            unsafe { swapchain_loader.destroy_swapchain(old_swapchain, GLOBAL) };
        }

        let images = unsafe {
            swapchain_loader
                .get_swapchain_images(inner.swap_chain)
                .map_err(|e| log::error!("Platform Error: {:#?}", e))?
        };

        const SWAP_NAMES: [&'static NStr; 8] = [
            nstr!(obj_name!("SwapImage-0")),
            nstr!(obj_name!("SwapImage-1")),
            nstr!(obj_name!("SwapImage-2")),
            nstr!(obj_name!("SwapImage-3")),
            nstr!(obj_name!("SwapImage-4")),
            nstr!(obj_name!("SwapImage-5")),
            nstr!(obj_name!("SwapImage-6")),
            nstr!(obj_name!("SwapImage-7")),
        ];
        let mut new_images = BVec::with_capacity_in(images.len(), Default::default());
        let iter = images.iter().enumerate().map(|(i, image)| {
            use ResourceUsageFlags as F;

            // Apply name to swap images when we query them
            set_name_nstr(
                self.device.debug_loader.as_ref(),
                *image,
                Some(SWAP_NAMES[i]),
            );

            // This shadows swap_create_info to a reference to itself so the new_cyclic move
            // closure moves the reference and not the object itself
            let swap_create_info = &swap_create_info;
            let desc = TextureDesc {
                width: swap_create_info.image_extent.width,
                height: swap_create_info.image_extent.height,
                depth: 1,
                format: config.format,
                dimension: TextureDimension::Texture2D,
                clear_value: None,
                array_size: 1,
                mip_levels: 1,
                sample_count: 1,
                sample_quality: 0,
                usage: F::COPY_DEST | F::RENDER_TARGET,
                name: Some("Vulkan Internal SwapChain Image"),
            };
            let out = Texture {
                _device: self.device.clone(),
                id: self.device.object_counter.next_texture(),
                image: *image,
                // creation_flags: create_info.flags,
                // created_usage: create_info.usage,
                allocation: None,
                is_owned: false,
                views: Default::default(),
                rtvs: Default::default(),
                dsvs: Default::default(),
                desc: OwnedTextureDesc::new(desc),
            };
            Object::new_arc(out)
        });
        new_images.extend(iter);

        for mut pool in inner.semaphore_pools.drain(..) {
            let pool = Arc::get_mut(&mut pool).unwrap();
            unsafe { pool.destroy(&self.device.device) };
        }

        let mut new_semaphore_pools = BVec::with_capacity_in(images.len(), Default::default());
        let iter = std::iter::repeat_n((), images.len()).map(|_| Arc::new(SemaphorePool::new()));
        new_semaphore_pools.extend(iter);

        inner.extent = extents;
        inner.format = config.format;
        inner.vk_format = format;
        inner.color_space = color_space;
        inner.vk_present_mode = present_mode;
        inner.images = new_images;
        inner.semaphore_pools = new_semaphore_pools;

        Ok(())
    }

    fn select_buffer_count(wanted: u32, capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
        let count = wanted
            .min(capabilities.max_image_count)
            .max(capabilities.min_image_count);
        log::info!("Wanted buffer_count = '{wanted}'. Got buffer_count = '{count}'.");
        count
    }

    fn select_format_and_color_space(
        config: &SwapChainConfiguration,
        formats: &[vk::SurfaceFormatKHR],
    ) -> Result<(vk::Format, vk::ColorSpaceKHR), SwapChainCreateError> {
        // Translate our format into the vulkan format
        let format = texture_format_to_vk(config.format);

        // Filter all entries for the format we are trying to use
        formats
            .iter()
            .copied()
            .find_map(|v| {
                if v.format == format && v.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                    Some((v.format, v.color_space))
                } else {
                    None
                }
            })
            .ok_or(SwapChainCreateError::UnsupportedFormat(config.format))
    }

    fn select_extents(
        config: &SwapChainConfiguration,
        capabilities: &vk::SurfaceCapabilitiesKHR,
    ) -> Result<vk::Extent2D, SwapChainCreateError> {
        // We can either use the wanted size or the size demanded by vulkan
        let extents = match (
            capabilities.current_extent.width,
            capabilities.current_extent.height,
        ) {
            // Size we want is allowed in this case
            (u32::MAX, u32::MAX) => {
                log::info!(
                    "Selecting exact width/height ({}, {})",
                    config.width,
                    config.height
                );
                vk::Extent2D {
                    width: config.width,
                    height: config.height,
                }
            }
            // Otherwise only the exact size here is allowed
            (width, height) => {
                log::info!(
                    "Requested width/height unsupported ({}, {}), using ({}, {})",
                    config.width,
                    config.height,
                    width,
                    height
                );
                vk::Extent2D { width, height }
            }
        };

        // Check if we're outside the valid width range
        if extents.width > capabilities.max_image_extent.width
            || extents.width < capabilities.min_image_extent.width
        {
            return Err(SwapChainCreateError::UnsupportedWidth(extents.width));
        }

        // Check if we're outside the valid height range
        if extents.height > capabilities.max_image_extent.height
            || extents.height < capabilities.min_image_extent.height
        {
            return Err(SwapChainCreateError::UnsupportedHeight(extents.height));
        }

        Ok(extents)
    }

    fn select_presentation_mode(
        config: &SwapChainConfiguration,
        present_modes: &[vk::PresentModeKHR],
    ) -> vk::PresentModeKHR {
        // We need to pick a presentation mode from one of the supported presentation modes.
        let wanted_mode = present_mode_to_vk(config.present_mode);

        // Give the user exactly the mode they want, or fallback to FIFO
        if present_modes.contains(&wanted_mode) {
            log::info!("Got wanted presentation mode {}", config.present_mode);
            wanted_mode
        } else {
            log::info!(
                "Wanted presentation mode unsupported '{}', falling back to FIFO. Options {:?}",
                config.present_mode,
                present_modes
            );
            vk::PresentModeKHR::FIFO
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        let inner = self.inner.get_mut();
        let loader = self.device.swapchain.as_ref().unwrap();
        unsafe {
            if inner.swap_chain != vk::SwapchainKHR::null() {
                loader.destroy_swapchain(inner.swap_chain, GLOBAL);
            }
        }
        unsafe {
            for mut pool in inner.semaphore_pools.drain(..) {
                let pool = Arc::get_mut(&mut pool).unwrap();
                pool.destroy(&self.device.device);
            }
        }
    }
}

pub struct SwapChainState {
    pub swap_chain: vk::SwapchainKHR,
    pub format: Format,
    pub vk_format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub present_mode: PresentationMode,
    pub vk_present_mode: vk::PresentModeKHR,
    pub extent: vk::Extent2D,
    pub images: BVec<Arc<Object<Texture>>, RhiSystem>,
    pub semaphore_pools: BVec<Arc<SemaphorePool>, RhiSystem>,
}

impl SwapChainState {
    pub fn get_config(&self, queue_support: QueuePresentSupportFlags) -> SwapChainConfiguration {
        let present_queue = if queue_support.contains(QueuePresentSupportFlags::GENERAL) {
            QueueType::General
        } else if queue_support.contains(QueuePresentSupportFlags::COMPUTE) {
            QueueType::Compute
        } else if queue_support.contains(QueuePresentSupportFlags::TRANSFER) {
            QueueType::Transfer
        } else {
            panic!("ISwapChain with no supported present queues (how?)")
        };

        SwapChainConfiguration {
            format: self.format,
            width: self.extent.width,
            height: self.extent.height,
            present_mode: self.present_mode,
            buffer_count: self.images.len() as u32,
            present_queue,
        }
    }
}
