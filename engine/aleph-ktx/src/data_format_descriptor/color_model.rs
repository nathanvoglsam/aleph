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

    /// YCbCr, stencil, depth, alpha
    YUVSDA = MODEL_YUVSDA,

    /// YIQ, stencil, depth, alpha
    YIQSDA = MODEL_YIQSDA,

    /// Perceptual color (CIE L*a*b*) + alpha, depth and stencil
    LABSDA = MODEL_LABSDA,

    /// Subtractive colors (cyan, magenta, yellow, black) + alpha
    CMYKA = MODEL_CMYKA,

    /// Non-color coordinate data (X, Y, Z, W)
    XYZW = MODEL_XYZW,

    /// Hue, saturation, value, hue angle on color circle, plus alpha
    HSVAAng = MODEL_HSVA_ANG,

    /// Hue, saturation, lightness, hue angle on color circle, plus alpha
    HSLAAng = MODEL_HSLA_ANG,

    /// Hue, saturation, value, hue on color hexagon, plus alpha
    HSVAHex = MODEL_HSVA_HEX,

    /// Hue, saturation, lightness, hue on color hexagon, plus alpha
    HSLAHex = MODEL_HSLA_HEX,

    /// Lightweight approximate color difference (luma, orange, green)
    YCGCOA = MODEL_YCGCOA,

    /// ITU BT.2020 constant luminance YcCbcCrc
    YCCBCCRC = MODEL_YCCBCCRC,

    /// ITU BT.2100 constant intensity ICtCp
    ICTCP = MODEL_ICTCP,

    /// CIE 1931 XYZ color coordinates (X, Y, Z)
    CIEXYZ = MODEL_CIEXYZ,

    /// CIE 1931 xyY color coordinates (X, Y, Y)
    CIEXYY = MODEL_CIEXYY,

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

    /// UASTC compressed
    UASTC = MODEL_UASTC,
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
            MODEL_YUVSDA => Some(ColorModel::YUVSDA),
            MODEL_YIQSDA => Some(ColorModel::YIQSDA),
            MODEL_LABSDA => Some(ColorModel::LABSDA),
            MODEL_CMYKA => Some(ColorModel::CMYKA),
            MODEL_XYZW => Some(ColorModel::XYZW),
            MODEL_HSVA_ANG => Some(ColorModel::HSVAAng),
            MODEL_HSLA_ANG => Some(ColorModel::HSLAAng),
            MODEL_HSVA_HEX => Some(ColorModel::HSVAHex),
            MODEL_HSLA_HEX => Some(ColorModel::HSLAHex),
            MODEL_YCGCOA => Some(ColorModel::YCGCOA),
            MODEL_YCCBCCRC => Some(ColorModel::YCCBCCRC),
            MODEL_ICTCP => Some(ColorModel::ICTCP),
            MODEL_CIEXYZ => Some(ColorModel::CIEXYZ),
            MODEL_CIEXYY => Some(ColorModel::CIEXYY),
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
            MODEL_UASTC => Some(ColorModel::UASTC),
            _ => None,
        }
    }

    ///
    /// Returns the raw `u8` value of the enum variant
    ///
    #[inline]
    pub const fn into_raw(self) -> u8 {
        self as u8
    }
}

const MODEL_UNSPECIFIED: u8 = 0;
const MODEL_RGBSDA: u8 = 1;
const MODEL_YUVSDA: u8 = 2;
const MODEL_YIQSDA: u8 = 3;
const MODEL_LABSDA: u8 = 4;
const MODEL_CMYKA: u8 = 5;
const MODEL_XYZW: u8 = 6;
const MODEL_HSVA_ANG: u8 = 7;
const MODEL_HSLA_ANG: u8 = 8;
const MODEL_HSVA_HEX: u8 = 9;
const MODEL_HSLA_HEX: u8 = 10;
const MODEL_YCGCOA: u8 = 11;
const MODEL_YCCBCCRC: u8 = 12;
const MODEL_ICTCP: u8 = 13;
const MODEL_CIEXYZ: u8 = 14;
const MODEL_CIEXYY: u8 = 15;
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
const MODEL_UASTC: u8 = 166;
