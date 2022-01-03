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

use erupt::vk;
use interfaces::gpu::TextureFormat;

/// Internal function for converting texture format to DXGI_FORMAT
pub fn texture_format_to_vk(format: TextureFormat) -> vk::Format {
    match format {
        TextureFormat::R8Unorm => vk::Format::R8_UNORM,
        TextureFormat::R8Snorm => vk::Format::R8_SNORM,
        TextureFormat::R8Uint => vk::Format::R8_UINT,
        TextureFormat::R8Sint => vk::Format::R8_SINT,
        TextureFormat::R16Uint => vk::Format::R16_UINT,
        TextureFormat::R16Sint => vk::Format::R16_SINT,
        TextureFormat::R16Unorm => vk::Format::R16_UNORM,
        TextureFormat::R16Snorm => vk::Format::R16_SNORM,
        TextureFormat::R16Float => vk::Format::R16_SFLOAT,
        TextureFormat::Rg8Unorm => vk::Format::R8G8_UNORM,
        TextureFormat::Rg8Snorm => vk::Format::R8G8_SNORM,
        TextureFormat::Rg8Uint => vk::Format::R8G8_UINT,
        TextureFormat::Rg8Sint => vk::Format::R8G8_SINT,
        TextureFormat::R32Uint => vk::Format::R32_UINT,
        TextureFormat::R32Sint => vk::Format::R32_SINT,
        TextureFormat::R32Float => vk::Format::R32_SFLOAT,
        TextureFormat::Rg16Uint => vk::Format::R16G16_UINT,
        TextureFormat::Rg16Sint => vk::Format::R16G16_SINT,
        TextureFormat::Rg16Unorm => vk::Format::R16G16_UNORM,
        TextureFormat::Rg16Snorm => vk::Format::R16G16_SNORM,
        TextureFormat::Rg16Float => vk::Format::R16G16_SFLOAT,
        TextureFormat::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
        TextureFormat::Rgba8UnormSrgb => vk::Format::R8G8B8A8_SRGB,
        TextureFormat::Rgba8Snorm => vk::Format::R8G8B8A8_SNORM,
        TextureFormat::Rgba8Uint => vk::Format::R8G8B8A8_UINT,
        TextureFormat::Rgba8Sint => vk::Format::R8G8B8A8_SINT,
        TextureFormat::Bgra8Unorm => vk::Format::B8G8R8A8_UNORM,
        TextureFormat::Bgra8UnormSrgb => vk::Format::B8G8R8A8_SRGB,
        TextureFormat::Rgb10a2Unorm => vk::Format::A2B10G10R10_UNORM_PACK32,
        TextureFormat::Rg11b10Float => vk::Format::B10G11R11_UFLOAT_PACK32,
        TextureFormat::Rg32Uint => vk::Format::R32G32B32_UINT,
        TextureFormat::Rg32Sint => vk::Format::R32G32B32_SINT,
        TextureFormat::Rg32Float => vk::Format::R32G32B32_SFLOAT,
        TextureFormat::Rgba16Uint => vk::Format::R16G16B16A16_UINT,
        TextureFormat::Rgba16Sint => vk::Format::R16G16B16A16_SINT,
        TextureFormat::Rgba16Unorm => vk::Format::R16G16B16A16_UNORM,
        TextureFormat::Rgba16Snorm => vk::Format::R16G16B16A16_SNORM,
        TextureFormat::Rgba16Float => vk::Format::R16G16B16A16_SFLOAT,
        TextureFormat::Rgba32Uint => vk::Format::R32G32B32A32_UINT,
        TextureFormat::Rgba32Sint => vk::Format::R32G32B32A32_SINT,
        TextureFormat::Rgba32Float => vk::Format::R32G32B32A32_SFLOAT,
        TextureFormat::Depth32Float => vk::Format::D32_SFLOAT,
        TextureFormat::Depth24Stencil8 => vk::Format::D24_UNORM_S8_UINT,
    }
}
