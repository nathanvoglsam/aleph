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

use crate::image::{ImageSingle2D, ImageSingle2DBuilder};
use aleph_vulkan_alloc::Allocator;
use aleph_vulkan_core::Device;

use aleph_vulkan_core::erupt::vk1_0::{Format, ImageAspectFlags, ImageUsageFlags};
use std::ops::Deref;

///
/// Builder for creating a DepthImage
///
pub struct DepthImageBuilder<'a> {
    inner: ImageSingle2DBuilder<'a>,
}

impl<'a> DepthImageBuilder<'a> {
    ///
    /// Creates a new builder object
    ///
    pub fn new() -> DepthImageBuilder<'a> {
        Self {
            inner: ImageSingle2DBuilder::new()
                .aspect(ImageAspectFlags::DEPTH)
                .usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        }
    }

    ///
    /// The debug name to attach to the created image
    ///
    pub fn debug_name(mut self, debug_name: &'a str) -> Self {
        self.inner = self.inner.debug_name(debug_name);
        self
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
            Format::D16_UNORM_S8_UINT | Format::D24_UNORM_S8_UINT | Format::D32_SFLOAT_S8_UINT => {
                self.inner = self.inner.aspect(ImageAspectFlags::STENCIL)
            }
            Format::D32_SFLOAT | Format::D16_UNORM => {}
            _ => panic!("Expected depth format"),
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
    pub unsafe fn build(self, device: &Device, allocator: &Allocator) -> DepthImage {
        let image = self.inner.build(device, allocator);

        DepthImage { inner: image }
    }
}

///
/// Represents an image that will be used as a depth/stencil render target
///
/// This will always be a single layer, non mip-mapped, optimally tiled, queue exclusive, non
/// multi-sampled image
///
pub struct DepthImage {
    inner: ImageSingle2D,
}

impl DepthImage {
    ///
    /// Get a builder
    ///
    pub fn builder<'a>() -> DepthImageBuilder<'a> {
        DepthImageBuilder::new()
    }
}

impl Deref for DepthImage {
    type Target = ImageSingle2D;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
