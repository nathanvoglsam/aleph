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

#[derive(
    serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(rename_all = "camelCase")]
pub enum ScalarType {
    Void,
    Bool,
    Float16,
    Float32,
    Float64,
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Intptr,
    Uintptr,
}

#[derive(
    serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(rename_all = "camelCase")]
pub enum ShaderStage {
    None,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Compute,
    Mesh,
    Amplification,
}

impl Default for ShaderStage {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(rename_all = "camelCase")]
pub enum ResourceAccess {
    Read,
    Write,
    ReadWrite,
    RasterOrdered,
    Append,
    Consume,
    Feedback,
}

impl Default for ResourceAccess {
    fn default() -> Self {
        Self::Read
    }
}
