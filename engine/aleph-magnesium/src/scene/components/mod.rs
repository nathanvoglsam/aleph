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

use aleph_math::projection::perspective_reversed_infinite_z_wgpu_dx_gl;
use aleph_math::{Bivec3, DVec3, Mat4, Rotor3, Vec3};

use crate::material_instance::MaterialInstanceHandle;
use crate::resource::buffer::BufferHandle;

/// A tag component that should be attached to any entity in the render scene that is considered
/// dynamic.
///
/// A dynamic entity is one that will move every frame, or very nearly every frame.
///
/// Dynamic entities are tagged as such so that they can be filtered.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct DynamicObject;

aleph_ecs::register_component!(DynamicObject);

/// Generic transform component that fully describes the transformation of a render object in
/// world space.
#[derive(Clone, PartialEq, Debug)]
pub struct RenderTransform {
    /// The 3D position of some object in world space
    pub position: DVec3,

    /// The 3D rotation of some object in world space
    pub rotation: Rotor3,

    /// The 3D scale of some object
    pub scale: Vec3,
}

impl RenderTransform {
    /// Constructs a new 'identity' transformation.
    ///
    /// This means position = (0,0,0), no rotation and (1,1,1) scale.
    pub const fn identity() -> Self {
        Self {
            position: DVec3::new(0.0, 0.0, 0.0),
            rotation: Rotor3::new(1.0, Bivec3::new(0.0, 0.0, 0.0)),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for RenderTransform {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

aleph_ecs::register_component!(RenderTransform);

/// Component data for a static mesh entity. When paired with a [`RenderTransform`] component this
/// will cause a mesh to be rendered with the given transform data.
#[derive(Clone, Default, Debug)]
pub struct StaticMesh {
    /// Vertex buffer...
    pub vtx: Option<BufferHandle>,

    /// Index buffer...
    pub idx: Option<BufferHandle>,

    /// Material instance...
    pub material_instance: Option<MaterialInstanceHandle>,
}

aleph_ecs::register_component!(StaticMesh);

/// A description of a camera's projection information, assuming a perspective projection.
///
/// This struct describes the parameters for an infinite, reverse-z projection matrix following the
/// DirectX convention.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct PerspectiveCamera {
    /// The vertical field-of-view of the perspective projection, in _degrees_.
    pub vertical_fov: f32,

    /// The near plane distance from the camera. We always use a reversed-z infinite perspective
    /// projection so there is no far plane to specify.
    pub z_near: f32,
}

impl Default for PerspectiveCamera {
    /// Constructs a new [`crate::scene::objects::camera::PerspectiveInfo`] with a 90 degree field-of-view and a near plane of 0.1.
    fn default() -> Self {
        Self::new(90.0, 0.1)
    }
}

impl PerspectiveCamera {
    /// Constructs a new [`crate::scene::objects::camera::PerspectiveInfo`] with the given parameters. 'fov' is vertical
    /// field-of-view in degrees (see [`crate::scene::objects::camera::PerspectiveInfo::vertical_fov`]).
    pub const fn new(fov: f32, near: f32) -> Self {
        Self {
            vertical_fov: fov,
            z_near: near,
        }
    }

    /// Returns a projection matrix that represents the projection described by this
    /// [`crate::scene::objects::camera::PerspectiveInfo`] struct. This will _always_ yield an infinite, reversed-z projection
    /// matrix.
    #[inline]
    pub fn get_matrix(&self, aspect: f32) -> Mat4 {
        perspective_reversed_infinite_z_wgpu_dx_gl(
            self.vertical_fov.to_radians(),
            aspect,
            self.z_near,
        )
    }
}

aleph_ecs::register_component!(PerspectiveCamera);
