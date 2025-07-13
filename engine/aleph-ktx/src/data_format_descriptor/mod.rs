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
mod sample_info;
mod transfer_function;

use std::io::{Read, Seek, SeekFrom};

use aleph_vk_format::VkFormat;
use aleph_vk2dfd::{LONGEST_DFD, vk2dfd};
use byteorder::{LittleEndian, ReadBytesExt};
pub use channel_type::{
    ASTCChannelType, BC1ChannelType, BC2ChannelType, BC3ChannelType, BC4ChannelType,
    BC5ChannelType, BC6ChannelType, BC7ChannelType, ETC1ChannelType, ETC1SChannelType,
    ETC2ChannelType, PVRTC2ChannelType, PVRTCChannelType, RGBSDAChannelType,
};
pub use color_model::ColorModel;
pub use color_primaries::ColorPrimaries;
pub use flags::{DFDFlags, SampleFlags};
pub use sample_info::{SampleInfo, SampleInfoIterator};
use thiserror::Error;
pub use transfer_function::TransferFunction;

use crate::document::FileIndex;
use crate::{KtxReadError, SuperCompressionScheme};

///
/// The set of errors that can be produced when reading the DFD section of a ktx file
///
#[derive(Error, PartialEq, Eq, Debug)]
pub enum DFDError {
    #[error("The dfd vendor id is non zero (we don't support any vendor extensions). Got '{0}'.")]
    InvalidVendor(u32),

    #[error("The dfd descriptor type is non zero (we don't support any other types). Got '{0}'.")]
    InvalidDescriptorType(u16),

    #[error("The `dfdTotalSize` is invalid. Got '{0}'.")]
    InvalidTotalSize(u32),

    #[error("The `versionNumber` is not 2 (the only DFD version we support). Got '{0}'.")]
    InvalidVersionNumber(u16),

    #[error("The `flags` value is invalid. Got '{0}'.")]
    InvalidFlags(u8),

    #[error("The `transferFunction` value is invalid. Got '{0}'.")]
    InvalidTransferFunction(u8),

    #[error("The `transferFunction` value is invalid. Got '{0}'.")]
    InvalidColorModel(u8),

    #[error("The `transferFunction` value is invalid. Got '{0}'.")]
    InvalidColorPrimaries(u8),

    #[error("The descriptor does not match what the KTX's declare VkFormat claims it should be")]
    DescriptorFormatMismatch,

    #[error(
        "The `colorModel` value is not of the expected value for the format specified earlier. Got '{0}, {1}'."
    )]
    ColorModelMismatch(VkFormat, ColorModel),

    #[error(
        "The `transferFunction` value was not compatible with the format specified earlier. Got '{0}, {1}'."
    )]
    TransferFunctionMismatch(VkFormat, TransferFunction),

    #[error("The block width (`texelBlockDimension0`) was invalid. Got '{0}'.")]
    InvalidBlockWidth(u32),

    #[error("The block height (`texelBlockDimension1`) was invalid. Got '{0}'.")]
    InvalidBlockHeight(u32),

    #[error("The block depth (`texelBlockDimension2`) was invalid. Got '{0}'.")]
    InvalidBlockDepth(u32),

    #[error("The block 4th dimension (`texelBlockDimension3`) was invalid. Got '{0}'.")]
    InvalidBlock4thDimension(u32),
}

///
/// Struct for unpacking and validating the data format descriptor in a KTX document
///
pub(crate) struct DataFormatDescriptor {
    pub flags: DFDFlags,
    pub color_primaries: ColorPrimaries,
    pub transfer_function: TransferFunction,
}

impl DataFormatDescriptor {
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
        format: VkFormat,
        super_compression_scheme: SuperCompressionScheme,
    ) -> Result<Self, KtxReadError> {
        assert!(super_compression_scheme.is_known());
        assert!(super_compression_scheme.is_supported());

        reader.seek(SeekFrom::Start(file_index.dfd_offset as _))?;

        let expected_dfd_bytes = file_index.kvd_offset - file_index.dfd_offset;
        let num_words = expected_dfd_bytes as usize / 4;
        let clamped_num_words = num_words.min(LONGEST_DFD); // number of words to read
        let skip_num_words = num_words - clamped_num_words; // number of words to skip

        // Truncate the DFD. We know the length of the longest DFD we know how to handle as we
        // have enumarated all our supported VkFormats.
        let mut stack_space = [0u32; LONGEST_DFD];
        let dfd_words = &mut stack_space[0..(expected_dfd_bytes as usize / 4)];
        reader.read_u32_into::<LittleEndian>(dfd_words)?;
        reader.seek(SeekFrom::Current(skip_num_words as i64))?;

        assert!(dfd_words.len() >= 7);

        // Undefined format we just pull the color space info and validate the size is correct
        // as there's not much else I want to bother implementing
        let dfd = RawDataFormatDescriptor::from_words(&dfd_words)?;
        let dfd = dfd.validate()?;

        // Assert the dfd_total_size field matches the constraints of the KTX format
        if dfd.dfd_total_size != file_index.kvd_offset - file_index.dfd_offset {
            return Err(DFDError::InvalidTotalSize(dfd.dfd_total_size).into());
        }

        if format.is_known() && format != VkFormat::UNDEFINED {
            // We should've already checked the format is valid so we panic here instead.
            let expected_words = vk2dfd(format.0).unwrap();
            let expected = RawDataFormatDescriptor::from_words(&expected_words).unwrap();
            assert!(expected_words.len() >= 7);

            if dfd_words.len() != expected_words.len() {
                // Length must match
                return Err(DFDError::DescriptorFormatMismatch.into());
            }

            if &dfd_words[0..3] != &expected_words[0..3] {
                // We only validate the first few words of the DFD as they're basic structural parts
                // of the DFD. The actual DFD rules are very complex. I've simply resigned to not
                // bothering as the DFD is useless for our purposes anyway
                return Err(DFDError::DescriptorFormatMismatch.into());
            }

            // dfd_words[4] should always match vk2dfd
            if dfd_words[4] != expected_words[4] {
                return Err(DFDError::DescriptorFormatMismatch.into());
            }

            // word[3] we validate more explicitly below, but the rest we ignore. We don't actually
            // care to validate it because without a VkFormat we're hosed anyway.

            // Assert the transfer function is valid for the format specified by the file earlier
            if !dfd.transfer_function.is_compatible_with_format(format) {
                return Err(
                    DFDError::TransferFunctionMismatch(format, dfd.transfer_function).into(),
                );
            }

            // Assert the color model is valid for the format specified by the file earlier
            if dfd.color_model.into_raw() != expected.color_model {
                return Err(DFDError::ColorModelMismatch(format, dfd.color_model).into());
            }

            // Super compressed formats must have all bytePlanes set to 0 size
            let expected_byte_planes = match super_compression_scheme {
                SuperCompressionScheme::NONE => [expected_words[5], expected_words[6]],
                _ => [0, 0],
            };

            if dfd_words[5..7] != expected_byte_planes {
                eprintln!("g {:?}", dfd_words);
                eprintln!("e {:?}", expected_words);
                return Err(DFDError::DescriptorFormatMismatch.into());
            }

            // let got_samples = SampleInfoIterator::from_words(&dfd_words[7..]);
            // let exp_samples = SampleInfoIterator::from_words(&expected_words[7..]);
            //
            // let mut failed = false;
            // for (g, e) in got_samples.zip(exp_samples) {
            //     if g != e || failed {
            //         failed = true;
            //         eprintln!("g {:?}", g);
            //         eprintln!("e {:?}", e);
            //     }
            // }
            // if failed {
            //     return Err(DFDError::DescriptorFormatMismatch.into());
            // }
        }

        Ok(Self {
            flags: dfd.flags,
            color_primaries: dfd.color_primaries,
            transfer_function: dfd.transfer_function,
        })
    }
}

struct ValidatedDataFormatDescriptor {
    dfd_total_size: u32,
    _vendor_id: u32,
    _descriptor_type: u16,
    _version_number: u16,
    _descriptor_block_size: u16,
    color_model: ColorModel,
    color_primaries: ColorPrimaries,
    transfer_function: TransferFunction,
    flags: DFDFlags,
}

struct RawDataFormatDescriptor {
    dfd_total_size: u32,
    vendor_id: u32,
    descriptor_type: u16,
    version_number: u16,
    _descriptor_block_size: u16,
    color_model: u8,
    color_primaries: u8,
    transfer_function: u8,
    flags: u8,
}

impl RawDataFormatDescriptor {
    fn from_words(words: &[u32]) -> Result<Self, KtxReadError> {
        assert!(words.len() >= 4);

        // Unpack the next word
        let dfd_total_size = words[0];

        // Unpack the next word
        let vendor_id = words[1] & 0x1FFFF;
        let descriptor_type = (words[1] >> 17) as u16;

        // Unpack the next word
        let version_number = (words[2] & 0xFFFF) as u16;
        let _descriptor_block_size = (words[2] >> 16) as u16;

        // Unpack the next word
        let color_model = ((words[3] >> 0) & 0xFF) as u8;
        let color_primaries = ((words[3] >> 8) & 0xFF) as u8;
        let transfer_function = ((words[3] >> 16) & 0xFF) as u8;
        let flags = ((words[3] >> 24) & 0xFF) as u8;

        Ok(Self {
            dfd_total_size,
            vendor_id,
            descriptor_type,
            version_number,
            _descriptor_block_size,
            color_model,
            color_primaries,
            transfer_function,
            flags,
        })
    }

    fn validate(self) -> Result<ValidatedDataFormatDescriptor, KtxReadError> {
        if self.vendor_id != 0 {
            return Err(DFDError::InvalidVendor(self.vendor_id).into());
        }

        if self.descriptor_type != 0 {
            return Err(DFDError::InvalidDescriptorType(self.descriptor_type).into());
        }

        if self.version_number != 2 {
            return Err(DFDError::InvalidVersionNumber(self.version_number).into());
        }

        let color_model = ColorModel::from_raw(self.color_model)
            .ok_or(DFDError::InvalidColorModel(self.color_model))?;

        let color_primaries = ColorPrimaries::from_raw(self.color_primaries)
            .ok_or(DFDError::InvalidColorPrimaries(self.color_primaries))?;

        let transfer_function = TransferFunction::from_raw(self.transfer_function)
            .ok_or(DFDError::InvalidTransferFunction(self.transfer_function))?;

        let flags = DFDFlags::from_bits(self.flags).ok_or(DFDError::InvalidFlags(self.flags))?;

        Ok(ValidatedDataFormatDescriptor {
            dfd_total_size: self.dfd_total_size,
            _vendor_id: self.vendor_id,
            _descriptor_type: self.descriptor_type,
            _version_number: self.version_number,
            _descriptor_block_size: self._descriptor_block_size,
            color_model,
            color_primaries,
            transfer_function,
            flags,
        })
    }
}
