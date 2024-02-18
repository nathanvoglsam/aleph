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

use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::renderer::backbuffer_import_pass::BackBufferHandle;
use crate::renderer::tone_map_pass::TonemapPassOutput;
use crate::shader_db_accessor::ShaderDatabaseAccessor;

struct CopyTexturePassPayload {
    input: ResourceRef,
    output: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    _device: &dyn IDevice,
    pin_board: &PinBoard,
    _shader_db: &ShaderDatabaseAccessor,
) {
    frame_graph.add_pass(
        "CopyTexturePass",
        |data: &mut Payload<CopyTexturePassPayload>, resources| {
            let tonemap_pass: &TonemapPassOutput = pin_board.get().unwrap();
            let BackBufferHandle { back_buffer } = pin_board.get().unwrap();

            let input =
                resources.read_texture(tonemap_pass.output, ResourceUsageFlags::COPY_SOURCE);
            let output = resources.write_texture(*back_buffer, ResourceUsageFlags::COPY_DEST);

            data.write(CopyTexturePassPayload { input, output });
            pin_board.publish(BackBufferHandle {
                back_buffer: output,
            });
        },
        |data, encoder, resources| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            let input = resources.get_texture(data.input).unwrap();
            let output = resources.get_texture(data.output).unwrap();
            let desc = input.desc_ref();

            let info = TextureSubresourceCopyInfo {
                mip_level: 0,
                array_layer: 0,
                aspect: TextureCopyAspect::Color,
                offset: Default::default(),
            };
            encoder.copy_texture_regions(
                input,
                output,
                &[TextureToTextureCopyInfo {
                    src: info.clone(),
                    dst: info.clone(),
                    extent: desc.get_extent_3d(),
                }],
            );
        },
    );
}
