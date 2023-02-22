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

use std::fmt::{Debug, Error, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

///
/// Higher level bit flag type for representing a vulkan queue family's type
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueFamilyType(u8);

impl QueueFamilyType {
    pub const GRAPHICS: Self = QueueFamilyType(0b00001);
    pub const COMPUTE: Self = QueueFamilyType(0b00010);
    pub const TRANSFER: Self = QueueFamilyType(0b00100);
    pub const SPARSE_BINDING: Self = QueueFamilyType(0b01000);
    pub const PRESENT: Self = QueueFamilyType(0b10000);

    ///
    /// Constructs a FamilyType with a default internal value of 0
    ///
    pub fn new() -> QueueFamilyType {
        QueueFamilyType(0)
    }

    ///
    /// Whether any of the flags of other can be found in self
    ///
    pub fn intersect(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    ///
    /// Whether all of the flags of other can be found in self
    ///
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    ///
    /// Set the flags in the other mask onto self
    ///
    pub fn set(&mut self, other: Self) {
        self.0 |= other.0;
    }

    ///
    /// Unset all flags in the other mask that are also in self
    ///
    pub fn unset(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl BitAnd for QueueFamilyType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for QueueFamilyType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for QueueFamilyType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for QueueFamilyType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for QueueFamilyType {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for QueueFamilyType {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl From<QueueFamilyType> for u8 {
    fn from(v: QueueFamilyType) -> u8 {
        v.0
    }
}

impl Default for QueueFamilyType {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for QueueFamilyType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if f.alternate() {
            let mut flags = String::new();

            if self.intersect(QueueFamilyType::GRAPHICS) {
                flags.push_str("\n    GRAPHICS,");
            }

            if self.intersect(QueueFamilyType::COMPUTE) {
                flags.push_str("\n    COMPUTE,");
            }

            if self.intersect(QueueFamilyType::TRANSFER) {
                flags.push_str("\n    TRANSFER,");
            }

            if self.intersect(QueueFamilyType::SPARSE_BINDING) {
                flags.push_str("\n    SPARSE_TRANSFER,");
            }

            if self.intersect(QueueFamilyType::PRESENT) {
                flags.push_str("\n    PRESENT,");
            }

            if flags.is_empty() {
                write!(f, "FamilyType {{ }}")
            } else {
                write!(f, "FamilyType {{{flags}\n}}")
            }
        } else {
            let mut flags = String::new();

            if self.intersect(QueueFamilyType::GRAPHICS) {
                flags.push_str("GRAPHICS ");
            }

            if self.intersect(QueueFamilyType::COMPUTE) {
                flags.push_str("COMPUTE ");
            }

            if self.intersect(QueueFamilyType::TRANSFER) {
                flags.push_str("TRANSFER ");
            }

            if self.intersect(QueueFamilyType::SPARSE_BINDING) {
                flags.push_str("SPARSE_TRANSFER ");
            }

            if self.intersect(QueueFamilyType::PRESENT) {
                flags.push_str("PRESENT ");
            }

            write!(f, "FamilyType {{ {flags}}}")
        }
    }
}

///
/// High level wrapper for a vulkan queue family. Stores the family index, the number of queues that
/// this device supports for this family and the operations supported on this queue
///
#[derive(Clone, Debug)]
pub struct QueueFamily {
    pub index: u32,
    pub count: u32,
    pub family_type: QueueFamilyType,
}

impl QueueFamily {
    ///
    /// Default construction for a QueueFamily
    ///
    pub fn new() -> QueueFamily {
        QueueFamily {
            index: 0,
            count: 0,
            family_type: QueueFamilyType::default(),
        }
    }

    ///
    /// If the queue family is compatible with any types of queue
    ///
    pub fn is_complete(&self) -> bool {
        self.family_type != QueueFamilyType::default()
    }

    ///
    /// Whether this is a general queue that supports graphics, compute and transfer and present
    ///
    pub fn is_general(&self) -> bool {
        let general = QueueFamilyType::GRAPHICS
            | QueueFamilyType::COMPUTE
            | QueueFamilyType::TRANSFER
            | QueueFamilyType::PRESENT;
        self.family_type.contains(general)
    }

    ///
    /// Detects whether this family is an async compute queue family (no graphics)
    ///
    pub fn is_async_compute(&self) -> bool {
        let compute = QueueFamilyType::COMPUTE | QueueFamilyType::TRANSFER;
        self.family_type.contains(compute) && !self.family_type.contains(QueueFamilyType::GRAPHICS)
    }

    ///
    /// Detects whether this family is a transfer queue family (only transfer)
    ///
    pub fn is_transfer(&self) -> bool {
        let transfer = QueueFamilyType::TRANSFER;
        self.family_type.contains(transfer)
            && !self.family_type.contains(QueueFamilyType::GRAPHICS)
            && !self.family_type.contains(QueueFamilyType::COMPUTE)
    }
}

impl Default for QueueFamily {
    fn default() -> Self {
        Self::new()
    }
}
