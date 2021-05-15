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

mod archive_id;
mod asset_id;
mod asset_local_id;
mod error;
mod utils;

use crate::asset::AssetDescriptor;
pub use archive_id::ArchiveID;
pub use asset_id::AssetID;
pub use asset_local_id::AssetLocalID;
pub use error::AssetInsertError;
pub use error::AssetLookupError;
pub use error::AssetRemoveError;

///
/// `IArchive` provides a generic interface for accessing an asset archive that is separated from
/// concrete implementation. By using this trait it is possible to abstract over development assets
/// on an OS filesystem and packaged release assets in an archive.
///
pub trait IArchive: IAny + Send + Sync + 'static {
    /// Returns the archive's identifier. This is used as part of `AssetID` and must be a stable
    /// value that is not separable from the archive itself.
    fn identifier(&self) -> ArchiveID;

    /// Will lookup a value by its ID that is local to this specific archive.
    fn lookup(&self, id: AssetLocalID) -> Result<AssetDescriptor, AssetLookupError>;

    /// Will attempt to resolve a path to an ID.
    fn resolve_path(&self, path: &str) -> Result<AssetLocalID, AssetLookupError>;

    /// Will lookup an asset by path.
    ///
    /// Logically is simply a combination of `Self::resolve_path` and `Self::lookup` to combine
    /// resolving a path and looking up by ID into a single operation, which is exactly what the
    /// default implementation does.
    ///
    /// An `IArchive` implementation could also provide a more efficient custom implementation.
    ///
    /// For example an `IArchive` implementation could be based on a `.zip` archive where paths are
    /// the only way to lookup assets so a custom implementation could skip converting an ID to a
    /// path inside the zip file.
    fn lookup_path(&self, path: &str) -> Result<AssetDescriptor, AssetLookupError> {
        let id = self.resolve_path(path)?;
        self.lookup(id)
    }
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
pub trait IMutableArchive: IArchive {
    /// Will attempt to insert a new asset into the archive.
    ///
    /// This function takes a virtual path that the asset should be inserted under, such as
    /// "textures/img.png".
    ///
    /// This function will return a valid local ID for the asset that has been generated for the new
    /// asset by the `IArchive` implementation.
    fn insert(&self, path: &str) -> Result<AssetLocalID, AssetInsertError>;

    /// Will attempt to remove the given asset from the archive.
    fn remove(&self, id: AssetLocalID) -> Result<(), AssetRemoveError>;

    /// Will attempt to remove an asset at the specified path from the archive.
    ///
    /// Similar to `IArchive::lookup_path`, this is logically a combination of
    /// `IArchive::resolve_path` and `IMutableArchive::remove` which resolves the path and then
    /// removes it by ID.
    fn remove_path(&self, path: &str) -> Result<(), AssetRemoveError> {
        let id = self.resolve_path(path)?;
        self.remove(id)
    }
}
