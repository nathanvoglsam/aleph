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

pub mod binding;
pub mod standard_material;

use std::num::NonZero;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use aleph_any::AnyArc;
use aleph_rhi_api::*;
use aleph_shader_db::{Fragment, ShaderName, Vertex};
use smallbox::space::S2;
use smallbox::{SmallBox, smallbox};
pub use standard_material::{StandardMaterial, StandardMaterialLayout};

use crate::material::binding::MaterialBinding;
use crate::material_instance::MaterialInstanceReader;
use crate::resource::buffer::BufferPoolAccessor;
use crate::resource::texture::TexturePoolAccessor;

pub struct Material {
    /// Globally unique ID that uniquely identifies a specific [`Material`] instance.
    id: MaterialId,

    /// The material implementation object this material will be based around
    pub(crate) material: SmallBox<dyn IMaterial, S2>,
}

impl Material {
    pub fn new<T: IMaterial>(material: T) -> Arc<Self> {
        /// Generate a new material ID that is guaranteed to be unique
        fn next_id() -> MaterialId {
            static MATERIAL_ID: AtomicU32 = AtomicU32::new(0);

            let id = MATERIAL_ID.fetch_add(1, Ordering::Relaxed);
            let id = id + 1;
            let id = NonZero::new(id)
                .expect("Overflowed MATERIAL_ID counter. Can't allocate any more Materials!");
            MaterialId(id)
        }

        let v = Self {
            id: next_id(),
            material: smallbox!(material),
        };
        Arc::new(v)
    }

    /// Returns the [`MaterialId`] that identifies 'self'.
    pub const fn id(&self) -> MaterialId {
        self.id
    }
}

/// Unique ID that identifies a [`Material`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MaterialId(NonZero<u32>);

impl MaterialId {
    /// Returns the inner 'u32' value.
    pub const fn get(&self) -> u32 {
        self.0.get()
    }
}

/// The set of material classes available. These are used to control what pass the material will be
/// rendered in.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MaterialClass {
    /// The material is opaque. There is no transparency to the material.
    ///
    /// 'Opaque' materials will be rendered in the deferred pass.
    Opaque,

    /// The material is transparent and requires blending with existing frame buffer data.
    ///
    /// 'Blended' materials will be rendered separately to the main deferred pass, typically using
    /// some form of forward rendering.
    Blended,
}

pub unsafe trait IMaterial: Send + Sync + 'static {
    fn frag_name(&self) -> ShaderName<'static, Fragment>;

    fn vert_name(&self) -> ShaderName<'static, Vertex>;

    fn create_parameter_block_layout(
        &self,
        device: &dyn IDevice,
    ) -> AnyArc<dyn IParameterBlockLayout>;

    fn check_binding_type(&self, binding: u32, binding_type: MaterialBindingType) -> bool;

    fn binding_template(&self) -> &[MaterialBinding];

    unsafe fn update_parameter_block(
        &self,
        block_layout: &dyn IParameterBlockLayout,
        buffer_pool: BufferPoolAccessor,
        texture_pool: TexturePoolAccessor,
        device: &dyn IDevice,
        instance: MaterialInstanceReader,
        dst: ParameterBlockHandle,
    );
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MaterialBindingType {
    Texture,
    Buffer,
}
