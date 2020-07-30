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

use crate::{KTXReadError, SuperCompressionScheme};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

///
/// Internal struct for loading and storing the file index
///
pub struct FileIndex {
    pub dfd_offset: u32,
    pub dfd_size: u32,
    pub kvd_offset: u32,
    pub kvd_size: u32,
    pub sgd_offset: u64,
    pub sgd_size: u64,
}

impl FileIndex {
    ///
    /// Internal function for reading the index from a reader instance
    ///
    pub fn from_reader(
        reader: &mut impl Read,
        super_compression_scheme: SuperCompressionScheme,
    ) -> Result<Self, KTXReadError> {
        let dfd_offset = Self::read_dfd_offset(reader)?;
        let dfd_size = Self::read_dfd_size(reader)?;
        let kvd_offset = Self::read_kvd_offset(reader)?;
        let kvd_size = Self::read_kvd_size(reader, kvd_offset)?;
        let sgd_offset = Self::read_sgd_offset(reader)?;
        let sgd_size = Self::read_sgd_size(reader, sgd_offset, super_compression_scheme)?;

        Ok(Self {
            dfd_offset,
            dfd_size,
            kvd_offset,
            kvd_size,
            sgd_offset,
            sgd_size,
        })
    }

    ///
    /// Internal function for reading the dfd_offset
    ///
    fn read_dfd_offset(reader: &mut impl Read) -> Result<u32, KTXReadError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the dfd_size
    ///
    fn read_dfd_size(reader: &mut impl Read) -> Result<u32, KTXReadError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the kvd_offset
    ///
    fn read_kvd_offset(reader: &mut impl Read) -> Result<u32, KTXReadError> {
        Ok(reader.read_u32::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the kvd_size
    ///
    fn read_kvd_size(reader: &mut impl Read, kvd_offset: u32) -> Result<u32, KTXReadError> {
        let kvd_size = reader.read_u32::<LittleEndian>()?;

        if kvd_size == 0 && kvd_offset != 0 {
            Err(KTXReadError::InvalidKeyValueDataOffset(kvd_offset))
        } else {
            Ok(kvd_size)
        }
    }

    ///
    /// Internal function for reading the sgd_offset
    ///
    fn read_sgd_offset(reader: &mut impl Read) -> Result<u64, KTXReadError> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the sgd_size
    ///
    fn read_sgd_size(
        reader: &mut impl Read,
        sgd_offset: u64,
        super_compression_scheme: SuperCompressionScheme,
    ) -> Result<u64, KTXReadError> {
        let sgd_size = reader.read_u64::<LittleEndian>()?;

        if sgd_size == 0 && sgd_offset != 0 {
            Err(KTXReadError::InvalidSuperCompressionGlobalDataOffset(
                sgd_offset,
            ))
        } else if super_compression_scheme.has_global_data() && sgd_size == 0 {
            Err(KTXReadError::CompressionSchemeGlobalDataNotFound(
                super_compression_scheme,
            ))
        } else {
            Ok(sgd_size)
        }
    }
}
