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
/// Returns 0 if there is no red component
///
#[inline]
pub fn format_red_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16 => 4,
        VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => 5,
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
        | VkFormat::B8G8R8A8_SRGB => 8,
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => 10,
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
        | VkFormat::R16G16B16A16_SFLOAT => 16,
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
        | VkFormat::R32G32B32A32_SFLOAT => 32,
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
        | VkFormat::R64G64B64A64_SFLOAT => 64,
        VkFormat::B10G11R11_UFLOAT_PACK32 => 11,
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => 9,
        _ => 0,
    }
}

///
/// Returns the number of bits in the green component of the given format.
///
/// Returns 0 if there is no green component
///
#[inline]
pub fn format_green_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16 => 4,
        VkFormat::R5G6B5_UNORM_PACK16 | VkFormat::B5G6R5_UNORM_PACK16 => 6,
        VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => 5,
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
        | VkFormat::B8G8R8A8_SRGB => 8,
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => 10,
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
        | VkFormat::R16G16B16A16_SFLOAT => 16,
        VkFormat::R32G32_UINT
        | VkFormat::R32G32_SINT
        | VkFormat::R32G32_SFLOAT
        | VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => 32,
        VkFormat::R64G64_UINT
        | VkFormat::R64G64_SINT
        | VkFormat::R64G64_SFLOAT
        | VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => 64,
        VkFormat::B10G11R11_UFLOAT_PACK32 => 11,
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => 9,
        _ => 0,
    }
}

///
/// Returns the number of bits in the blue component of the given format.
///
/// Returns 0 if there is no blue component
///
#[inline]
pub fn format_blue_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::R4G4B4A4_UNORM_PACK16 | VkFormat::B4G4R4A4_UNORM_PACK16 => 4,
        VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => 5,
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
        | VkFormat::B8G8R8A8_SRGB => 8,
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => 10,
        VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => 16,
        VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => 32,
        VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => 64,
        VkFormat::B10G11R11_UFLOAT_PACK32 => 11,
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => 9,
        _ => 0,
    }
}

///
/// Returns the number of bits in the blue component of the given format.
///
/// Returns 0 if there is no blue component
///
#[inline]
pub fn format_alpha_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::R4G4B4A4_UNORM_PACK16 | VkFormat::B4G4R4A4_UNORM_PACK16 => 4,
        VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16 => 1,
        VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB => 8,
        VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => 2,
        VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT => 16,
        VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT => 32,
        VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => 64,
        _ => 0,
    }
}

///
/// Returns the number of bits in the depth component of the given format.
///
/// Returns 0 if there is no depth component
///
#[inline]
pub fn format_depth_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::D16_UNORM | VkFormat::D16_UNORM_S8_UINT => 16,
        VkFormat::D24_UNORM_S8_UINT | VkFormat::X8_D24_UNORM_PACK32 => 24,
        VkFormat::D32_SFLOAT | VkFormat::D32_SFLOAT_S8_UINT => 32,
        _ => 0,
    }
}

///
/// Returns the number of bits in the stencil component of the given format.
///
/// Returns 0 if there is no stencil component
///
#[inline]
pub fn format_stencil_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::S8_UINT
        | VkFormat::D16_UNORM_S8_UINT
        | VkFormat::D24_UNORM_S8_UINT
        | VkFormat::D32_SFLOAT_S8_UINT => 8,
        _ => 0,
    }
}

///
/// Returns the number of bits in the exponent component of the given format.
///
/// Returns 0 if there is no exponent component
///
#[inline]
pub fn format_exponent_bits(format: VkFormat) -> u16 {
    match format {
        VkFormat::E5B9G9R9_UFLOAT_PACK32 => 5,
        _ => 0,
    }
}
