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

use std::sync::Arc;

use crate::{BufferHandle, Material, MaterialBindingType, TextureHandle};

pub struct MaterialInstanceObject {
    /// The material that this object is an instance of.
    pub(crate) material: Arc<Material>,

    pub(crate) double_sided: bool,

    /// Indexed list of material bindings. The layout of this list is defined by the material we are
    /// instancing.
    pub(crate) bindings: Vec<MaterialBinding>,
}

impl MaterialInstanceObject {
    pub fn new(material: Arc<Material>) -> Self {
        let bindings = material.material.instantiate_bindings();
        Self {
            material,
            double_sided: false,
            bindings,
        }
    }

    pub fn set_double_sided(&mut self, double_sided: bool) {
        self.double_sided = double_sided;
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure whatever resource is referenced by the given
    /// binding is valid to use within the context of this material. This could mean synchronization
    /// state, or it could mean resource usage flags, or other requirements.
    pub fn update_binding(&mut self, binding: u32, v: MaterialBinding) {
        debug_assert!(
            self.material
                .material
                .check_binding_type(binding, v.binding_type())
        );
        match (v, &mut self.bindings[binding as usize]) {
            (MaterialBinding::Texture(v), MaterialBinding::Texture(dst)) => {
                *dst = v;
            }
            (MaterialBinding::Buffer(v), MaterialBinding::Buffer(dst)) => {
                *dst = v;
            }
            _ => panic!(),
        }
    }
}

pub enum MaterialBinding {
    /// Data for a 'texture' type material binding. The type of view is specific to the binding slot
    /// and the material it is associated with.
    Texture(Option<TextureHandle>),

    /// Data for a 'buffer' type material binding. The type of buffer expected is specific to the
    /// binding slot and the material it is associated with.
    Buffer(Option<BufferHandle>),
}

impl MaterialBinding {
    pub const fn binding_type(&self) -> MaterialBindingType {
        match self {
            MaterialBinding::Texture(_) => MaterialBindingType::Texture,
            MaterialBinding::Buffer(_) => MaterialBindingType::Buffer,
        }
    }

    pub fn unwrap_texture(&self) -> Option<TextureHandle> {
        match self {
            MaterialBinding::Texture(v) => *v,
            MaterialBinding::Buffer(_) => panic!("Expected self to be 'Texture' but is 'Buffer'"),
        }
    }

    pub fn unwrap_buffer(&self) -> Option<BufferHandle> {
        match self {
            MaterialBinding::Texture(_) => panic!("Expected self to be 'Buffer' but is 'Texture'"),
            MaterialBinding::Buffer(v) => *v,
        }
    }
}
