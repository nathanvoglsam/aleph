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

use std::io::{Read, Seek};

/// A readable and seekable stream
pub trait IAssetStream: Read + Seek {}

///
/// Enumerates the possible locations for the asset system to load data from.
///
/// An `AssetDescriptor` describes where the data is located so it can be loaded into
///
pub enum AssetDescriptor {
    /// A slice to the asset's data that will be resident for the remainder of the program's
    /// lifetime. This is distinct from `AssetDescriptor::MemoryMapped` in that the data is backed
    /// by committed memory and not a memory mapped file.
    ///
    /// This variant should be used to flag asset data that needs no further loading and will
    /// always be accessible. A user could optimize by choosing to use this buffer directly.
    PermanentlyResident(&'static [u8]),

    /// A slice to the asset's data within a permanently memory mapped archive. This is distinct
    /// from `AssetDescriptor::PermanentlyResident` in that the data is not actually guaranteed to
    /// be resident in memory so reading the data may cause page faults to load from disk, which
    /// will significantly impact performance.
    ///
    /// This variant should be used to flag memory mapped resources so that users can optimize by
    /// copying out of the memory mapped region into a committed address range.
    MemoryMapped(&'static [u8]),

    /// A readable and seekable stream that can yield the asset's data. This would typically be used
    /// when the data is located in a file on disk and the implementation returns an object to read
    /// from the file.
    Stream(Box<dyn IAssetStream>),
}
