//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

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
