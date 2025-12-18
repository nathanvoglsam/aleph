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

use crate::material::MaterialBindingType;
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum MaterialBinding {
    /// Data for a 'texture' type material binding. The type of view is specific to the binding slot
    /// and the material it is associated with.
    Texture(Option<TextureHandle>),

    /// Data for a 'buffer' type material binding. The type of buffer expected is specific to the
    /// binding slot and the material it is associated with.
    Buffer(Option<BufferHandle>),
}

impl MaterialBinding {
    /// Overwrite the handle 'self' contains with the handle in the source [`MaterialBinding`].
    ///
    /// Will return [`None`] if the variant of 'self' and 'src' do not match.
    pub fn overwrite(&mut self, src: &Self) -> Option<()> {
        match (self, src) {
            (MaterialBinding::Texture(dst), MaterialBinding::Texture(v)) => {
                *dst = *v;
            }
            (MaterialBinding::Buffer(dst), MaterialBinding::Buffer(v)) => {
                *dst = *v;
            }
            _ => {
                return None;
            }
        }
        Some(())
    }

    /// Returns a [`MaterialBindingType`] that correctly describes the binding type of 'self'
    pub const fn binding_type(&self) -> MaterialBindingType {
        match self {
            MaterialBinding::Texture(_) => MaterialBindingType::Texture,
            MaterialBinding::Buffer(_) => MaterialBindingType::Buffer,
        }
    }

    /// Assume 'self' contains a [`MaterialBinding::Texture`] and return the inner handle.
    ///
    /// Panics if 'self' does is the wrong type.
    pub fn unwrap_texture(&self) -> Option<TextureHandle> {
        match self {
            MaterialBinding::Texture(v) => *v,
            MaterialBinding::Buffer(_) => panic!("Expected self to be 'Texture' but is 'Buffer'"),
        }
    }

    /// Assume 'self' contains a [`MaterialBinding::Buffer`] and return the inner handle.
    ///
    /// Panics if 'self' does is the wrong type.
    pub fn unwrap_buffer(&self) -> Option<BufferHandle> {
        match self {
            MaterialBinding::Texture(_) => panic!("Expected self to be 'Buffer' but is 'Texture'"),
            MaterialBinding::Buffer(v) => *v,
        }
    }
}
