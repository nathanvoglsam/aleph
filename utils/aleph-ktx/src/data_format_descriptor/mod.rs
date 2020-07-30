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

mod channel_type;
mod color_model;
mod color_primaries;
mod flags;
mod raw;
mod sample_info;
mod transfer_function;

pub use channel_type::ASTCChannelType;
pub use channel_type::BC1ChannelType;
pub use channel_type::BC2ChannelType;
pub use channel_type::BC3ChannelType;
pub use channel_type::BC4ChannelType;
pub use channel_type::BC5ChannelType;
pub use channel_type::BC6ChannelType;
pub use channel_type::BC7ChannelType;
pub use channel_type::ETC1ChannelType;
pub use channel_type::ETC1SChannelType;
pub use channel_type::ETC2ChannelType;
pub use channel_type::PVRTC2ChannelType;
pub use channel_type::PVRTCChannelType;
pub use channel_type::RGBSDAChannelType;
pub use color_model::ColorModel;
pub use color_primaries::ColorPrimaries;
pub use flags::DFDFlags;
pub use flags::SampleFlags;
pub use sample_info::SampleInfo;
pub use sample_info::SampleInfoIterator;
pub use transfer_function::TransferFunction;

use crate::data_format_descriptor::raw::RawDataFormatDescriptor;
use crate::document::FileIndex;
use crate::{format_bytes_per_block, KTXReadError};
use aleph_vk_format::VkFormat;
use std::io::{Read, Seek, SeekFrom};

///
/// The set of errors that can be produced when reading the DFD section of a ktx file
///
#[derive(Debug)]
pub enum DFDError {
    /// The dfd vendor id is non zero (we don't support any vendor extensions)
    InvalidVendor(u32),

    /// The dfd descriptor type is non zero (we don't support any other types)
    InvalidDescriptorType(u16),

    /// The `dfdTotalSize` is invalid
    InvalidTotalSize(u32),

    /// The `versionNumber` is not 2 (the only DFD version we support)
    InvalidVersionNumber(u16),

    /// The `flags` value is invalid
    InvalidFlags(u8),

    /// The `transferFunction` value is invalid
    InvalidTransferFunction(u8),

    /// The `transferFunction` value is invalid
    InvalidColorModel(u8),

    /// The `transferFunction` value is invalid
    InvalidColorPrimaries(u8),

    /// The `colorModel` value is not of the expected value for the format specified earlier
    ColorModelMismatch(VkFormat, ColorModel),

    /// The `transferFunction` value was not compatible with the format specified earlier
    TransferFunctionMismatch(VkFormat, TransferFunction),

    /// If the block width (`texelBlockDimension0`) was invalid
    InvalidBlockWidth(u32),

    /// If the block height (`texelBlockDimension1`) was invalid
    InvalidBlockHeight(u32),

    /// If the block depth (`texelBlockDimension2`) was invalid
    InvalidBlockDepth(u32),

    /// If the block 4th dimension (`texelBlockDimension3`) was invalid
    InvalidBlock4thDimension(u32),

    /// If one of the `bytePlane` values is invalid, (byte plane index, byte plane value)
    InvalidBytePlaneSize(u8, u8),

    /// If the sample information block itself is incompatible with the earlier stated format
    InvalidSampleInfo,

    /// If one of the sample information blocks in the DFD was invalid for the earlier stated format
    IncorrectSampleInfo(SampleInfo, SampleInfo),
}

///
/// Struct for unpacking and validating the data format descriptor in a KTX document
///
pub struct DataFormatDescriptor {
    pub flags: DFDFlags,
    pub color_primaries: ColorPrimaries,
}

impl DataFormatDescriptor {
    ///
    /// Reads the data format descriptor from the given reader, validating that the DFD values match
    /// what what the given VkFormat expects.
    ///
    /// # Panics
    ///
    /// Will panic if `format` is `VkFormat::UNDEFINED`
    ///
    fn validate_against_format(
        reader: &mut (impl Read + Seek),
        raw_dfd: RawDataFormatDescriptor,
        file_index: &FileIndex,
        format: VkFormat,
    ) -> Result<Self, KTXReadError> {
        assert_ne!(format, VkFormat::UNDEFINED);

        //
        // CHECK THE VALUES ARE VALID AFTER READING THEM OUT
        //

        // Assert the dfd_total_size field matches the constraints of the KTX format
        if raw_dfd.dfd_total_size != file_index.kvd_offset - file_index.dfd_offset {
            return Err(DFDError::InvalidTotalSize(raw_dfd.dfd_total_size).into());
        }

        // Assert the vendor id is 0, which is the Khronos group, we don't support any vendor
        // extensions in this reader.
        if raw_dfd.vendor_id != 0 {
            return Err(DFDError::InvalidVendor(raw_dfd.vendor_id).into());
        }

        // Assert the descriptor type is 0, the only type we support.
        if raw_dfd.descriptor_type != 0 {
            return Err(DFDError::InvalidDescriptorType(raw_dfd.descriptor_type).into());
        }

        // We only support DFD version 2 so assert the file is the right version
        if raw_dfd.version_number != 2 {
            return Err(DFDError::InvalidVersionNumber(raw_dfd.version_number).into());
        }

        // Assert that the file at least specifies a color model that exists
        let color_model = ColorModel::from_raw(raw_dfd.color_model)
            .ok_or(DFDError::InvalidColorModel(raw_dfd.color_model))?;

        // Assert that the file at least specifies a color primaries value that exists
        let color_primaries = ColorPrimaries::from_raw(raw_dfd.color_primaries)
            .ok_or(DFDError::InvalidColorPrimaries(raw_dfd.color_primaries))?;

        // Assert that the file at least specifies a transfer function that exists
        let transfer_function = TransferFunction::from_raw(raw_dfd.transfer_function)
            .ok_or(DFDError::InvalidTransferFunction(raw_dfd.transfer_function))?;

        // Assert that the file only specifies valid flags
        let flags: DFDFlags =
            DFDFlags::from_bits(raw_dfd.flags).ok_or(DFDError::InvalidFlags(raw_dfd.flags))?;

        // Assert the color model is valid for the format specified by the file earlier
        if !color_model.is_compatible_with_format(format) {
            return Err(DFDError::ColorModelMismatch(format, color_model).into());
        }

        // Assert the transfer function is valid for the format specified by the file earlier
        if !transfer_function.is_compatible_with_format(format) {
            return Err(DFDError::TransferFunctionMismatch(format, transfer_function).into());
        }

        // Assert the DFD block width matches the format specified by the file earlier
        if raw_dfd.block_dimensions[0] as u32 + 1 != format.block_width() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockWidth(raw_dfd.block_dimensions[0] as _).into());
        }

        // Assert the DFD block height matches the format specified by the file earlier
        if raw_dfd.block_dimensions[1] as u32 + 1 != format.block_height() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockHeight(raw_dfd.block_dimensions[1] as _).into());
        }

        // There's no image formats known to Vulkan with a 3D block size so this should always be 1
        // but there ARE ASTC formats with 3D block sizes so we're at least prepared for when they
        // become available
        //
        // Long story short, assert that the blocks depth matches the format specified earlier by
        // the file
        if raw_dfd.block_dimensions[2] as u32 + 1 != format.block_depth() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockDepth(raw_dfd.block_dimensions[2] as _).into());
        }

        // If we're far enough in the future to be using a 4D block size we've probably transcended
        // our physical forms so I think it's safe to just call it an error and not worry about it.
        //
        // Long story short, assert that the blocks W dimension is 1
        if raw_dfd.block_dimensions[3] + 1 != 1 {
            // + 1 to decode value
            return Err(
                DFDError::InvalidBlock4thDimension(raw_dfd.block_dimensions[3] as _).into(),
            );
        }

        // Only single plane images are supported by KTX so bytePlanes1-7 should be 0
        for (i, val) in raw_dfd.byte_planes[1..].iter().enumerate() {
            if *val != 0 {
                return Err(DFDError::InvalidBytePlaneSize(i as u8, *val).into());
            }
        }

        // The value of byte plane 0 must match the size of a single block for the format
        if raw_dfd.byte_planes[0] != format_bytes_per_block(format).unwrap() {
            return Err(DFDError::InvalidBytePlaneSize(1, raw_dfd.byte_planes[0]).into());
        }

        // Derive the sample count so we can iterate over the DFD sample descriptions
        let sample_count = (raw_dfd.descriptor_block_size - 24) / 16;

        // Get the sample infos expected for the format
        let mut sample_infos: [SampleInfo; 8] = Default::default();
        let needed_infos = SampleInfo::for_format(format, &mut sample_infos).unwrap();

        // Check the lengths are equal
        if needed_infos != sample_count as usize {
            return Err(DFDError::InvalidSampleInfo.into());
        }

        // Make sure all sample info DFD stuff matches
        for (index, sample) in SampleInfoIterator::from_reader_count(reader, sample_count) {
            if !sample.compatible_with(&sample_infos[index]) {
                return Err(
                    DFDError::IncorrectSampleInfo(sample, sample_infos[index].clone()).into(),
                );
            }
        }

        Ok(Self {
            flags,
            color_primaries,
        })
    }

    ///
    /// Reads the data format descriptor from the given reader.
    ///
    /// Takes a mutable reference to a format which *may* be written into if the format needs to be
    /// deduced from the DFD.
    ///
    /// If the existing value of `format` is `VkFormat::UNDEFINED` then the value of format will be
    /// set to a VkFormat value deduced from the DFD.
    ///
    pub fn from_reader(
        reader: &mut (impl Read + Seek),
        file_index: &FileIndex,
        format: &mut VkFormat,
    ) -> Result<Self, KTXReadError> {
        reader.seek(SeekFrom::Start(file_index.dfd_offset as _))?;

        let raw_dfd = RawDataFormatDescriptor::from_reader(reader)?;

        if *format == VkFormat::UNDEFINED {
            unimplemented!()
        } else {
            Self::validate_against_format(reader, raw_dfd, file_index, *format)
        }
    }
}
