//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::data_format_descriptor::{
    ASTCChannelType, BC1ChannelType, BC2ChannelType, BC3ChannelType, BC4ChannelType,
    BC5ChannelType, BC6ChannelType, BC7ChannelType, ETC1ChannelType, ETC1SChannelType,
    ETC2ChannelType, PVRTC2ChannelType, PVRTCChannelType, SampleFlags,
};
use crate::format::{format_has_alpha, is_format_alpha_first_ordered, is_format_rgb_ordered};
use crate::{format_sample_info_count, is_format_prohibited, is_format_unsupported};
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

        let sample_upper = words[2];
        let sample_lower = words[3];

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
    pub fn for_non_block_format(
        format: VkFormat,
        sample_infos: &mut [SampleInfo],
    ) -> Option<usize> {
        if format.is_block_format() || is_format_unsupported(format) || is_format_prohibited(format)
        {
            None
        } else {
            let has_alpha = format_has_alpha(format);
            let is_rgb_ordered = is_format_rgb_ordered(format);
            let alpha_first = is_format_alpha_first_ordered(format);
            let count = format_sample_info_count(format)?;
            let is_float = format.is_floating_point();
            let is_signed = format.is_signed();
            let is_depth = format.is_depth_format();
            let is_stencil = format.is_stencil_format();
            let is_normalized = format.is_normalized();

            let flags: SampleFlags = match (is_float, is_signed, is_normalized) {
                (true, _, true) => return None, // Doesn't make sense (float + norm?)
                (true, true, false) => SampleFlags::FLOAT | SampleFlags::SIGNED,
                (true, false, false) => SampleFlags::FLOAT,
                (_, true, _) => SampleFlags::SIGNED,
                (_, _, _) => SampleFlags::empty(),
            };

            // TODO: Function for getting bit width of RGBA channels + depth and stencil channels
            // TODO: Tick off the list of formats below and make sure we can produce a DFD for every
            //       supported format
            // TODO: Maybe special case any oddball formats rather than making the pattern too
            //       complex

            // Check there's enough space to write the sample infos
            if (count as usize) < sample_infos.len() {
                return None;
            }

            match (has_alpha, alpha_first, is_rgb_ordered, count) {
                (false, _, _, 1) => None,
                (_, _, _, _) => None,
            }

            //VkFormat::R4G4_UNORM_PACK8,
            //VkFormat::R4G4B4A4_UNORM_PACK16,
            //VkFormat::B4G4R4A4_UNORM_PACK16,
            //VkFormat::R5G6B5_UNORM_PACK16,
            //VkFormat::B5G6R5_UNORM_PACK16,
            //VkFormat::R5G5B5A1_UNORM_PACK16,
            //VkFormat::B5G5R5A1_UNORM_PACK16,
            //VkFormat::A1R5G5B5_UNORM_PACK16,
            //VkFormat::R8_UNORM,
            //VkFormat::R8_SNORM,
            //VkFormat::R8_UINT,
            //VkFormat::R8_SINT,
            //VkFormat::R8_SRGB,
            //VkFormat::R8G8_UNORM,
            //VkFormat::R8G8_SNORM,
            //VkFormat::R8G8_UINT,
            //VkFormat::R8G8_SINT,
            //VkFormat::R8G8_SRGB,
            //VkFormat::R8G8B8_UNORM,
            //VkFormat::R8G8B8_SNORM,
            //VkFormat::R8G8B8_UINT,
            //VkFormat::R8G8B8_SINT,
            //VkFormat::R8G8B8_SRGB,
            //VkFormat::B8G8R8_UNORM,
            //VkFormat::B8G8R8_SNORM,
            //VkFormat::B8G8R8_UINT,
            //VkFormat::B8G8R8_SINT,
            //VkFormat::B8G8R8_SRGB,
            //VkFormat::R8G8B8A8_UNORM,
            //VkFormat::R8G8B8A8_SNORM,
            //VkFormat::R8G8B8A8_UINT,
            //VkFormat::R8G8B8A8_SINT,
            //VkFormat::R8G8B8A8_SRGB,
            //VkFormat::B8G8R8A8_UNORM,
            //VkFormat::B8G8R8A8_SNORM,
            //VkFormat::B8G8R8A8_UINT,
            //VkFormat::B8G8R8A8_SINT,
            //VkFormat::B8G8R8A8_SRGB,
            //VkFormat::A2R10G10B10_UNORM_PACK32,
            //VkFormat::A2R10G10B10_SNORM_PACK32,
            //VkFormat::A2R10G10B10_UINT_PACK32,
            //VkFormat::A2R10G10B10_SINT_PACK32,
            //VkFormat::A2B10G10R10_UNORM_PACK32,
            //VkFormat::A2B10G10R10_SNORM_PACK32,
            //VkFormat::A2B10G10R10_UINT_PACK32,
            //VkFormat::A2B10G10R10_SINT_PACK32,
            //VkFormat::R16_UNORM,
            //VkFormat::R16_SNORM,
            //VkFormat::R16_UINT,
            //VkFormat::R16_SINT,
            //VkFormat::R16_SFLOAT,
            //VkFormat::R16G16_UNORM,
            //VkFormat::R16G16_SNORM,
            //VkFormat::R16G16_UINT,
            //VkFormat::R16G16_SINT,
            //VkFormat::R16G16_SFLOAT,
            //VkFormat::R16G16B16_UNORM,
            //VkFormat::R16G16B16_SNORM,
            //VkFormat::R16G16B16_UINT,
            //VkFormat::R16G16B16_SINT,
            //VkFormat::R16G16B16_SFLOAT,
            //VkFormat::R16G16B16A16_UNORM,
            //VkFormat::R16G16B16A16_SNORM,
            //VkFormat::R16G16B16A16_UINT,
            //VkFormat::R16G16B16A16_SINT,
            //VkFormat::R16G16B16A16_SFLOAT,
            //VkFormat::R32_UINT,
            //VkFormat::R32_SINT,
            //VkFormat::R32_SFLOAT,
            //VkFormat::R32G32_UINT,
            //VkFormat::R32G32_SINT,
            //VkFormat::R32G32_SFLOAT,
            //VkFormat::R32G32B32_UINT,
            //VkFormat::R32G32B32_SINT,
            //VkFormat::R32G32B32_SFLOAT,
            //VkFormat::R32G32B32A32_UINT,
            //VkFormat::R32G32B32A32_SINT,
            //VkFormat::R32G32B32A32_SFLOAT,
            //VkFormat::R64_UINT,
            //VkFormat::R64_SINT,
            //VkFormat::R64_SFLOAT,
            //VkFormat::R64G64_UINT,
            //VkFormat::R64G64_SINT,
            //VkFormat::R64G64_SFLOAT,
            //VkFormat::R64G64B64_UINT,
            //VkFormat::R64G64B64_SINT,
            //VkFormat::R64G64B64_SFLOAT,
            //VkFormat::R64G64B64A64_UINT,
            //VkFormat::R64G64B64A64_SINT,
            //VkFormat::R64G64B64A64_SFLOAT,
            //VkFormat::B10G11R11_UFLOAT_PACK32,
            //VkFormat::E5B9G9R9_UFLOAT_PACK32,
            //VkFormat::D16_UNORM,
            //VkFormat::D32_SFLOAT,
            //VkFormat::S8_UINT,
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
