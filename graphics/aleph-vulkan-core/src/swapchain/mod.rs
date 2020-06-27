//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::Device;
use erupt::extensions::khr_surface::{
    ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceFormatKHR, SurfaceKHR,
};
use erupt::extensions::khr_swapchain::{
    KhrSwapchainDeviceLoaderExt, PresentInfoKHRBuilder, SwapchainCreateInfoKHRBuilder, SwapchainKHR,
};
use erupt::vk1_0::{
    ComponentMappingBuilder, ComponentSwizzle, Extent2D, Fence, Format, ImageAspectFlags,
    ImageSubresourceRangeBuilder, ImageUsageFlags, ImageViewCreateInfoBuilder, ImageViewType,
    Queue, Semaphore, SharingMode, Vk10DeviceLoaderExt,
};
use std::sync::Arc;
use std::time::Duration;

mod acquire_error;
mod rebuild_error;
mod support;
mod swap_image;

pub use acquire_error::AcquireError;
pub use rebuild_error::RebuildError;
pub use support::SwapChainSupport;
pub use swap_image::SwapImage;

///
/// Assign a score of "how much we want" this particular presentation mode if we dont care about
/// screen tearing
///
fn score_present_mode_tearing(mode: PresentModeKHR) -> u32 {
    match mode {
        PresentModeKHR::IMMEDIATE_KHR => 4,
        PresentModeKHR::MAILBOX_KHR => 3,
        PresentModeKHR::FIFO_RELAXED_KHR => 2,
        PresentModeKHR::FIFO_KHR => 1,
        _ => 0,
    }
}

///
/// Assign a score of "how much we want" this particular presentation mode if we do care about
/// screen tearing
///
fn score_present_mode_vsync(mode: PresentModeKHR) -> u32 {
    match mode {
        PresentModeKHR::MAILBOX_KHR => 4,
        PresentModeKHR::FIFO_KHR => 3,
        PresentModeKHR::FIFO_RELAXED_KHR => 2,
        PresentModeKHR::IMMEDIATE_KHR => 1,
        _ => 0,
    }
}

///
/// Given the list of surface formats select the one we want
///
fn select_surface_format(formats: &[SurfaceFormatKHR]) -> SurfaceFormatKHR {
    for format in formats.iter() {
        if format.format == Format::B8G8R8A8_UNORM
            && format.color_space == ColorSpaceKHR::SRGB_NONLINEAR_KHR
        {
            return *format;
        }
    }
    formats[0]
}

///
/// A builder wrapper for construction of a vulkan swapchain from a given surface
///
pub struct SwapchainBuilder {
    target_image_count: u32,
    target_present_mode: PresentModeKHR,
    favour_vsync: bool,
}

//==================================================================================================
impl SwapchainBuilder {
    ///
    /// Create the builder with a safe minimum default
    ///
    pub fn new() -> Self {
        SwapchainBuilder {
            target_image_count: 2,
            target_present_mode: PresentModeKHR::FIFO_KHR,
            favour_vsync: true,
        }
    }

    ///
    /// Safest, minimum supported swap-chain present mode and back buffer count
    ///
    /// FIFO with 2 back buffers
    ///
    pub fn compatibility(self) -> Self {
        self.fifo().double_buffer()
    }

    ///
    /// Fastest v-sync option.
    ///
    /// MAILBOX with 3 back buffers
    ///
    pub fn vsync(self) -> Self {
        self.mailbox().triple_buffer()
    }

    ///
    /// Will v-sync when above refresh rate but will allow tearing when below refresh rate
    ///
    /// FIFO_RELAXED with 3 back buffers
    ///
    pub fn fast_vsync(self) -> Self {
        self.fifo_relaxed().triple_buffer()
    }

    ///
    /// Fastest present mode and back buffer count. Will produce tearing
    ///
    /// IMMEDIATE with 2 back buffers
    ///
    pub fn fast(self) -> Self {
        self.immediate().double_buffer()
    }

    ///
    /// If the target present mode is not available then make the swapchain favour using one of the
    /// available present modes that provides the best vsync capabilities
    ///
    pub fn want_vsync(self) -> Self {
        self.vsync_affinity(true)
    }

    ///
    /// If the target present mode is not available then make the swapchain favour using a fast, low
    /// latency present mode like IMMEDIATE or MAILBOX
    ///
    pub fn want_fast(self) -> Self {
        self.vsync_affinity(false)
    }

    ///
    /// Set whether to favour vsync present modes or fast present modes
    ///
    pub fn vsync_affinity(mut self, favour_vsync: bool) -> Self {
        self.favour_vsync = favour_vsync;
        self
    }

    ///
    /// Short-form for setting back buffer count to 2
    ///
    pub fn double_buffer(self) -> Self {
        self.back_buffers(2)
    }

    ///
    /// Short-form for setting back buffer count to 3
    ///
    pub fn triple_buffer(self) -> Self {
        self.back_buffers(3)
    }

    ///
    /// Set the target number of back buffers
    ///
    pub fn back_buffers(mut self, count: u32) -> Self {
        self.target_image_count = count;
        self
    }

    ///
    /// Short-form for setting target presentation mode to FIFO
    ///
    pub fn fifo(self) -> Self {
        self.present_mode(PresentModeKHR::FIFO_KHR)
    }

    ///
    /// Short-form for setting target presentation mode to FIFO
    ///
    pub fn fifo_relaxed(self) -> Self {
        self.present_mode(PresentModeKHR::FIFO_RELAXED_KHR)
    }

    ///
    /// Short-form for setting target presentation mode to MAILBOX
    ///
    pub fn mailbox(self) -> Self {
        self.present_mode(PresentModeKHR::MAILBOX_KHR)
    }

    ///
    /// Short-form for setting target presentation mode to IMMEDIATE
    ///
    pub fn immediate(self) -> Self {
        self.present_mode(PresentModeKHR::IMMEDIATE_KHR)
    }

    ///
    /// Set the target presentation mode
    ///
    pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.target_present_mode = present_mode;
        self
    }

    ///
    /// Construct the swapchain
    ///
    pub fn build(self, device: &Arc<Device>, drawable_size: (u32, u32)) -> Swapchain {
        let mut swap = Swapchain {
            swapchain: SwapchainKHR::null(),
            surface: device.surface(),
            present_mode: self.target_present_mode,
            surface_format: Default::default(),
            extents: Default::default(),
            target_image_count: self.target_image_count,
            images: Vec::new(),
            device: device.clone(),
            favour_vsync: self.favour_vsync,
            requires_rebuild: true,
        };

        swap.rebuild(drawable_size)
            .expect("Failed to construct swapchain");

        swap
    }
}

pub struct Swapchain {
    swapchain: SwapchainKHR,
    surface: SurfaceKHR,
    present_mode: PresentModeKHR,
    surface_format: SurfaceFormatKHR,
    extents: Extent2D,
    target_image_count: u32,
    images: Vec<SwapImage>,
    device: Arc<Device>,
    favour_vsync: bool,
    requires_rebuild: bool,
}

impl Swapchain {
    ///
    /// Get a builder for constructing a swapchain. Just a wrapper for `SwapchainBuilder::new`
    ///
    pub fn builder() -> SwapchainBuilder {
        SwapchainBuilder::new()
    }

    ///
    /// Get the SurfaceKHR handle
    ///
    pub fn surface(&self) -> SurfaceKHR {
        self.surface
    }

    ///
    /// Get the SwapchainKHR handle
    ///
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }

    ///
    /// Get the current present mode
    ///
    pub fn present_mode(&self) -> PresentModeKHR {
        self.present_mode
    }

    ///
    /// Get the current surface format
    ///
    pub fn format(&self) -> SurfaceFormatKHR {
        self.surface_format
    }

    ///
    /// Get the current extents of the swapchain
    ///
    pub fn extents(&self) -> Extent2D {
        self.extents
    }

    ///
    /// Get the list of images the swapchain owns
    ///
    pub fn images(&self) -> &[SwapImage] {
        &self.images
    }

    ///
    /// If the swapchain wants to be rebuilt
    ///
    pub fn requires_rebuild(&self) -> bool {
        self.requires_rebuild
    }

    ///
    /// This functions as a wrapper around the vkAcquireNextImage
    ///
    pub fn acquire_next(
        &mut self,
        timeout: Duration,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result<usize, AcquireError> {
        // If we require a rebuild just exit out immediately, there's no point even trying to do
        // anything more
        if self.requires_rebuild {
            return Err(AcquireError::OutOfDate);
        }

        // Convert the duration to the units required by vkAcquireNextImage
        let timeout = timeout.as_nanos() as u64;

        let result = unsafe {
            self.device.loader().acquire_next_image_khr(
                self.swapchain,
                timeout,
                semaphore,
                fence,
                None,
            )
        };

        // Type alias for brevity
        type VkResult = erupt::vk1_0::Result;

        if result.raw == VkResult::SUCCESS {
            // Success, so return the index + image
            let index = result.value.unwrap();
            Ok(index as usize)
        } else if result.raw == VkResult::ERROR_OUT_OF_DATE_KHR {
            // Swapchain out of date, mark as needing a rebuild and error out
            self.requires_rebuild = true;
            Err(AcquireError::OutOfDate)
        } else if result.raw == VkResult::SUBOPTIMAL_KHR {
            // Suboptimal will work but we should probably rebuild anyway so we queue up a rebuild
            // for next frame but just keep going with this one
            self.requires_rebuild = true;
            let index = result.value.unwrap();
            Ok(index as usize)
        } else if result.raw == VkResult::TIMEOUT {
            // We hit the timeout specified and so can't yield a value, inform the caller about the
            // timeout with the error code
            Err(AcquireError::Timeout)
        } else {
            // Any other result is probably unrecoverable in a reasonable fashion so just panic and
            // be done with it
            panic!(
                "Unhandled error when acquiring next swapchain image: {:?}",
                result.raw
            );
        }
    }

    ///
    /// this function is a wrapper around vkQueuePresent
    ///
    pub fn present(&mut self, queue: Queue, index: usize, wait_semaphores: &[Semaphore]) {
        let swapchains = [self.swapchain];
        let image_indices = [index as u32];

        let present_info = PresentInfoKHRBuilder::new()
            .swapchains(&swapchains)
            .wait_semaphores(wait_semaphores)
            .image_indices(&image_indices);

        let result = unsafe { self.device.loader().queue_present_khr(queue, &present_info) };

        // Type alias for brevity
        type VkResult = erupt::vk1_0::Result;

        if result.raw == VkResult::SUCCESS {
            return;
        } else if result.raw == VkResult::SUBOPTIMAL_KHR
            || result.raw == VkResult::ERROR_OUT_OF_DATE_KHR
        {
            self.requires_rebuild = true;
        } else {
            // Any other result is probably unrecoverable in a reasonable fashion so just panic and
            // be done with it
            panic!(
                "Unhandled error when presenting swapchain image: {:?}",
                result.raw
            );
        }
    }

    ///
    /// Rebuild the swapchain. We need to do this if the swapchain is no longer up to date, such as
    /// if the window has resized or minimized. We need to recreate the swapchain to update the
    /// resources and get the new set of images for the swapchain
    ///
    pub fn rebuild(&mut self, drawable_size: (u32, u32)) -> Result<(), RebuildError> {
        aleph_log::trace!("Attempting swapchain rebuild");
        unsafe {
            // We're rebuilding the swapchain, I don't care about performance here this is going to
            // be slow no matter what I do and the most sane way to wait for all GPU work to be done
            // before destroying the old one is to put a big fat vkDeviceWaitIdle here
            self.device
                .loader()
                .device_wait_idle()
                .expect("Failed to wait for device to become idle");
        }
        aleph_log::trace!("Successfully waited for device to be idle");

        let support = self.device.swapchain_support();
        let capabilities = &support.capabilities;

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.current_extent.width == 0 || capabilities.current_extent.height == 0 {
            aleph_log::error!("Swapchain current extent 0 when rebuilding swapchain");
            return Err(RebuildError::ExtentsZero);
        }

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.max_image_extent.width == 0 || capabilities.max_image_extent.width == 0 {
            aleph_log::error!("Swapchain max extent 0 when rebuilding swapchain");
            return Err(RebuildError::ExtentsZero);
        }

        aleph_log::trace!("=== NEW SWAPCHAIN INFO ===");
        // We need need to decide on the extents of the swapchain to create. If one is provided to
        // us by the swapchain capabilities retrieved from vulkan then we use that. If not we have
        // to pick our own so we query the window state and use that
        //
        // We can detect when we have to pick our own swapchain extent when `current_extent` is
        // equal to (0xFFFFFFFF, 0xFFFFFFFF). This is a special case in the vulkan API.
        let extents = if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            // Get the latest info we have on the window's drawable surface size
            let (w, h) = drawable_size;

            // Convert it to an extent
            let mut actual = Extent2D::default();
            actual.width = w;
            actual.height = h;

            // Now we need to clamp it to fit inside the limits imposed by the surface capabilities
            // we queried earlier. We just clamp to be >= than the minimum and <= than the maximum
            let width = capabilities.max_image_extent.width.min(actual.width);
            let height = capabilities.max_image_extent.height.min(actual.height);

            actual.width = capabilities.min_image_extent.width.max(width);
            actual.height = capabilities.min_image_extent.height.max(height);

            actual
        };
        aleph_log::trace!("Extents       : ({}, {})", extents.width, extents.height);

        // We need to pick a presentation mode from one of the supported presentation modes.
        let present_mode = {
            let modes = &support.present_modes;

            let mut best_score = 0u32;
            let mut best_mode = PresentModeKHR::default();
            for mode in modes.iter() {
                let score = if self.favour_vsync {
                    score_present_mode_vsync(*mode)
                } else {
                    score_present_mode_tearing(*mode)
                };

                if *mode == self.present_mode {
                    best_score = score;
                    best_mode = self.present_mode;
                    break;
                } else if score > best_score {
                    best_score = score;
                    best_mode = *mode;
                }
            }

            if best_score == 0 {
                panic!("Failed to find a usable presentation mode");
            }

            best_mode
        };
        aleph_log::trace!("Present Mode  : {:#?}", present_mode);

        let formats = &support.formats;
        let surface_format = select_surface_format(&formats);
        aleph_log::trace!("Format        : {:?}", surface_format);

        let image_count = {
            if (self.target_image_count) < capabilities.min_image_count {
                capabilities.min_image_count
            } else {
                self.target_image_count
            }
        };
        aleph_log::trace!("Image Count   : {}", image_count);

        let old_swapchain = self.swapchain;
        let swap_create_info = SwapchainCreateInfoKHRBuilder::new()
            .surface(self.surface)
            .min_image_count(image_count)
            .present_mode(present_mode)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extents)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .clipped(true)
            .old_swapchain(old_swapchain);

        aleph_log::trace!("Creating new swapchain");
        let swapchain = unsafe {
            self.device
                .loader()
                .create_swapchain_khr(&swap_create_info, None, None)
                .expect("Failed to create swapchain")
        };

        //
        // Create get the image handles and create corresponding image views
        //
        let images = unsafe {
            self.device
                .loader()
                .get_swapchain_images_khr(swapchain, None)
                .expect("Failed to retrieve swapchain images")
                .drain(..)
                .map(|image| {
                    let components = ComponentMappingBuilder::new()
                        .r(ComponentSwizzle::R)
                        .g(ComponentSwizzle::G)
                        .b(ComponentSwizzle::B)
                        .a(ComponentSwizzle::A);
                    let subresource_range = ImageSubresourceRangeBuilder::new()
                        .level_count(1)
                        .layer_count(1)
                        .base_mip_level(0)
                        .base_array_layer(0)
                        .aspect_mask(ImageAspectFlags::COLOR);
                    let create_info = ImageViewCreateInfoBuilder::new()
                        .format(surface_format.format)
                        .image(image)
                        .subresource_range(*subresource_range)
                        .components(*components)
                        .view_type(ImageViewType::_2D);
                    let image_view = self
                        .device
                        .loader()
                        .create_image_view(&create_info, None, None)
                        .expect("Failed to create ImageView for swapchain");
                    SwapImage::internal_create(
                        image,
                        image_view,
                        surface_format.format,
                        extents.width,
                        extents.height,
                    )
                })
                .collect()
        };

        if old_swapchain != SwapchainKHR::null() {
            unsafe {
                aleph_log::trace!("Destroying old swapchain ImageViews");
                self.images.iter().for_each(|i| {
                    self.device
                        .loader()
                        .destroy_image_view(i.image_view(), None);
                });

                aleph_log::trace!("Destroying old swapchain");
                self.device
                    .loader()
                    .destroy_swapchain_khr(old_swapchain, None);
            }
        }

        self.surface_format = surface_format;
        self.extents = extents;
        self.present_mode = present_mode;
        self.images = images;
        self.swapchain = swapchain;
        self.requires_rebuild = false;

        Ok(())
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            aleph_log::trace!("Destroying swapchain ImageViews");
            self.images.iter().for_each(|i| {
                self.device
                    .loader()
                    .destroy_image_view(i.image_view(), None);
            });

            aleph_log::trace!("Destroying swapchain");
            self.device
                .loader()
                .destroy_swapchain_khr(self.swapchain, None);
        }
    }
}
