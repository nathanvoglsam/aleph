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

use crate::data_format_descriptor::{
    ASTCChannelType, BC1ChannelType, BC2ChannelType, BC3ChannelType, BC4ChannelType,
    BC5ChannelType, BC6ChannelType, BC7ChannelType, ETC1ChannelType, ETC1SChannelType,
    ETC2ChannelType, PVRTC2ChannelType, PVRTCChannelType, SampleFlags,
};
use crate::{
    format_alpha_bits, format_blue_bits, format_depth_bits, format_exponent_bits,
    format_green_bits, format_pack_bits, format_red_bits, format_sample_info_count,
    format_stencil_bits, is_format_alpha_first_ordered, is_format_prohibited,
    is_format_rgbds_ordered, is_format_unsupported, RGBSDAChannelType,
};
use aleph_vk_format::VkFormat;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SampleInfo {
    bit_offset: u16,
    bit_length: u8,
    channel_type: u8,
    sample_flags: SampleFlags,
    sample_positions: [u8; 4],
    sample_lower: u32,
    sample_upper: u32,
}

impl SampleInfo {
    ///
    /// Unpacks the sample info from 4 words (should have been read from a file)
    ///
    pub fn unpack_from(words: &[u32; 4]) -> Self {
        let bit_offset = ((words[0] >> 0) & 0xFFFF) as u16;
        let bit_length = ((words[0] >> 16) & 0xFF) as u8;
        let channel_type = ((words[0] >> 24) & 0xF) as u8;
        let sample_flags = ((words[0] >> 28) & 0xF) as u8;
        let sample_flags = SampleFlags::from_bits_truncate(sample_flags);

        let sample_pos_0 = ((words[1] >> 0) & 0xFF) as u8;
        let sample_pos_1 = ((words[1] >> 8) & 0xFF) as u8;
        let sample_pos_2 = ((words[1] >> 16) & 0xFF) as u8;
        let sample_pos_3 = ((words[1] >> 24) & 0xFF) as u8;
        let sample_positions = [sample_pos_0, sample_pos_1, sample_pos_2, sample_pos_3];

        let sample_lower = words[2];
        let sample_upper = words[3];

        Self {
            bit_offset,
            bit_length,
            channel_type,
            sample_flags,
            sample_positions,
            sample_upper,
            sample_lower,
        }
    }

    ///
    /// Gets the bit offset
    ///
    pub fn bit_offset(&self) -> u16 {
        self.bit_offset
    }

    ///
    /// Gets the decoded bit length (the file stores the bit length minus 1)
    ///
    /// # Info
    ///
    /// We cast the length up to u16 so we don't overflow
    ///
    pub fn bit_length(&self) -> u16 {
        self.bit_length as u16 + 1
    }

    ///
    /// Gets sample flags set by the sample info
    ///
    pub fn sample_flags(&self) -> SampleFlags {
        self.sample_flags
    }

    ///
    /// List of sample positions
    ///
    pub fn sample_positions(&self) -> &[u8; 4] {
        &self.sample_positions
    }

    ///
    /// Gets the sample upper value
    ///
    pub fn sample_upper(&self) -> u32 {
        self.sample_upper
    }

    ///
    /// Gets the sample lower value
    ///
    pub fn sample_lower(&self) -> u32 {
        self.sample_lower
    }
}

impl SampleInfo {
    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc1() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC1ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc1_alpha() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC1ChannelType::ColorAndAlpha.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc2() -> [Self; 2] {
        let a = Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC2ChannelType::Alpha.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        let b = Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: BC2ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        [a, b]
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc3() -> [Self; 2] {
        let a = Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC3ChannelType::Alpha.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        let b = Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: BC3ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        [a, b]
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc4_unsigned() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC4ChannelType::Data.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc4_signed() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC4ChannelType::Data.to_raw(),
            sample_flags: SampleFlags::SIGNED,
            sample_positions: [0, 0, 0, 0],
            sample_lower: i32::MIN as u32,
            sample_upper: i32::MAX as u32,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc5_unsigned() -> [Self; 2] {
        let a = Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC5ChannelType::Red.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        let b = Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: BC5ChannelType::Green.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        };
        [a, b]
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc5_signed() -> [Self; 2] {
        let a = Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC5ChannelType::Red.to_raw(),
            sample_flags: SampleFlags::SIGNED,
            sample_positions: [0, 0, 0, 0],
            sample_lower: i32::MIN as u32,
            sample_upper: i32::MAX as u32,
        };
        let b = Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: BC5ChannelType::Green.to_raw(),
            sample_flags: SampleFlags::SIGNED,
            sample_positions: [0, 0, 0, 0],
            sample_lower: i32::MIN as u32,
            sample_upper: i32::MAX as u32,
        };
        [a, b]
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc6h_unsigned() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC6ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::FLOAT,
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0xBF800000,
            sample_upper: 0x7F800000,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc6h_signed() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: BC6ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::SIGNED | SampleFlags::FLOAT,
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0xBF800000,
            sample_upper: 0x7F800000,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_bc7() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 127,
            channel_type: BC7ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc1() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC1ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_red_unsigned() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC2ChannelType::Red.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_green_unsigned() -> Self {
        Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: ETC2ChannelType::Green.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_red_signed() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC2ChannelType::Red.to_raw(),
            sample_flags: SampleFlags::SIGNED,
            sample_positions: [0, 0, 0, 0],
            sample_lower: i32::MIN as u32,
            sample_upper: i32::MAX as u32,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_green_signed() -> Self {
        Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: ETC2ChannelType::Green.to_raw(),
            sample_flags: SampleFlags::SIGNED,
            sample_positions: [0, 0, 0, 0],
            sample_lower: i32::MIN as u32,
            sample_upper: i32::MAX as u32,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_color() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC2ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_color_separate_alpha() -> Self {
        Self {
            bit_offset: 64,
            bit_length: 63,
            channel_type: ETC2ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc2_alpha() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC2ChannelType::Alpha.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_astc_ldr() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 127,
            channel_type: ASTCChannelType::Data.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_astc_hdr() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 127,
            channel_type: ASTCChannelType::Data.to_raw(),
            sample_flags: SampleFlags::SIGNED | SampleFlags::FLOAT,
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0xBF800000,
            sample_upper: 0x7F800000,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_etc1s() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: ETC1SChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_pvrtc() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: PVRTCChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Produces a `SampleInfo` for the block format specified by the function name
    ///
    #[inline]
    pub fn for_pvrtc2() -> Self {
        Self {
            bit_offset: 0,
            bit_length: 63,
            channel_type: PVRTC2ChannelType::Color.to_raw(),
            sample_flags: SampleFlags::empty(),
            sample_positions: [0, 0, 0, 0],
            sample_lower: 0,
            sample_upper: u32::MAX,
        }
    }

    ///
    /// Returns a `SampleInfo` for the given format, as long as it is supported and allowed by the
    /// KTX2 spec. Otherwise this will return `None`
    ///
    /// This will write the output sample info objects into the `sample_infos` slice passed into the
    /// function. The function will return `None` if an error is occurred, leaving `sample_infos` in
    /// an undefined state as it may have exited mid write
    ///
    /// On success this function will return the number of `SampleInfo` objects written into the
    /// slice.
    ///
    pub fn for_format(format: VkFormat, sample_infos: &mut [SampleInfo]) -> Option<usize> {
        if !format.is_block_format() {
            Self::for_non_block_format(format, sample_infos)
        } else if format.is_bc1() {
            if sample_infos.len() < 1 {
                return None;
            }
            sample_infos[0] = Self::for_bc1();
            Some(1)
        } else if format.is_bc1_alpha() {
            if sample_infos.len() < 1 {
                return None;
            }
            sample_infos[0] = Self::for_bc1_alpha();
            Some(1)
        } else if format.is_bc2() {
            if sample_infos.len() < 2 {
                return None;
            }
            let info = Self::for_bc2();
            sample_infos[0] = info[0].clone();
            sample_infos[1] = info[1].clone();
            Some(2)
        } else if format.is_bc3() {
            if sample_infos.len() < 2 {
                return None;
            }
            let info = Self::for_bc3();
            sample_infos[0] = info[0].clone();
            sample_infos[1] = info[1].clone();
            Some(2)
        } else if format.is_bc4() {
            if sample_infos.len() < 1 {
                return None;
            }
            if format.is_signed() {
                sample_infos[0] = Self::for_bc4_signed();
                Some(1)
            } else {
                sample_infos[0] = Self::for_bc4_unsigned();
                Some(1)
            }
        } else if format.is_bc5() {
            if sample_infos.len() < 2 {
                return None;
            }
            if format.is_signed() {
                let info = Self::for_bc5_signed();
                sample_infos[0] = info[0].clone();
                sample_infos[1] = info[1].clone();
                Some(2)
            } else {
                let info = Self::for_bc5_unsigned();
                sample_infos[0] = info[0].clone();
                sample_infos[1] = info[1].clone();
                Some(2)
            }
        } else if format.is_bc6h() {
            if sample_infos.len() < 1 {
                return None;
            }
            if format.is_signed() {
                sample_infos[0] = Self::for_bc6h_signed();
                Some(1)
            } else {
                sample_infos[0] = Self::for_bc6h_unsigned();
                Some(1)
            }
        } else if format.is_bc7() {
            if sample_infos.len() < 1 {
                return None;
            }
            sample_infos[0] = Self::for_bc7();
            Some(1)
        } else if format.is_astc() {
            if sample_infos.len() < 1 {
                return None;
            }
            if format.is_floating_point() {
                sample_infos[0] = Self::for_astc_hdr();
                Some(1)
            } else {
                sample_infos[0] = Self::for_astc_ldr();
                Some(1)
            }
        } else if format.is_pvrtc1() {
            if sample_infos.len() < 1 {
                return None;
            }
            sample_infos[0] = Self::for_pvrtc();
            Some(1)
        } else if format.is_pvrtc2() {
            if sample_infos.len() < 1 {
                return None;
            }
            sample_infos[0] = Self::for_pvrtc2();
            Some(1)
        } else if format.is_etc2() {
            if format.has_alpha() {
                if sample_infos.len() < 2 {
                    return None;
                }
                if format.is_1bit_alpha() {
                    sample_infos[0] = Self::for_etc2_color();
                    sample_infos[1] = Self::for_etc2_alpha();
                    Some(2)
                } else {
                    sample_infos[0] = Self::for_etc2_alpha();
                    sample_infos[1] = Self::for_etc2_color_separate_alpha();
                    Some(2)
                }
            } else {
                if sample_infos.len() < 1 {
                    return None;
                }
                sample_infos[0] = Self::for_etc2_color();
                Some(1)
            }
        } else if format.is_eac() {
            match format {
                VkFormat::EAC_R11_UNORM_BLOCK => {
                    sample_infos[0] = Self::for_etc2_red_unsigned();
                    Some(1)
                }
                VkFormat::EAC_R11_SNORM_BLOCK => {
                    sample_infos[0] = Self::for_etc2_red_signed();
                    Some(1)
                }
                VkFormat::EAC_R11G11_UNORM_BLOCK => {
                    sample_infos[0] = Self::for_etc2_red_unsigned();
                    sample_infos[1] = Self::for_etc2_green_unsigned();
                    Some(2)
                }
                VkFormat::EAC_R11G11_SNORM_BLOCK => {
                    sample_infos[0] = Self::for_etc2_red_signed();
                    sample_infos[1] = Self::for_etc2_green_signed();
                    Some(2)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    ///
    /// Returns a `SampleInfo` for the given format, as long as it is supported and allowed by the
    /// KTX2 spec. Otherwise this will return `None`
    ///
    /// This will write the output sample info objects into the `sample_infos` slice passed into the
    /// function. The function will return `None` if an error is occurred, leaving `sample_infos` in
    /// an undefined state as it may have exited mid write
    ///
    /// On success this function will return the number of `SampleInfo` objects written into the
    /// slice.
    ///
    pub fn for_non_block_format(
        format: VkFormat,
        sample_infos: &mut [SampleInfo],
    ) -> Option<usize> {
        if format.is_block_format() || is_format_unsupported(format) || is_format_prohibited(format)
        {
            None
        } else {
            let is_packed = format_pack_bits(format).is_some();
            let is_rgb_ordered = is_format_rgbds_ordered(format);
            let alpha_first = is_format_alpha_first_ordered(format);
            let count = format_sample_info_count(format)?;
            let is_srgb = format.is_srgb();
            let is_float = format.is_floating_point();
            let is_signed = format.is_signed();
            let is_normalized = format.is_normalized();
            let r_bits = format_red_bits(format);
            let g_bits = format_green_bits(format);
            let b_bits = format_blue_bits(format);
            let s_bits = format_stencil_bits(format);
            let d_bits = format_depth_bits(format);
            let a_bits = format_alpha_bits(format);
            let e_bits = format_exponent_bits(format);

            // Invert based on whether the format is packed
            let is_rgb_ordered = if is_packed {
                !is_rgb_ordered
            } else {
                is_rgb_ordered
            };
            let alpha_first = if is_packed { !alpha_first } else { alpha_first };

            let sample_flags: SampleFlags = match (is_float, is_signed, is_normalized) {
                (true, _, true) => return None, // Doesn't make sense (float + norm?)
                (true, true, false) => SampleFlags::FLOAT | SampleFlags::SIGNED,
                (true, false, false) => SampleFlags::FLOAT,
                (false, true, _) => SampleFlags::SIGNED,
                (_, _, _) => SampleFlags::empty(),
            };
            let alpha_flags: SampleFlags = match (is_srgb, is_float) {
                (true, true) => return None, // Doesn't make sense
                (true, false) => sample_flags | SampleFlags::LINEAR,
                (false, _) => sample_flags,
            };

            // Check there's enough space to write the sample infos
            if (count as usize) > sample_infos.len() {
                return None;
            }

            match (
                format,
                r_bits,
                g_bits,
                b_bits,
                s_bits,
                d_bits,
                a_bits,
                e_bits,
                is_rgb_ordered,
                alpha_first,
            ) {
                //
                // SINGLE CHANNEL RED FORMATS
                //

                // all R int formats
                (_, Some(r), None, None, None, None, None, None, _, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    Some(1)
                }

                //
                // RED GREEN FORMATS
                //

                // all RG unsigned int formats rgb ordered
                (_, Some(r), Some(g), None, None, None, None, None, true, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: r as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    Some(2)
                }
                (_, Some(r), Some(g), None, None, None, None, None, false, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: g as u16,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    Some(2)
                }

                //
                // RED GREEN BLUE FORMATS
                //

                // all RGB int formats rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, None, None, false, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: b as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: b as u16 + g as u16,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    Some(3)
                }
                // all RGB int formats reverse rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, None, None, true, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: r as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: r as u16 + g as u16,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    Some(3)
                }
                // all ERGB int formats reverse rgb ordered
                (
                    VkFormat::E5B9G9R9_UFLOAT_PACK32,
                    Some(r),
                    Some(g),
                    Some(b),
                    None,
                    None,
                    None,
                    Some(e),
                    _,
                    _,
                ) => {
                    let er = SampleInfo {
                        bit_offset: 27,
                        bit_length: e - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, e),
                        sample_upper: max_val(false, false, e),
                    };
                    let eg = SampleInfo {
                        bit_offset: 27,
                        bit_length: e - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, e),
                        sample_upper: max_val(false, false, e),
                    };
                    let eb = SampleInfo {
                        bit_offset: 27,
                        bit_length: e - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, e),
                        sample_upper: max_val(false, false, e),
                    };

                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, r),
                        sample_upper: max_val(false, false, r),
                    };
                    sample_infos[1] = er;
                    sample_infos[2] = SampleInfo {
                        bit_offset: r as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, g),
                        sample_upper: max_val(false, false, g),
                    };
                    sample_infos[3] = eg;
                    sample_infos[4] = SampleInfo {
                        bit_offset: r as u16 + g as u16,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(false, false, b),
                        sample_upper: max_val(false, false, b),
                    };
                    sample_infos[5] = eb;
                    Some(6)
                }

                //
                // RED GREEN BLUE ALPHA FORMATS
                //

                // all RGBA formats rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, Some(a), None, false, false) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: b as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: b as u16 + g as u16,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    sample_infos[3] = SampleInfo {
                        bit_offset: r as u16 + b as u16 + g as u16,
                        bit_length: a - 1,
                        channel_type: RGBSDAChannelType::Alpha.to_raw(),
                        sample_flags: alpha_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, a),
                        sample_upper: max_val(is_float, is_signed, a),
                    };
                    Some(4)
                }
                // all RGBA formats reverse rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, Some(a), None, true, false) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: r as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: r as u16 + g as u16,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    sample_infos[3] = SampleInfo {
                        bit_offset: r as u16 + g as u16 + b as u16,
                        bit_length: a - 1,
                        channel_type: RGBSDAChannelType::Alpha.to_raw(),
                        sample_flags: alpha_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, a),
                        sample_upper: max_val(is_float, is_signed, a),
                    };
                    Some(4)
                }
                // all ARGB formats rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, Some(a), None, false, true) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: a - 1,
                        channel_type: RGBSDAChannelType::Alpha.to_raw(),
                        sample_flags: alpha_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, a),
                        sample_upper: max_val(is_float, is_signed, a),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: a as u16,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: a as u16 + b as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[3] = SampleInfo {
                        bit_offset: a as u16 + b as u16 + g as u16,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    Some(4)
                }
                // all ARGB formats reverse rgb ordered
                (_, Some(r), Some(g), Some(b), None, None, Some(a), None, true, true) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: a - 1,
                        channel_type: RGBSDAChannelType::Alpha.to_raw(),
                        sample_flags: alpha_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, a),
                        sample_upper: max_val(is_float, is_signed, a),
                    };
                    sample_infos[1] = SampleInfo {
                        bit_offset: a as u16,
                        bit_length: r - 1,
                        channel_type: RGBSDAChannelType::Red.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, r),
                        sample_upper: max_val(is_float, is_signed, r),
                    };
                    sample_infos[2] = SampleInfo {
                        bit_offset: a as u16 + r as u16,
                        bit_length: g - 1,
                        channel_type: RGBSDAChannelType::Green.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, g),
                        sample_upper: max_val(is_float, is_signed, g),
                    };
                    sample_infos[3] = SampleInfo {
                        bit_offset: a as u16 + r as u16 + g as u16,
                        bit_length: b - 1,
                        channel_type: RGBSDAChannelType::Blue.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, b),
                        sample_upper: max_val(is_float, is_signed, b),
                    };
                    Some(4)
                }

                //
                // SINGLE CHANNEL DEPTH FORMATS
                //

                // all depth formats
                (_, None, None, None, None, Some(d), None, None, _, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: d - 1,
                        channel_type: RGBSDAChannelType::Depth.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, d),
                        sample_upper: max_val(is_float, is_signed, d),
                    };
                    Some(1)
                }

                //
                // SINGLE CHANNEL STENCIL FORMATS
                //

                // all stencil formats
                (_, None, None, None, Some(s), None, None, None, _, _) => {
                    sample_infos[0] = SampleInfo {
                        bit_offset: 0,
                        bit_length: s - 1,
                        channel_type: RGBSDAChannelType::Stencil.to_raw(),
                        sample_flags,
                        sample_positions: [0, 0, 0, 0],
                        sample_lower: min_val(is_float, is_signed, s),
                        sample_upper: max_val(is_float, is_signed, s),
                    };
                    Some(1)
                }

                //
                // SINGLE CHANNEL DEPTH STENCIL FORMATS
                //

                // all stencil formats
                (_, None, None, None, Some(s), Some(d), None, None, _, _) => {
                    if format != VkFormat::D32_SFLOAT_S8_UINT {
                        sample_infos[0] = SampleInfo {
                            bit_offset: 0,
                            bit_length: s - 1,
                            channel_type: RGBSDAChannelType::Stencil.to_raw(),
                            sample_flags,
                            sample_positions: [0, 0, 0, 0],
                            sample_lower: min_val(is_float, is_signed, s),
                            sample_upper: max_val(is_float, is_signed, s),
                        };
                        sample_infos[1] = SampleInfo {
                            bit_offset: s as u16,
                            bit_length: d - 1,
                            channel_type: RGBSDAChannelType::Depth.to_raw(),
                            sample_flags,
                            sample_positions: [0, 0, 0, 0],
                            sample_lower: min_val(is_float, is_signed, d),
                            sample_upper: max_val(is_float, is_signed, d),
                        };
                        Some(2)
                    } else {
                        sample_infos[0] = SampleInfo {
                            bit_offset: 0,
                            bit_length: s - 1,
                            channel_type: RGBSDAChannelType::Stencil.to_raw(),
                            sample_flags,
                            sample_positions: [0, 0, 0, 0],
                            sample_lower: min_val(false, false, s),
                            sample_upper: max_val(false, false, s),
                        };
                        sample_infos[1] = SampleInfo {
                            bit_offset: s as u16,
                            bit_length: d - 1,
                            channel_type: RGBSDAChannelType::Depth.to_raw(),
                            sample_flags,
                            sample_positions: [0, 0, 0, 0],
                            sample_lower: min_val(true, true, d),
                            sample_upper: max_val(true, true, d),
                        };
                        Some(2)
                    }
                }
                (_, _, _, _, _, _, _, _, _, _) => None,
            }
        }
    }
}

///
/// Iterates over the sample information blocks in the DFD block
///
pub struct SampleInfoIterator<'a, R: Read + Seek> {
    reader: &'a mut R,
    index: u16,
    count: u16,
}

impl<'a, R: Read + Seek> SampleInfoIterator<'a, R> {
    ///
    /// Will create an iterator that will yield `count` samples from the reader
    ///
    /// # Info
    ///
    /// There can't be more than `u16::max_value` sample infos because of how the number is stored
    /// in the file so we use a u16 for `count` to make this explicit (also makes the struct small)
    ///
    pub fn from_reader_count(reader: &'a mut R, count: u16) -> Self {
        Self {
            reader,
            index: 0,
            count,
        }
    }
}

impl SampleInfo {
    pub fn compatible_with(&self, other: &Self) -> bool {
        let compat = self.bit_offset == other.bit_offset
            && self.bit_length == other.bit_length
            && self.channel_type == other.channel_type
            && self.sample_positions == other.sample_positions
            && self.sample_lower == other.sample_lower
            && self.sample_upper == other.sample_upper;
        let flags_compat = self.sample_flags.compatible_with(other.sample_flags);
        compat && flags_compat
    }
}

impl<'a, R: Read + Seek> Iterator for SampleInfoIterator<'a, R> {
    type Item = (usize, SampleInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            None
        } else {
            let mut words = [0; 4];
            self.reader.read_u32_into::<LittleEndian>(&mut words).ok()?;

            // Build output and iterate the index
            let out = Some((self.index as usize, SampleInfo::unpack_from(&words)));
            self.index += 1;
            out
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

#[inline]
fn min_val(is_float: bool, is_signed: bool, bits: u8) -> u32 {
    let bits_clamped = u32::min(bits as u32, 32);
    if is_float {
        0xBF800000
    } else if is_signed {
        1 << (bits_clamped - 1)
    } else {
        0
    }
}

#[inline]
fn max_val(is_float: bool, is_signed: bool, bits: u8) -> u32 {
    let bits_clamped = u32::min(bits as u32, 32);
    if is_float {
        0x7F800000
    } else if is_signed {
        !0 ^ (1 << (bits_clamped - 1))
    } else {
        let max = 0xFFFFFFFFu64 << bits_clamped as u64;
        let max = (max & 0xFFFFFFFFu64) as u32;
        max ^ 0xFFFFFFFFu32
    }
}
