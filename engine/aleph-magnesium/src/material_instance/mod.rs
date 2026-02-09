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

use aleph_gen_arena::{Handle, make_handle_id};

use crate::internal::material_instance::{MaterialInstanceObject, MaterialInstanceStore};
use crate::material::Material;
use crate::material::binding::MaterialBinding;

make_handle_id!(MaterialInstance);
pub type MaterialInstanceHandle = Handle<MaterialInstance>;

#[derive(Copy, Clone)]
pub struct MaterialInstancePoolAccessor<'a>(pub(crate) &'a MaterialInstanceStore);

impl<'a> MaterialInstancePoolAccessor<'a> {
    pub fn get_ref(&self, handle: MaterialInstanceHandle) -> Option<MaterialInstanceReader<'_>> {
        self.0.pool.get_ref(handle).map(MaterialInstanceReader)
    }
}

#[derive(Copy, Clone)]
pub struct MaterialInstanceReader<'a>(pub(crate) &'a MaterialInstanceObject);

impl<'a> MaterialInstanceReader<'a> {
    #[inline(always)]
    pub fn same_instance(self, other: Self) -> bool {
        core::ptr::addr_eq(self.0, other.0)
    }

    pub fn material(&self) -> &Material {
        &self.0.material
    }

    pub const fn double_sided(&self) -> bool {
        self.0.double_sided
    }

    #[inline(always)]
    pub fn bindings(&self) -> &[MaterialBinding] {
        self.0.bindings.as_slice()
    }
}

pub struct MaterialInstanceDesc<'a> {
    pub double_sided: bool,
    pub bindings: &'a [MaterialBinding],
}
