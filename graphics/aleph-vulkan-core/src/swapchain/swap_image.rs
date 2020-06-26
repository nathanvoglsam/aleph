//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use erupt::vk1_0::{
    AttachmentDescriptionBuilder, AttachmentLoadOp, AttachmentStoreOp, Format, Image, ImageLayout,
    ImageView, SampleCountFlagBits,
};

///
/// Represents an image from a swapchain
///
#[derive(Clone)]
pub struct SwapImage {
    image: Image,
    image_view: ImageView,
    format: Format,
    extent: (u32, u32),
}

impl SwapImage {
    ///
    /// An internal function for creating a swap image handle
    ///
    pub(crate) fn internal_create(
        image: Image,
        image_view: ImageView,
        format: Format,
        width: u32,
        height: u32,
    ) -> SwapImage {
        SwapImage {
            image,
            image_view,
            format,
            extent: (width, height),
        }
    }

    ///
    /// Gets the width of the image
    ///
    pub fn width(&self) -> u32 {
        self.extent.0
    }

    ///
    /// Gets the height of the image
    ///
    pub fn height(&self) -> u32 {
        self.extent.1
    }

    ///
    /// Gets the internal image handle
    ///
    pub fn image(&self) -> Image {
        self.image
    }

    ///
    /// Gets the internal image view handle
    ///
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }

    ///
    /// Gets the format of the image
    ///
    pub fn format(&self) -> Format {
        self.format
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
}
