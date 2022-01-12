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

use dx12::dxgi;
use interfaces::gpu::TextureFormat;

/// Internal function for converting texture format to DXGI_FORMAT
pub fn texture_format_to_dxgi(format: TextureFormat) -> dxgi::Format {
    match format {
        TextureFormat::R8Unorm => dxgi::Format::R8Unorm,
        TextureFormat::R8Snorm => dxgi::Format::R8Snorm,
        TextureFormat::R8Uint => dxgi::Format::R8Uint,
        TextureFormat::R8Sint => dxgi::Format::R8Sint,
        TextureFormat::R16Uint => dxgi::Format::R16Uint,
        TextureFormat::R16Sint => dxgi::Format::R16Sint,
        TextureFormat::R16Unorm => dxgi::Format::R16Unorm,
        TextureFormat::R16Snorm => dxgi::Format::R16Snorm,
        TextureFormat::R16Float => dxgi::Format::R16Float,
        TextureFormat::Rg8Unorm => dxgi::Format::R8G8Unorm,
        TextureFormat::Rg8Snorm => dxgi::Format::R8G8Snorm,
        TextureFormat::Rg8Uint => dxgi::Format::R8G8Uint,
        TextureFormat::Rg8Sint => dxgi::Format::R8G8Sint,
        TextureFormat::R32Uint => dxgi::Format::R32Uint,
        TextureFormat::R32Sint => dxgi::Format::R32Sint,
        TextureFormat::R32Float => dxgi::Format::R32Float,
        TextureFormat::Rg16Uint => dxgi::Format::R16G16Uint,
        TextureFormat::Rg16Sint => dxgi::Format::R16G16Sint,
        TextureFormat::Rg16Unorm => dxgi::Format::R16G16Unorm,
        TextureFormat::Rg16Snorm => dxgi::Format::R16G16Snorm,
        TextureFormat::Rg16Float => dxgi::Format::R16G16Float,
        TextureFormat::Rgba8Unorm => dxgi::Format::R8G8B8A8Unorm,
        TextureFormat::Rgba8UnormSrgb => dxgi::Format::R8G8B8A8UnormSRGB,
        TextureFormat::Rgba8Snorm => dxgi::Format::R8G8B8A8Snorm,
        TextureFormat::Rgba8Uint => dxgi::Format::R8G8B8A8Uint,
        TextureFormat::Rgba8Sint => dxgi::Format::R8G8B8A8Sint,
        TextureFormat::Bgra8Unorm => dxgi::Format::B8G8R8A8Unorm,
        TextureFormat::Bgra8UnormSrgb => dxgi::Format::B8G8R8A8UnormSRGB,
        TextureFormat::Rgb10a2Unorm => dxgi::Format::R10G10B10A2Unorm,
        TextureFormat::Rg11b10Float => dxgi::Format::R11G11B10Float,
        TextureFormat::Rg32Uint => dxgi::Format::R32G32B32Uint,
        TextureFormat::Rg32Sint => dxgi::Format::R32G32B32Sint,
        TextureFormat::Rg32Float => dxgi::Format::R32G32B32Float,
        TextureFormat::Rgba16Uint => dxgi::Format::R16G16B16A16Uint,
        TextureFormat::Rgba16Sint => dxgi::Format::R16G16B16A16Sint,
        TextureFormat::Rgba16Unorm => dxgi::Format::R16G16B16A16Unorm,
        TextureFormat::Rgba16Snorm => dxgi::Format::R16G16B16A16Snorm,
        TextureFormat::Rgba16Float => dxgi::Format::R16G16B16A16Float,
        TextureFormat::Rgba32Uint => dxgi::Format::R32G32B32A32Uint,
        TextureFormat::Rgba32Sint => dxgi::Format::R32G32B32A32Sint,
        TextureFormat::Rgba32Float => dxgi::Format::R32G32B32A32Float,
        TextureFormat::Depth32Float => dxgi::Format::D32Float,
        TextureFormat::Depth24Stencil8 => dxgi::Format::D24UnormS8Uint,
    }
}
