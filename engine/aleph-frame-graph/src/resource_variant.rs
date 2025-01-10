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

use aleph_rhi_api::*;

pub enum ResourceVariant {
    Buffer(BufferHandle),
    Texture(TextureHandle),
}

impl ResourceVariant {
    pub const fn is_buffer(&self) -> bool {
        match self {
            ResourceVariant::Buffer(_) => true,
            ResourceVariant::Texture(_) => false,
        }
    }

    pub const fn is_texture(&self) -> bool {
        match self {
            ResourceVariant::Buffer(_) => false,
            ResourceVariant::Texture(_) => true,
        }
    }
}

impl From<BufferHandle> for ResourceVariant {
    fn from(value: BufferHandle) -> Self {
        Self::Buffer(value)
    }
}

impl From<TextureHandle> for ResourceVariant {
    fn from(value: TextureHandle) -> Self {
        Self::Texture(value)
    }
}

impl<'a> From<&'a BufferHandle> for ResourceVariant {
    fn from(value: &'a BufferHandle) -> Self {
        let value = value.clone();
        Self::Buffer(value)
    }
}

impl<'a> From<&'a TextureHandle> for ResourceVariant {
    fn from(value: &'a TextureHandle) -> Self {
        let value = value.clone();
        Self::Texture(value)
    }
}
