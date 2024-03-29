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

use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SuperCompressionScheme(pub u32);

impl SuperCompressionScheme {
    ///
    /// None, or uncompressed data
    ///
    pub const NONE: Self = Self(0);

    ///
    /// Compressed with the BasisLZ scheme as described by the KTX 2.0 spec
    ///
    pub const BASIS_LZ: Self = Self(1);

    ///
    /// Compressed with the Zstandard scheme as described by the KTX 2.0 spec
    ///
    pub const ZSTD: Self = Self(2);

    ///
    /// Compressed with the Zlib scheme as described by the KTX 2.0 spec
    ///
    pub const ZLIB: Self = Self(3);

    ///
    /// Is this compression scheme supported by our implementation
    ///
    pub fn is_supported(self) -> bool {
        matches!(self, SuperCompressionScheme::NONE)
    }

    ///
    /// Is this a compression scheme provided by the KTX 2.0 spec itself
    ///
    pub fn is_core_scheme(self) -> bool {
        matches!(
            self,
            SuperCompressionScheme::NONE
                | SuperCompressionScheme::BASIS_LZ
                | SuperCompressionScheme::ZSTD
                | SuperCompressionScheme::ZLIB
        )
    }

    ///
    /// Whether this compression scheme expects to have global data
    ///
    pub fn has_global_data(self) -> bool {
        match self {
            Self::NONE => false,
            Self::BASIS_LZ => true,
            Self::ZSTD => false,
            Self::ZLIB => false,
            _ => false,
        }
    }

    ///
    /// Returns whether the compression scheme is known to the implementation.
    ///
    /// # Info
    ///
    /// See `VkFormat::is_known`'s documentation for more info. This function exists for the same
    /// reasons, but for any new compression schemes that may be added
    ///
    pub fn is_known(self) -> bool {
        matches!(
            self,
            SuperCompressionScheme::NONE
                | SuperCompressionScheme::BASIS_LZ
                | SuperCompressionScheme::ZSTD
                | SuperCompressionScheme::ZLIB
        )
    }

    ///
    /// Is this scheme ID in the range reserved for use exclusively by the KTX itself
    ///
    pub fn is_in_ktx_reserved(self) -> bool {
        self.0 >= 4 && self.0 <= 0xffff
    }

    ///
    /// Is this scheme ID in the range reserved for use for third party compression schemes that
    /// can be provided by other vendors.
    ///
    /// This function is useful for detecting if the file is using a non standard compression scheme
    ///
    pub fn is_in_vendor_reserved(self) -> bool {
        self.0 >= 0x10000 && self.0 <= 0x1ffff
    }

    ///
    /// Is this scheme ID in the range reserved as "do not use".
    ///
    /// This is an error so we should at least provide a way for it to be checked I suppose
    ///
    pub fn is_in_invalid_reserved(self) -> bool {
        self.0 >= 0x20000
    }
}

impl Debug for SuperCompressionScheme {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str(match *self {
            Self::NONE => "NONE",
            Self::BASIS_LZ => "BASIS_LZ",
            Self::ZSTD => "ZSTD",
            Self::ZLIB => "ZLIB",
            _ => {
                if self.is_in_ktx_reserved() {
                    "(unknown):KTX_RESERVED"
                } else if self.is_in_vendor_reserved() {
                    "(unknown):VENDOR_RESERVED"
                } else if self.is_in_invalid_reserved() {
                    "(unknown):INVALID_RESERVED"
                } else {
                    "(unknown)"
                }
            }
        })
    }
}
