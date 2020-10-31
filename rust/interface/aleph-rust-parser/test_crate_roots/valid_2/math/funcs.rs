use crate::TestSingleton;
use super::Vector3;
pub use super::Vector2 as Vec56;
pub use self::Vec56 as Vec21;

#[aleph::interface]
#[repr(C)]
pub struct Test {

}

impl Test {
    pub fn test(vec: Vector3) {}
}