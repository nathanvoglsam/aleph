//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::format::is_format_prohibited;
use aleph_vk_format::VkFormat;

///
/// Represents the set of supported `transferFunction` values
///
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum TransferFunction {
    Linear = 1,
    SRGB = 2,
}

impl TransferFunction {
    ///
    /// Tries to convert `v` into an enum variant. Returns `None` if v does not match one of the
    /// supported variants
    ///
    #[inline]
    pub fn from_raw(v: u8) -> Option<Self> {
        match v {
            1 => Some(TransferFunction::Linear),
            2 => Some(TransferFunction::SRGB),
            _ => None,
        }
    }

    ///
    /// Gets the raw `u8` value for the enum variant
    ///
    #[inline]
    pub const fn into_raw(self) -> u8 {
        self as u8
    }

    ///
    /// Is the color model correct for the given VkFormat
    ///
    #[inline]
    pub fn is_compatible_with_format(self, format: VkFormat) -> bool {
        let is_srgb = format.is_srgb();
        let allowed = format == VkFormat::UNDEFINED || is_format_prohibited(format);
        match (allowed, is_srgb, self) {
            (true, true, TransferFunction::SRGB) => true,
            (true, false, TransferFunction::Linear) => true,
            _ => false,
        }
    }

    ///
    /// Gets a TransferFunction value from the given format
    ///
    #[inline]
    pub fn for_format(format: VkFormat) -> Option<Self> {
        if is_format_prohibited(format) || format == VkFormat::UNDEFINED {
            None
        } else if format.is_srgb() {
            Some(TransferFunction::SRGB)
        } else {
            Some(TransferFunction::Linear)
        }
    }
}
