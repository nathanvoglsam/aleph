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

use crate::ResourceRef;
use aleph_any::AnyArc;
use aleph_rhi_api::*;

pub enum ResourceVariant {
    Buffer(AnyArc<dyn IBuffer>),
    Texture(AnyArc<dyn ITexture>),
}

impl From<AnyArc<dyn IBuffer>> for ResourceVariant {
    fn from(value: AnyArc<dyn IBuffer>) -> Self {
        Self::Buffer(value)
    }
}

impl From<AnyArc<dyn ITexture>> for ResourceVariant {
    fn from(value: AnyArc<dyn ITexture>) -> Self {
        Self::Texture(value)
    }
}

impl<'a> From<&'a dyn IBuffer> for ResourceVariant {
    fn from(value: &'a dyn IBuffer) -> Self {
        let value = value.upgrade();
        Self::Buffer(value)
    }
}

impl<'a> From<&'a dyn ITexture> for ResourceVariant {
    fn from(value: &'a dyn ITexture) -> Self {
        let value = value.upgrade();
        Self::Texture(value)
    }
}

impl<'a> From<&'a AnyArc<dyn IBuffer>> for ResourceVariant {
    fn from(value: &'a AnyArc<dyn IBuffer>) -> Self {
        let value = value.upgrade();
        Self::Buffer(value)
    }
}

impl<'a> From<&'a AnyArc<dyn ITexture>> for ResourceVariant {
    fn from(value: &'a AnyArc<dyn ITexture>) -> Self {
        let value = value.upgrade();
        Self::Texture(value)
    }
}

#[derive(Default)]
pub struct ImportBundle {
    pub(crate) imports: std::collections::HashMap<u16, ResourceVariant>,
}

impl ImportBundle {
    pub fn add_resource(
        &mut self,
        id: impl Into<ResourceRef>,
        r: impl Into<ResourceVariant>,
    ) -> &mut Self {
        let id = id.into();
        let r = r.into();

        let existed = self.imports.insert(id.0.root_id(), r).is_some();
        assert!(
            !existed,
            "It is invalid to insert a handle for the same resource ID twice"
        );

        self
    }

    pub fn get_resource(&self, root_id: u16) -> &ResourceVariant {
        self.imports
            .get(&root_id)
            .expect("Declared imported resource not present in provided ImportBundle")
    }
}
