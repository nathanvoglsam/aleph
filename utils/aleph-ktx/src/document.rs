//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{SuperCompressionScheme, VkFormat};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Error, Read};

///
/// Represents the set of errors that could occur when trying to pass/read a ktx file from a stream
/// of bytes
///
#[derive(Debug)]
pub enum KTXReadError {
    /// This error is produced when the stream of bytes does not contain a ktx file. This will
    /// usually be thrown if file identifier is invalid
    NotKtxDocument,

    /// The KTX file tried to specify a prohibited format
    ProhibitedFormat(VkFormat),

    /// The KTX file specified a format this implementation doesn't support
    UnsupportedFormat(VkFormat),

    /// The KTX file specified a super compression scheme that this implementation doesn't support
    UnsupportedSuperCompressionScheme(SuperCompressionScheme),

    /// The `typeSize` value != 1 when `vkFormat` was a block format as required by the spec
    WrongTypeSizeForBlockFormat(u32, VkFormat),

    /// An error occurred while reading the data itself
    IOError(std::io::Error),
}

impl From<std::io::Error> for KTXReadError {
    fn from(err: Error) -> Self {
        KTXReadError::IOError(err)
    }
}

pub struct KTXDocument {
    vk_format: VkFormat,
    type_size: u32,
    pixel_width: u32,
    pixel_height: u32,
    pixel_depth: u32,
    layer_count: u32,
    face_count: u32,
    level_count: u32,
    super_compression_scheme: SuperCompressionScheme,
}

impl KTXDocument {
    ///
    /// Gets the Vulkan format of the image this document stores
    ///
    pub fn format(&self) -> VkFormat {
        self.vk_format
    }

    ///
    /// The size of a single texel component
    ///
    pub fn type_size(&self) -> u32 {
        self.type_size
    }

    ///
    /// Pixel width of the image
    ///
    pub fn pixel_width(&self) -> u32 {
        self.pixel_width
    }

    ///
    /// Pixel height of the image
    ///
    pub fn pixel_height(&self) -> u32 {
        self.pixel_height
    }

    ///
    /// Pixel depth of the image
    ///
    pub fn pixel_depth(&self) -> u32 {
        self.pixel_depth
    }

    ///
    /// The number of image layers
    ///
    pub fn layout_count(&self) -> u32 {
        self.layer_count
    }

    ///
    /// The number of image faces
    ///
    /// # Info
    ///
    /// Should always either be 1, or 6 (1 for single image, 6 for cube map)
    ///
    pub fn face_count(&self) -> u32 {
        self.face_count
    }

    ///
    /// The number of image levels
    ///
    pub fn level_count(&self) -> u32 {
        self.level_count
    }

    ///
    /// The super compression scheme the document uses
    ///
    pub fn super_compression_scheme(&self) -> SuperCompressionScheme {
        self.super_compression_scheme
    }
}

impl KTXDocument {
    pub fn from_reader(mut reader: impl Read) -> Result<Self, KTXReadError> {
        // Read off the file identifier
        let mut identifier = [0u8; 12];
        reader.read_exact(&mut identifier)?;

        // A valid KTX 2.0 file must, at the very least, start with the given sequence of bytes
        if identifier != FILE_IDENTIFIER {
            return Err(KTXReadError::NotKtxDocument);
        }

        let vk_format = Self::read_vk_format(&mut reader)?;
        let type_size = Self::read_type_size(&mut reader, vk_format)?;

        let pixel_width = reader.read_u32::<LittleEndian>()?;
        let pixel_height = reader.read_u32::<LittleEndian>()?;
        let pixel_depth = reader.read_u32::<LittleEndian>()?;
        let layer_count = reader.read_u32::<LittleEndian>()?;
        let face_count = reader.read_u32::<LittleEndian>()?;
        let level_count = reader.read_u32::<LittleEndian>()?;

        // Load and validate the super compression scheme
        let super_compression_scheme = reader.read_u32::<LittleEndian>()?;
        let super_compression_scheme = SuperCompressionScheme(super_compression_scheme);

        if !super_compression_scheme.is_supported() {
            return Err(KTXReadError::UnsupportedSuperCompressionScheme(
                super_compression_scheme,
            ));
        }

        let out = Self {
            vk_format,
            type_size,
            pixel_width,
            pixel_height,
            pixel_depth,
            layer_count,
            face_count,
            level_count,
            super_compression_scheme,
        };
        Ok(out)
    }

    ///
    /// Internal function for reading the vk_format
    ///
    fn read_vk_format(reader: &mut impl Read) -> Result<VkFormat, KTXReadError> {
        // Load and validate the format
        let vk_format = reader.read_u32::<LittleEndian>()?;
        let vk_format = VkFormat(vk_format);

        // Check if format is valid and supported
        if vk_format.is_prohibited() {
            Err(KTXReadError::ProhibitedFormat(vk_format))
        } else if vk_format.is_unsupported() {
            Err(KTXReadError::UnsupportedFormat(vk_format))
        } else {
            Ok(vk_format)
        }
    }

    ///
    /// Internal function for reading the type_size
    ///
    fn read_type_size(reader: &mut impl Read, vk_format: VkFormat) -> Result<u32, KTXReadError> {
        let type_size = reader.read_u32::<LittleEndian>()?;

        if type_size == 1 && !vk_format.is_block_format() {
            Err(KTXReadError::WrongTypeSizeForBlockFormat(type_size, vk_format))
        } else {
            Ok(type_size)
        }
    }
}

///
/// The file identifier that should be at the start of every KTX, byte for byte
///
const FILE_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
