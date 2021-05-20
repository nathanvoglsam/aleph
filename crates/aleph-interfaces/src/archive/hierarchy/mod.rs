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

use crate::any::IAny;
use crate::archive::{AssetLocalID, EntryInsertError, EntryLookupError, EntryRemoveError};

mod entry;
mod flags;

pub use entry::Entry;
pub use entry::EntryType;
pub use flags::EntryInsertFlags;
pub use flags::EntryRemoveFlags;

///
///
///
pub trait IFolderHierarchy: IAny + Send + Sync + 'static {
    /// Lookup an entry relative to the root of the hierarchy. The `path` parameter specifies the
    /// path relative to the folder `self` represents to lookup the entry under.
    fn lookup(&self, path: &str) -> Result<Entry, EntryLookupError>;

    /// Lookup an entry relative to the root of the hierarchy. The `path` parameter specifies the
    /// path relative to the folder `self` represents to lookup the entry under.
    fn lookup_type(&self, path: &str) -> Result<EntryType, EntryLookupError>;

    fn read_dir(&self, path: &str) -> Result<Box<dyn IReadDir>, EntryLookupError>;
}

///
///
///
pub trait IFolderHierarchyMut: IFolderHierarchy {
    ///
    /// Creates a new file inside the root of the hierarchy. The `path` parameter specifies the path
    /// relative to the folder `self` represents to create the file under.
    ///
    /// The `asset` parameter specifies the local asset id that the file refers to.
    ///
    fn create_file(
        &self,
        path: &str,
        asset: AssetLocalID,
        flags: EntryInsertFlags,
    ) -> Result<(), EntryInsertError>;

    ///
    /// Creates a new folder inside the root of the hierarchy. The `path` parameter specifies the
    /// path relative to the folder `self` represents to create the folder under.
    ///
    fn create_folder(&self, path: &str, flags: EntryInsertFlags) -> Result<(), EntryInsertError>;

    ///
    /// Removes the entry inside the root of the hierarchy. The `path` parameter specifies the path
    /// relative to the folder `self` represents to remove the entry on.
    ///
    fn remove_entry(&self, path: &str, flags: EntryRemoveFlags) -> Result<(), EntryRemoveError>;
}

///
/// Trait alias for an iterator that yields `Entry` enums.
///
pub trait IReadDir: Iterator<Item = Result<Entry, EntryLookupError>> {}
