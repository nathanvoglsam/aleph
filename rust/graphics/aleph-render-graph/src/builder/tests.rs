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

use crate::{
    BufferExport, BufferImport, BufferReadDescription, BufferWriteDescription, ClosurePass,
    ImageExport, ImageImport, ImageReadDescription, ImageWriteDescription, RenderGraphBuilder,
};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, Buffer, BufferUsageFlags, Extent2D, Format, ImageLayout, ImageView,
    PipelineStageFlagBits,
};

#[test]
fn graph_construct_1() {
    // Create the builder for making a new render graph
    let mut builder = RenderGraphBuilder::new();

    // Import a dummy image
    let image_view = ImageView::null();
    let initial_layout = ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    let format = Format::R8G8B8A8_UNORM;
    let extent = Extent2D {
        width: 56,
        height: 56,
    };
    let test_import = ImageImport::new(image_view, initial_layout, format, extent)
        .access_type(AccessFlagBits::COLOR_ATTACHMENT_WRITE)
        .in_stage(PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT);
    builder.import_image("TEST_IMPORT", test_import);

    // Import a dummy buffer
    let buffer = Buffer::null();
    let usage_flags = BufferUsageFlags::UNIFORM_BUFFER;
    let size = 56;
    let test_import = BufferImport::new(buffer, usage_flags, size)
        .access_type(AccessFlagBits::TRANSFER_WRITE)
        .in_stage(PipelineStageFlagBits::TRANSFER);
    builder.import_buffer("TEST_IMPORT_B", test_import);

    // Export an image
    let test_export = ImageExport::new(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .access_type(AccessFlagBits::SHADER_READ)
        .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);
    builder.export_image("TEST_EXPORT", test_export);

    // Export a buffer
    let test_export = BufferExport::new()
        .access_type(AccessFlagBits::UNIFORM_READ)
        .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);
    builder.export_buffer("TEST_EXPORT_B", test_export);

    let pass = ClosurePass::new(
        |accesses| {
            let read =
                ImageReadDescription::new("TEST_IMPORT", ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .access_type(AccessFlagBits::SHADER_READ)
                    .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);
            accesses.read_image(read);

            let read = BufferReadDescription::new("TEST_IMPORT_B")
                .access_type(AccessFlagBits::UNIFORM_READ)
                .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);
            accesses.read_buffer(read);

            let write =
                ImageWriteDescription::new("TEST_EXPORT", ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .access_type(AccessFlagBits::COLOR_ATTACHMENT_WRITE)
                    .in_stage(PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT);
            accesses.write_image(write);

            let write = BufferWriteDescription::new("TEST_EXPORT_B")
                .access_type(AccessFlagBits::TRANSFER_WRITE)
                .in_stage(PipelineStageFlagBits::TRANSFER);
            accesses.write_buffer(write);
        },
        || {},
        || {},
    );
    builder.pass(pass);

    let graph = builder.build().unwrap();
}
