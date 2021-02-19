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

use raw::windows::win32::dxgi::DXGI_FORMAT;

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

impl Into<DXGI_FORMAT> for Format {
    fn into(self) -> DXGI_FORMAT {
        DXGI_FORMAT(self as u32)
    }
}
