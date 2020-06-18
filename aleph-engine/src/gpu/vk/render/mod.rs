//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use crate::gpu::vk::Device;
use erupt::vk1_0::{
    ComponentMappingBuilder, ComponentSwizzle, Extent2D, Extent3D, Format, Framebuffer,
    FramebufferCreateInfoBuilder, Image, ImageAspectFlags, ImageCreateInfoBuilder, ImageLayout,
    ImageSubresourceRangeBuilder, ImageTiling, ImageUsageFlags, ImageView,
    ImageViewCreateInfoBuilder, ImageViewType, RenderPass, SampleCountFlagBits, SharingMode,
    Vk10DeviceLoaderExt,
};

///
/// Represents an image that will be used as a render target
///
pub struct RenderImage {
    allocation: Allocation,
    image: Image,
    image_view: ImageView,
    format: Format,
    extent: Extent2D,
}

impl RenderImage {
    ///
    /// Allocates memory for an image of the given size and format then creates the Image handle
    /// and an ImageView for the image
    ///
    pub fn new(device: &Device, alloc: &Allocator, extent: Extent2D, format: Format) -> Self {
        let extent3d = Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        };

        let create_info = ImageCreateInfoBuilder::new()
            .format(format)
            .extent(extent3d)
            .array_layers(1)
            .mip_levels(1)
            .usage(ImageUsageFlags::COLOR_ATTACHMENT | ImageUsageFlags::INPUT_ATTACHMENT)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .tiling(ImageTiling::OPTIMAL)
            .initial_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .samples(SampleCountFlagBits::_1);
        let alloc_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::GPUOnly)
            .build();
        let (image, allocation) = unsafe {
            alloc
                .create_image(&create_info, &alloc_info)
                .expect("Failed to create RenderImage Image")
        };

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
            .aspect_mask(ImageAspectFlags::COLOR);
        let create_info = ImageViewCreateInfoBuilder::new()
            .format(format)
            .image(image)
            .view_type(ImageViewType::_2D)
            .components(component_mapping)
            .subresource_range(subresource_range);
        let image_view = unsafe {
            device
                .loader()
                .create_image_view(&create_info, None, None)
                .expect("Failed to create RenderImage ImageView")
        };

        Self {
            allocation,
            image,
            image_view,
            format,
            extent,
        }
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
    /// Gets the width of the underlying image
    ///
    pub fn width(&self) -> u32 {
        self.extent.width
    }

    ///
    /// Gets the height of the underlying image
    ///
    pub fn height(&self) -> u32 {
        self.extent.height
    }

    ///
    /// Destroys the image
    ///
    pub fn destroy(self, device: &Device, alloc: &Allocator) {
        unsafe {
            device.loader().destroy_image_view(self.image_view, None);
            alloc.destroy_image(self.image, self.allocation);
        }
    }
}

///
/// Represents a framebuffer for an image
///
pub struct RenderFramebuffer {
    framebuffer: Framebuffer,
}

impl RenderFramebuffer {
    ///
    /// Creates a new framebuffer to be used with the given image
    ///
    pub fn new(device: &Device, render_pass: RenderPass, image: &RenderImage) -> Self {
        let attachments = [image.image_view()];
        let create_info = FramebufferCreateInfoBuilder::new()
            .render_pass(render_pass)
            .width(image.width())
            .height(image.height())
            .layers(1)
            .attachments(&attachments);
        let framebuffer = unsafe {
            device
                .loader()
                .create_framebuffer(&create_info, None, None)
                .expect("Failed to create RenderFramebuffer Framebuffer")
        };
        Self { framebuffer }
    }

    ///
    /// Destroys the framebuffer object
    ///
    pub fn destroy(self, device: &Device) {
        unsafe { device.loader().destroy_framebuffer(self.framebuffer, None) }
    }
}

///
///
///
pub struct Renderer {}
