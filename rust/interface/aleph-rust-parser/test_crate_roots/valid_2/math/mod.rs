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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    x: f32,
    y: f32
}

impl Vector3 {
    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[aleph::interface]
#[repr(C)]
#[derive(Clone)]
pub struct Ray3D {
    origin: Vector3,
    dir: Vector3
}

#[aleph::interface]
#[repr(C)]
#[derive(Clone)]
pub struct Ray2D {
    origin: Vector2,
    dir: Vector2
}
