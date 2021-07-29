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

///
/// The base `Generation` type that represents an arbitrary generation index.
///
/// There are also some new-type wrappers `AliveGeneration` and `DeadGeneration` that reduce the
/// set of valid indices to alive or dead indices respectively.
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generation(u32);

impl Generation {
    /// Returns a newly created generation. The initial state is 0, which encodes a dead generation.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Constructs a generation with the given initial value
    pub const fn from_raw(v: u32) -> Self {
        Self(v)
    }

    /// Returns the inner u32 value the generation wraps
    pub const fn into_inner(self) -> u32 {
        self.0
    }

    /// Returns whether this generation marks a generation that is alive
    #[inline]
    pub const fn is_alive(self) -> bool {
        self.0 % 2 != 0
    }

    /// Returns whether this generation marks a generation that si dead
    #[inline]
    pub const fn is_dead(self) -> bool {
        !self.is_alive()
    }

    /// Increments the generation index.
    #[inline]
    pub const fn increment(self) -> Generation {
        Self(self.0.wrapping_add(1))
    }

    /// Increments the generation index and assigns the result to self
    #[inline]
    pub fn increment_assign(&mut self) {
        *self = self.increment()
    }
}

impl Default for Generation {
    #[inline]
    fn default() -> Self {
        Generation(0)
    }
}
