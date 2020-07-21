//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod channel_type;
mod color_model;
mod color_primaries;
mod flags;
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

use crate::document::FileIndex;
use crate::KTXReadError;
use aleph_vk_format::VkFormat;
use byteorder::{LittleEndian, ReadBytesExt};
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
    /// Reads the data format descriptor from the given reader
    ///
    pub fn from_reader(
        reader: &mut (impl Read + Seek),
        file_index: &FileIndex,
        format: VkFormat,
    ) -> Result<Self, KTXReadError> {
        reader.seek(SeekFrom::Start(file_index.dfd_offset as _))?;

        //
        // Read off the next 7 words of the DFD and unpack them into the different parts
        //

        // Read in the next 7 words.
        let mut words = [0; 7];
        reader.read_u32_into::<LittleEndian>(&mut words)?;

        // Unpack the next word
        let dfd_total_size = words[0];

        // Unpack the next word
        let vendor_id = words[0] & 0x1FFFF;
        let descriptor_type = (words[0] >> 17) as u16;

        // Unpack the next word
        let descriptor_block_size = (words[1] & 0xFFFF) as u16;
        let version_number = (words[1] >> 16) as u16;

        // Unpack the next word
        let color_model = ((words[2] >> 0) & 0xFF) as u8;
        let color_primaries = ((words[2] >> 8) & 0xFF) as u8;
        let transfer_function = ((words[2] >> 16) & 0xFF) as u8;
        let flags = ((words[2] >> 24) & 0xFF) as u8;

        // Unpack the next word
        let texel_block_dimensions_0 = (words[3] >> 0) & 0xFF;
        let texel_block_dimensions_1 = (words[3] >> 8) & 0xFF;
        let texel_block_dimensions_2 = (words[3] >> 16) & 0xFF;
        let texel_block_dimensions_3 = (words[3] >> 24) & 0xFF;
        let block_dimensions = [
            texel_block_dimensions_0,
            texel_block_dimensions_1,
            texel_block_dimensions_2,
            texel_block_dimensions_3,
        ];

        // Unpack byte plane sizes
        let byte_planes_0 = ((words[4] >> 0) & 0xFF) as u8;
        let byte_planes_1 = ((words[4] >> 8) & 0xFF) as u8;
        let byte_planes_2 = ((words[4] >> 16) & 0xFF) as u8;
        let byte_planes_3 = ((words[4] >> 24) & 0xFF) as u8;
        let byte_planes_4 = ((words[5] >> 0) & 0xFF) as u8;
        let byte_planes_5 = ((words[5] >> 8) & 0xFF) as u8;
        let byte_planes_6 = ((words[5] >> 16) & 0xFF) as u8;
        let byte_planes_7 = ((words[5] >> 24) & 0xFF) as u8;
        let byte_planes = [
            byte_planes_0,
            byte_planes_1,
            byte_planes_2,
            byte_planes_3,
            byte_planes_4,
            byte_planes_5,
            byte_planes_6,
            byte_planes_7,
        ];

        //
        // CHECK THE VALUES ARE VALID AFTER READING THEM OUT
        //

        // Assert the dfd_total_size field matches the constraints of the KTX format
        if dfd_total_size != file_index.kvd_offset - file_index.dfd_offset {
            return Err(DFDError::InvalidTotalSize(dfd_total_size).into());
        }

        // Assert the vendor id is 0, which is the Khronos group, we don't support any vendor
        // extensions in this reader.
        if vendor_id != 0 {
            return Err(DFDError::InvalidVendor(vendor_id).into());
        }

        // Assert the descriptor type is 0, the only type we support.
        if descriptor_type != 0 {
            return Err(DFDError::InvalidDescriptorType(descriptor_type).into());
        }

        // We only support DFD version 2 so assert the file is the right version
        if version_number != 2 {
            return Err(DFDError::InvalidVersionNumber(version_number).into());
        }

        // Assert that the file at least specifies a color model that exists
        let color_model =
            ColorModel::from_raw(color_model).ok_or(DFDError::InvalidColorModel(color_model))?;

        // Assert that the file at least specifies a color primaries value that exists
        let color_primaries = ColorPrimaries::from_raw(color_primaries)
            .ok_or(DFDError::InvalidColorPrimaries(color_primaries))?;

        // Assert that the file at least specifies a transfer function that exists
        let transfer_function = TransferFunction::from_raw(transfer_function)
            .ok_or(DFDError::InvalidTransferFunction(transfer_function))?;

        // Assert that the file only specifies valid flags
        let flags: DFDFlags = DFDFlags::from_bits(flags).ok_or(DFDError::InvalidFlags(flags))?;

        // Assert the color model is valid for the format specified by the file earlier
        if !color_model.is_compatible_with_format(format) {
            return Err(DFDError::ColorModelMismatch(format, color_model).into());
        }

        // Assert the transfer function is valid for the format specified by the file earlier
        if !transfer_function.is_compatible_with_format(format) {
            return Err(DFDError::TransferFunctionMismatch(format, transfer_function).into());
        }

        // Assert the DFD block width matches the format specified by the file earlier
        if block_dimensions[0] + 1 != format.block_width() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockWidth(texel_block_dimensions_0).into());
        }

        // Assert the DFD block height matches the format specified by the file earlier
        if block_dimensions[1] + 1 != format.block_height() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockHeight(texel_block_dimensions_0).into());
        }

        // There's no image formats known to Vulkan with a 3D block size so this should always be 1
        // but there ARE ASTC formats with 3D block sizes so we're at least prepared for when they
        // become available
        //
        // Long story short, assert that the blocks depth matches the format specified earlier by
        // the file
        if block_dimensions[2] + 1 != format.block_depth() {
            // + 1 to decode value
            return Err(DFDError::InvalidBlockDepth(texel_block_dimensions_2).into());
        }

        // If we're far enough in the future to be using a 4D block size we've probably transcended
        // our physical forms so I think it's safe to just call it an error and not worry about it.
        //
        // Long story short, assert that the blocks W dimension is 1
        if block_dimensions[3] + 1 != 1 {
            // + 1 to decode value
            return Err(DFDError::InvalidBlock4thDimension(texel_block_dimensions_2).into());
        }

        // Only single plane images are supported by KTX so bytePlanes1-7 should be 0
        for (i, val) in byte_planes[1..].iter().enumerate() {
            if *val != 0 {
                return Err(DFDError::InvalidBytePlaneSize(i as u8, *val).into());
            }
        }

        // Derive the sample count so we can iterate over the DFD sample descriptions
        let sample_count = (descriptor_block_size - 24) / 16;
        for (index, sample) in SampleInfoIterator::from_reader_count(reader, sample_count) {}

        Ok(Self {
            flags,
            color_primaries,
        })
    }
}
