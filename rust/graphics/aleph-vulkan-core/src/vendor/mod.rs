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

const VK_VENDOR_ID_AMD: u32 = 0x1002;
const VK_VENDOR_ID_IMGTECH: u32 = 0x1010;
const VK_VENDOR_ID_NVIDIA: u32 = 0x10DE;
const VK_VENDOR_ID_ARM: u32 = 0x13B5;
const VK_VENDOR_ID_QUALCOMM: u32 = 0x5143;
const VK_VENDOR_ID_INTEL: u32 = 0x8086;

///
/// A simple enum to represent a vulkan device ID
///
/// Provides explicit variants for known vendors, otherwise stores the ID in the `Unknown` variant
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum VendorID {
    AMD,
    ImgTech,
    NVIDIA,
    ARM,
    Qualcomm,
    Intel,
    Unknown(u32),
}

impl VendorID {
    ///
    /// Given a raw vendor ID, convert it into our wrapper type
    ///
    #[inline]
    pub fn from_raw(id: u32) -> Self {
        if id == VK_VENDOR_ID_AMD {
            VendorID::AMD
        } else if id == VK_VENDOR_ID_IMGTECH {
            VendorID::ImgTech
        } else if id == VK_VENDOR_ID_NVIDIA {
            VendorID::NVIDIA
        } else if id == VK_VENDOR_ID_ARM {
            VendorID::ARM
        } else if id == VK_VENDOR_ID_QUALCOMM {
            VendorID::Qualcomm
        } else if id == VK_VENDOR_ID_INTEL {
            VendorID::Intel
        } else {
            VendorID::Unknown(id)
        }
    }

    ///
    /// Convert our wrapped type back into the raw vulkan vendor ID
    ///
    #[inline]
    pub fn to_raw(self) -> u32 {
        match self {
            VendorID::AMD => VK_VENDOR_ID_AMD,
            VendorID::ImgTech => VK_VENDOR_ID_IMGTECH,
            VendorID::NVIDIA => VK_VENDOR_ID_NVIDIA,
            VendorID::ARM => VK_VENDOR_ID_ARM,
            VendorID::Qualcomm => VK_VENDOR_ID_QUALCOMM,
            VendorID::Intel => VK_VENDOR_ID_INTEL,
            VendorID::Unknown(id) => id,
        }
    }

    ///
    /// Get the name of the vendor
    ///
    /// Just returns "Unknown" for unknown vendors
    ///
    #[inline]
    pub fn vendor_name(self) -> &'static str {
        match self {
            VendorID::AMD => "AMD",
            VendorID::ImgTech => "Imagination Technologies",
            VendorID::NVIDIA => "NVIDIA",
            VendorID::ARM => "ARM",
            VendorID::Qualcomm => "Qualcomm",
            VendorID::Intel => "Intel",
            VendorID::Unknown(_) => "Unknown",
        }
    }
}
