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

use crate::archive::{ArchiveID, AssetID};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::num::{NonZeroU64, TryFromIntError};

///
/// A unique ID into an *archive* that refers to a distinct asset. This ID is not globally unique
/// to all assets mounted by the asset system, it is only unique within an individual archive.
/// To uniquely identify an asset globally use `AssetID`.
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct AssetLocalID(NonZeroU64);

impl Display for AssetLocalID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        super::utils::display_id(f, self.0)
    }
}

impl AssetLocalID {
    /// Produces a global `AssetID` by pairing this local ID with the given archive ID
    pub fn associate_with_archive(self, archive: ArchiveID) -> AssetID {
        AssetID(archive, self)
    }
}

impl From<NonZeroU64> for AssetLocalID {
    fn from(val: NonZeroU64) -> Self {
        Self(val)
    }
}

impl TryFrom<u64> for AssetLocalID {
    type Error = TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let out = Self(NonZeroU64::try_from(value)?);
        Ok(out)
    }
}

impl Into<NonZeroU64> for AssetLocalID {
    fn into(self) -> NonZeroU64 {
        self.0
    }
}

impl Into<u64> for AssetLocalID {
    fn into(self) -> u64 {
        self.0.get()
    }
}
