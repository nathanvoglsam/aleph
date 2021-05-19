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
use crate::archive::{
    AssetLocalID, EntryError, EntryInsertError, EntryLookupError, EntryRemoveError,
};

mod entry;
mod flags;

pub use entry::Entry;
pub use entry::EntryMut;
pub use entry::EntryType;
pub use flags::EntryInsertFlags;
pub use flags::EntryRemoveFlags;

///
///
///
pub trait IFolderHierarchy: IAny + Send + Sync + 'static {
    /// Lookup an entry relative to the folder the `IFolder` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to lookup the entry
    /// under.
    fn lookup(&self, path: &str) -> Result<Entry, EntryLookupError>;

    /// Returns an `IFolder` instance that refers to the root directory
    fn folder(&self) -> Box<dyn IFolder>;
}

///
///
///
pub trait IFolderHierarchyMut: IFolderHierarchy {
    /// Lookup an entry relative to the folder the `IFolder` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to lookup the entry
    /// under.
    fn lookup_mut(&self, path: &str) -> Result<EntryMut, EntryLookupError>;

    ///
    /// Creates a new file inside the root of the hierarchy. The `path`
    /// parameter specifies the path relative to the folder `self` represents to create the file
    /// under.
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
    fn create_folder(
        &self,
        path: &str,
        flags: EntryInsertFlags,
    ) -> Result<Box<dyn IFolderMut>, EntryInsertError>;

    ///
    /// Removes the entry inside the root of the hierarchy. The `path` parameter specifies the path
    /// relative to the folder `self` represents to remove the entry on.
    ///
    fn remove_entry(&self, path: &str, flags: EntryRemoveFlags) -> Result<(), EntryRemoveError>;

    /// Returns an `IFolder` instance that refers to the root directory
    fn folder_mut(&self) -> Box<dyn IFolderMut>;
}

///
/// An interface that implements a weak reference to a folder entry within an `IFolderHierarchy`.
///
/// An instance is only a weak reference to the underlying folder. The underlying folder may be
/// deleted while an `IFolder` still refers to it. When this happens all operations will produce a
/// `DanglingEntry` error and the `IFolder` should be discarded as it can not be re-associated with
/// a different folder, or even the same folder if it is recreated.
///
pub trait IFolder: IAny + Send + Sync + 'static {
    /// Returns the fully qualified path to the folder `self` refers to.
    fn path(&self) -> Result<String, EntryError>;

    /// Lookup an entry relative to the folder the `IFolder` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to lookup the entry
    /// under.
    fn lookup(&self, path: &str) -> Result<Entry, EntryLookupError>;

    /// Returns an iterator over all child entries inside the folder `self` refers to.
    fn child_iter(&self) -> Result<Box<dyn IFolderIter>, EntryError>;
}

///
/// An interface that implements a weak reference to a folder entry within an `IFolderHierarchy`.
///
/// This interface extents `IFolder` by allowing the hierarchy to be mutated through the interface.
///
pub trait IFolderMut: IFolder {
    /// Lookup an entry relative to the folder the `IFolderMut` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to lookup the entry
    /// under.
    fn lookup_mut(&self, path: &str) -> Result<EntryMut, EntryLookupError>;

    ///
    /// Creates a new file inside the folder the `IFolderMut` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to create the file
    /// under.
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
    /// Creates a new folder inside the folder the `IFolderMut` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to create the folder
    /// under.
    ///
    fn create_folder(
        &self,
        path: &str,
        flags: EntryInsertFlags,
    ) -> Result<Box<dyn IFolderMut>, EntryInsertError>;

    ///
    /// Removes the entry inside the folder the `IFolderMut` instance represents. The `path`
    /// parameter specifies the path relative to the folder `self` represents to remove the entry
    /// on.
    ///
    fn remove_entry(&self, path: &str, flags: EntryRemoveFlags) -> Result<(), EntryRemoveError>;

    /// Returns an iterator over all child entries inside the folder `self` refers to.
    fn child_iter_mut(&self) -> Result<Box<dyn IFolderIterMut>, EntryError>;
}

///
/// Trait alias for an iterator that yields `Entry` enums.
///
/// Used by `IFolder`
///
pub trait IFolderIter: Iterator<Item = Result<Entry, EntryLookupError>> {}

///
/// Trait alias for an iterator that yields `EntryMut` enums.
///
pub trait IFolderIterMut: Iterator<Item = Result<EntryMut, EntryLookupError>> {}
