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

use crate::archive::{ArchiveID, EntryType, PathError};
use std::error::Error;
use thiserror::Error;

///
/// Error enum for lookup operations with `IFolderHierarchy`
///
#[derive(Error, Debug)]
pub enum EntryLookupError {
    #[error("Entry lookup failed in archive {archive}, no entry with the path \"{path}\"")]
    NotFound {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The path that was used to attempt to lookup an entry
        path: String,
    },

    #[error("Entry lookup failed in archive {archive}. Reason: {error}")]
    PathError {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying path error
        error: PathError,
    },

    #[error("Entry lookup failed in archive {archive}, path \"{path}\" wrong entry type. Expected {expected}, got {actual}.")]
    WrongEntryType {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The path that was used to attempt to lookup an entry
        path: String,

        /// The type of entry that the operation expected
        expected: EntryType,

        /// The type of entry that was actually found
        actual: EntryType,
    },

    #[error("Entry lookup failed for unknown reason in archive {archive}. Error: \"{error}\"")]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}

///
/// Error enum for insertion operations with `IFolderHierarchyMut`
///
#[derive(Error, Debug)]
pub enum EntryInsertError {
    #[error("Entry insertion failed in archive {archive}, attempted to create \"{path}\" but an entry already exists there")]
    AlreadyExists {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The path to the entry that was dangling
        path: String,

        /// The type of the existing entity
        existing_type: EntryType,
    },

    #[error("Entry lookup failed in archive {archive}. Reason: {error}")]
    PathError {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying path error
        error: PathError,
    },

    #[error("Entry removal failed for unknown reason in archive {archive}. Error: \"{error}\"")]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}

///
/// Error enum for removal operations with `IFolderHierarchyMut`
///
#[derive(Error, Debug)]
pub enum EntryRemoveError {
    #[error("Entry removal failed in archive {archive}, no entry with the path \"{path}\"")]
    NotFound {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The path that was used to attempt to lookup an entry
        path: String,
    },

    #[error("Entry lookup failed in archive {archive}. Reason: {error}")]
    PathError {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying path error
        error: PathError,
    },

    #[error("Entry removal failed for unknown reason in archive {archive}. Error: \"{error}\"")]
    Unknown {
        /// The ID of the archive that the error was caused by
        archive: ArchiveID,

        /// The underlying error that was thrown by the implementation
        error: Box<dyn Error>,
    },
}
