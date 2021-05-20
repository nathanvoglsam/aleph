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

use crate::archive::{ArchiveID, AssetLocalID};
use std::error::Error;
use thiserror::Error;

///
/// Error enum for lookup operations with `IArchive`
///
#[derive(Error, Debug)]
pub enum AssetLookupError {
    #[error("Asset lookup failed in archive {archive}, there is no asset with the id {id}")]
    NotFound {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The id that was used to attempt to lookup an asset
        id: AssetLocalID,
    },

    #[error(
        "Asset lookup failed due to an unknown reason in archive {archive}. Error: \"{error}\""
    )]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}

///
/// Error enum for insertion operations with `IArchiveMut`
///
#[derive(Error, Debug)]
pub enum AssetInsertError {
    #[error(
        "Asset insert failed due to an unknown reason in archive {archive}. Error: \"{error}\""
    )]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}

///
/// Error enum for removal operations with `IArchiveMut`
///
#[derive(Error, Debug)]
pub enum AssetRemoveError {
    #[error("Asset removal failed in archive {archive}, there is no asset with the id {id}")]
    NotFound {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The id that was used to attempt to remove an asset
        id: AssetLocalID,
    },

    #[error(
        "Asset removal failed due to an unknown reason in archive {archive}. Error: \"{error}\""
    )]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}
