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

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_gen_arena::GenArena;

use crate::internal::Magnesium;
use crate::material::Material;
use crate::material::binding::MaterialBinding;
use crate::material_instance::{MaterialInstanceHandle, MaterialInstancePoolAccessor};

/// Manager that owns the material instance pool, and any other resources directly associated with
/// our pooled material instance resources.
pub struct MaterialInstanceStore {
    pub pool: GenArena<MaterialInstanceObject, MaterialInstanceHandle, MgMatInstSystem>,
}

impl MaterialInstanceStore {
    /// Constructs a new, empty store
    pub fn new() -> Self {
        Self {
            pool: GenArena::new_in(),
        }
    }

    /// Create a read-only [`MaterialInstancePoolAccessor`]. See the accessor type for more
    /// information.
    pub const fn accessor(&self) -> MaterialInstancePoolAccessor<'_> {
        MaterialInstancePoolAccessor(self)
    }

    /// Cleanup code invoked when the renderer object is dropped.
    pub fn clean_up(&mut self) {
        self.pool.clear();
    }
}

/// The object we store _inside_ the object pool managed by a [`MaterialInstanceStore`].
///
/// This contains a few things:
///
/// - A handle to the [`Material`] instance that the object is an instance off. This impacts what
///   shader is selected and the shape of the bindings array.
/// - A list of [`MaterialBinding`] objects that should match the shape specified by the material
///   archetype. This array contains pool handles to the buffers and textures that must be bound
///   to draw an object with the material instance.
/// - Flags and options that are not part of the material archetype that may otherwise affect how
///   the material may be rendered. These may configure other, non-shader parts of the raster
///   pipeline.
pub struct MaterialInstanceObject {
    /// The material that this object is an instance of.
    pub material: Arc<Material>,

    /// Whether the material is double-sided or not.
    pub double_sided: bool,

    /// Indexed list of material bindings. The layout of this list is defined by the material we are
    /// instancing.
    pub bindings: BVec<MaterialBinding, MgMatInstSystem>,
}

impl MaterialInstanceObject {
    /// Constructs a new, zero initialized material instance object.
    ///
    /// # Important
    ///
    /// This calls into the given [`Material`] to create a the initial bindings array. This array
    /// will be laid out matching the material's layout, with each slot containing bindings of the
    /// expected type. However, all resource handles will be 'null' handles.
    ///
    /// Other flags and options will in general be default initialized.
    pub fn new(material: Arc<Material>) -> Self {
        let mut bindings = BVec::new_in(system());
        bindings.extend_from_slice(material.material.binding_template());
        Self {
            material,
            double_sided: false,
            bindings,
        }
    }

    /// Validate as many bindings as will fit into the 'bindings' array in 'self' from the array of
    /// bindings in 'src'.
    ///
    /// This will return [`None`] if _any_ of the bindings in 'src' don't match the corresponding
    /// binding in 'self'.
    ///
    /// This is function can be used to type-check before writing bindings into a material instance
    /// with [`MaterialInstanceObject::write_bindings_from_array`]. This performs a dry run of the
    /// same algorithm, but only verifies the types of each src/dst pair matches _without doing the
    /// copy_.
    ///
    /// Using this function allows gracefully recovering from incorrectly typed material bindings
    /// without leaving the instance's binding array partially written.
    pub fn pre_validate_binding_write(&self, src: &[MaterialBinding]) -> Option<()> {
        let iter = self.bindings.iter().zip(src.iter());
        for (dst, src) in iter {
            if dst.binding_type() != src.binding_type() {
                return None;
            }
        }
        Some(())
    }

    /// Copy as many bindings as will fit into the 'bindings' array in 'self' from the array of
    /// bindings in 'src'.
    ///
    /// This will return [`None`] if _any_ of the bindings in 'src' don't match the corresponding
    /// binding in 'self'.
    ///
    /// # Warning
    ///
    /// In the event of a type mismatch this may return before the entire array has been copied
    /// into the instance object. If this is undesirable use
    /// [`MaterialInstanceObject::pre_validate_binding_write`] to type-check the copy before calling
    /// this function.
    pub fn write_bindings_from_array(&mut self, src: &[MaterialBinding]) -> Option<()> {
        let iter = self.bindings.iter_mut().zip(src.iter());
        for (dst, src) in iter {
            dst.overwrite(src)?;
        }
        Some(())
    }

    /// Update the binding in the requested spot, writing the provided handle into the slot.
    ///
    /// This will check that the type of the provided binding matches the destination, panicking if
    /// the type does not match.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure whatever resource is referenced by the given
    /// binding is valid to use within the context of this material. This could mean synchronization
    /// state, or it could mean resource usage flags, or other requirements.
    pub fn update_binding(&mut self, binding: u32, v: &MaterialBinding) {
        debug_assert!(
            self.material
                .material
                .check_binding_type(binding, v.binding_type())
        );
        self.bindings[binding as usize].overwrite(v);
    }
}

/// Allocator category for the buffer object store
pub struct MaterialInstance;
aleph_alloc::new_child_alloc_category!(
    Magnesium,
    MaterialInstance,
    "019b314e-9a6a-7250-9006-6dca7282c5b5"
);

pub type MgMatInstSystem = aleph_alloc::instrumentation::Instrumented<MaterialInstance>;
