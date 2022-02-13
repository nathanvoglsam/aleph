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

use erupt::vk1_0::{
    AttachmentDescriptionBuilder, AttachmentLoadOp, AttachmentStoreOp, Extent2D, Format, Image,
    ImageLayout, ImageView, Offset2D, Rect2DBuilder, SampleCountFlagBits, ViewportBuilder,
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
}