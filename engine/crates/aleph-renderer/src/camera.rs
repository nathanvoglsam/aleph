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
use aleph_math::{Mat4, Rotor3, Vec3};
use aleph_pin_board::BoardParamId;

/// A description of a camera as a combination of a pos+rot transform and a description of the
/// projection parameters.
#[derive(Clone, PartialEq, Debug)]
pub struct CameraInfo {
    /// The world-space position of the camera. This is a double-precision vector.
    pub position: Vec3,

    /// The world-space oritentation of the camera.
    pub orientation: Rotor3,

    /// The projection that the camera is using
    pub projection: PerspectiveInfo,
}

impl BoardParamId for CameraInfo {
    type Output<'a> = Self;
}

impl CameraInfo {
    pub const fn new(pos: Vec3, rot: Rotor3, proj: PerspectiveInfo) -> Self {
        Self {
            position: pos,
            orientation: rot,
            projection: proj,
        }
    }

    #[inline]
    pub fn default_with_aspect(aspect: f32) -> Self {
        let pos = Vec3::zero();
        let rot = Rotor3::identity();
        let proj = PerspectiveInfo::default_with_aspect(aspect);
        Self::new(pos, rot, proj)
    }

    #[inline]
    pub fn get_view_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(-self.position);
        let orientation = self.orientation.reversed().into_matrix().into_homogeneous();
        translation * orientation
    }

    #[inline]
    pub fn get_proj_matrix(&self) -> Mat4 {
        self.projection.get_matrix()
    }
}

/// A description of a camera's projection information, assuming a perspective projection.
///
/// This struct describes the parameters for an infinite, reverse-z projection matrix following the
/// DirectX convention.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct PerspectiveInfo {
    /// The vertical field-of-view of the perspective projection, in _degrees_.
    pub vertical_fov: f32,

    /// The near plane distance from the camera. We always use a reversed-z infinite perspective
    /// projection so there is no far plane to specify.
    pub z_near: f32,

    /// The aspect ratio parameter of the projection. This should be deduced from the render surface
    /// width/height.
    pub aspect_ratio: f32, // TODO: we could just take this from the main target inside the renderer?
}

impl PerspectiveInfo {
    /// Constructs a new [`PerspectiveInfo`] with the given parameters. 'fov' is vertical
    /// field-of-view in degrees (see [`PerspectiveInfo::vertical_fov`]).
    pub const fn new(fov: f32, near: f32, aspect: f32) -> Self {
        Self {
            vertical_fov: fov,
            z_near: near,
            aspect_ratio: aspect,
        }
    }

    /// Constructs a new [`PerspectiveInfo`] with a 90 degree field-of-view and a near plane of 0.1.
    /// The caller must still provide the aspect ratio as we can't assume one.
    pub const fn default_with_aspect(aspect: f32) -> Self {
        Self::new(90.0, 0.1, aspect)
    }

    /// Returns a projection matrix that represents the projection described by this
    /// [`PerspectiveInfo`] struct. This will _always_ yield an infinite, reversed-z projection
    /// matrix.
    #[inline]
    pub fn get_matrix(&self) -> Mat4 {
        perspective_reversed_infinite_z_wgpu_dx_gl(
            self.vertical_fov.to_radians(),
            self.aspect_ratio,
            self.z_near,
        )
    }
}
