//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{
    ClosurePass, ImageExport, ImageImport, ImageReadDescription, ImageWriteDescription,
    RenderGraphBuilder,
};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, Extent2D, Format, ImageLayout, ImageView, PipelineStageFlagBits,
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

    let test_export = ImageExport::new(ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .access_type(AccessFlagBits::SHADER_READ)
        .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);

    builder.export_image("TEST_EXPORT", test_export);

    let pass = ClosurePass::new(
        |accesses| {
            let read =
                ImageReadDescription::new("TEST_IMPORT", ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .access_type(AccessFlagBits::SHADER_READ)
                    .in_stage(PipelineStageFlagBits::FRAGMENT_SHADER);
            accesses.read_image(read);

            let write =
                ImageWriteDescription::new("TEST_EXPORT", ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .access_type(AccessFlagBits::COLOR_ATTACHMENT_WRITE)
                    .in_stage(PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT);
            accesses.write_image(write);
        },
        || {},
        || {},
    );
    builder.pass(pass);

    let graph = builder.build().unwrap();
}
