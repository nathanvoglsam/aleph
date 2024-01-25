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

use aleph_vk_format::VkFormat;

use crate::format::is_format_prohibited;
use crate::is_format_unsupported;

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

    ///
    /// Is the color model correct for the given VkFormat
    ///
    #[inline]
    pub fn is_compatible_with_format(self, format: VkFormat) -> bool {
        let is_srgb = format.is_srgb();
        let allowed = !is_format_unsupported(format) && !is_format_prohibited(format);
        matches!(
            (allowed, is_srgb, self),
            (true, true, TransferFunction::SRGB) | (true, false, TransferFunction::Linear)
        )
    }

    ///
    /// Gets a TransferFunction value from the given format
    ///
    #[inline]
    pub fn for_format(format: VkFormat) -> Option<Self> {
        if is_format_prohibited(format) || is_format_unsupported(format) {
            None
        } else if format.is_srgb() {
            Some(TransferFunction::SRGB)
        } else {
            Some(TransferFunction::Linear)
        }
    }
}
