//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{KTXReadError, SuperCompressionScheme};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

///
/// Internal struct for holding a single member object of the level indicies list
///
#[derive(Clone, Default)]
pub struct LevelIndex {
    pub offset: u64,
    pub size: u64,
    pub size_uncompressed: u64,
}

impl LevelIndex {
    ///
    /// Read the level index from the given reader
    ///
    pub fn from_reader(
        reader: &mut impl Read,
        layer_num: u32,
        face_num: u32,
        compression: SuperCompressionScheme,
    ) -> Result<Self, KTXReadError> {
        let offset = Self::read_offset(reader)?;
        let size = Self::read_size(reader)?;
        let size_uncompressed =
            Self::read_size_uncompressed(reader, size, layer_num, face_num, compression)?;

        Ok(Self {
            offset,
            size,
            size_uncompressed,
        })
    }

    ///
    /// Internal function for reading the offset
    ///
    fn read_offset(reader: &mut impl Read) -> Result<u64, KTXReadError> {
        Ok(reader.read_u64::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the size
    ///
    fn read_size(reader: &mut impl Read) -> Result<u64, KTXReadError> {
        // TODO: More validation, see 3.9.7. Level Index
        Ok(reader.read_u64::<LittleEndian>()?)
    }

    ///
    /// Internal function for reading the size_uncompressed
    ///
    fn read_size_uncompressed(
        reader: &mut impl Read,
        size: u64,
        layer_num: u32,
        face_num: u32,
        compression: SuperCompressionScheme,
    ) -> Result<u64, KTXReadError> {
        let size_uncompressed = reader.read_u64::<LittleEndian>()?;

        if compression == SuperCompressionScheme::BASIS_LZ && size_uncompressed != 0 {
            Err(KTXReadError::InvalidLevelIndexUncompressedSize)
        } else if compression == SuperCompressionScheme::NONE && size_uncompressed != size {
            Err(KTXReadError::InvalidLevelIndexUncompressedSize)
        } else if size_uncompressed % (face_num as u64 * layer_num as u64) != 0 {
            Err(KTXReadError::InvalidLevelIndexUncompressedSize)
        } else {
            Ok(size_uncompressed)
        }
    }
}
