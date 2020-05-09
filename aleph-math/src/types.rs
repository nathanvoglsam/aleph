//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::matrix::TMat4x4;
use crate::vector::{TVec2, TVec3, TVec4};

use crate::quaternion::TQuat;

///
/// A 2 component f32 vector
///
pub type Vec2 = TVec2<f32>;

///
/// A 3 component f32 vector
///
pub type Vec3 = TVec3<f32>;

///
/// A 4 component f32 vector
///
pub type Vec4 = TVec4<f32>;

///
/// A 2 component f64 vector
///
pub type DVec2 = TVec2<f64>;

///
/// A 3 component f64 vector
///
pub type DVec3 = TVec3<f64>;

///
/// A 4 component f64 vector
///
pub type DVec4 = TVec4<f64>;

///
/// A f32 quaternion
///
pub type Quat = TQuat<f32>;

///
/// A f64 quaternion
///
pub type DQuat = TQuat<f64>;

///
/// A 4x4 f32 matrix
///
pub type Mat4x4 = TMat4x4<f32>;

///
/// A 4x4 f64 matrix
///
pub type DMat4x4 = TMat4x4<f64>;
