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

use std::cell::Cell;
use std::ffi::CStr;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::num::NonZeroUsize;

use aleph_vk_format::VkFormat;
use byteorder::{LittleEndian, ReadBytesExt};
pub use file_index::FileIndex;
pub use level_index::LevelIndex;
pub use super_compression_scheme::SuperCompressionScheme;

use crate::data_format_descriptor::DataFormatDescriptor;
use crate::format::is_format_prohibited;
use crate::{
    format_type_size, ColorPrimaries, DFDError, DFDFlags, KtxOrientation, KtxSwizzle,
    TransferFunction,
};

///
/// Represents the set of errors that could occur when trying to pass/read a ktx file from a stream
/// of bytes
///
#[derive(Debug)]
pub enum KtxReadError {
    /// Could not find the requested key in the KVD section
    NoSuchKey,

    /// This error is produced when the stream of bytes does not contain a ktx file. This will
    /// usually be thrown if file identifier is invalid
    NotKtxDocument,

    /// A file reading function was called but the reader has been taken from the document.
    NoReader,

    /// The KTX file tried to specify a prohibited format
    ProhibitedFormat(VkFormat),

    /// The KTX file specified a format this implementation doesn't support
    UnsupportedFormat(VkFormat),

    /// The declared typeSize field does not match the value expected by the format
    InvalidTypeSize(u32),

    /// The KTX file specified a super compression scheme that this implementation doesn't support
    UnsupportedSuperCompressionScheme(SuperCompressionScheme),

    /// The file's format is incorrect for the declared supercompression scheme.
    InvalidFormatForSuperCompressionScheme(SuperCompressionScheme, VkFormat),

    /// The super compression global data is either missing or broken.
    BadSuperCompressionGlobalData,

    /// The declared dimensions are invalid for the declared format
    InvalidDimensions((u32, u32, u32)),

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

    /// We found a key in the key/value data but the data is malformed.
    BadKeyValueData,

    /// We found a key in the key/value data but the target buffer is too small to extract the data.
    /// This error will also contain the number of bytes that we would've needed.
    DestBufferTooSmall(usize),

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

impl From<std::io::Error> for KtxReadError {
    fn from(err: std::io::Error) -> Self {
        KtxReadError::IOError(err)
    }
}

impl From<std::string::FromUtf8Error> for KtxReadError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        KtxReadError::FromUtf8Error(err)
    }
}

impl From<DFDError> for KtxReadError {
    fn from(err: DFDError) -> Self {
        KtxReadError::DFDError(err)
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

pub struct KtxDocument<R: Read + Seek> {
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
    level_indices: LevelIndices,
}

impl<R: Read + Seek> KtxDocument<R> {
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

    /// Gets the transfer function that the file states the image data matches
    pub fn transfer_function(&self) -> TransferFunction {
        self.dfd.transfer_function
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
        self.level_num == 0
    }

    ///
    /// The super compression scheme the document uses
    ///
    pub fn super_compression_scheme(&self) -> SuperCompressionScheme {
        self.super_compression_scheme
    }

    /// Removes the reader from the document, left in whatever state it was left in.
    ///
    /// # Warning
    ///
    /// Once the reader has been taken some functions ([`Self::lookup_key`] for example) will fail
    /// as the reader is needed to extract the key and value. Only call this once you've queried all
    /// required information from the document.
    pub fn take_reader(&self) -> Option<R> {
        self.reader.take()
    }

    ///
    /// Reads the image data for the given mip level, cube face and array layer into the writer
    /// provided.
    ///
    /// This function requires the `KTXDocument` mutably as the reader will be mutated while being
    /// read from (a reader is a stateful object).
    ///
    pub fn get_level_info(&self, level: usize) -> Result<LevelIndex, std::io::Error> {
        use std::io::{Error, ErrorKind};

        // Check we're in bounds for the number of levels, faces and levels
        if level >= self.level_num() as usize {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Level index out of bounds",
            ));
        }

        // Read the offset from the level indices
        let info = self.level_indices.as_slice()[level].clone();

        Ok(info)
    }

    /// Lookup, and read out the data for, the given key in the documents key/value store.
    ///
    /// The data will be written into 'dst'. On success the number of bytes written into 'dst' will
    /// be returned.
    pub fn lookup_key(&self, key: &str, dst: &mut [u8]) -> Result<NonZeroUsize, KtxReadError> {
        if self.file_index.kvd_offset == 0 {
            // Early exit if there is no key-value data segment in the file.
            return Err(KtxReadError::NoSuchKey);
        }

        // Get reader from cell
        let mut reader = self.reader.take().ok_or(KtxReadError::NoReader)?;

        // Wrap the inner failible function so we can ensure we return the reader to it's slot even
        // if we hit an error.
        match self.inner_lookup_key(&mut reader, key, dst) {
            Ok(v) => {
                self.reader.set(Some(reader));
                Ok(v)
            }
            Err(e) => {
                self.reader.set(Some(reader));
                Err(e)
            }
        }
    }

    fn inner_lookup_key(
        &self,
        reader: &mut R,
        key: &str,
        dst: &mut [u8],
    ) -> Result<NonZeroUsize, KtxReadError> {
        // Go to start of key value section
        reader.seek(SeekFrom::Start(self.file_index.kvd_offset as u64))?;

        // We need to know when the key value section ends so we can stop reading once we reach it
        let end_of_section = self.file_index.kvd_offset + self.file_index.kvd_size;
        let end_of_section = end_of_section as u64;

        loop {
            // If we've reached the end of the key value section we stop and return None
            if reader.stream_position()? >= end_of_section {
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
                        return Err(KtxReadError::InvalidKeyMissingNullTerminator);
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
            let value_len = key_and_val_length as usize;
            let value_len = value_len.saturating_sub(key_to_check.len());
            let value_len = value_len.saturating_sub(1);
            let value_len = if let Some(v) = NonZeroUsize::new(value_len) {
                v
            } else {
                return Err(KtxReadError::BadKeyValueData);
            };

            // If we've found the key we're looking for
            if key_to_check == key {
                // Read the value into a buffer
                let out = dst
                    .get_mut(0..value_len.get())
                    .ok_or(KtxReadError::DestBufferTooSmall(value_len.get()))?;
                reader.read_exact(out)?;

                return Ok(value_len);
            } else {
                // Seek past the value
                reader.seek(SeekFrom::Current(value_len.get() as i64))?;

                // Get the current offset in the file so we can align forward over the padding
                // bytes
                let current_pos = reader.stream_position()?;

                // Align forward, only if we're not already aligned
                let distance_from_alignment = current_pos % 4;
                let aligned_pos = if distance_from_alignment != 0 {
                    let aligned_down = current_pos & (!3);
                    aligned_down + 4
                } else {
                    current_pos
                };

                reader.seek(SeekFrom::Start(aligned_pos))?;
            }
        }

        Err(KtxReadError::NoSuchKey)
    }

    /// Utility for looking up the standard 'KTXorientation' key/value data.
    pub fn lookup_orientation(&self) -> Result<KtxOrientation, KtxReadError> {
        let mut bytes = [0u8; 4];
        let _ = self.lookup_key("KTXorientation", &mut bytes)?;
        let orientation = KtxOrientation::new(bytes).ok_or(KtxReadError::BadKeyValueData)?;
        Ok(orientation)
    }

    /// Utility for looking up the standard 'KTXswizzle' key/value data.
    pub fn lookup_swizzle(&self) -> Result<KtxSwizzle, KtxReadError> {
        let mut bytes = [0u8; 5];
        let _ = self.lookup_key("KTXswizzle", &mut bytes)?;
        let swizzle = KtxSwizzle::new(bytes).ok_or(KtxReadError::BadKeyValueData)?;
        Ok(swizzle)
    }

    /// Will lookup the 'KTXwriter' key and return a &str containing the declared name of the writer
    /// if the key is present.
    ///
    /// This function takes 'scratch' as a scratch buffer, and may fail if scratch is not big enough
    /// to contain the key.
    pub fn lookup_writer<'b>(&self, scratch: &'b mut [u8]) -> Result<&'b str, KtxReadError> {
        let read = self.lookup_key("KTXwriter", scratch)?;
        let writer_name = &scratch[0..read.get()];
        let writer_name =
            CStr::from_bytes_until_nul(writer_name).map_err(|_| KtxReadError::BadKeyValueData)?;
        let writer_name = writer_name
            .to_str()
            .map_err(|_| KtxReadError::BadKeyValueData)?;
        Ok(writer_name)
    }
}

impl<'a> KtxDocument<Cursor<&'a [u8]>> {
    ///
    /// Creates a new `KTXDocument` from the given reader
    ///
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, KtxReadError> {
        Self::from_reader(Cursor::new(bytes))
    }
}

impl<R: Read + Seek> KtxDocument<R> {
    ///
    /// Creates a new `KTXDocument` from the given reader
    ///
    pub fn from_reader(mut reader: R) -> Result<Self, KtxReadError> {
        Self::validate_file_identifier(&mut reader)?;
        let format = Self::read_vk_format(&mut reader)?;
        let type_size = Self::read_type_size(&mut reader, format)?;
        let dimensions = Self::read_dimensions(&mut reader, format)?;
        let layer_num = Self::read_layer_count(&mut reader)?;
        let face_num = Self::read_face_count(&mut reader, dimensions.2)?;
        let level_num = Self::read_level_count(&mut reader, dimensions, format)?;
        let super_compression_scheme = Self::read_super_compression_scheme(&mut reader)?;
        let file_index = FileIndex::from_reader(&mut reader, super_compression_scheme)?;
        let level_indices = LevelIndices::from_reader(
            &mut reader,
            level_num,
            layer_num,
            face_num,
            super_compression_scheme,
        )?;

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

        // Assert that the BASIZ_LZ scheme has the format set correctly
        match super_compression_scheme {
            SuperCompressionScheme::NONE
            | SuperCompressionScheme::ZSTD
            | SuperCompressionScheme::ZLIB => {
                // Intentionally blank
            }
            SuperCompressionScheme::BASIS_LZ => {
                if format != VkFormat::UNDEFINED {
                    return Err(KtxReadError::InvalidFormatForSuperCompressionScheme(
                        super_compression_scheme,
                        format,
                    ));
                }

                if file_index.sgd_size == 0 {
                    return Err(KtxReadError::BadSuperCompressionGlobalData);
                }
            }
            _ => unreachable!(),
        }

        let dfd = DataFormatDescriptor::from_reader(
            &mut reader,
            &file_index,
            format,
            super_compression_scheme,
        )?;

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
    fn validate_file_identifier(reader: &mut impl Read) -> Result<(), KtxReadError> {
        // Read off the file identifier
        let mut identifier = [0u8; 12];
        reader.read_exact(&mut identifier)?;

        // A valid KTX 2.0 file must, at the very least, start with the given sequence of bytes
        if identifier != FILE_IDENTIFIER {
            Err(KtxReadError::NotKtxDocument)
        } else {
            Ok(())
        }
    }

    ///
    /// Internal function for reading the vk_format
    ///
    fn read_vk_format(reader: &mut impl Read) -> Result<VkFormat, KtxReadError> {
        // Load and validate the format
        let format = reader.read_u32::<LittleEndian>()?;
        let format = VkFormat(format);

        // Check if format is valid and supported
        if is_format_prohibited(format) {
            Err(KtxReadError::ProhibitedFormat(format))
        } else if !format.is_known() {
            // If we don't know anything about the format we won't know how to read it
            Err(KtxReadError::UnsupportedFormat(format))
        } else {
            Ok(format)
        }
    }

    ///
    /// Internal function for reading the type_size
    ///
    fn read_type_size(reader: &mut impl Read, format: VkFormat) -> Result<u32, KtxReadError> {
        let type_size = reader.read_u32::<LittleEndian>()?;
        let expected_type_size = format_type_size(format).unwrap();
        if type_size != expected_type_size {
            return Err(KtxReadError::InvalidTypeSize(type_size));
        }
        Ok(type_size)
    }

    ///
    /// Internal function for reading the image dimensions
    ///
    fn read_dimensions(
        reader: &mut impl Read,
        format: VkFormat,
    ) -> Result<(u32, u32, u32), KtxReadError> {
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;
        let depth = reader.read_u32::<LittleEndian>()?;

        if width == 0 {
            return Err(KtxReadError::InvalidDimensions((width, height, depth)));
        }

        if !format.is_known() {
            return Ok((width, height, depth));
        }

        if format.is_depth_format() && depth != 0 {
            return Err(KtxReadError::InvalidDimensions((width, height, depth)));
        }

        Ok((width, height, depth))
    }

    ///
    /// Internal function for reading the layer_count
    ///
    fn read_layer_count(reader: &mut impl Read) -> Result<u32, KtxReadError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the face_count
    ///
    fn read_face_count(reader: &mut impl Read, depth: u32) -> Result<u32, KtxReadError> {
        let face_count = reader.read_u32::<LittleEndian>()?;

        if face_count != 1 {
            if face_count == 6 {
                if depth != 0 {
                    Err(KtxReadError::InvalidDepthForCubeMap(depth))
                } else {
                    Ok(face_count)
                }
            } else {
                Err(KtxReadError::InvalidFaceCount(face_count))
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
    ) -> Result<u32, KtxReadError> {
        let level_count = reader.read_u32::<LittleEndian>()?;

        // Select the highest of the three dimensions
        let max_dim = u32::max(dimensions.0, dimensions.1);
        let max_dim = u32::max(max_dim, dimensions.2);

        // Max level is equal to log2 of the highest image dimension
        let max_levels = max_dim as f64;
        let max_levels = max_levels.log2();
        let max_levels = max_levels as u32;

        if level_count > max_levels + 1 {
            Err(KtxReadError::TooManyLevels(level_count))
        } else if level_count == 0 && format.is_block_compressed() {
            // It's not legal for a block compressed image to require the reader to generate mip
            // levels.
            Err(KtxReadError::InvalidLevelCountForBlockFormat(level_count))
        } else {
            // Unknown formats will take this branch
            Ok(level_count)
        }
    }

    ///
    /// Internal function for reading the super_compression_scheme
    ///
    fn read_super_compression_scheme(
        reader: &mut impl Read,
    ) -> Result<SuperCompressionScheme, KtxReadError> {
        let super_compression_scheme = reader.read_u32::<LittleEndian>()?;
        let super_compression_scheme = SuperCompressionScheme(super_compression_scheme);

        if !super_compression_scheme.is_supported() {
            Err(KtxReadError::UnsupportedSuperCompressionScheme(
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
pub(crate) const FILE_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

enum LevelIndices {
    // Inline storage, which we will use 99.999999% of the time.
    Inline([LevelIndex; 16], usize),

    // Heap storage fallback, which we'll only use for truly enormous textures 32k * 32k !!!
    Heap(Vec<LevelIndex>),
}

impl LevelIndices {
    fn as_slice(&self) -> &[LevelIndex] {
        match self {
            LevelIndices::Inline(arr, len) => &arr[0..*len],
            LevelIndices::Heap(vec) => vec.as_slice(),
        }
    }

    fn from_reader<R: Read + Seek>(
        reader: &mut R,
        level_num: u32,
        layer_num: u32,
        face_num: u32,
        super_compression_scheme: SuperCompressionScheme,
    ) -> Result<Self, KtxReadError> {
        // We always need to read at least one level, even if it says 0 as 0 means 1, but the reader
        // should generate mip levels
        let levels_to_read = u32::max(1, level_num);

        if levels_to_read > 32 {
            // 33+ levels would apply mip 0 is wider than (2^32) on some dimension. We can't encode
            // that as dimensions are stored in 32 bits so something has gone very wrong and we have
            // a corrupted file.
            return Err(KtxReadError::TooManyLevels(level_num));
        }

        // We use inline storage for the first 16 levels in an image. Images with more than this
        // many levels are exceedingly rare. This way we only hit the heap allocated path on the
        // rarest of rare images.
        if levels_to_read <= 16 {
            let mut level_indices: [_; 16] = std::array::from_fn(|_| LevelIndex::default());
            for i in 0..levels_to_read {
                level_indices[i as usize] = LevelIndex::from_reader(
                    reader,
                    layer_num.max(1),
                    face_num,
                    super_compression_scheme,
                )?;
            }
            Ok(Self::Inline(level_indices, levels_to_read as usize))
        } else {
            let mut level_indices = Vec::new();
            for _ in 0..levels_to_read {
                level_indices.push(LevelIndex::from_reader(
                    reader,
                    layer_num.max(1),
                    face_num,
                    super_compression_scheme,
                )?);
            }
            Ok(Self::Heap(level_indices))
        }
    }
}
