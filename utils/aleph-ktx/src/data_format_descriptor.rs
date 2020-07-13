//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::file_index::FileIndex;
use crate::KTXReadError;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

///
/// Represents the set of supported `transferFunction` values
///
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum TransferFunction {
    Linear = 1,
    SRGB = 2,
}

impl TransferFunction {
    ///
    /// Tries to convert `v` into an enum variant. Returns `None` if v does not match one of the
    /// supported variants
    ///
    #[inline]
    pub fn from_raw(v: u8) -> Option<Self> {
        match v {
            1 => Some(TransferFunction::Linear),
            2 => Some(TransferFunction::SRGB),
            _ => None,
        }
    }

    ///
    /// Gets the raw `u8` value for the enum variant
    ///
    #[inline]
    pub const fn into_raw(self) -> u8 {
        self as u8
    }
}

///
/// Represents the set of `colorPrimaries` specified by the DFD spec
///
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ColorPrimaries {
    /// Unknown.
    Unspecified = PRIM_UNSPECIFIED,

    /// This value represents the Color Primaries defined by the ITU-R BT.709 specification (sRGB).
    BT709 = PRIM_BT709,

    /// This value represents the Color Primaries defined in the ITU-R BT.601 specification for
    /// standard-definition television, particularly for 625-line signals.
    BT601EBU = PRIM_BT601EBU,

    /// This value represents the Color Primaries defined in the ITU-R BT.601 specification for
    /// standard-definition television, particularly for 525-line signals.
    BT601SMPTE = PRIM_BT601SMPTE,

    /// This value represents the Color Primaries defined in the ITU-R BT.2020 specification for
    /// ultra-high-definition television
    BT2020 = PRIM_BT2020,

    /// This value represents the theoretical Color Primaries defined by the International Color
    /// Consortium for the ICC XYZ linear color space.
    CIEXYZ = PRIM_CIEXYZ,

    /// This value represents the Color Primaries defined for the Academy Color Encoding System
    ACES = PRIM_ACES,

    /// This value represents the Color Primaries defined for the Academy Color Encoding System
    /// compositor
    ACESCC = PRIM_ACESCC,

    /// This value represents the Color Primaries defined for the NTSC 1953 color television
    /// transmission standard
    NTSC1953 = PRIM_NTSC1953,

    /// This value represents the Color Primaries defined for 525-line PAL signals
    PAL525 = PRIM_PAL525,

    /// This value represents the Color Primaries defined for the Display P3 color space
    DISPLAYP3 = PRIM_DISPLAYP3,

    /// This value represents the Color Primaries defined in Adobe RGB (1998)
    ADOBERGB = PRIM_ADOBERGB,
}

impl ColorPrimaries {
    ///
    /// Tries to convert `v` into an enum variant. Returns `None` if v does not match one of the
    /// supported variants
    ///
    #[inline]
    pub fn from_raw(v: u8) -> Option<Self> {
        match v {
            PRIM_UNSPECIFIED => Some(ColorPrimaries::Unspecified),
            PRIM_BT709 => Some(ColorPrimaries::BT709),
            PRIM_BT601EBU => Some(ColorPrimaries::BT601EBU),
            PRIM_BT601SMPTE => Some(ColorPrimaries::BT601SMPTE),
            PRIM_BT2020 => Some(ColorPrimaries::BT2020),
            PRIM_CIEXYZ => Some(ColorPrimaries::CIEXYZ),
            PRIM_ACES => Some(ColorPrimaries::ACES),
            PRIM_ACESCC => Some(ColorPrimaries::ACESCC),
            PRIM_NTSC1953 => Some(ColorPrimaries::NTSC1953),
            PRIM_PAL525 => Some(ColorPrimaries::PAL525),
            PRIM_DISPLAYP3 => Some(ColorPrimaries::DISPLAYP3),
            PRIM_ADOBERGB => Some(ColorPrimaries::ADOBERGB),
            _ => None,
        }
    }

    ///
    /// Gets the raw `u8` value for the enum variant
    ///
    #[inline]
    pub const fn into_raw(self) -> u8 {
        self as u8
    }
}

///
/// Represents the supported set of DFD color models
///
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum ColorModel {
    /// Colour model left unspecified
    Unspecified = MODEL_UNSPECIFIED,

    /// Red, green, blue, stencil, depth, alpha
    RGBSDA = MODEL_RGBSDA,

    /// BC1 compressed
    BC1A = MODEL_BC1A,

    /// BC2 compressed
    BC2 = MODEL_BC2,

    /// BC3 compressed
    BC3 = MODEL_BC3,

    /// BC4 compressed
    BC4 = MODEL_BC4,

    /// BC5 compressed
    BC5 = MODEL_BC5,

    /// BC6 compressed
    BC6H = MODEL_BC6H,

    /// BC7 compressed
    BC7 = MODEL_BC7,

    /// ETC1 compressed
    ETC1 = MODEL_ETC1,

    /// ETC2 compressed
    ETC2 = MODEL_ETC2,

    /// ASTC compressed
    ASTC = MODEL_ASTC,

    /// ASTC compressed
    ETC1S = MODEL_ETC1S,

    /// PVRTC compressed
    PVRTC = MODEL_PVRTC,

    /// PVRTC2 compressed
    PVRTC2 = MODEL_PVRTC2,
}

impl ColorModel {
    ///
    /// Tries to convert `v` into an enum variant. Returns `None` if v does not match one of the
    /// supported variants
    ///
    #[inline]
    pub fn from_raw(v: u8) -> Option<Self> {
        match v {
            MODEL_UNSPECIFIED => Some(ColorModel::Unspecified),
            MODEL_RGBSDA => Some(ColorModel::RGBSDA),
            MODEL_BC1A => Some(ColorModel::BC1A),
            MODEL_BC2 => Some(ColorModel::BC2),
            MODEL_BC3 => Some(ColorModel::BC3),
            MODEL_BC4 => Some(ColorModel::BC4),
            MODEL_BC5 => Some(ColorModel::BC5),
            MODEL_BC6H => Some(ColorModel::BC6H),
            MODEL_BC7 => Some(ColorModel::BC7),
            MODEL_ETC1 => Some(ColorModel::ETC1),
            MODEL_ETC2 => Some(ColorModel::ETC2),
            MODEL_ASTC => Some(ColorModel::ASTC),
            MODEL_ETC1S => Some(ColorModel::ETC1S),
            MODEL_PVRTC => Some(ColorModel::PVRTC),
            MODEL_PVRTC2 => Some(ColorModel::PVRTC2),
            _ => None,
        }
    }

    ///
    /// Returns the raw `u8` value of the enum variant
    ///
    #[inline]
    pub const fn into_raw(self) -> u16 {
        self as u16
    }
}

///
/// Struct for unpacking and validating the data format descriptor in a KTX document
///
pub struct DataFormatDescriptor {
    pub dfd_total_size: u32,
    pub descriptor_type: u16,
    pub vendor_id: u32,
    pub descriptor_block_size: u16,
    pub version_number: u16,
    pub sample_count: u16,
}

impl DataFormatDescriptor {
    ///
    /// Reads the data format descriptor from the given reader
    ///
    pub fn from_reader(
        reader: &mut (impl Read + Seek),
        file_index: &FileIndex,
    ) -> Result<Self, KTXReadError> {
        reader.seek(SeekFrom::Start(file_index.dfd_offset as _))?;

        // Read and validate total size
        let dfd_total_size = reader.read_u32::<LittleEndian>()?;
        if dfd_total_size != file_index.kvd_offset - file_index.dfd_offset {
            return Err(KTXReadError::InvalidDFDTotalSize(dfd_total_size));
        }

        // Read the next word and unpack the vendor id and descriptor type
        let word = reader.read_u32::<LittleEndian>()?;
        let vendor_id = word & 0b11111111111111111;
        let descriptor_type = (word >> 17) as u16;

        // Read the next word and unpack the block size and version number
        let word = reader.read_u32::<LittleEndian>()?;
        let descriptor_block_size = (word & 0b1111111111111111) as u16;
        let version_number = (word >> 16) as u16;

        // Resolve the sample count
        let sample_count = (descriptor_block_size - 24) / 16;

        Ok(Self {
            dfd_total_size,
            descriptor_type,
            vendor_id,
            descriptor_block_size,
            version_number,
            sample_count,
        })
    }
}

const MODEL_UNSPECIFIED: u8 = 0;
const MODEL_RGBSDA: u8 = 1;
const MODEL_BC1A: u8 = 128;
const MODEL_BC2: u8 = 129;
const MODEL_BC3: u8 = 130;
const MODEL_BC4: u8 = 131;
const MODEL_BC5: u8 = 132;
const MODEL_BC6H: u8 = 133;
const MODEL_BC7: u8 = 134;
const MODEL_ETC1: u8 = 160;
const MODEL_ETC2: u8 = 161;
const MODEL_ASTC: u8 = 162;
const MODEL_ETC1S: u8 = 163;
const MODEL_PVRTC: u8 = 164;
const MODEL_PVRTC2: u8 = 165;

const PRIM_UNSPECIFIED: u8 = 0;
const PRIM_BT709: u8 = 1;
const PRIM_BT601EBU: u8 = 2;
const PRIM_BT601SMPTE: u8 = 3;
const PRIM_BT2020: u8 = 4;
const PRIM_CIEXYZ: u8 = 5;
const PRIM_ACES: u8 = 6;
const PRIM_ACESCC: u8 = 7;
const PRIM_NTSC1953: u8 = 8;
const PRIM_PAL525: u8 = 9;
const PRIM_DISPLAYP3: u8 = 10;
const PRIM_ADOBERGB: u8 = 11;
