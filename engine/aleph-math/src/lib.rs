//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#![no_std]

pub mod gradient;
pub mod matrix;
pub mod quantize;
pub mod quaternion;
pub mod traits;
pub mod types;
pub mod units;
pub mod vector;

use core::cmp::PartialOrd;

///
/// Returns whether a value is within the range of the two given values
///
#[inline]
pub fn in_range<T: PartialOrd>(num: T, lower_bound: T, upper_bound: T) -> bool {
    num.ge(&lower_bound) && num.le(&upper_bound)
}
