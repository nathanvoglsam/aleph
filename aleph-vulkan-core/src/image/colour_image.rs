//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::Allocator;
use crate::gpu::vk::core::Device;
use crate::gpu::vk::image::{ImageSingle2D, ImageSingle2DBuilder};

use erupt::vk1_0::{Format, ImageAspectFlags, ImageUsageFlags};
use std::ops::Deref;

///
/// Builder for creating a ColourImage
///
pub struct ColourImageBuilder {
    inner: ImageSingle2DBuilder,
}

impl ColourImageBuilder {
    ///
    /// Creates a new builder object
    ///
    pub fn new() -> ColourImageBuilder {
        Self {
            inner: ImageSingle2DBuilder::new()
                .aspect(ImageAspectFlags::COLOR)
                .usage(ImageUsageFlags::COLOR_ATTACHMENT),
        }
    }

    ///
    /// Sets the width of the image
    ///
    pub fn width(mut self, width: u32) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    ///
    /// Sets the height of the image
    ///
    pub fn height(mut self, height: u32) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    ///
    /// Sets the format of the image. Panics if given a colour format
    ///
    pub fn format(mut self, format: Format) -> Self {
        // Reject colour formats
        match format {
            Format::D16_UNORM_S8_UINT
            | Format::D24_UNORM_S8_UINT
            | Format::D32_SFLOAT
            | Format::D32_SFLOAT_S8_UINT
            | Format::X8_D24_UNORM_PACK32
            | Format::D16_UNORM => panic!("Expected colour format"),
            _ => {}
        }

        self.inner = self.inner.format(format);
        self
    }

    ///
    /// Mark this image as being used as an input attachment
    ///
    pub fn usage_input_attachment(mut self) -> Self {
        self.inner = self.inner.usage(ImageUsageFlags::INPUT_ATTACHMENT);
        self
    }

    ///
    /// Mark this image as being used for sampling from
    ///
    pub fn usage_sampled(mut self) -> Self {
        self.inner = self.inner.usage(ImageUsageFlags::SAMPLED);
        self
    }

    ///
    /// Mark this image as being used as a transfer src
    ///
    pub fn usage_transfer_src(mut self) -> Self {
        self.inner = self.inner.usage(ImageUsageFlags::TRANSFER_SRC);
        self
    }

    ///
    /// Mark this image as being used as a transfer dst
    ///
    pub fn usage_transfer_dst(mut self) -> Self {
        self.inner = self.inner.usage(ImageUsageFlags::TRANSFER_DST);
        self
    }

    ///
    /// Create the image
    ///
    pub unsafe fn build(self, device: &Device, allocator: &Allocator) -> ColourImage {
        let image = self.inner.build(device, allocator);

        ColourImage { inner: image }
    }
}

///
/// Represents an image that will be used as a colour image
///
/// This will always be a single layer, non mip-mapped, optimally tiled, queue exclusive, non
/// multi-sampled image
///
pub struct ColourImage {
    inner: ImageSingle2D,
}

impl ColourImage {
    ///
    /// Get a builder
    ///
    pub fn builder() -> ColourImageBuilder {
        ColourImageBuilder::new()
    }
}

impl Deref for ColourImage {
    type Target = ImageSingle2D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
