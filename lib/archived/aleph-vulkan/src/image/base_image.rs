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

use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    AttachmentDescriptionBuilder, AttachmentLoadOp, AttachmentStoreOp, ComponentMappingBuilder,
    ComponentSwizzle, Extent2D, Extent3D, Format, Image, ImageAspectFlags, ImageCreateInfoBuilder,
    ImageLayout, ImageSubresourceRangeBuilder, ImageTiling, ImageType, ImageUsageFlags, ImageView,
    ImageViewCreateInfoBuilder, ImageViewType, Offset2D, Rect2DBuilder, SampleCountFlagBits,
    SharingMode, ViewportBuilder,
};
use aleph_vulkan_core::{DebugName, Device};
use std::ffi::CString;

///
/// Builder for creating a ColourImage
///
pub struct ImageSingle2DBuilder<'a> {
    width: u32,
    height: u32,
    format: Format,
    usage: ImageUsageFlags,
    aspect: ImageAspectFlags,
    debug_name: Option<&'a str>,
}

impl<'a> ImageSingle2DBuilder<'a> {
    ///
    /// Creates a new builder object
    ///
    pub fn new() -> ImageSingle2DBuilder<'a> {
        Self {
            width: 0,
            height: 0,
            format: Default::default(),
            usage: Default::default(),
            aspect: Default::default(),
            debug_name: None,
        }
    }

    ///
    /// The debug name to attach to the created image
    ///
    pub fn debug_name(mut self, debug_name: &'a str) -> Self {
        self.debug_name = Some(debug_name);
        self
    }

    ///
    /// Sets the width of the image
    ///
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    ///
    /// Sets the height of the image
    ///
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    ///
    /// Sets the format of the image. Panics if given a depth format
    ///
    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    ///
    /// Add the given usage flag
    ///
    pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
        self.usage |= usage;
        self
    }

    ///
    /// Add the given image aspect
    ///
    pub fn aspect(mut self, aspect: ImageAspectFlags) -> Self {
        self.aspect |= aspect;
        self
    }

    ///
    /// Create the image
    ///
    pub unsafe fn build(self, device: &Device, allocator: &Allocator) -> ImageSingle2D {
        // Build the extents
        let extent = Extent3D {
            width: self.width,
            height: self.height,
            depth: 1,
        };

        let create_info = ImageCreateInfoBuilder::new()
            .format(self.format)
            .usage(self.usage)
            .extent(extent)
            .array_layers(1)
            .mip_levels(1)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .tiling(ImageTiling::OPTIMAL)
            .initial_layout(ImageLayout::UNDEFINED)
            .image_type(ImageType::_2D)
            .samples(SampleCountFlagBits::_1);
        let alloc_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::GPUOnly)
            .build();
        let (image, allocation) = allocator
            .create_image(&create_info, &alloc_info)
            .expect("Failed to create BaseImage Image");

        let component_mapping = *ComponentMappingBuilder::new()
            .a(ComponentSwizzle::A)
            .r(ComponentSwizzle::R)
            .g(ComponentSwizzle::G)
            .b(ComponentSwizzle::B);
        let subresource_range = *ImageSubresourceRangeBuilder::new()
            .base_mip_level(0)
            .base_array_layer(0)
            .level_count(1)
            .layer_count(1)
            .aspect_mask(self.aspect);
        let create_info = ImageViewCreateInfoBuilder::new()
            .format(self.format)
            .image(image)
            .view_type(ImageViewType::_2D)
            .components(component_mapping)
            .subresource_range(subresource_range);
        let image_view = device
            .create_image_view(&create_info, None)
            .expect("Failed to create BaseImage ImageView");

        if let Some(name) = self.debug_name {
            let image_name = format!("{}::Image", name);
            let image_name = CString::new(image_name).unwrap();
            let image_view_name = format!("{}::ImageView", name);
            let image_view_name = CString::new(image_view_name).unwrap();
            image.add_debug_name(device, &image_name);
            image_view.add_debug_name(device, &image_view_name);
        }

        ImageSingle2D {
            allocation,
            image,
            image_view,
            format: self.format,
            usage: self.usage,
            extent: (self.width, self.height),
        }
    }
}

impl<'a> Default for ImageSingle2DBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

///
/// Represents an image that will be used as a render target
///
/// This will always be a single layer, non mip-mapped, optimally tiled, queue exclusive, non
/// multi-sampled image
///
pub struct ImageSingle2D {
    allocation: Allocation,
    image: Image,
    image_view: ImageView,
    format: Format,
    usage: ImageUsageFlags,
    extent: (u32, u32),
}

impl ImageSingle2D {
    ///
    /// Get a builder
    ///
    pub fn builder<'a>() -> ImageSingle2DBuilder<'a> {
        ImageSingle2DBuilder::new()
    }

    ///
    /// Get the image
    ///
    pub fn image(&self) -> Image {
        self.image
    }

    ///
    /// Gets the image view
    ///
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }

    ///
    /// Gets the format of the underlying image
    ///
    pub fn format(&self) -> Format {
        self.format
    }

    ///
    /// Gets the underlying image usage flags
    ///
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }

    ///
    /// Checks if the given layout is valid to be used with this image
    ///
    pub fn compatible_layout(&self, layout: ImageLayout) -> bool {
        match layout {
            // Undefined is always valid
            ImageLayout::UNDEFINED => true,

            // Depth stencil layout only valid if that image was created with the usage flag
            ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => self
                .usage
                .contains(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),

            // Shader read only valid if created to be sampled or as an input attachment
            ImageLayout::SHADER_READ_ONLY_OPTIMAL => self
                .usage
                .contains(ImageUsageFlags::SAMPLED | ImageUsageFlags::INPUT_ATTACHMENT),

            // Can be transfer src if created with that usage
            ImageLayout::TRANSFER_SRC_OPTIMAL => self.usage.contains(ImageUsageFlags::TRANSFER_SRC),

            // Can be transfer dst if created with that usage
            ImageLayout::TRANSFER_DST_OPTIMAL => self.usage.contains(ImageUsageFlags::TRANSFER_DST),

            // Anything else is invalid
            _ => false,
        }
    }

    ///
    /// Creates an attachment description for the given image
    ///
    pub fn attachment_description<'a>(
        &self,
        initial_layout: ImageLayout,
        final_layout: ImageLayout,
        load_op: AttachmentLoadOp,
        store_op: AttachmentStoreOp,
    ) -> AttachmentDescriptionBuilder<'a> {
        AttachmentDescriptionBuilder::new()
            .format(self.format)
            .samples(SampleCountFlagBits::_1)
            .initial_layout(initial_layout)
            .final_layout(final_layout)
            .load_op(load_op)
            .store_op(store_op)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
    }

    ///
    /// Gets a viewport for rendering to the whole image
    ///
    pub fn get_viewport_full(&self) -> ViewportBuilder {
        ViewportBuilder::new()
            .width(self.extent.0 as f32)
            .height(self.extent.1 as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .x(0.0)
            .y(0.0)
    }

    ///
    /// Gets a scissor for rendering to the whole image
    ///
    pub fn get_scissor_full(&self) -> Rect2DBuilder {
        Rect2DBuilder::new()
            .extent(Extent2D {
                width: self.extent.0,
                height: self.extent.1,
            })
            .offset(Offset2D { x: 0, y: 0 })
    }

    ///
    /// Gets the width of the underlying image
    ///
    pub fn width(&self) -> u32 {
        self.extent.0
    }

    ///
    /// Gets the height of the underlying image
    ///
    pub fn height(&self) -> u32 {
        self.extent.1
    }

    ///
    /// Destroys the image
    ///
    pub unsafe fn destroy(&self, device: &Device, alloc: &Allocator) {
        device.destroy_image_view(self.image_view, None);
        alloc.destroy_image(self.image, self.allocation);
    }
}
