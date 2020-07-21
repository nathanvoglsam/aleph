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
/// If this format has been marked as prohibited by the KTX 2.0 spec
///
#[inline]
pub fn is_format_prohibited(format: VkFormat) -> bool {
    match format {
        VkFormat::R8_USCALED
        | VkFormat::R8_SSCALED
        | VkFormat::R8G8_USCALED
        | VkFormat::R8G8_SSCALED
        | VkFormat::R8G8B8_USCALED
        | VkFormat::R8G8B8_SSCALED
        | VkFormat::B8G8R8_USCALED
        | VkFormat::B8G8R8_SSCALED
        | VkFormat::R8G8B8A8_USCALED
        | VkFormat::R8G8B8A8_SSCALED
        | VkFormat::B8G8R8A8_USCALED
        | VkFormat::B8G8R8A8_SSCALED
        | VkFormat::A8B8G8R8_UNORM_PACK32
        | VkFormat::A8B8G8R8_SNORM_PACK32
        | VkFormat::A8B8G8R8_USCALED_PACK32
        | VkFormat::A8B8G8R8_SSCALED_PACK32
        | VkFormat::A8B8G8R8_UINT_PACK32
        | VkFormat::A8B8G8R8_SINT_PACK32
        | VkFormat::A8B8G8R8_SRGB_PACK32
        | VkFormat::A2R10G10B10_USCALED_PACK32
        | VkFormat::A2R10G10B10_SSCALED_PACK32
        | VkFormat::A2B10G10R10_USCALED_PACK32
        | VkFormat::A2B10G10R10_SSCALED_PACK32
        | VkFormat::R16_USCALED
        | VkFormat::R16_SSCALED
        | VkFormat::R16G16_USCALED
        | VkFormat::R16G16_SSCALED
        | VkFormat::R16G16B16_USCALED
        | VkFormat::R16G16B16_SSCALED
        | VkFormat::R16G16B16A16_USCALED
        | VkFormat::R16G16B16A16_SSCALED
        | VkFormat::G8B8G8R8_422_UNORM
        | VkFormat::B8G8R8G8_422_UNORM
        | VkFormat::G8_B8_R8_3PLANE_420_UNORM
        | VkFormat::G8_B8R8_2PLANE_420_UNORM
        | VkFormat::G8_B8_R8_3PLANE_422_UNORM
        | VkFormat::G8_B8R8_2PLANE_422_UNORM
        | VkFormat::G8_B8_R8_3PLANE_444_UNORM
        | VkFormat::R10X6_UNORM_PACK16
        | VkFormat::R10X6G10X6_UNORM_2PACK16
        | VkFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16
        | VkFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16
        | VkFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16
        | VkFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16
        | VkFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16
        | VkFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16
        | VkFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16
        | VkFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16
        | VkFormat::R12X4_UNORM_PACK16
        | VkFormat::R12X4G12X4_UNORM_2PACK16
        | VkFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16
        | VkFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16
        | VkFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16
        | VkFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16
        | VkFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16
        | VkFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16
        | VkFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16
        | VkFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16
        | VkFormat::G16B16G16R16_422_UNORM
        | VkFormat::B16G16R16G16_422_UNORM
        | VkFormat::G16_B16_R16_3PLANE_420_UNORM
        | VkFormat::G16_B16R16_2PLANE_420_UNORM
        | VkFormat::G16_B16_R16_3PLANE_422_UNORM
        | VkFormat::G16_B16R16_2PLANE_422_UNORM
        | VkFormat::G16_B16_R16_3PLANE_444_UNORM => true,
        _ => false,
    }
}

///
/// If this format has been marked explicitly as unsupported by our implementation
///
#[inline]
pub fn is_format_unsupported(format: VkFormat) -> bool {
    match format {
        VkFormat::UNDEFINED
        | VkFormat::D16_UNORM_S8_UINT
        | VkFormat::D24_UNORM_S8_UINT
        | VkFormat::X8_D24_UNORM_PACK32
        | VkFormat::D32_SFLOAT_S8_UINT => true,
        _ => false,
    }
}
