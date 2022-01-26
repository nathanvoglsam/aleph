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

use aleph_vulkan_core::erupt::vk1_0::{AttachmentReferenceBuilder, ImageLayout};

///
/// Namespace struct for constructing `VkAttachmentReference` objects in a more succinct way
///
pub struct AttachmentReference {}

impl AttachmentReference {
    ///
    /// Creates a new attachment reference object with the given values
    ///
    pub fn builder(attachment: u32, layout: ImageLayout) -> AttachmentReferenceBuilder<'static> {
        AttachmentReferenceBuilder::new()
            .attachment(attachment)
            .layout(layout)
    }

    ///
    /// Creates a new attachment reference object for a `COLOR_ATTACHMENT_OPTIMAL` attachment with
    /// the given attachment index
    ///
    pub fn color(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::builder(attachment, ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
    }

    ///
    /// Creates a new attachment reference object for a `DEPTH_STENCIL_ATTACHMENT_OPTIMAL`
    /// attachment with the given attachment index
    ///
    pub fn depth_stencil(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::builder(attachment, ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
    }

    ///
    /// Creates a new attachment reference object for a `SHADER_READ_ONLY_OPTIMAL`
    /// attachment with the given attachment index
    ///
    pub fn shader_read_only(attachment: u32) -> AttachmentReferenceBuilder<'static> {
        Self::builder(attachment, ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    }
}
