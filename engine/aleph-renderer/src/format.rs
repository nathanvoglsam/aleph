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

use aleph_rhi_api::Format;
use aleph_vk_format::VkFormat;

pub const fn vk_to_rhi_format(v: VkFormat) -> Option<Format> {
    let v = match v {
        VkFormat::R8_UNORM => Format::R8Unorm,
        VkFormat::R8_SNORM => Format::R8Snorm,
        VkFormat::R8_UINT => Format::R8Uint,
        VkFormat::R8_SINT => Format::R8Sint,
        VkFormat::R16_UINT => Format::R16Uint,
        VkFormat::R16_SINT => Format::R16Sint,
        VkFormat::R16_UNORM => Format::R16Unorm,
        VkFormat::R16_SNORM => Format::R16Snorm,
        VkFormat::R16_SFLOAT => Format::R16Float,
        VkFormat::R32_UINT => Format::R32Uint,
        VkFormat::R32_SINT => Format::R32Sint,
        VkFormat::R32_SFLOAT => Format::R32Float,
        VkFormat::R8G8_UNORM => Format::Rg8Unorm,
        VkFormat::R8G8_SNORM => Format::Rg8Snorm,
        VkFormat::R8G8_UINT => Format::Rg8Uint,
        VkFormat::R8G8_SINT => Format::Rg8Sint,
        VkFormat::R16G16_UINT => Format::Rg16Uint,
        VkFormat::R16G16_SINT => Format::Rg16Sint,
        VkFormat::R16G16_UNORM => Format::Rg16Unorm,
        VkFormat::R16G16_SNORM => Format::Rg16Snorm,
        VkFormat::R16G16_SFLOAT => Format::Rg16Float,
        VkFormat::R32G32_UINT => Format::Rg32Uint,
        VkFormat::R32G32_SINT => Format::Rg32Sint,
        VkFormat::R32G32_SFLOAT => Format::Rg32Float,
        VkFormat::R32G32B32_UINT => Format::Rgb32Uint,
        VkFormat::R32G32B32_SINT => Format::Rgb32Sint,
        VkFormat::R32G32B32_SFLOAT => Format::Rgb32Float,
        VkFormat::R8G8B8A8_UNORM => Format::Rgba8Unorm,
        VkFormat::R8G8B8A8_SRGB => Format::Rgba8UnormSrgb,
        VkFormat::R8G8B8A8_SNORM => Format::Rgba8Snorm,
        VkFormat::R8G8B8A8_UINT => Format::Rgba8Uint,
        VkFormat::R8G8B8A8_SINT => Format::Rgba8Sint,
        VkFormat::B8G8R8A8_UNORM => Format::Bgra8Unorm,
        VkFormat::B8G8R8A8_SRGB => Format::Bgra8UnormSrgb,
        VkFormat::A2B10G10R10_UNORM_PACK32 => Format::Rgb10a2Unorm,
        VkFormat::B10G11R11_UFLOAT_PACK32 => Format::Rg11b10Float,
        VkFormat::R16G16B16A16_UINT => Format::Rgba16Uint,
        VkFormat::R16G16B16A16_SINT => Format::Rgba16Sint,
        VkFormat::R16G16B16A16_UNORM => Format::Rgba16Unorm,
        VkFormat::R16G16B16A16_SNORM => Format::Rgba16Snorm,
        VkFormat::R16G16B16A16_SFLOAT => Format::Rgba16Float,
        VkFormat::R32G32B32A32_UINT => Format::Rgba32Uint,
        VkFormat::R32G32B32A32_SINT => Format::Rgba32Sint,
        VkFormat::R32G32B32A32_SFLOAT => Format::Rgba32Float,
        VkFormat::D32_SFLOAT => Format::Depth32Float,
        VkFormat::D32_SFLOAT_S8_UINT => Format::Depth32FloatStencil8,
        VkFormat::D24_UNORM_S8_UINT => Format::Depth24Stencil8,
        _ => return None,
    };
    Some(v)
}
