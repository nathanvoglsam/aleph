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
use interfaces::gpu::{PresentationMode, Format};

/// Internal function for converting texture format to VkFormat
pub fn texture_format_to_vk(format: Format) -> vk::Format {
    match format {
        Format::R8Unorm => vk::Format::R8_UNORM,
        Format::R8Snorm => vk::Format::R8_SNORM,
        Format::R8Uint => vk::Format::R8_UINT,
        Format::R8Sint => vk::Format::R8_SINT,
        Format::R16Uint => vk::Format::R16_UINT,
        Format::R16Sint => vk::Format::R16_SINT,
        Format::R16Unorm => vk::Format::R16_UNORM,
        Format::R16Snorm => vk::Format::R16_SNORM,
        Format::R16Float => vk::Format::R16_SFLOAT,
        Format::Rg8Unorm => vk::Format::R8G8_UNORM,
        Format::Rg8Snorm => vk::Format::R8G8_SNORM,
        Format::Rg8Uint => vk::Format::R8G8_UINT,
        Format::Rg8Sint => vk::Format::R8G8_SINT,
        Format::R32Uint => vk::Format::R32_UINT,
        Format::R32Sint => vk::Format::R32_SINT,
        Format::R32Float => vk::Format::R32_SFLOAT,
        Format::Rg16Uint => vk::Format::R16G16_UINT,
        Format::Rg16Sint => vk::Format::R16G16_SINT,
        Format::Rg16Unorm => vk::Format::R16G16_UNORM,
        Format::Rg16Snorm => vk::Format::R16G16_SNORM,
        Format::Rg16Float => vk::Format::R16G16_SFLOAT,
        Format::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
        Format::Rgba8UnormSrgb => vk::Format::R8G8B8A8_SRGB,
        Format::Rgba8Snorm => vk::Format::R8G8B8A8_SNORM,
        Format::Rgba8Uint => vk::Format::R8G8B8A8_UINT,
        Format::Rgba8Sint => vk::Format::R8G8B8A8_SINT,
        Format::Bgra8Unorm => vk::Format::B8G8R8A8_UNORM,
        Format::Bgra8UnormSrgb => vk::Format::B8G8R8A8_SRGB,
        Format::Rgb10a2Unorm => vk::Format::A2B10G10R10_UNORM_PACK32,
        Format::Rg11b10Float => vk::Format::B10G11R11_UFLOAT_PACK32,
        Format::Rg32Uint => vk::Format::R32G32B32_UINT,
        Format::Rg32Sint => vk::Format::R32G32B32_SINT,
        Format::Rg32Float => vk::Format::R32G32B32_SFLOAT,
        Format::Rgba16Uint => vk::Format::R16G16B16A16_UINT,
        Format::Rgba16Sint => vk::Format::R16G16B16A16_SINT,
        Format::Rgba16Unorm => vk::Format::R16G16B16A16_UNORM,
        Format::Rgba16Snorm => vk::Format::R16G16B16A16_SNORM,
        Format::Rgba16Float => vk::Format::R16G16B16A16_SFLOAT,
        Format::Rgba32Uint => vk::Format::R32G32B32A32_UINT,
        Format::Rgba32Sint => vk::Format::R32G32B32A32_SINT,
        Format::Rgba32Float => vk::Format::R32G32B32A32_SFLOAT,
        Format::Depth32Float => vk::Format::D32_SFLOAT,
        Format::Depth24Stencil8 => vk::Format::D24_UNORM_S8_UINT,
    }
}

/// Internal function for converting present mode to VkPresentModeKHR
pub fn present_mode_to_vk(mode: PresentationMode) -> vk::PresentModeKHR {
    match mode {
        PresentationMode::Immediate => vk::PresentModeKHR::IMMEDIATE_KHR,
        PresentationMode::Mailbox => vk::PresentModeKHR::MAILBOX_KHR,
        PresentationMode::Fifo => vk::PresentModeKHR::FIFO_KHR,
    }
}
