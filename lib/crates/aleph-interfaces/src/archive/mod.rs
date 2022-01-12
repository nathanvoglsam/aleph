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

mod archive;
mod archive_id;
mod asset_id;
mod asset_local_id;
mod error;
mod hierarchy;
mod utils;

pub use archive::AssetInsertFlags;
pub use archive::AssetRemoveFlags;
pub use archive::IArchive;
pub use archive::IArchiveMut;
pub use archive_id::ArchiveID;
pub use asset_id::AssetID;
pub use asset_local_id::AssetLocalID;
pub use error::AssetInsertError;
pub use error::AssetLookupError;
pub use error::AssetRemoveError;
pub use error::EntryInsertError;
pub use error::EntryLookupError;
pub use error::EntryRemoveError;
pub use error::PathError;
pub use hierarchy::Entry;
pub use hierarchy::EntryInsertFlags;
pub use hierarchy::EntryRemoveFlags;
pub use hierarchy::EntryType;
pub use hierarchy::IFolderHierarchy;
pub use hierarchy::IFolderHierarchyMut;
pub use hierarchy::IReadDir;
