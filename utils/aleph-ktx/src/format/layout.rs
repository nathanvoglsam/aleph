//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vk_format::VkFormat;

///
/// Is the given format one of the PACKxx formats
///
#[inline]
pub fn is_pack_format(format: VkFormat) -> bool {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16
        | VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16
        | VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32
        | VkFormat::B10G11R11_UFLOAT_PACK32
        | VkFormat::E5B9G9R9_UFLOAT_PACK32 => true,
        _ => false,
    }
}
