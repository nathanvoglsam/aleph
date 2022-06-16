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

/// Internal struct that caches the descriptor increment sizes needed for allocating space in
/// descriptor heaps.
pub struct DescriptorHeapInfo {
    /// Descriptor increment for shader resource views
    pub resource_inc: u32,

    /// Descriptor increment for unordered access views
    pub rtv_inc: u32,

    /// Descriptor increment for constant buffer views
    pub dsv_inc: u32,

    /// Descriptor increment for samplers
    pub sampler_inc: u32,
}

impl DescriptorHeapInfo {
    pub fn new(device: &dx12::Device) -> Self {
        Self {
            resource_inc: device
                .get_descriptor_handle_increment_size(dx12::DescriptorHeapType::CbvSrvUav),
            rtv_inc: device
                .get_descriptor_handle_increment_size(dx12::DescriptorHeapType::RenderTargetView),
            dsv_inc: device
                .get_descriptor_handle_increment_size(dx12::DescriptorHeapType::DepthStencilView),
            sampler_inc: device
                .get_descriptor_handle_increment_size(dx12::DescriptorHeapType::Sampler),
        }
    }
}
