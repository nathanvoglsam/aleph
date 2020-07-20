//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod file_index;
mod level_index;
mod super_compression_scheme;

pub use file_index::FileIndex;
pub use level_index::LevelIndex;
pub use super_compression_scheme::SuperCompressionScheme;

use crate::data_format_descriptor::DataFormatDescriptor;
use crate::{DFDError, VkFormat};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Error, Read, Seek};

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

    /// The depth value in `pixelDepth` is not valid for the `vkFormat` specified by the document
    InvalidDepthForFormat(u32, VkFormat),

    /// The depth value in `pixelDepth` must be 0 for cube maps `faceCount == 6`
    InvalidDepthForCubeMap(u32),

    /// `faceCount` must be either 1 or 6 (single image or cube map)
    InvalidFaceCount(u32),

    /// The `levelCount` value specified too many image levels for the size of the image
    TooManyLevels(u32),

    /// The image uses a block format but has specified `layerCount` of 0, which is invalid
    InvalidLevelCountForBlockFormat(u32),

    /// The `kvdByteOffset` must be 0 if `kvdByteLength` is 0, otherwise it is considered an error
    InvalidKeyValueDataOffset(u32),

    /// The `sgdByteOffset` must be 0 if `sgdByteLength` is 0, otherwise it is considered an error
    InvalidSuperCompressionGlobalDataOffset(u64),

    /// If a compression scheme wants global data but none is provided in the file
    CompressionSchemeGlobalDataNotFound(SuperCompressionScheme),

    /// Uncompressed size in a level index was invalid
    InvalidLevelIndexUncompressedSize,

    /// Produced if the document describes an invalid image type
    InvalidDocumentType,

    /// Whether the dimensions in the image are invalid (i.e 0,0,0)
    InvalidImageDimensions,

    /// An error when reading the data format descriptor
    DFDError(DFDError),

    /// An error occurred while reading the data itself
    IOError(std::io::Error),
}

impl From<std::io::Error> for KTXReadError {
    fn from(err: Error) -> Self {
        KTXReadError::IOError(err)
    }
}

impl From<DFDError> for KTXReadError {
    fn from(err: DFDError) -> Self {
        KTXReadError::DFDError(err)
    }
}

///
/// Represents the set of image types a KTX document can describe
///
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DocumentType {
    /// A one dimensional image
    Image1D,

    /// A two dimensional image
    Image2D,

    /// A three dimensional image
    Image3D,

    /// A cube map
    Cube,

    /// An array of one dimensional images
    Array1D,

    /// An array of two dimensional images
    Array2D,

    /// An array of three dimensional images
    Array3D,

    /// An array of cube maps
    CubeArray,
}

pub struct KTXDocument {
    format: VkFormat,
    type_size: u32,
    width: u32,
    height: u32,
    depth: u32,
    layer_num: u32,
    face_num: u32,
    level_num: u32,
    super_compression_scheme: SuperCompressionScheme,
    file_index: FileIndex,
    dfd: DataFormatDescriptor,
    document_type: DocumentType,
    level_indices: Vec<LevelIndex>,
}

impl KTXDocument {
    ///
    /// Gets the Vulkan format of the image this document stores
    ///
    pub fn format(&self) -> VkFormat {
        self.format
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
    pub fn width(&self) -> u32 {
        self.width
    }

    ///
    /// Pixel height of the image
    ///
    pub fn height(&self) -> u32 {
        self.height
    }

    ///
    /// Pixel depth of the image
    ///
    pub fn depth(&self) -> u32 {
        self.depth
    }

    ///
    /// The number of image layers
    ///
    /// # Info
    ///
    /// This is *NOT* the raw `layerCount` from the KTX 2.0 document. This is exactly equal to
    /// `u32::max(1, layerCount)`. A `layerCount` of 0 is a marker value for whether the KTX
    /// document describes an image array but still specifies that the image contains a single
    /// layer.
    ///
    pub fn layer_num(&self) -> u32 {
        u32::max(1, self.layer_num)
    }

    ///
    /// Returns the type of image this KTX document describes
    ///
    pub fn document_type(&self) -> DocumentType {
        self.document_type
    }

    ///
    /// The number of image faces
    ///
    /// # Info
    ///
    /// Should always either be 1, or 6 (1 for single image, 6 for cube map)
    ///
    pub fn face_num(&self) -> u32 {
        self.face_num
    }

    ///
    /// The number of image levels
    ///
    /// # Info
    ///
    /// This is *NOT* the raw `levelCount` from the KTX 2.0 document. This is exactly equal to
    /// `u32::max(1, levelCount)`. A `levelCount` of 0 is a marker value for whether the KTX
    /// document describes an image with a single level, or an image where only a single level has
    /// been provided and the implementation should generate the mip chain itself.
    ///
    pub fn level_num(&self) -> u32 {
        u32::max(1, self.level_num)
    }

    ///
    /// Whether the KTX document asks the loader/user to generate a mip chain based in the single
    /// image level provided by the file.
    ///
    pub fn requests_mip_generation(&self) -> bool {
        self.layer_num == 0
    }

    ///
    /// The super compression scheme the document uses
    ///
    pub fn super_compression_scheme(&self) -> SuperCompressionScheme {
        self.super_compression_scheme
    }
}

impl KTXDocument {
    ///
    /// Creates a new `KTXDocument` from the given reader
    ///
    pub fn from_reader(mut reader: (impl Read + Seek)) -> Result<Self, KTXReadError> {
        Self::validate_file_identifier(&mut reader)?;
        let format = Self::read_vk_format(&mut reader)?;
        let type_size = Self::read_type_size(&mut reader, format)?;
        let dimensions = Self::read_dimensions(&mut reader, format)?;
        let layer_num = Self::read_layer_count(&mut reader)?;
        let face_num = Self::read_face_count(&mut reader, dimensions.2)?;
        let level_num = Self::read_level_count(&mut reader, dimensions, format)?;
        let super_compression_scheme = Self::read_super_compression_scheme(&mut reader)?;
        let file_index = FileIndex::from_reader(&mut reader, super_compression_scheme)?;

        // We always need to read at least one level, even if it says 0 as 0 means 1, but the reader
        // should generate mip levels
        let levels_to_read = u32::max(1, level_num);

        // Preallocate a list to read into
        let mut level_indices = Vec::new();
        for _ in 0..levels_to_read {
            level_indices.push(LevelIndex::from_reader(
                &mut reader,
                layer_num,
                face_num,
                super_compression_scheme,
            )?);
        }

        // Unpack dimensions
        let (width, height, depth) = dimensions;

        // Resolve the image dimensionality
        let is_1d = width > 0 && height == 0 && depth == 0;
        let is_2d = width > 0 && height > 0 && depth == 0;
        let is_3d = width > 0 && height > 0 && depth > 0;
        let dimensionality = if is_1d {
            1
        } else if is_2d {
            2
        } else if is_3d {
            3
        } else {
            panic!("Invalid image dimensions");
        };

        // Resolve if is a cubemap or array and pack into a tuple
        let is_cube = face_num == 6;
        let is_array = layer_num > 0;
        let type_flags = (dimensionality, is_cube, is_array);

        // Deduce the type image type the document describes
        let document_type = match type_flags {
            (1, false, false) => DocumentType::Image1D,
            (2, false, false) => DocumentType::Image2D,
            (3, false, false) => DocumentType::Image3D,
            (2, true, false) => DocumentType::Cube,
            (1, false, true) => DocumentType::Array1D,
            (2, false, true) => DocumentType::Array2D,
            (3, false, true) => DocumentType::Array3D,
            (2, true, true) => DocumentType::CubeArray,
            _ => panic!("Unable to deduce valid document type"),
        };

        let dfd = DataFormatDescriptor::from_reader(&mut reader, &file_index, format)?;

        let out = Self {
            format,
            type_size,
            width,
            height,
            depth,
            layer_num,
            face_num,
            level_num,
            super_compression_scheme,
            file_index,
            dfd,
            document_type,
            level_indices,
        };
        Ok(out)
    }

    ///
    /// Internal function for reading the vk_format
    ///
    fn validate_file_identifier(reader: &mut impl Read) -> Result<(), KTXReadError> {
        // Read off the file identifier
        let mut identifier = [0u8; 12];
        reader.read_exact(&mut identifier)?;

        // A valid KTX 2.0 file must, at the very least, start with the given sequence of bytes
        if identifier != FILE_IDENTIFIER {
            Err(KTXReadError::NotKtxDocument)
        } else {
            Ok(())
        }
    }

    ///
    /// Internal function for reading the vk_format
    ///
    fn read_vk_format(reader: &mut impl Read) -> Result<VkFormat, KTXReadError> {
        // Load and validate the format
        let format = reader.read_u32::<LittleEndian>()?;
        let format = VkFormat(format);

        // Check if format is valid and supported
        if format.is_prohibited() {
            Err(KTXReadError::ProhibitedFormat(format))
        } else if format.is_unsupported() {
            Err(KTXReadError::UnsupportedFormat(format))
        } else {
            Ok(format)
        }
    }

    ///
    /// Internal function for reading the type_size
    ///
    fn read_type_size(reader: &mut impl Read, format: VkFormat) -> Result<u32, KTXReadError> {
        let type_size = reader.read_u32::<LittleEndian>()?;

        if type_size == 1 && !format.is_block_format() {
            Err(KTXReadError::WrongTypeSizeForBlockFormat(type_size, format))
        } else {
            Ok(type_size)
        }
    }

    ///
    /// Internal function for reading the image dimensions
    ///
    fn read_dimensions(
        reader: &mut impl Read,
        format: VkFormat,
    ) -> Result<(u32, u32, u32), KTXReadError> {
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;
        let depth = reader.read_u32::<LittleEndian>()?;

        if format.is_depth_format() && depth != 0 {
            Err(KTXReadError::InvalidDepthForFormat(depth, format))
        } else {
            Ok((width, height, depth))
        }
    }

    ///
    /// Internal function for reading the layer_count
    ///
    fn read_layer_count(reader: &mut impl Read) -> Result<u32, KTXReadError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the face_count
    ///
    fn read_face_count(reader: &mut impl Read, depth: u32) -> Result<u32, KTXReadError> {
        let face_count = reader.read_u32::<LittleEndian>()?;

        if face_count != 1 || face_count != 6 {
            Err(KTXReadError::InvalidFaceCount(face_count))
        } else if face_count == 6 && depth != 0 {
            Err(KTXReadError::InvalidDepthForCubeMap(depth))
        } else {
            Ok(face_count)
        }
    }

    ///
    /// Internal function for reading the layer_count
    ///
    fn read_level_count(
        reader: &mut impl Read,
        dimensions: (u32, u32, u32),
        format: VkFormat,
    ) -> Result<u32, KTXReadError> {
        let level_count = reader.read_u32::<LittleEndian>()?;

        // Select the highest of the three dimensions
        let max_dim = u32::max(dimensions.0, dimensions.1);
        let max_dim = u32::max(max_dim, dimensions.2);

        // Max level is equal to log2 of the highest image dimension
        let max_levels = (max_dim as f64).log2() as u32;

        if level_count > max_levels {
            Err(KTXReadError::TooManyLevels(level_count))
        } else if level_count == 0 && format.is_block_format() {
            Err(KTXReadError::InvalidLevelCountForBlockFormat(level_count))
        } else {
            Ok(level_count)
        }
    }

    ///
    /// Internal function for reading the super_compression_scheme
    ///
    fn read_super_compression_scheme(
        reader: &mut impl Read,
    ) -> Result<SuperCompressionScheme, KTXReadError> {
        let super_compression_scheme = reader.read_u32::<LittleEndian>()?;
        let super_compression_scheme = SuperCompressionScheme(super_compression_scheme);

        if !super_compression_scheme.is_supported() {
            Err(KTXReadError::UnsupportedSuperCompressionScheme(
                super_compression_scheme,
            ))
        } else {
            Ok(super_compression_scheme)
        }
    }
}

///
/// The file identifier that should be at the start of every KTX, byte for byte
///
const FILE_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];
