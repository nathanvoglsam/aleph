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
use crate::internal::conv::{present_mode_to_vk, texture_format_to_vk};
use crate::internal::queue_present_support::QueuePresentSupportFlags;
use crate::surface::Surface;
use crate::swap_texture::SwapTexture;
use erupt::vk;
use erupt::vk1_0::{Extent2D, Format};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    AcquireImageError, IAcquiredTexture, IDevice, INamedObject, ISwapChain, QueueType,
    ResourceStates, SwapChainConfiguration, SwapChainCreateError, TextureDesc, TextureDimension,
    TextureFormat,
};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject};
use std::sync::Mutex;

// TODO: Track out of date status with a flag and trigger a transparent rebuild.
// TODO: We only need to handle rebuilds on image acquire. Eat the cost of a rebuild transparently
//       by rebuilding within the acquire function. This won't happen often.
// TODO: Figure out what host safe behavior will need. How long do we need to extend the lifetime
//       of the old swap chain before we can safely destroy it. Likely need to do a full queue flush
//       to ensure the images aren't in use.
// TODO: Maybe API could enforce that the user can only hold a reference to a single swap image at
//       any one time so we can more reliably reason about when the swap chain is safe to destroy.
//       This is applicable to D3D12 as well which requires all references to all images to be
//       dropped before rebuilding.
// TODO: If the wanted extents aren't supported then we could fake the support with a
//       custom texture and blit operation, but requires presentation images to support
//       TRANSFER_DST

ref_ptr_object! {
    pub struct SwapChain {
        pub(crate) inner: Mutex<SwapChainState>,
        pub(crate) queue_support: QueuePresentSupportFlags,
        pub(crate) device: RefPtr<Device>,
        pub(crate) surface: RefPtr<Surface>,
    }
}

pub struct SwapChainState {
    pub swap_chain: vk::SwapchainKHR,
    pub acquire_fence: vk::Fence,
    pub images_in_flight: u32,
    pub format: TextureFormat,
    pub vk_format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub present_mode: vk::PresentModeKHR,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub queued_resize: Option<(u32, u32)>,
}

impl SwapChain {
    pub(crate) unsafe fn build(
        &self,
        inner: &mut SwapChainState,
        config: &SwapChainConfiguration,
    ) -> Result<(), SwapChainCreateError> {
        let (capabilities, formats, present_modes) = Context::get_device_surface_support(
            &self.device.context.instance_loader,
            self.device.adapter.physical_device,
            self.surface.surface,
        )
        .map_err(|e| anyhow!(e))?;

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.current_extent.width == 0
            || capabilities.current_extent.height == 0
            || capabilities.max_image_extent.width == 0
            || capabilities.max_image_extent.height == 0
        {
            return Err(SwapChainCreateError::SurfaceNotAvailable);
        }

        let extents = Self::select_extents(config, &capabilities)?;
        let present_mode = Self::select_presentation_mode(config, &present_modes);
        let (format, color_space) = Self::select_format_and_color_space(config, &formats)?;
        let buffer_count = Self::select_buffer_count(present_mode, &capabilities);

        let image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC;
        if !capabilities.supported_usage_flags.contains(image_usage) {
            // TODO: Make this return an error
            panic!("swap chain doesn't support all required usage flags");
        }

        let swap_create_info = vk::SwapchainCreateInfoKHRBuilder::new()
            .surface(self.surface.surface)
            .min_image_count(buffer_count)
            .present_mode(present_mode)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(extents)
            .image_array_layers(1)
            .image_usage(image_usage)
            .pre_transform(vk::SurfaceTransformFlagBitsKHR::IDENTITY_KHR)
            .composite_alpha(vk::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .old_swapchain(inner.swap_chain)
            .clipped(true);

        // TODO: Handle destroying the old swap chain

        inner.swap_chain = self
            .device
            .device_loader
            .create_swapchain_khr(&swap_create_info, None)
            .result()
            .map_err(|e| anyhow!(e))?;

        let images = self
            .device
            .device_loader
            .get_swapchain_images_khr(inner.swap_chain, None)
            .result()
            .map_err(|e| anyhow!(e))?;

        inner.extent = extents;
        inner.format = config.format;
        inner.vk_format = format;
        inner.color_space = color_space;
        inner.present_mode = present_mode;
        inner.images = images.into_vec();
        inner.queued_resize = None;

        Ok(())
    }

    unsafe fn select_buffer_count(
        present_mode: vk::PresentModeKHR,
        capabilities: &vk::SurfaceCapabilitiesKHR,
    ) -> u32 {
        let buffer_count = match present_mode {
            vk::PresentModeKHR::IMMEDIATE_KHR => 2,
            vk::PresentModeKHR::MAILBOX_KHR => 3,
            vk::PresentModeKHR::FIFO_KHR | vk::PresentModeKHR::FIFO_RELAXED_KHR => 2,
            _ => unreachable!(),
        };
        buffer_count
            .min(capabilities.max_image_count)
            .max(capabilities.min_image_count)
    }

    fn select_format_and_color_space(
        config: &SwapChainConfiguration,
        formats: &[vk::SurfaceFormatKHR],
    ) -> Result<(Format, vk::ColorSpaceKHR), SwapChainCreateError> {
        // Translate our format into the vulkan format
        let format = texture_format_to_vk(config.format);

        // Filter all entries for the format we are trying to use
        formats
            .iter()
            .copied()
            .find_map(|v| {
                if v.format == format && v.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR_KHR {
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
    ) -> Result<Extent2D, SwapChainCreateError> {
        // We can either use the wanted size or the size demanded by vulkan
        let extents = match (
            capabilities.current_extent.width,
            capabilities.current_extent.height,
        ) {
            // Size we want is allowed in this case
            (u32::MAX, u32::MAX) => vk::Extent2D {
                width: config.width,
                height: config.height,
            },
            // Otherwise only the exact size here is allowed
            (width, height) => vk::Extent2D { width, height },
        };

        // Check if we're outside the valid width range
        if extents.width > capabilities.max_image_extent.width
            || extents.width < capabilities.max_image_extent.height
        {
            return Err(SwapChainCreateError::UnsupportedWidth(extents.width));
        }

        // Check if we're outside the valid height range
        if extents.height > capabilities.min_image_extent.width
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
            wanted_mode
        } else {
            vk::PresentModeKHR::FIFO_KHR
        }
    }
}

impl ISwapChain for SwapChain {
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

    fn queue_resize(&self, width: u32, height: u32) {
        let mut inner = self.inner.lock().unwrap();
        inner.queued_resize = Some((width, height));
    }

    fn get_config(&self) -> SwapChainConfiguration {
        todo!()
    }

    fn acquire_image(&self) -> Result<Box<dyn IAcquiredTexture>, AcquireImageError> {
        let mut inner = self.inner.lock().unwrap();

        if let Some((width, height)) = inner.queued_resize.take() {
            unsafe {
                // TODO: Need to investigate how to correctly synchronize this. It should only
                //       require handling when the old swap chain is destroyed as oldSwapChain is
                //       specifically designed to allow already in flight frames to finish
                self.device.wait_idle();
                self.device.garbage_collect();

                let width = if width == u32::MAX {
                    inner.extent.width
                } else {
                    width
                };
                let height = if height == u32::MAX {
                    inner.extent.height
                } else {
                    height
                };
                let config = SwapChainConfiguration {
                    format: inner.format,
                    width,
                    height,
                    present_mode: todo!(),
                    preferred_queue: todo!(),
                };
                self.build(&mut inner, &config)
                    .map_err(|_| AcquireImageError::SurfaceNotAvailable)?;
            }
        }

        unsafe {
            let result = self.device.device_loader.acquire_next_image_khr(
                inner.swap_chain,
                u64::MAX,
                None,
                Some(inner.acquire_fence),
            );
            match result.raw {
                vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => {
                    let value = result.value.unwrap();

                    let image = inner.images[value as usize];

                    if result.raw == vk::Result::SUBOPTIMAL_KHR {
                        inner.queued_resize = Some((u32::MAX, u32::MAX));
                    }

                    // As an initial solution we'll just use a fence to ensure the image is ready
                    // to use.
                    // TODO: Profile to see if it's worth the effort being smarter about this. Is
                    //       blocking the CPU here that big of a deal?
                    self.device
                        .device_loader
                        .wait_for_fences(&[inner.acquire_fence], true, u64::MAX)
                        .map_err(|e| {
                            anyhow!("Failed to wait on acquire fence with code '{}'", e)
                        })?;
                    self.device
                        .device_loader
                        .reset_fences(&[inner.acquire_fence])
                        .map_err(|e| anyhow!("Failed to reset acquire fence with code '{}'", e))?;

                    let image = ref_ptr_init! {
                        SwapTexture {
                            image: image,
                            desc: TextureDesc {
                                width: inner.extent.width,
                                height: inner.extent.height,
                                depth: 1,
                                format: inner.format,
                                dimension: TextureDimension::Texture2D,
                                initial_state: ResourceStates::UNKNOWN,
                                clear_value: None,
                                array_size: 1,
                                mip_levels: 1,
                                sample_count: 1,
                                sample_quality: 0,
                                allow_unordered_access: false,
                                allow_cube_face: false,
                                is_render_target: true
                            },
                            vk_format: inner.vk_format,
                            swap_chain: self.as_ref_ptr(),
                        }
                    };
                    let image: RefPtr<SwapTexture> = RefPtr::new(image);
                    todo!()
                    // Ok(image.query_interface::<dyn ITexture>().unwrap())
                }
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    inner.queued_resize = Some((u32::MAX, u32::MAX));
                    todo!()
                }
                _ => Err(AcquireImageError::Platform(anyhow!(
                    "Failed to acquire swap chain image with error '{}'",
                    result
                ))),
            }
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        let inner = self.inner.lock().unwrap();
        unsafe {
            self.device
                .device_loader
                .destroy_swapchain_khr(Some(inner.swap_chain), None);
        }
    }
}

pub trait ISwapChainExt: ISwapChain {}

impl ISwapChainExt for SwapChain {}

impl INamedObject for SwapChain {
    fn set_name(&self, _name: &str) {
        todo!()
    }
}
