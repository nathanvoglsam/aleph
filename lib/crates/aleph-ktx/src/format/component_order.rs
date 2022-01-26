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

use aleph_vk_format::VkFormat;

///
/// If the format places the alpha channel first.
///
/// # Info
///
/// This refers to whether the format lists the alpha channel first in the name, like
/// A1R5G5B5_UNORM_PACK16, regardless of data layout.
///
#[inline]
pub fn is_format_alpha_first_ordered(format: VkFormat) -> bool {
    matches!(
        format,
        VkFormat::A1R5G5B5_UNORM_PACK16
            | VkFormat::A2R10G10B10_UNORM_PACK32
            | VkFormat::A2R10G10B10_SNORM_PACK32
            | VkFormat::A2R10G10B10_UINT_PACK32
            | VkFormat::A2R10G10B10_SINT_PACK32
            | VkFormat::A2B10G10R10_UNORM_PACK32
            | VkFormat::A2B10G10R10_SNORM_PACK32
            | VkFormat::A2B10G10R10_UINT_PACK32
            | VkFormat::A2B10G10R10_SINT_PACK32
    )
}

///
/// This returns if the format is laid out in RGBDS order.
///
/// # Info
///
/// This refers to how the name describes the format, not necessarily how it is laid out in memory.
/// That is, R4G4B4A4_UNORM_PACK16 and R8G8B8_UNORM are considered rgb ordered because of the name
/// regardless of data layout.
///
/// This also handles ordering of formats where depth is before stencil.
///
#[inline]
pub fn is_format_rgbds_ordered(format: VkFormat) -> bool {
    matches!(
        format,
        VkFormat::R4G4_UNORM_PACK8
            | VkFormat::R4G4B4A4_UNORM_PACK16
            | VkFormat::R5G6B5_UNORM_PACK16
            | VkFormat::R5G5B5A1_UNORM_PACK16
            | VkFormat::A1R5G5B5_UNORM_PACK16
            | VkFormat::R8_UNORM
            | VkFormat::R8_SNORM
            | VkFormat::R8_UINT
            | VkFormat::R8_SINT
            | VkFormat::R8_SRGB
            | VkFormat::R8G8_UNORM
            | VkFormat::R8G8_SNORM
            | VkFormat::R8G8_UINT
            | VkFormat::R8G8_SINT
            | VkFormat::R8G8_SRGB
            | VkFormat::R8G8B8_UNORM
            | VkFormat::R8G8B8_SNORM
            | VkFormat::R8G8B8_UINT
            | VkFormat::R8G8B8_SINT
            | VkFormat::R8G8B8_SRGB
            | VkFormat::R8G8B8A8_UNORM
            | VkFormat::R8G8B8A8_SNORM
            | VkFormat::R8G8B8A8_UINT
            | VkFormat::R8G8B8A8_SINT
            | VkFormat::R8G8B8A8_SRGB
            | VkFormat::A2R10G10B10_UNORM_PACK32
            | VkFormat::A2R10G10B10_SNORM_PACK32
            | VkFormat::A2R10G10B10_UINT_PACK32
            | VkFormat::A2R10G10B10_SINT_PACK32
            | VkFormat::R16_UNORM
            | VkFormat::R16_SNORM
            | VkFormat::R16_UINT
            | VkFormat::R16_SINT
            | VkFormat::R16_SFLOAT
            | VkFormat::R16G16_UNORM
            | VkFormat::R16G16_SNORM
            | VkFormat::R16G16_UINT
            | VkFormat::R16G16_SINT
            | VkFormat::R16G16_SFLOAT
            | VkFormat::R16G16B16_UNORM
            | VkFormat::R16G16B16_SNORM
            | VkFormat::R16G16B16_UINT
            | VkFormat::R16G16B16_SINT
            | VkFormat::R16G16B16_SFLOAT
            | VkFormat::R16G16B16A16_UNORM
            | VkFormat::R16G16B16A16_SNORM
            | VkFormat::R16G16B16A16_UINT
            | VkFormat::R16G16B16A16_SINT
            | VkFormat::R16G16B16A16_SFLOAT
            | VkFormat::R32_UINT
            | VkFormat::R32_SINT
            | VkFormat::R32_SFLOAT
            | VkFormat::R32G32_UINT
            | VkFormat::R32G32_SINT
            | VkFormat::R32G32_SFLOAT
            | VkFormat::R32G32B32_UINT
            | VkFormat::R32G32B32_SINT
            | VkFormat::R32G32B32_SFLOAT
            | VkFormat::R32G32B32A32_UINT
            | VkFormat::R32G32B32A32_SINT
            | VkFormat::R32G32B32A32_SFLOAT
            | VkFormat::R64_UINT
            | VkFormat::R64_SINT
            | VkFormat::R64_SFLOAT
            | VkFormat::R64G64_UINT
            | VkFormat::R64G64_SINT
            | VkFormat::R64G64_SFLOAT
            | VkFormat::R64G64B64_UINT
            | VkFormat::R64G64B64_SINT
            | VkFormat::R64G64B64_SFLOAT
            | VkFormat::R64G64B64A64_UINT
            | VkFormat::R64G64B64A64_SINT
            | VkFormat::R64G64B64A64_SFLOAT
            | VkFormat::D16_UNORM
            | VkFormat::D32_SFLOAT
            | VkFormat::S8_UINT
            | VkFormat::D16_UNORM_S8_UINT
            | VkFormat::D24_UNORM_S8_UINT
            | VkFormat::X8_D24_UNORM_PACK32
            | VkFormat::D32_SFLOAT_S8_UINT
    )
}
