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

use crate::format_red_bits;

///
/// Returns the bit size of a PACKxx format. If the format isn't a PACK format this function returns
/// `None`
///
#[inline]
pub fn format_pack_bits(format: VkFormat) -> Option<u16> {
    match format {
        VkFormat::R4G4_UNORM_PACK8 => Some(8),
        VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16
        | VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(16),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32
        | VkFormat::B10G11R11_UFLOAT_PACK32
        | VkFormat::E5B9G9R9_UFLOAT_PACK32
        | VkFormat::X8_D24_UNORM_PACK32 => Some(32),
        _ => None,
    }
}

///
/// Returns the expected value of `typeSize` in the KTX2 header
///
pub fn format_type_size(format: VkFormat) -> u32 {
    if format.is_block_format() {
        1
    } else if let Some(bits) = format_pack_bits(format) {
        bits as u32 / 8
    } else if format.is_depth_format() || format.is_stencil_format() {
        match format {
            VkFormat::D32_SFLOAT => 4,
            VkFormat::D32_SFLOAT_S8_UINT => 8,
            VkFormat::D24_UNORM_S8_UINT => 4,
            VkFormat::D16_UNORM_S8_UINT => 2,
            VkFormat::D16_UNORM => 2,
            VkFormat::S8_UINT => 1,
            _ => unreachable!(),
        }
    } else if format == VkFormat::UNDEFINED {
        1
    } else if let Some(bits) = format_red_bits(format) {
        bits as u32 / 8
    } else {
        unreachable!();
    }
}
