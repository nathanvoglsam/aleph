//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{AttachmentReferenceBuilder, ImageLayout};

///
/// Namespace struct for constructing `VkAttachmentReference` objects in a more succinct way
///
pub struct AttachmentReference {}

impl AttachmentReference {
    ///
    /// Creates a new attachment reference object with the given values
    ///
    pub fn new(attachment: u32, layout: ImageLayout) -> AttachmentReferenceBuilder<'static> {
        AttachmentReferenceBuilder::new()
            .attachment(attachment)
            .layout(layout)
    }

    ///
    /// Creates a new attachment reference object for a `COLOR_ATTACHMENT_OPTIMAL` attachment with
    /// the given attachment index
    ///
    pub fn color(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::new(attachment, ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
    }

    ///
    /// Creates a new attachment reference object for a `DEPTH_STENCIL_ATTACHMENT_OPTIMAL`
    /// attachment with the given attachment index
    ///
    pub fn depth_stencil(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::new(attachment, ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
    }

    ///
    /// Creates a new attachment reference object for a `SHADER_READ_ONLY_OPTIMAL`
    /// attachment with the given attachment index
    ///
    pub fn shader_read_only(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::new(attachment, ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    }
}
