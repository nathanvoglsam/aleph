//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::traits::Real;

///
/// Convert degrees to radians
///
pub fn radians<T: Real>(deg: T) -> T {
    (deg / T::from(180).unwrap()) * T::pi()
}

///
/// Convert radians to degrees
///
pub fn degrees<T: Real>(rad: T) -> T {
    rad * (T::from(180).unwrap() / T::pi())
}
