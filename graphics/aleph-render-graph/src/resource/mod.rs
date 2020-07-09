//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{Extent2D, Format, ImageLayout, ImageView};

pub(crate) const SWAP_IMAGE_RESERVED_NAME: &str = "__RESERVED_INTERNAL_SWAP_IMAGE_NAME";

///
/// Represents an image resource
///
pub struct ImageResource {
    pub(crate) image_view: ImageView,
    pub(crate) initial_layout: ImageLayout,
    pub(crate) format: Format,
    pub(crate) extent: Extent2D,
}

///
/// Internal struct that represents the supported set of resources
///
pub enum Resource {
    Image(ImageResource),
}
