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

use crate::archive::{AssetLocalID, IFolder, IFolderMut};
use std::fmt::{Display, Formatter};

///
/// An enum over the types of entries within a folder hierarchy
///
pub enum Entry {
    /// The entity is a folder
    Folder(Box<dyn IFolder>),

    /// The entity is a file, with its local asset id attached
    File(AssetLocalID),
}

impl Entry {
    /// Returns the `EntryType` that matches the variant of `self`.
    ///
    /// This essentially just discards the associated values, useful for when you need to store the
    /// type without the associated values in a struct somewhere.
    pub fn entry_type(&self) -> EntryType {
        match self {
            Entry::Folder(_) => EntryType::Folder,
            Entry::File(_) => EntryType::File,
        }
    }

    /// Converts to an optional `IFolder` to shorten cases where a folder is assumed
    pub fn folder(self) -> Option<Box<dyn IFolder>> {
        match self {
            Entry::Folder(v) => Some(v),
            Entry::File(_) => None,
        }
    }

    /// Converts to an optional `AssetLocalID` to shorten cases where a file is assumed
    pub fn file(self) -> Option<AssetLocalID> {
        match self {
            Entry::Folder(_) => None,
            Entry::File(v) => Some(v),
        }
    }
}

///
/// An enum over the types of entries within a folder hierarchy
///
pub enum EntryMut {
    /// The entity is a folder
    Folder(Box<dyn IFolderMut>),

    /// The entity is a file, with its local asset id attached
    File(AssetLocalID),
}

impl EntryMut {
    /// Returns the `EntryType` that matches the variant of `self`.
    ///
    /// This essentially just discards the associated values, useful for when you need to store the
    /// type without the associated values in a struct somewhere.
    pub fn entry_type(&self) -> EntryType {
        match self {
            EntryMut::Folder(_) => EntryType::Folder,
            EntryMut::File(_) => EntryType::File,
        }
    }

    /// Converts to an optional `IFolderMut` to shorten cases where a folder is assumed
    pub fn folder(self) -> Option<Box<dyn IFolderMut>> {
        match self {
            EntryMut::Folder(v) => Some(v),
            EntryMut::File(_) => None,
        }
    }

    /// Converts to an optional `AssetLocalID` to shorten cases where a file is assumed
    pub fn file(self) -> Option<AssetLocalID> {
        match self {
            EntryMut::Folder(_) => None,
            EntryMut::File(v) => Some(v),
        }
    }
}

///
/// An enum over the types of entries within a folder hierarchy
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EntryType {
    /// The entity is a folder
    Folder,

    /// The entity is a file
    File,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Folder => f.write_str("EntryType::Folder"),
            EntryType::File => f.write_str("EntryType::File"),
        }
    }
}
