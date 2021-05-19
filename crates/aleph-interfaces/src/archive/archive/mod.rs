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

use crate::any::{AnyArc, IAny};
use crate::archive::{
    ArchiveID, AssetInsertError, AssetLocalID, AssetLookupError, AssetRemoveError,
    IFolderHierarchy, IFolderHierarchyMut,
};
use crate::asset::AssetDescriptor;

mod flags;

pub use flags::AssetInsertFlags;
pub use flags::AssetRemoveFlags;

///
/// `IArchive` provides the required interface for an archive format to integrate with the aleph
/// framework.
///
pub trait IArchive: IAny + Send + Sync + 'static {
    /// Returns the archive's identifier. This is used as part of `AssetID` and must be a stable
    /// value that is inseparable from the archive.
    fn identifier(&self) -> ArchiveID;

    /// Will lookup a value by its ID that is local to this specific archive.
    fn lookup(&self, id: AssetLocalID) -> Result<AssetDescriptor, AssetLookupError>;

    /// Returns the `IFolderHierarchy` implementation for the archive, if one exists.
    fn hierarchy(&self) -> Option<AnyArc<dyn IFolderHierarchy>>;
}

///
/// A specialised version of `IArchive` that supports modifying assets within the archive. This
/// will typically be used by an `IArchive` implementation backed by the OS filesystem rather than
/// a package file.
///
/// This interface should not be used within a shipped game. This interface's intended use-case is
/// for dev tooling, such as an editor providing an interface for inserting or removing assets from
/// the project.
///
pub trait IArchiveMut: IArchive {
    /// Will attempt to insert a new asset into the archive.
    ///
    /// This function will return a valid local ID for the asset that has been generated for the new
    /// asset by the `IArchive` implementation.
    fn insert(&self, flags: AssetInsertFlags) -> Result<AssetLocalID, AssetInsertError>;

    /// Will attempt to remove the given asset from the archive.
    fn remove(&self, id: AssetLocalID, flags: AssetRemoveFlags) -> Result<(), AssetRemoveError>;

    /// Returns the `IFolderHierarchyMut` implementation for the archive, if one exists.
    fn hierarchy_mut(&self) -> Option<AnyArc<dyn IFolderHierarchyMut>>;
}
