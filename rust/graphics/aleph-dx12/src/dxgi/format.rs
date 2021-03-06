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

use std::convert::TryFrom;
use windows_raw::win32::dxgi::DXGI_FORMAT;

#[repr(u32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Format {
    Unknown = 0,
    R32G32B32A32Typeless = 1,
    R32G32B32A32Float = 2,
    R32G32B32A32Uint = 3,
    R32G32B32A32Sint = 4,
    R32G32B32Typeless = 5,
    R32G32B32Float = 6,
    R32G32B32Uint = 7,
    R32G32B32Sint = 8,
    R16G16B16A16Typeless = 9,
    R16G16B16A16Float = 10,
    R16G16B16A16Unorm = 11,
    R16G16B16A16Uint = 12,
    R16G16B16A16Snorm = 13,
    R16G16B16A16Sint = 14,
    R32G32Typeless = 15,
    R32G32Float = 16,
    R32G32Uint = 17,
    R32G32Sint = 18,
    R32G8X24Typeless = 19,
    D32FloatS8X24Uint = 20,
    R32FloatX8X24Typeless = 21,
    X32TypelessG8X24Uint = 22,
    R10G10B10A2Typeless = 23,
    R10G10B10A2Unorm = 24,
    R10G10B10A2Uint = 25,
    R11G11B10Float = 26,
    R8G8B8A8Typeless = 27,
    R8G8B8A8Unorm = 28,
    R8G8B8A8UnormSRGB = 29,
    R8G8B8A8Uint = 30,
    R8G8B8A8Snorm = 31,
    R8G8B8A8Sint = 32,
    R16G16Typeless = 33,
    R16G16Float = 34,
    R16G16Unorm = 35,
    R16G16Uint = 36,
    R16G16Snorm = 37,
    R16G16Sint = 38,
    R32Typeless = 39,
    D32Float = 40,
    R32Float = 41,
    R32Uint = 42,
    R32Sint = 43,
    R24G8Typeless = 44,
    D24UnormS8Uint = 45,
    R24UnormX8Typeless = 46,
    X24TypelessG8Uint = 47,
    R8G8Typeless = 48,
    R8G8Unorm = 49,
    R8G8Uint = 50,
    R8G8Snorm = 51,
    R8G8Sint = 52,
    R16Typeless = 53,
    R16Float = 54,
    D16Unorm = 55,
    R16Unorm = 56,
    R16Uint = 57,
    R16Snorm = 58,
    R16Sint = 59,
    R8Typeless = 60,
    R8Unorm = 61,
    R8Uint = 62,
    R8Snorm = 63,
    R8Sint = 64,
    A8Unorm = 65,
    R1Unorm = 66,
    R9G9B9E5SharedExp = 67,
    R8G8UnormB8G8Unorm = 68,
    G8R8UnormG8B8Unorm = 69,
    BC1Typeless = 70,
    BC1Unorm = 71,
    BC1UnormSRGB = 72,
    BC2Typeless = 73,
    BC2Unorm = 74,
    BC2UnormSRGB = 75,
    BC3Typeless = 76,
    BC3Unorm = 77,
    BC3UnormSRGB = 78,
    BC4Typeless = 79,
    BC4Unorm = 80,
    BC4Snorm = 81,
    BC5Typeless = 82,
    BC5Unorm = 83,
    BC5Snorm = 84,
    B5G6R5Unorm = 85,
    B5G5R5A1Unorm = 86,
    B8G8R8A8Unorm = 87,
    B8G8R8X8Unorm = 88,
    R10G10B10XrBiasA2Unorm = 89,
    B8G8R8A8Typeless = 90,
    B8G8R8A8UnormSRGB = 91,
    B8G8R8X8Typeless = 92,
    B8G8R8X8UnormSRGB = 93,
    BC6HTypeless = 94,
    BC6HUFloat16 = 95,
    BC6HSFloat16 = 96,
    BC7Typeless = 97,
    BC7Unorm = 98,
    BC7UnormSRGB = 99,
    AYUV = 100,
    Y410 = 101,
    Y416 = 102,
    NV12 = 103,
    P010 = 104,
    P016 = 105,
    Opaque420 = 106,
    YUY2 = 107,
    Y210 = 108,
    Y216 = 109,
    NV11 = 110,
    AI44 = 111,
    IA44 = 112,
    P8 = 113,
    A8P8 = 114,
    B4G4R4A4Unorm = 115,
    P208 = 130,
    V208 = 131,
    V408 = 132,
    SamplerFeedbackMinMipOpaque = 189,
    SamplerFeedbackMipRegionUsedOpaque = 190,
}

impl Format {
    pub fn is_depth_stencil(&self) -> bool {
        match self {
            Self::D32Float | Self::D24UnormS8Uint | Self::D16Unorm | Self::D32FloatS8X24Uint => {
                true
            }
            _ => false,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Unknown
    }
}

impl TryFrom<DXGI_FORMAT> for Format {
    type Error = ();

    fn try_from(value: DXGI_FORMAT) -> Result<Self, Self::Error> {
        match value {
            DXGI_FORMAT::DXGI_FORMAT_UNKNOWN => Ok(Format::Unknown),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS => Ok(Format::R32G32B32A32Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT => Ok(Format::R32G32B32A32Float),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_UINT => Ok(Format::R32G32B32A32Uint),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_SINT => Ok(Format::R32G32B32A32Sint),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_TYPELESS => Ok(Format::R32G32B32Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_FLOAT => Ok(Format::R32G32B32Float),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_UINT => Ok(Format::R32G32B32Uint),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_SINT => Ok(Format::R32G32B32Sint),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_TYPELESS => Ok(Format::R16G16B16A16Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_FLOAT => Ok(Format::R16G16B16A16Float),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UNORM => Ok(Format::R16G16B16A16Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UINT => Ok(Format::R16G16B16A16Uint),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_SNORM => Ok(Format::R16G16B16A16Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_SINT => Ok(Format::R16G16B16A16Sint),
            DXGI_FORMAT::DXGI_FORMAT_R32G32_TYPELESS => Ok(Format::R32G32Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R32G32_FLOAT => Ok(Format::R32G32Float),
            DXGI_FORMAT::DXGI_FORMAT_R32G32_UINT => Ok(Format::R32G32Uint),
            DXGI_FORMAT::DXGI_FORMAT_R32G32_SINT => Ok(Format::R32G32Sint),
            DXGI_FORMAT::DXGI_FORMAT_R32G8X24_TYPELESS => Ok(Format::R32G8X24Typeless),
            DXGI_FORMAT::DXGI_FORMAT_D32_FLOAT_S8X24_UINT => Ok(Format::D32FloatS8X24Uint),
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS => Ok(Format::R32FloatX8X24Typeless),
            DXGI_FORMAT::DXGI_FORMAT_X32_TYPELESS_G8X24_UINT => Ok(Format::X32TypelessG8X24Uint),
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_TYPELESS => Ok(Format::R10G10B10A2Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_UNORM => Ok(Format::R10G10B10A2Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_UINT => Ok(Format::R10G10B10A2Uint),
            DXGI_FORMAT::DXGI_FORMAT_R11G11B10_FLOAT => Ok(Format::R11G11B10Float),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_TYPELESS => Ok(Format::R8G8B8A8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM => Ok(Format::R8G8B8A8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => Ok(Format::R8G8B8A8UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UINT => Ok(Format::R8G8B8A8Uint),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_SNORM => Ok(Format::R8G8B8A8Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_SINT => Ok(Format::R8G8B8A8Sint),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_TYPELESS => Ok(Format::R16G16Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_FLOAT => Ok(Format::R16G16Float),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_UNORM => Ok(Format::R16G16Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_UINT => Ok(Format::R16G16Uint),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_SNORM => Ok(Format::R16G16Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_SINT => Ok(Format::R16G16Sint),
            DXGI_FORMAT::DXGI_FORMAT_R32_TYPELESS => Ok(Format::R32Typeless),
            DXGI_FORMAT::DXGI_FORMAT_D32_FLOAT => Ok(Format::D32Float),
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT => Ok(Format::R32Float),
            DXGI_FORMAT::DXGI_FORMAT_R32_UINT => Ok(Format::R32Uint),
            DXGI_FORMAT::DXGI_FORMAT_R32_SINT => Ok(Format::R32Sint),
            DXGI_FORMAT::DXGI_FORMAT_R24G8_TYPELESS => Ok(Format::R24G8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_D24_UNORM_S8_UINT => Ok(Format::D24UnormS8Uint),
            DXGI_FORMAT::DXGI_FORMAT_R24_UNORM_X8_TYPELESS => Ok(Format::R24UnormX8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_X24_TYPELESS_G8_UINT => Ok(Format::X24TypelessG8Uint),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_TYPELESS => Ok(Format::R8G8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_UNORM => Ok(Format::R8G8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_UINT => Ok(Format::R8G8Uint),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_SNORM => Ok(Format::R8G8Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_SINT => Ok(Format::R8G8Sint),
            DXGI_FORMAT::DXGI_FORMAT_R16_TYPELESS => Ok(Format::R16Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R16_FLOAT => Ok(Format::R16Float),
            DXGI_FORMAT::DXGI_FORMAT_D16_UNORM => Ok(Format::D16Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R16_UNORM => Ok(Format::R16Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R16_UINT => Ok(Format::R16Uint),
            DXGI_FORMAT::DXGI_FORMAT_R16_SNORM => Ok(Format::R16Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R16_SINT => Ok(Format::R16Sint),
            DXGI_FORMAT::DXGI_FORMAT_R8_TYPELESS => Ok(Format::R8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_R8_UNORM => Ok(Format::R8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R8_UINT => Ok(Format::R8Uint),
            DXGI_FORMAT::DXGI_FORMAT_R8_SNORM => Ok(Format::R8Snorm),
            DXGI_FORMAT::DXGI_FORMAT_R8_SINT => Ok(Format::R8Sint),
            DXGI_FORMAT::DXGI_FORMAT_A8_UNORM => Ok(Format::A8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R1_UNORM => Ok(Format::R1Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R9G9B9E5_SHAREDEXP => Ok(Format::R9G9B9E5SharedExp),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_B8G8_UNORM => Ok(Format::R8G8UnormB8G8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_G8R8_G8B8_UNORM => Ok(Format::G8R8UnormG8B8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS => Ok(Format::BC1Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC1_UNORM => Ok(Format::BC1Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC1_UNORM_SRGB => Ok(Format::BC1UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_BC2_TYPELESS => Ok(Format::BC2Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC2_UNORM => Ok(Format::BC2Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC2_UNORM_SRGB => Ok(Format::BC2UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_BC3_TYPELESS => Ok(Format::BC3Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC3_UNORM => Ok(Format::BC3Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC3_UNORM_SRGB => Ok(Format::BC3UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_BC4_TYPELESS => Ok(Format::BC4Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC4_UNORM => Ok(Format::BC4Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC4_SNORM => Ok(Format::BC4Snorm),
            DXGI_FORMAT::DXGI_FORMAT_BC5_TYPELESS => Ok(Format::BC5Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC5_UNORM => Ok(Format::BC5Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC5_SNORM => Ok(Format::BC5Snorm),
            DXGI_FORMAT::DXGI_FORMAT_B5G6R5_UNORM => Ok(Format::B5G6R5Unorm),
            DXGI_FORMAT::DXGI_FORMAT_B5G5R5A1_UNORM => Ok(Format::B5G5R5A1Unorm),
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM => Ok(Format::B8G8R8A8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8X8_UNORM => Ok(Format::B8G8R8X8Unorm),
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM => {
                Ok(Format::R10G10B10XrBiasA2Unorm)
            }
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_TYPELESS => Ok(Format::B8G8R8A8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => Ok(Format::B8G8R8A8UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8X8_TYPELESS => Ok(Format::B8G8R8X8Typeless),
            DXGI_FORMAT::DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => Ok(Format::B8G8R8X8UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_BC6H_TYPELESS => Ok(Format::BC6HTypeless),
            DXGI_FORMAT::DXGI_FORMAT_BC6H_UF16 => Ok(Format::BC6HUFloat16),
            DXGI_FORMAT::DXGI_FORMAT_BC6H_SF16 => Ok(Format::BC6HSFloat16),
            DXGI_FORMAT::DXGI_FORMAT_BC7_TYPELESS => Ok(Format::BC7Typeless),
            DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM => Ok(Format::BC7Unorm),
            DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM_SRGB => Ok(Format::BC7UnormSRGB),
            DXGI_FORMAT::DXGI_FORMAT_AYUV => Ok(Format::AYUV),
            DXGI_FORMAT::DXGI_FORMAT_Y410 => Ok(Format::Y410),
            DXGI_FORMAT::DXGI_FORMAT_Y416 => Ok(Format::Y416),
            DXGI_FORMAT::DXGI_FORMAT_NV12 => Ok(Format::NV12),
            DXGI_FORMAT::DXGI_FORMAT_P010 => Ok(Format::P010),
            DXGI_FORMAT::DXGI_FORMAT_P016 => Ok(Format::P016),
            DXGI_FORMAT::DXGI_FORMAT_420_OPAQUE => Ok(Format::Opaque420),
            DXGI_FORMAT::DXGI_FORMAT_YUY2 => Ok(Format::YUY2),
            DXGI_FORMAT::DXGI_FORMAT_Y210 => Ok(Format::Y210),
            DXGI_FORMAT::DXGI_FORMAT_Y216 => Ok(Format::Y216),
            DXGI_FORMAT::DXGI_FORMAT_NV11 => Ok(Format::NV11),
            DXGI_FORMAT::DXGI_FORMAT_AI44 => Ok(Format::AI44),
            DXGI_FORMAT::DXGI_FORMAT_IA44 => Ok(Format::IA44),
            DXGI_FORMAT::DXGI_FORMAT_P8 => Ok(Format::P8),
            DXGI_FORMAT::DXGI_FORMAT_A8P8 => Ok(Format::A8P8),
            DXGI_FORMAT::DXGI_FORMAT_B4G4R4A4_UNORM => Ok(Format::B4G4R4A4Unorm),
            DXGI_FORMAT::DXGI_FORMAT_P208 => Ok(Format::P208),
            DXGI_FORMAT::DXGI_FORMAT_V208 => Ok(Format::V208),
            DXGI_FORMAT::DXGI_FORMAT_V408 => Ok(Format::V408),
            DXGI_FORMAT::DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE => {
                Ok(Format::SamplerFeedbackMinMipOpaque)
            }
            DXGI_FORMAT::DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE => {
                Ok(Format::SamplerFeedbackMipRegionUsedOpaque)
            }
            _ => Err(()),
        }
    }
}

impl Into<DXGI_FORMAT> for Format {
    fn into(self) -> DXGI_FORMAT {
        DXGI_FORMAT(self as u32)
    }
}
