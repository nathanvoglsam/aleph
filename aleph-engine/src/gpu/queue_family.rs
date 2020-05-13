//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::fmt::{Debug, Error, Formatter};

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

    pub const GENERAL: Self = QueueFamilyType(0b10111);

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

impl Into<u8> for QueueFamilyType {
    fn into(self) -> u8 {
        self.0
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
                write!(f, "FamilyType {{{}\n}}", flags)
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

            write!(f, "FamilyType {{ {}}}", flags)
        }
    }
}

///
/// High level wrapper for a vulkan queue family. Stores the family index, the number of queues that
/// this device supports for this family and the operations supported on this queue
///
#[derive(Clone, Debug)]
pub struct QueueFamily {
    pub i: u32,
    pub count: u32,
    pub family_type: QueueFamilyType,
}

impl QueueFamily {
    ///
    /// Default construction for a QueueFamily
    ///
    pub fn new() -> QueueFamily {
        QueueFamily {
            i: 0,
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
}

impl Default for QueueFamily {
    fn default() -> Self {
        Self::new()
    }
}
