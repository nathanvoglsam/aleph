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

use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;

pub struct BackBufferHandle {
    pub back_buffer: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    _device: &dyn IDevice,
    pin_board: &PinBoard,
    _shader_db: &ShaderDatabaseAccessor,
) {
    frame_graph.add_pass(
        "BackBufferImportPass",
        |_data: &mut Payload<()>, resources| {
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
            let back_buffer_desc = back_buffer_info.desc.clone().with_name("Swap Chain Image");
            let back_buffer = resources.import_texture(
                &TextureImportDesc {
                    desc: &back_buffer_desc,
                    before_sync: BarrierSync::ALL,
                    before_access: BarrierAccess::NONE,
                    before_layout: ImageLayout::Undefined,
                    after_sync: BarrierSync::NONE,
                    after_access: BarrierAccess::NONE,
                    after_layout: ImageLayout::PresentSrc,
                },
                ResourceUsageFlags::NONE,
            );

            pin_board.publish(BackBufferHandle { back_buffer });
        },
        |_data, _encoder, _resources| {
            // Nothing lol
        },
    );
}
