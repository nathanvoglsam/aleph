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
/// Returns the number of bits in the red component of the given format.
///
/// Returns `None` if there is no red component
///
#[inline]
pub fn format_red_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16 => Some(4),
        VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(5),
        VkFormat::R8_UNORM
        | VkFormat::R8_SNORM
        | VkFormat::R8_UINT
        | VkFormat::R8_SINT
        | VkFormat::R8_SRGB
        | VkFormat::R8G8_UNORM
        | VkFormat::R8G8_SNORM
        | VkFormat::R8G8_UINT
        | VkFormat::R8G8_SINT
        | VkFormat::R8G8_SRGB
        | VkFormat::R8G8B8_UNORM
        | VkFormat::R8G8B8_SNORM
        | VkFormat::R8G8B8_UINT
        | VkFormat::R8G8B8_SINT
        | VkFormat::R8G8B8_SRGB
        | VkFormat::B8G8R8_UNORM
        | VkFormat::B8G8R8_SNORM
        | VkFormat::B8G8R8_UINT
        | VkFormat::B8G8R8_SINT
        | VkFormat::B8G8R8_SRGB
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => Some(8),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => Some(10),
        VkFormat::R16_UNORM
        | VkFormat::R16_SNORM
        | VkFormat::R16_UINT
        | VkFormat::R16_SINT
        | VkFormat::R16_SFLOAT
        | VkFormat::R16G16_UNORM
        | VkFormat::R16G16_SNORM
        | VkFormat::R16G16_UINT
        | VkFormat::R16G16_SINT
        | VkFormat::R16G16_SFLOAT
        | VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => Some(16),
        VkFormat::R32_UINT
        | VkFormat::R32_SINT
        | VkFormat::R32_SFLOAT
        | VkFormat::R32G32_UINT
        | VkFormat::R32G32_SINT
        | VkFormat::R32G32_SFLOAT
        | VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => Some(32),
        VkFormat::R64_UINT
        | VkFormat::R64_SINT
        | VkFormat::R64_SFLOAT
        | VkFormat::R64G64_UINT
        | VkFormat::R64G64_SINT
        | VkFormat::R64G64_SFLOAT
        | VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => Some(64),
        VkFormat::B10G11R11_UFLOAT_PACK32 => Some(11),
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(9),
        _ => None,
    }
}

///
/// Returns the number of bits in the green component of the given format.
///
/// Returns `None` if there is no green component
///
#[inline]
pub fn format_green_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16 => Some(4),
        VkFormat::R5G6B5_UNORM_PACK16 | VkFormat::B5G6R5_UNORM_PACK16 => Some(6),
        VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(5),
        VkFormat::R8G8_UNORM
        | VkFormat::R8G8_SNORM
        | VkFormat::R8G8_UINT
        | VkFormat::R8G8_SINT
        | VkFormat::R8G8_SRGB
        | VkFormat::R8G8B8_UNORM
        | VkFormat::R8G8B8_SNORM
        | VkFormat::R8G8B8_UINT
        | VkFormat::R8G8B8_SINT
        | VkFormat::R8G8B8_SRGB
        | VkFormat::B8G8R8_UNORM
        | VkFormat::B8G8R8_SNORM
        | VkFormat::B8G8R8_UINT
        | VkFormat::B8G8R8_SINT
        | VkFormat::B8G8R8_SRGB
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => Some(8),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => Some(10),
        VkFormat::R16G16_UNORM
        | VkFormat::R16G16_SNORM
        | VkFormat::R16G16_UINT
        | VkFormat::R16G16_SINT
        | VkFormat::R16G16_SFLOAT
        | VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => Some(16),
        VkFormat::R32G32_UINT
        | VkFormat::R32G32_SINT
        | VkFormat::R32G32_SFLOAT
        | VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => Some(32),
        VkFormat::R64G64_UINT
        | VkFormat::R64G64_SINT
        | VkFormat::R64G64_SFLOAT
        | VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => Some(64),
        VkFormat::B10G11R11_UFLOAT_PACK32 => Some(11),
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(9),
        _ => None,
    }
}

///
/// Returns the number of bits in the blue component of the given format.
///
/// Returns `None` if there is no blue component
///
#[inline]
pub fn format_blue_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::R4G4B4A4_UNORM_PACK16 | VkFormat::B4G4R4A4_UNORM_PACK16 => Some(4),
        VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(5),
        VkFormat::R8G8B8_UNORM
        | VkFormat::R8G8B8_SNORM
        | VkFormat::R8G8B8_UINT
        | VkFormat::R8G8B8_SINT
        | VkFormat::R8G8B8_SRGB
        | VkFormat::B8G8R8_UNORM
        | VkFormat::B8G8R8_SNORM
        | VkFormat::B8G8R8_UINT
        | VkFormat::B8G8R8_SINT
        | VkFormat::B8G8R8_SRGB
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => Some(8),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => Some(10),
        VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => Some(16),
        VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => Some(32),
        VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => Some(64),
        VkFormat::B10G11R11_UFLOAT_PACK32 => Some(11),
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(9),
        _ => None,
    }
}

///
/// Returns the number of bits in the blue component of the given format.
///
/// Returns `None` if there is no blue component
///
#[inline]
pub fn format_alpha_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::R4G4B4A4_UNORM_PACK16 | VkFormat::B4G4R4A4_UNORM_PACK16 => Some(4),
        VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(1),
        VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => Some(8),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => Some(2),
        VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => Some(16),
        VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => Some(32),
        VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => Some(64),
        _ => None,
    }
}

///
/// The number of bytes for a single pixel (or block for compressed formats) for a given format.
///
/// Returns `None` for prohibited formats
///
#[inline]
pub fn format_bytes_per_block(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::R4G4_UNORM_PACK8 => Some(1),
        VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16
        | VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => Some(2),
        VkFormat::R8_UNORM
        | VkFormat::R8_SNORM
        | VkFormat::R8_UINT
        | VkFormat::R8_SINT
        | VkFormat::R8_SRGB => Some(1),
        VkFormat::R8G8_UNORM
        | VkFormat::R8G8_SNORM
        | VkFormat::R8G8_UINT
        | VkFormat::R8G8_SINT
        | VkFormat::R8G8_SRGB => Some(2),
        VkFormat::R8G8B8_UNORM
        | VkFormat::R8G8B8_SNORM
        | VkFormat::R8G8B8_UINT
        | VkFormat::R8G8B8_SINT
        | VkFormat::R8G8B8_SRGB
        | VkFormat::B8G8R8_UNORM
        | VkFormat::B8G8R8_SNORM
        | VkFormat::B8G8R8_UINT
        | VkFormat::B8G8R8_SINT
        | VkFormat::B8G8R8_SRGB => Some(3),
        VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => Some(4),
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => Some(4),
        VkFormat::R16_UNORM
        | VkFormat::R16_SNORM
        | VkFormat::R16_UINT
        | VkFormat::R16_SINT
        | VkFormat::R16_SFLOAT => Some(2),
        VkFormat::R16G16_UNORM
        | VkFormat::R16G16_SNORM
        | VkFormat::R16G16_UINT
        | VkFormat::R16G16_SINT
        | VkFormat::R16G16_SFLOAT => Some(4),
        VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT => Some(6),
        VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => Some(8),
        VkFormat::R32_UINT | VkFormat::R32_SINT | VkFormat::R32_SFLOAT => Some(4),
        VkFormat::R32G32_UINT | VkFormat::R32G32_SINT | VkFormat::R32G32_SFLOAT => Some(8),
        VkFormat::R32G32B32_UINT | VkFormat::R32G32B32_SINT | VkFormat::R32G32B32_SFLOAT => {
            Some(12)
        }
        VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => Some(16),
        VkFormat::R64_UINT | VkFormat::R64_SINT | VkFormat::R64_SFLOAT => Some(8),
        VkFormat::R64G64_UINT | VkFormat::R64G64_SINT | VkFormat::R64G64_SFLOAT => Some(16),
        VkFormat::R64G64B64_UINT | VkFormat::R64G64B64_SINT | VkFormat::R64G64B64_SFLOAT => {
            Some(24)
        }
        VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => Some(32),
        VkFormat::B10G11R11_UFLOAT_PACK32 | VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(4),
        VkFormat::D16_UNORM => Some(2),
        VkFormat::X8_D24_UNORM_PACK32 | VkFormat::D32_SFLOAT => Some(4),
        VkFormat::S8_UINT => Some(1),
        VkFormat::D16_UNORM_S8_UINT => Some(2),
        VkFormat::D24_UNORM_S8_UINT => Some(4),
        VkFormat::D32_SFLOAT_S8_UINT => Some(8),
        VkFormat::BC1_RGB_UNORM_BLOCK
        | VkFormat::BC1_RGB_SRGB_BLOCK
        | VkFormat::BC1_RGBA_UNORM_BLOCK
        | VkFormat::BC1_RGBA_SRGB_BLOCK => Some(8),
        VkFormat::BC2_UNORM_BLOCK | VkFormat::BC2_SRGB_BLOCK => Some(16),
        VkFormat::BC3_UNORM_BLOCK | VkFormat::BC3_SRGB_BLOCK => Some(16),
        VkFormat::BC4_UNORM_BLOCK | VkFormat::BC4_SNORM_BLOCK => Some(8),
        VkFormat::BC5_UNORM_BLOCK | VkFormat::BC5_SNORM_BLOCK => Some(16),
        VkFormat::BC6H_UFLOAT_BLOCK | VkFormat::BC6H_SFLOAT_BLOCK => Some(8),
        VkFormat::BC7_UNORM_BLOCK | VkFormat::BC7_SRGB_BLOCK => Some(16),
        VkFormat::ETC2_R8G8B8_UNORM_BLOCK
        | VkFormat::ETC2_R8G8B8_SRGB_BLOCK
        | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
        | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK => Some(8),
        VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK => Some(16),
        VkFormat::EAC_R11_UNORM_BLOCK | VkFormat::EAC_R11_SNORM_BLOCK => Some(8),
        VkFormat::EAC_R11G11_UNORM_BLOCK | VkFormat::EAC_R11G11_SNORM_BLOCK => Some(16),
        VkFormat::ASTC_4X4_UNORM_BLOCK
        | VkFormat::ASTC_4X4_SRGB_BLOCK
        | VkFormat::ASTC_5X4_UNORM_BLOCK
        | VkFormat::ASTC_5X4_SRGB_BLOCK
        | VkFormat::ASTC_5X5_UNORM_BLOCK
        | VkFormat::ASTC_5X5_SRGB_BLOCK
        | VkFormat::ASTC_6X5_UNORM_BLOCK
        | VkFormat::ASTC_6X5_SRGB_BLOCK
        | VkFormat::ASTC_6X6_UNORM_BLOCK
        | VkFormat::ASTC_6X6_SRGB_BLOCK
        | VkFormat::ASTC_8X5_UNORM_BLOCK
        | VkFormat::ASTC_8X5_SRGB_BLOCK
        | VkFormat::ASTC_8X6_UNORM_BLOCK
        | VkFormat::ASTC_8X6_SRGB_BLOCK
        | VkFormat::ASTC_8X8_UNORM_BLOCK
        | VkFormat::ASTC_8X8_SRGB_BLOCK
        | VkFormat::ASTC_10X5_UNORM_BLOCK
        | VkFormat::ASTC_10X5_SRGB_BLOCK
        | VkFormat::ASTC_10X6_UNORM_BLOCK
        | VkFormat::ASTC_10X6_SRGB_BLOCK
        | VkFormat::ASTC_10X8_UNORM_BLOCK
        | VkFormat::ASTC_10X8_SRGB_BLOCK
        | VkFormat::ASTC_10X10_UNORM_BLOCK
        | VkFormat::ASTC_10X10_SRGB_BLOCK
        | VkFormat::ASTC_12X10_UNORM_BLOCK
        | VkFormat::ASTC_12X10_SRGB_BLOCK
        | VkFormat::ASTC_12X12_UNORM_BLOCK
        | VkFormat::ASTC_12X12_SRGB_BLOCK
        | VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT
        | VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT => Some(16),
        VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG => Some(8),
        _ => None,
    }
}

///
/// Returns the number of bits in the depth component of the given format.
///
/// Returns `None` if there is no depth component
///
#[inline]
pub fn format_depth_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::D16_UNORM | VkFormat::D16_UNORM_S8_UINT => Some(16),
        VkFormat::D24_UNORM_S8_UINT | VkFormat::X8_D24_UNORM_PACK32 => Some(24),
        VkFormat::D32_SFLOAT | VkFormat::D32_SFLOAT_S8_UINT => Some(32),
        _ => None,
    }
}

///
/// Returns the number of bits in the stencil component of the given format.
///
/// Returns `None` if there is no stencil component
///
#[inline]
pub fn format_stencil_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::S8_UINT
        | VkFormat::D16_UNORM_S8_UINT
        | VkFormat::D24_UNORM_S8_UINT
        | VkFormat::D32_SFLOAT_S8_UINT => Some(8),
        _ => None,
    }
}

///
/// Returns the number of bits in the exponent component of the given format.
///
/// Returns 0 if there is no exponent component
///
#[inline]
pub fn format_exponent_bits(format: VkFormat) -> Option<u8> {
    match format {
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(5),
        _ => None,
    }
}
