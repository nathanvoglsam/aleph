//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::traits::{Lerp, Real};
use crate::vector::{TVec2, TVec3};

// TODO: Document these structs

pub struct LinearGradientBuilder<T: Real> {
    start_pos: TVec2<T>,
    start_col: TVec3<T>,

    end_pos: TVec2<T>,
    end_col: TVec3<T>,
}

impl<T: Real> LinearGradientBuilder<T> {
    pub fn new() -> Self {
        LinearGradientBuilder {
            start_pos: TVec2::zero(),
            start_col: TVec3::zero(),
            end_pos: TVec2::zero(),
            end_col: TVec3::zero(),
        }
    }

    pub fn start(mut self, pos: TVec2<T>, color: TVec3<T>) -> Self {
        self.start_pos = pos;
        self.start_col = color;
        self
    }

    pub fn end(mut self, pos: TVec2<T>, color: TVec3<T>) -> Self {
        self.end_pos = pos;
        self.end_col = color;
        self
    }

    pub fn build(self) -> LinearGradient<T> {
        let a = self.end_pos[0] - self.start_pos[0];
        let b = self.end_pos[1] - self.start_pos[1];
        LinearGradient {
            start_col: self.start_col,
            end_col: self.end_col,
            a,
            b,
            c1: (a * self.start_pos[0]) + (b * self.start_pos[1]),
            c2: (a * self.end_pos[0]) + (b * self.end_pos[1]),
        }
    }
}

impl<T: Real> Default for LinearGradientBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct LinearGradient<T: Real> {
    start_col: TVec3<T>,
    end_col: TVec3<T>,

    a: T,
    b: T,

    c1: T,
    c2: T,
}

impl<T: Real> LinearGradient<T> {
    pub fn builder() -> LinearGradientBuilder<T> {
        LinearGradientBuilder::new()
    }

    pub fn sample(&self, point: TVec2<T>) -> TVec3<T> {
        let c = self.a * point[0] + self.b * point[1];
        if c <= self.c1 {
            self.start_col
        } else if c >= self.c2 {
            self.end_col
        } else {
            let factor =
                (T::zero() * (self.c2 - c)) + (T::one() * (c - self.c1)) / (self.c2 - self.c1);
            self.start_col.lerp(&self.end_col, factor)
        }
    }
}
