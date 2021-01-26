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

mod file_index;
mod level_index;
mod super_compression_scheme;

pub use file_index::FileIndex;
pub use level_index::LevelIndex;
pub use super_compression_scheme::SuperCompressionScheme;

use crate::data_format_descriptor::DataFormatDescriptor;
use crate::format::{is_format_prohibited, is_format_unsupported};
use crate::{format_bytes_for_image, format_type_size, ColorPrimaries, DFDError, DFDFlags};
use aleph_vk_format::VkFormat;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::cell::Cell;

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

    /// The width value in `pixelWidth` is not valid, this must always hold true: `pixelWidth > 0`
    InvalidWidth(u32),

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

    /// An error occurred when trying to read a key's name from the file which lead to the read
    /// over-running the declared size of the pair. Likely a missing null terminator
    InvalidKeyMissingNullTerminator,

    /// An error occurred while reading the data itself
    IOError(std::io::Error),

    /// An error occured with something to do with unicode
    FromUtf8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for KTXReadError {
    fn from(err: std::io::Error) -> Self {
        KTXReadError::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for KTXReadError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        KTXReadError::FromUtf8Error(err)
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

pub struct KTXDocument<R: Read + Seek> {
    reader: Cell<Option<R>>,
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

impl<R: Read + Seek> KTXDocument<R> {
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
    /// Pixel width of the image.
    ///
    pub fn width(&self) -> u32 {
        self.width
    }

    ///
    /// Pixel height of the image.
    ///
    /// # Warning
    ///
    /// In the case of a 1D image this will still return 1. Having this return a minimum of 1 (which
    /// is mathematically correct, as a 1D image is effectively a 2D image with a height of 1) makes
    /// dealing with images more generic. No surprise 0 will pop up here.
    ///
    /// This is in contrast to the KTX2.0 spec which requires a height of 0 to mark a 1D image. We
    /// resolve the document type while building the `KTXDocument`.
    ///
    /// To query what type of image the document holds use `KTXDocument::document_type`
    ///
    pub fn height(&self) -> u32 {
        self.height.max(1)
    }

    ///
    /// Pixel depth of the image.
    ///
    /// # Warning
    ///
    /// In the case of a 2D image this will still return 1. Having this return a minimum of 1 (which
    /// is mathematically correct, as a 2D image is effectively a 3D image with a depth of 1) makes
    /// dealing with images more generic. No surprise 0 will pop up here.
    ///
    /// This is in contrast to the KTX2.0 spec which requires a depth of 0 to mark a 2D image. We
    /// resolve the document type while building the `KTXDocument`.
    ///
    /// To query what type of image the document holds use `KTXDocument::document_type`
    ///
    pub fn depth(&self) -> u32 {
        self.depth.max(1)
    }

    ///
    /// Returns whether the file says the data it contains has premultiplied alpha.
    ///
    /// # Warning
    ///
    /// Faulty exporters could set this incorrectly. Trust this at your own discretion.
    ///
    pub fn is_premultiplied_alpha(&self) -> bool {
        self.dfd.flags.contains(DFDFlags::ALPHA_PREMULTIPLIED)
    }

    ///
    /// Gets the color primaries that the file states the image data matches
    ///
    /// # Warning
    ///
    /// Faulty exporters could set this incorrectly. Trust this at your own discretion.
    ///
    pub fn color_primaries(&self) -> ColorPrimaries {
        self.dfd.color_primaries
    }

    ///
    /// The number of image layers
    ///
    /// # Info
    ///
    /// This is *NOT* the raw `layerCount` from the KTX 2.0 document. This is exactly equal to
    /// `u32::max(1, layerCount)`. A `layerCount` of 0 is a marker value for when the document does
    /// not describe a texture array. A value > 0 describes an image array where there are
    /// `layerCount` elements.
    ///
    /// If `layerCount` is 1 then this means an image array with one element which is functionally
    /// equivalent to a non array image as both still have a single layer but the KTX2 spec
    /// uses 0 to differentiate between these two states.
    ///
    /// In any case it was deemed more useful to clamp to 1 for the return value for this function
    /// as a `layerCount` of 0 still means the image has a single layer. To check for an array
    /// use the `KTXDocument::document_type` function.
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
    /// `u32::max(1, levelCount)`.
    ///
    /// A `levelCount` of 0 is a special marker for when a document only provides a single mip layer
    /// but would like the loader to generate the rest of the mip levels. A `levelCount` greater
    /// than 0 specifies an image has exactly `levelCount` mip levels and they should be used as
    /// provided.
    ///
    /// A `levelCount` of 0 or 1 still functionally means 1 level of data is provided by the
    /// document and so this function clamps to 1 to simplify using this function.
    ///
    /// To check if the document requests mip generation use the
    /// `KTXDocument::requests_mip_generation` function.
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

    ///
    /// Reads the image data for the given mip level, cube face and array layer into the writer
    /// provided.
    ///
    /// This function requires the `KTXDocument` mutably as the reader will be mutated while being
    /// read from (a reader is a stateful object).
    ///
    pub fn read_image(
        &self,
        layer: usize,
        face: usize,
        level: usize,
        writer: &mut impl Write,
    ) -> Result<(), std::io::Error> {
        use std::io::Error;
        use std::io::ErrorKind;

        // Check we're in bounds for the number of levels, faces and levels
        if level >= self.level_num() as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Level index out of bounds",
            ));
        }
        if layer >= self.layer_num() as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Layer index out of bounds",
            ));
        }
        if face >= self.face_num() as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Face index out of bounds",
            ));
        }

        // Read the offset from the level indices
        let level_offset = self.level_indices[level].offset;

        // Gets the size of a single image at the mip level requested
        let image_bytes = self.image_bytes(level).unwrap();

        // Calculates the stride of a single face
        let face_stride = image_bytes as u64;

        // Calculate the stride of a single layer
        let layer_stride = (face_stride * self.face_num() as u64) * (self.layer_num() as u64);

        // Calculate the final offset of the image data in the file
        let data_offset =
            (layer_stride * layer as u64) + (face_stride * face as u64) + level_offset;

        // Take ownership of the reader from the document
        let mut reader = self.reader.take().unwrap();

        // Seek to the start of the image data
        reader.seek(SeekFrom::Start(data_offset))?;

        // Make a scoped reader that will read exactly the number of bytes required for the image
        let mut reader = reader.take(image_bytes as u64);

        // Perform the copy from the reader into writer
        std::io::copy(&mut reader, writer)?;

        // Return the reader to the document
        self.reader.set(Some(reader.into_inner()));

        Ok(())
    }

    ///
    /// Returns the number of bytes required for a single image at the given mip level. This will,
    /// roughly speaking, return the value of width * height * depth (bytes per block/pixel) for a
    /// given mip level. For block compressed formats it will be based on the dimensions and size of
    /// a block. For regular formats it will be based on dimensions and size of a single pixel.
    ///
    /// For cube maps this will be the size of a single face, not all 6 faces.
    ///
    /// Will return `None` if the level index provided is out of bounds.
    ///
    pub fn image_bytes(&self, level: usize) -> Option<usize> {
        if level >= self.level_num() as usize {
            None
        } else {
            let width = (self.width >> level).max(1) as usize;
            let height = (self.height >> level).max(1) as usize;
            let depth = (self.depth >> level).max(1) as usize;
            let bytes = format_bytes_for_image(self.format, width, height, depth).unwrap();
            Some(bytes)
        }
    }

    pub fn lookup_key(&self, key: &str) -> Result<Option<Vec<u8>>, KTXReadError> {
        // Get reader from cell
        let mut reader = self.reader.take().unwrap();

        // Go to start of key value section
        reader.seek(SeekFrom::Start(self.file_index.kvd_offset as u64))?;

        // We need to know when the key value section ends so we can stop reading once we reach it
        let end_of_section = self.file_index.kvd_offset + self.file_index.kvd_size;
        let end_of_section = end_of_section as u64;

        loop {
            // If we've reached the end of the key value section we stop and return None
            if reader.seek(SeekFrom::Current(0))? >= end_of_section {
                break;
            }

            // First we read off the length of the current section
            let key_and_val_length = reader.read_u32::<LittleEndian>()?;

            // Then we read off the key for this key value pair so we can see if it is the one we're
            // looking for
            let key_to_check = {
                // Keep reading until we hit a null byte or we hit the end of the declared length
                let mut vec = Vec::new();
                loop {
                    // If we've run past the total possible length for the key value pair emit an
                    // error
                    if vec.len() >= key_and_val_length as usize {
                        return Err(KTXReadError::InvalidKeyMissingNullTerminator);
                    }

                    // Read the next byte
                    let byte = reader.read_u8()?;

                    if byte != 0 {
                        vec.push(byte);
                    } else {
                        break;
                    }
                }
                String::from_utf8(vec)?
            };

            // Get the number of bytes for the value
            //
            // Subtract length of string bytes and -1 for the null terminator
            let value_len = key_and_val_length as usize - key_to_check.len() - 1;

            // If we've found the key we're looking for
            if key_to_check == key {
                // Read the value into a buffer
                let mut out = vec![0u8; value_len];
                reader.read_exact(&mut out)?;

                // Return the reader to the document and return the value
                self.reader.set(Some(reader));
                return Ok(Some(out));
            } else {
                // Seek past the value
                reader.seek(SeekFrom::Current(value_len as i64))?;

                // Get the current offset in the file so we can align forward over the padding
                // bytes
                let current_pos = reader.seek(SeekFrom::Current(0))?;

                // Align forward, only if we're not already aligned
                let distance_from_alignment = current_pos % 4;
                let aligned_pos = if distance_from_alignment != 0 {
                    let aligned_down = current_pos & (!3);
                    let aligned_up = aligned_down + 4;
                    aligned_up
                } else {
                    current_pos
                };

                reader.seek(SeekFrom::Start(aligned_pos))?;
            }
        }

        // If we've gotten here we've failed to find the key, return None to reflect this
        self.reader.set(Some(reader));
        Ok(None)
    }
}

impl<'a> KTXDocument<Cursor<&'a [u8]>> {
    ///
    /// Creates a new `KTXDocument` from the given reader
    ///
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, KTXReadError> {
        Self::from_reader(Cursor::new(bytes))
    }
}

impl<R: Read + Seek> KTXDocument<R> {
    ///
    /// Creates a new `KTXDocument` from the given reader
    ///
    pub fn from_reader(mut reader: R) -> Result<Self, KTXReadError> {
        Self::validate_file_identifier(&mut reader)?;
        let mut format = Self::read_vk_format(&mut reader)?;
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
                layer_num.max(1),
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

        let dfd = DataFormatDescriptor::from_reader(&mut reader, &file_index, &mut format)?;

        let reader = Cell::new(Some(reader));

        let out = Self {
            reader,
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
        if is_format_prohibited(format) {
            Err(KTXReadError::ProhibitedFormat(format))
        } else if is_format_unsupported(format) {
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
        let expected = format_type_size(format);

        if type_size != expected {
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
        } else if width == 0 {
            Err(KTXReadError::InvalidWidth(width))
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

        if face_count != 1 {
            if face_count == 6 {
                if depth != 0 {
                    Err(KTXReadError::InvalidDepthForCubeMap(depth))
                } else {
                    Ok(face_count)
                }
            } else {
                Err(KTXReadError::InvalidFaceCount(face_count))
            }
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
        let max_dim = max_dim as f64;
        let max_dim = max_dim.log2();
        let max_levels = max_dim as u32;

        if level_count > max_levels + 1 {
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
