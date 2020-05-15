//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::Device;
use erupt::extensions::khr_surface::{
    ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceFormatKHR, SurfaceKHR,
};
use erupt::extensions::khr_swapchain::{
    KhrSwapchainDeviceLoaderExt, SwapchainCreateInfoKHRBuilder, SwapchainKHR,
};
use erupt::vk1_0::{Extent2D, Format, Image, ImageUsageFlags, SharingMode, Vk10DeviceLoaderExt};
use std::sync::Arc;

mod rebuild_error;
mod support;

use crate::app::Window;
pub use rebuild_error::RebuildError;
pub use support::SwapChainSupport;

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
    pub fn build(self, device: &Arc<Device>) -> Swapchain {
        let mut swap = Swapchain {
            swapchain: SwapchainKHR::null(),
            surface: device.surface(),
            present_mode: self.target_present_mode,
            surface_format: Default::default(),
            extents: Default::default(),
            images: Vec::new(),
            device: device.clone(),
            favour_vsync: self.favour_vsync,
            requires_rebuild: true,
        };

        let new_len = self.target_image_count as usize;
        swap.images.resize(new_len, Image::default());
        swap.rebuild().expect("Failed to construct swapchain");

        swap
    }
}

pub struct Swapchain {
    swapchain: SwapchainKHR,
    surface: SurfaceKHR,
    present_mode: PresentModeKHR,
    surface_format: SurfaceFormatKHR,
    extents: Extent2D,
    images: Vec<Image>,
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
    ///
    ///
    pub fn rebuild(&mut self) -> Result<(), RebuildError> {
        log::trace!("Attempting swapchain rebuild");
        unsafe {
            // We're rebuilding the swapchain, I don't care about performance here this is going to
            // be slow no matter what I do and the most sane way to wait for all GPU work to be done
            // before destroying the old one is to put a big fat vkDeviceWaitIdle here
            self.device
                .loader()
                .device_wait_idle()
                .expect("Failed to wait for device to become idle");
        }
        log::trace!("Successfully waited for device to be idle");

        let support = self.device.swapchain_support();
        let capabilities = &support.capabilities;

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.current_extent.width == 0 || capabilities.current_extent.height == 0 {
            log::warn!("Swapchain current extent 0 when rebuilding swapchain");
            return Err(RebuildError::ExtentsZero);
        }

        // If any of these are zero than the window is minimized. We can't create a swap chain for
        // a minimized window as the extents are invalid so we error out and let the user handle it.
        if capabilities.max_image_extent.width == 0 || capabilities.max_image_extent.width == 0 {
            log::warn!("Swapchain max extent 0 when rebuilding swapchain");
            return Err(RebuildError::ExtentsZero);
        }

        log::trace!("=== NEW SWAPCHAIN INFO ===");
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
            let (w, h) = Window::drawable_size();

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
        log::trace!("Extents       : ({}, {})", extents.width, extents.height);

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
        log::trace!("Present Mode  : {:#?}", present_mode);

        let formats = &support.formats;
        let surface_format = select_surface_format(&formats);
        log::trace!("Format        : {:?}", surface_format);

        let image_count = {
            if (self.images.len() as u32) < capabilities.min_image_count {
                capabilities.min_image_count
            } else {
                self.images.len() as u32
            }
        };
        log::trace!("Image Count   : {}", image_count);
        log::trace!("");

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

        log::trace!("Creating new swapchain");
        let swapchain = unsafe {
            self.device
                .loader()
                .create_swapchain_khr(&swap_create_info, None, None)
                .expect("Failed to create swapchain")
        };

        let images = unsafe {
            self.device
                .loader()
                .get_swapchain_images_khr(swapchain, None)
                .expect("Failed to retrieve swapchain images")
        };

        self.surface_format = surface_format;
        self.extents = extents;
        self.present_mode = present_mode;
        self.images = images;
        self.swapchain = swapchain;
        self.requires_rebuild = false;

        let null_handle = SwapchainKHR::null();
        if old_swapchain != null_handle {
            unsafe {
                log::trace!("Destroying old swapchain");
                self.device
                    .loader()
                    .destroy_swapchain_khr(old_swapchain, None);
            }
        }

        Ok(())
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            log::trace!("Destroying swapchain");
            self.device
                .loader()
                .destroy_swapchain_khr(self.swapchain, None);
        }
    }
}
