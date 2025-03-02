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

mod equirectangular_conversion;
mod octahedral_conversion;

pub use equirectangular_conversion::*;
pub use octahedral_conversion::*;

use aleph_math::Vec3;
use thiserror::Error;

/// Semi-private trate used as part of the parametrization of [`equi_to_cube`]. Represents a compile
/// time interface for mapping a UV coordinate for a specific cube face back into the direction
/// vector that would refer to it as part of sampling a cube map.
///
/// This is used as a generic parameter for the [`equi_to_cube`] function to map 2D coordinates for
/// a face back to 3D coordinates. The 3D coordinates are then used to sample the source environment
/// map so we can fill out the cube faces.
pub trait IFaceSelector {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3;

    fn get_mapped(u: f32, v: f32) -> Vec3 {
        let u = u * 2.0 - 1.0;
        let v = v * 2.0 - 1.0;
        let dir = Self::map_uv_to_direction(u, v);
        dir.normalized()
    }
}

/// Face selector ([`IFaceSelector`]) for the +X cube face
pub struct FacePosX;

impl IFaceSelector for FacePosX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(1.0, -v, -u)
    }
}

/// Face selector ([`IFaceSelector`]) for the -X cube face
pub struct FaceNegX;

impl IFaceSelector for FaceNegX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-1.0, -v, u)
    }
}

/// Face selector ([`IFaceSelector`]) for the +Y cube face
pub struct FacePosY;

impl IFaceSelector for FacePosY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, 1.0, v)
    }
}

/// Face selector ([`IFaceSelector`]) for the -Y cube face
pub struct FaceNegY;

impl IFaceSelector for FaceNegY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -1.0, -v)
    }
}

/// Face selector ([`IFaceSelector`]) for the +Z cube face
pub struct FacePosZ;

impl IFaceSelector for FacePosZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -v, 1.0)
    }
}

/// Face selector ([`IFaceSelector`]) for the -Z cube face
pub struct FaceNegZ;

impl IFaceSelector for FaceNegZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-u, -v, -1.0)
    }
}

/// Errors that may occur when performing texture operations on a [`crate::TextureBuffer`].
#[derive(Error, Debug)]
pub enum TextureOpError {
    #[error("The source image format is invalid for the requested image operation")]
    InvalidSrcFormat,

    #[error("The source texture type is invalid for the requested texture operation")]
    InvalidSrcType,
}

pub type TextureOpResult<T> = Result<T, TextureOpError>;
