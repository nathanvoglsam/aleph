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

use crate::resource::{ImageFormat, ResourceAccessType};

///
/// Structure that provides the description of a texture resource that will be created by a render
/// graph
///
#[derive(Clone)]
pub struct TextureCreateDesc {
    /// The width of the render target in pixels
    pub width: u32,

    /// The height of the render target in pixels
    pub height: u32,

    /// The pixel format of the render target
    pub format: ImageFormat,
    // /// What type of memory heap should the texture be allocated in
    // pub memory_usage: vma::MemoryUsage,
}

impl From<TextureCreateDesc> for ResourceCreateDesc {
    fn from(v: TextureCreateDesc) -> Self {
        ResourceCreateDesc::Texture(v)
    }
}

///
/// Structure that provides the description of a buffer resource that will be created by a render
/// graph
///
#[derive(Clone)]
pub struct BufferCreateDesc {
    /// The size, in bytes, of the buffer
    pub size: u32,
    // /// What type of memory heap should the buffer be allocated in
    // pub memory_usage: vma::MemoryUsage,
}

impl From<BufferCreateDesc> for ResourceCreateDesc {
    fn from(v: BufferCreateDesc) -> Self {
        ResourceCreateDesc::Buffer(v)
    }
}

///
/// A sum type of [`TextureCreateDesc`] and [`BufferCreateDesc`]
///
#[derive(Clone)]
pub enum ResourceCreateDesc {
    Texture(TextureCreateDesc),
    Buffer(BufferCreateDesc),
}

/// TODO: Docs
#[derive(Clone)]
pub struct BufferReadDesc {
    pub usage: ResourceAccessType,
}

/// TODO: Docs
#[derive(Clone)]
pub struct BufferWriteDesc {
    pub usage: ResourceAccessType,
}

/// TODO: Docs
#[derive(Clone)]
pub struct TextureReadDesc {
    pub usage: ResourceAccessType,
}

/// TODO: Docs
#[derive(Clone)]
pub struct TextureWriteDesc {
    pub usage: ResourceAccessType,
}

/// TODO: Docs
#[derive(Clone)]
pub enum ResourceReadDesc {
    Buffer(BufferReadDesc),
    Texture(TextureReadDesc),
}

/// TODO: Docs
#[derive(Clone)]
pub enum ResourceWriteDesc {
    Buffer(BufferWriteDesc),
    Texture(TextureWriteDesc),
}

/// TODO: Docs
#[derive(Clone)]
pub struct TextureImportDesc {
    /// The width of the render target in pixels
    pub width: u32,

    /// The height of the render target in pixels
    pub height: u32,

    /// The pixel format of the render target
    pub format: ImageFormat,

    /// The initial state of the resource
    pub access_type: ResourceAccessType,
}

impl From<TextureImportDesc> for ResourceImportDesc {
    fn from(v: TextureImportDesc) -> Self {
        ResourceImportDesc::Texture(v)
    }
}

/// TODO: Docs
#[derive(Clone)]
pub struct BufferImportDesc {
    /// The size, in bytes, of the buffer
    pub size: u32,
}

impl From<BufferImportDesc> for ResourceImportDesc {
    fn from(v: BufferImportDesc) -> Self {
        ResourceImportDesc::Buffer(v)
    }
}

///
/// A sum type of [`TextureImportDesc`] and [`BufferImportDesc`]
///
#[derive(Clone)]
pub enum ResourceImportDesc {
    Texture(TextureImportDesc),
    Buffer(BufferImportDesc),
}
