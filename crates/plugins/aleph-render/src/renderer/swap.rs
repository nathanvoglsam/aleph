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

use crate::renderer::GlobalObjects;
use dx12::dxgi;

pub struct SwapDependentObjects {
    pub rtv_cpu: dx12::CPUDescriptorHandle,
}

impl SwapDependentObjects {
    pub fn new(
        device: &dx12::Device,
        global: &GlobalObjects,
        buffers: &[dx12::Resource],
        index: usize,
    ) -> Self {
        let rtv_cpu = unsafe { Self::create_rtv(device, global, buffers, index) };
        Self { rtv_cpu }
    }

    pub unsafe fn create_rtv(
        device: &dx12::Device,
        global: &GlobalObjects,
        buffers: &[dx12::Resource],
        index: usize,
    ) -> dx12::CPUDescriptorHandle {
        let size =
            device.get_descriptor_handle_increment_size(dx12::DescriptorHeapType::RenderTargetView);
        let dest = global
            .rtv_heap
            .get_cpu_descriptor_handle_for_heap_start()
            .unwrap()
            .add(index * size as usize);

        let format = dxgi::Format::R8G8B8A8UnormSRGB;
        let texture_2d = dx12::Tex2DRtv {
            mip_slice: 0,
            plane_slice: 0,
        };
        let rtv_desc = dx12::RenderTargetViewDesc::Texture2D { format, texture_2d };
        device.create_render_target_view(&buffers[index], &rtv_desc, dest);
        dest
    }
}
