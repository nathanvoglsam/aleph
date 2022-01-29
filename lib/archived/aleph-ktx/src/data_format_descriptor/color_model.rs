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

use crate::format::is_format_prohibited;
use aleph_vk_format::VkFormat;

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

    ///
    /// Is the color model correct for the given VkFormat
    ///
    #[inline]
    pub fn is_compatible_with_format(self, format: VkFormat) -> bool {
        match self {
            Self::Unspecified => false, // This will always be wrong
            Self::RGBSDA => !format.is_block_format(),
            Self::BC1A => format.is_bc1(),
            Self::BC2 => format.is_bc2(),
            Self::BC3 => format.is_bc3(),
            Self::BC4 => format.is_bc4(),
            Self::BC5 => format.is_bc5(),
            Self::BC6H => format.is_bc6h(),
            Self::BC7 => format.is_bc7(),
            Self::ETC1 => format.is_etc2(),
            Self::ETC2 => format.is_etc2() || format.is_eac(),
            Self::ASTC => format.is_astc(),
            Self::ETC1S => format.is_etc2() || format == VkFormat::UNDEFINED,
            Self::PVRTC => format.is_pvrtc1(),
            Self::PVRTC2 => format.is_pvrtc2(),
        }
    }

    ///
    /// Returns a color model for the given VkFormat
    ///
    pub fn for_format(format: VkFormat) -> Option<ColorModel> {
        if format == VkFormat::UNDEFINED || is_format_prohibited(format) {
            None
        } else if format.is_bc1() {
            Some(ColorModel::BC1A)
        } else if format.is_bc2() {
            Some(ColorModel::BC2)
        } else if format.is_bc3() {
            Some(ColorModel::BC3)
        } else if format.is_bc4() {
            Some(ColorModel::BC4)
        } else if format.is_bc5() {
            Some(ColorModel::BC5)
        } else if format.is_bc6h() {
            Some(ColorModel::BC6H)
        } else if format.is_bc7() {
            Some(ColorModel::BC7)
        } else if format.is_etc2() || format.is_eac() {
            Some(ColorModel::ETC2)
        } else if format.is_astc() {
            Some(ColorModel::ASTC)
        } else if format.is_pvrtc1() {
            Some(ColorModel::PVRTC)
        } else if format.is_pvrtc2() {
            Some(ColorModel::PVRTC2)
        } else {
            Some(ColorModel::RGBSDA)
        }
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
