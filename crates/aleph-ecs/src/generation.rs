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

use std::cmp::Ordering;
use std::convert::TryFrom;

///
/// The base `Generation` type that represents an arbitrary generation index.
///
/// There are also some new-type wrappers `AliveGeneration` and `DeadGeneration` that reduce the
/// set of valid indices to alive or dead indices respectively.
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generation(pub(crate) u32);

impl Generation {
    /// Returns whether this generation marks a generation that is alive
    #[inline]
    pub fn is_alive(self) -> bool {
        self.0 % 2 == 0
    }

    /// Returns whether this generation marks a generation that si dead
    #[inline]
    pub fn is_dead(self) -> bool {
        !self.is_alive()
    }

    /// Converts the generation to an `AliveGeneration` new-type if `self` is alive
    #[inline]
    pub fn into_alive(self) -> Option<AliveGeneration> {
        AliveGeneration::try_from(self).ok()
    }

    /// Converts the generation to an `DeadGeneration` new-type if `self` is dead
    #[inline]
    pub fn into_dead(self) -> Option<DeadGeneration> {
        DeadGeneration::try_from(self).ok()
    }

    /// Increments the generation index.
    #[inline]
    pub fn increment(self) -> Generation {
        Self(self.0.wrapping_add(1))
    }
}

impl Default for Generation {
    #[inline]
    fn default() -> Self {
        Generation(0)
    }
}

impl PartialEq<AliveGeneration> for Generation {
    #[inline]
    fn eq(&self, other: &AliveGeneration) -> bool {
        self.0.eq(&other.0 .0)
    }
}

impl PartialOrd<AliveGeneration> for Generation {
    #[inline]
    fn partial_cmp(&self, other: &AliveGeneration) -> Option<Ordering> {
        self.0.partial_cmp(&other.0 .0)
    }
}

impl PartialEq<DeadGeneration> for Generation {
    #[inline]
    fn eq(&self, other: &DeadGeneration) -> bool {
        self.0.eq(&other.0 .0)
    }
}

impl PartialOrd<DeadGeneration> for Generation {
    #[inline]
    fn partial_cmp(&self, other: &DeadGeneration) -> Option<Ordering> {
        self.0.partial_cmp(&other.0 .0)
    }
}

///
/// A new-type wrapper over `Generation` that can only hold a generation index that is alive
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct AliveGeneration(pub(crate) Generation);

impl AliveGeneration {
    /// "Revives" the generation, producing a dead generation, by incrementing the generation
    /// index
    #[inline]
    pub fn kill(self) -> DeadGeneration {
        DeadGeneration(self.0.increment())
    }
}

impl TryFrom<Generation> for AliveGeneration {
    type Error = ();

    #[inline]
    fn try_from(value: Generation) -> Result<Self, Self::Error> {
        if value.is_dead() {
            Err(())
        } else {
            Ok(AliveGeneration(value))
        }
    }
}

impl PartialEq<Generation> for AliveGeneration {
    #[inline]
    fn eq(&self, other: &Generation) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<Generation> for AliveGeneration {
    #[inline]
    fn partial_cmp(&self, other: &Generation) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<DeadGeneration> for AliveGeneration {
    #[inline]
    fn eq(&self, _: &DeadGeneration) -> bool {
        false
    }
}

///
/// A new-type wrapper over `Generation` that can only hold a generation index that is dead
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DeadGeneration(pub(crate) Generation);

impl DeadGeneration {
    /// "Revives" the generation, producing a live generation, by incrementing the generation
    /// index
    #[inline]
    pub fn revive(self) -> AliveGeneration {
        AliveGeneration(self.0.increment())
    }
}

impl TryFrom<Generation> for DeadGeneration {
    type Error = ();

    #[inline]
    fn try_from(value: Generation) -> Result<Self, Self::Error> {
        if value.is_alive() {
            Err(())
        } else {
            Ok(DeadGeneration(value))
        }
    }
}

impl PartialEq<Generation> for DeadGeneration {
    #[inline]
    fn eq(&self, other: &Generation) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<Generation> for DeadGeneration {
    #[inline]
    fn partial_cmp(&self, other: &Generation) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<AliveGeneration> for DeadGeneration {
    #[inline]
    fn eq(&self, _: &AliveGeneration) -> bool {
        false
    }
}
