mod funcs;

use funcs::Test;

#[aleph::interface]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    pub fn add(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[aleph::interface]
pub trait TestTrait {
    fn trait_fn(&self) -> Vector2;
}

impl TestTrait for Vector3 {
    fn trait_fn(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y
        }
    }
}

#[aleph::interface]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector2 {
    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn div(&mut self, other: &self::Vector2) {
        self.x /= other.x;
        self.y /= other.y;
    }

    pub fn mul(&mut self, other: &super::math::Vector2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

#[aleph::interface]
#[repr(C)]
#[derive(Clone)]
pub struct Ray3D {
    origin: self::Vector3,
    dir: Vector3
}

#[aleph::interface]
#[repr(C)]
#[derive(Clone)]
pub struct Ray2D {
    origin: super::math::Vector2,
    dir: crate::math::Vector2
}
