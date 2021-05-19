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
/// A bitflags struct that specifies configuration options for entry insertion operations on an
/// `IFolderHierarchyMut` implementation
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EntryInsertFlags(u32);

crate::flags_impl!(EntryInsertFlags, u32);

impl EntryInsertFlags {
    /// Empty flags
    pub const NONE: Self = Self(0);

    /// This flag enables overwriting files and replacing them with a new entry.
    pub const OVERWRITE_FILE: Self = Self(0b1);

    /// This flag enables overwriting folders and replacing them with a new entry.
    pub const OVERWRITE_FOLDER: Self = Self(0b10);

    /// A combination of `OVERWRITE_FILE` and `OVERWRITE_FOLDER`
    pub const OVERWRITE: Self = Self(Self::OVERWRITE_FILE.0 | Self::OVERWRITE_FOLDER.0);

    /// This flag tells the implementation to create any missing folders that would otherwise
    /// prevent the item from being created
    pub const CREATE_HIERARCHY: Self = Self(0b100);
}

///
/// A bitflags struct that specifies configuration options for entry removal operations on an
/// `IFolderHierarchyMut` implementation
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EntryRemoveFlags(u32);

crate::flags_impl!(EntryRemoveFlags, u32);

impl EntryRemoveFlags {
    /// Empty flags
    pub const NONE: Self = Self(0);

    /// This flag enables recursively deleting the contents of a folder. Without this flag set only
    /// empty folders or individual files can be removed.
    pub const RECURSIVE: Self = Self(0b1);

    /// This flag tells the implementation to not produce an error if the entry to be removed does
    /// not exist
    pub const IGNORE_MISSING: Self = Self(0b10);
}
