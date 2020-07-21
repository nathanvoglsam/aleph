//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(test)]
mod tests;

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

///
/// If the format will place the alpha channel in the first byte/most significant bits of the format
///
/// # Info
///
/// This will be true for formats like A1R5G5B5_UNORM_PACK16, where A is in the MSB and then the
/// rest of the channels follow.
///
/// This will also return `false` for any formats that don't have an alpha component.
///
/// # Warning
///
/// This will return `false` on any block format as there is no distinct channel ordering for those
/// formats.
///
pub fn is_format_alpha_first_ordered(format: VkFormat) -> bool {
    match format {
        VkFormat::A1R5G5B5_UNORM_PACK16
        | VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32 => true,
        _ => false,
    }
}

///
/// Returns whether this format has an alpha component
///
/// # Warning
///
/// This always returns false for block formats as there isn't really a concrete concept of "alpha"
/// channels for those formats.
///
/// There is no consistent meaning to having an alpha channel in compressed formats. For example:
/// sampling an ASTC texture will yield and alpha value but an actual alpha component will only be
/// stored depending on which format the individual block is in. For storage purposes the actual
/// component layout is completely opaque and so this function wouldn't say anything meaningful
/// about the format in the usage context expected of it.
///
pub fn format_has_alpha(format: VkFormat) -> bool {
    match format {
        VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB
        | VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT => true,
        _ => false,
    }
}

///
/// If the format lays out components in an order that matches the formats like R8G8B8
///
/// # Info
///
/// This will be true for any format where the RGB components are laid out where R is in the lowest
/// byte, G in the next, B in the next, etc
///
/// For PACKxx formats this would mean formats the put R in the most significant bits, G in the next
/// most significant, B in the next most significant, etc
///
/// # Warning
///
/// This will return `false` on any block format as there is no distinct channel ordering for those
/// formats.
///
pub fn is_format_rgb_ordered(format: VkFormat) -> bool {
    match format {
        VkFormat::R4G4_UNORM_PACK8
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16
        | VkFormat::R8_UNORM
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
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::R16_UNORM
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
        | VkFormat::R16G16B16A16_SFLOAT
        | VkFormat::R32_UINT
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
        | VkFormat::R32G32B32A32_SFLOAT
        | VkFormat::R64_UINT
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
        | VkFormat::R64G64B64A64_SFLOAT
        | VkFormat::D16_UNORM
        | VkFormat::D32_SFLOAT
        | VkFormat::S8_UINT => true,
        _ => false,
    }
}

///
/// Returns the number of sample information blocks that are expected in the DFD for the given
/// format.
///
#[inline]
pub fn format_sample_info_count(format: VkFormat) -> Option<u16> {
    match format {
        //1
        VkFormat::R8_UNORM
        | VkFormat::R8_SNORM
        | VkFormat::R8_UINT
        | VkFormat::R8_SINT
        | VkFormat::R8_SRGB
        | VkFormat::R16_UNORM
        | VkFormat::R16_SNORM
        | VkFormat::R16_UINT
        | VkFormat::R16_SINT
        | VkFormat::R16_SFLOAT
        | VkFormat::R32_UINT
        | VkFormat::R32_SINT
        | VkFormat::R32_SFLOAT
        | VkFormat::R64_UINT
        | VkFormat::R64_SINT
        | VkFormat::R64_SFLOAT
        | VkFormat::D16_UNORM
        | VkFormat::D32_SFLOAT
        | VkFormat::S8_UINT
        // BC1 uses a single block to produce a 4x4 RGB region. The alpha is encoded by reusing
        // degenerate states and so is inseparable from the RGB block
        | VkFormat::BC1_RGB_UNORM_BLOCK
        | VkFormat::BC1_RGB_SRGB_BLOCK
        | VkFormat::BC1_RGBA_UNORM_BLOCK
        | VkFormat::BC1_RGBA_SRGB_BLOCK
        // BC4 is inherently a 1 channel format, and produces 1 channel when sampled
        | VkFormat::BC4_UNORM_BLOCK
        | VkFormat::BC4_SNORM_BLOCK
        // BC6 uses a single block for floating point RGB
        | VkFormat::BC6H_UFLOAT_BLOCK
        | VkFormat::BC6H_SFLOAT_BLOCK
        // BC7 uses a single block for representing RGBA
        | VkFormat::BC7_UNORM_BLOCK
        | VkFormat::BC7_SRGB_BLOCK
        // All the ASTC formats use 1 inseparable block so are 1 storage channel formats
        | VkFormat::ASTC_4X4_UNORM_BLOCK
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
        | VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT
        // These EAC formats use a single block for their sample channel so are 1 storage channel
        // formats
        | VkFormat::EAC_R11_UNORM_BLOCK
        | VkFormat::EAC_R11_SNORM_BLOCK
        // The PVRTC formats are all 1 block -> samples so are all 1 storage component formats
        | VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
        | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
        | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        | VkFormat::ETC2_R8G8B8_UNORM_BLOCK
        | VkFormat::ETC2_R8G8B8_SRGB_BLOCK => Some(1),

        //2
        VkFormat::R8G8_UNORM
        | VkFormat::R8G8_SNORM
        | VkFormat::R8G8_UINT
        | VkFormat::R8G8_SINT
        | VkFormat::R8G8_SRGB
        | VkFormat::R16G16_UNORM
        | VkFormat::R16G16_SNORM
        | VkFormat::R16G16_UINT
        | VkFormat::R16G16_SINT
        | VkFormat::R16G16_SFLOAT
        | VkFormat::R32G32_UINT
        | VkFormat::R32G32_SINT
        | VkFormat::R32G32_SFLOAT
        | VkFormat::R64G64_UINT
        | VkFormat::R64G64_SINT
        | VkFormat::R64G64_SFLOAT
        | VkFormat::R4G4_UNORM_PACK8
        // BC2 is just BC1 + a 4bpp part after the BC1 block so the alpha can be separated, making
        // it a 2 storage component format
        | VkFormat::BC2_UNORM_BLOCK
        | VkFormat::BC2_SRGB_BLOCK
        // BC3 is just BC1 for rgb with a BC4 block for the alpha component. These could be
        // separated so that makes BC3 a 2 storage component format
        | VkFormat::BC3_UNORM_BLOCK
        | VkFormat::BC3_SRGB_BLOCK
        // BC5 is just two BC4 blocks for each sample channel so, making them both separable.
        // Therefor BC5 has 2 storage components
        | VkFormat::BC5_UNORM_BLOCK
        | VkFormat::BC5_SNORM_BLOCK
        // Uses 1 block for each sample component so that makes these 2 storage component formats
        | VkFormat::EAC_R11G11_UNORM_BLOCK
        | VkFormat::EAC_R11G11_SNORM_BLOCK
        // Stores two components with 6 bits of padding in each, making it a 2 storage component
        // format
        | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
        | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
        | VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK
        | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK => Some(2),

        //3
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
        | VkFormat::R16G16B16_UNORM
        | VkFormat::R16G16B16_SNORM
        | VkFormat::R16G16B16_UINT
        | VkFormat::R16G16B16_SINT
        | VkFormat::R16G16B16_SFLOAT
        | VkFormat::R32G32B32_UINT
        | VkFormat::R32G32B32_SINT
        | VkFormat::R32G32B32_SFLOAT
        | VkFormat::R64G64B64_UINT
        | VkFormat::R64G64B64_SINT
        | VkFormat::R64G64B64_SFLOAT
        | VkFormat::R5G6B5_UNORM_PACK16
        | VkFormat::B5G6R5_UNORM_PACK16
        | VkFormat::A2R10G10B10_UNORM_PACK32
        | VkFormat::A2R10G10B10_SNORM_PACK32
        | VkFormat::A2R10G10B10_UINT_PACK32
        | VkFormat::A2R10G10B10_SINT_PACK32
        | VkFormat::A2B10G10R10_UNORM_PACK32
        | VkFormat::A2B10G10R10_SNORM_PACK32
        | VkFormat::A2B10G10R10_UINT_PACK32
        | VkFormat::A2B10G10R10_SINT_PACK32
        | VkFormat::R5G5B5A1_UNORM_PACK16
        | VkFormat::B5G5R5A1_UNORM_PACK16
        | VkFormat::A1R5G5B5_UNORM_PACK16
        | VkFormat::B10G11R11_UFLOAT_PACK32 => Some(3),

        //4
        | VkFormat::R8G8B8A8_UNORM
        | VkFormat::R8G8B8A8_SNORM
        | VkFormat::R8G8B8A8_UINT
        | VkFormat::R8G8B8A8_SINT
        | VkFormat::R8G8B8A8_SRGB
        | VkFormat::B8G8R8A8_UNORM
        | VkFormat::B8G8R8A8_SNORM
        | VkFormat::B8G8R8A8_UINT
        | VkFormat::B8G8R8A8_SINT
        | VkFormat::B8G8R8A8_SRGB
        | VkFormat::R16G16B16A16_UNORM
        | VkFormat::R16G16B16A16_SNORM
        | VkFormat::R16G16B16A16_UINT
        | VkFormat::R16G16B16A16_SINT
        | VkFormat::R16G16B16A16_SFLOAT
        | VkFormat::R32G32B32A32_UINT
        | VkFormat::R32G32B32A32_SINT
        | VkFormat::R32G32B32A32_SFLOAT
        | VkFormat::R64G64B64A64_UINT
        | VkFormat::R64G64B64A64_SINT
        | VkFormat::R64G64B64A64_SFLOAT
        | VkFormat::R4G4B4A4_UNORM_PACK16
        | VkFormat::B4G4R4A4_UNORM_PACK16
        | VkFormat::E5B9G9R9_UFLOAT_PACK32 => Some(4),

        _ => None,
    }
}

///
/// A list of all formats allowed by the KTX2 spec
///
pub const ALLOWED_FORMATS: [VkFormat; 176] = [
    VkFormat::UNDEFINED,
    VkFormat::R4G4_UNORM_PACK8,
    VkFormat::R4G4B4A4_UNORM_PACK16,
    VkFormat::B4G4R4A4_UNORM_PACK16,
    VkFormat::R5G6B5_UNORM_PACK16,
    VkFormat::B5G6R5_UNORM_PACK16,
    VkFormat::R5G5B5A1_UNORM_PACK16,
    VkFormat::B5G5R5A1_UNORM_PACK16,
    VkFormat::A1R5G5B5_UNORM_PACK16,
    VkFormat::R8_UNORM,
    VkFormat::R8_SNORM,
    VkFormat::R8_UINT,
    VkFormat::R8_SINT,
    VkFormat::R8_SRGB,
    VkFormat::R8G8_UNORM,
    VkFormat::R8G8_SNORM,
    VkFormat::R8G8_UINT,
    VkFormat::R8G8_SINT,
    VkFormat::R8G8_SRGB,
    VkFormat::R8G8B8_UNORM,
    VkFormat::R8G8B8_SNORM,
    VkFormat::R8G8B8_UINT,
    VkFormat::R8G8B8_SINT,
    VkFormat::R8G8B8_SRGB,
    VkFormat::B8G8R8_UNORM,
    VkFormat::B8G8R8_SNORM,
    VkFormat::B8G8R8_UINT,
    VkFormat::B8G8R8_SINT,
    VkFormat::B8G8R8_SRGB,
    VkFormat::R8G8B8A8_UNORM,
    VkFormat::R8G8B8A8_SNORM,
    VkFormat::R8G8B8A8_UINT,
    VkFormat::R8G8B8A8_SINT,
    VkFormat::R8G8B8A8_SRGB,
    VkFormat::B8G8R8A8_UNORM,
    VkFormat::B8G8R8A8_SNORM,
    VkFormat::B8G8R8A8_UINT,
    VkFormat::B8G8R8A8_SINT,
    VkFormat::B8G8R8A8_SRGB,
    VkFormat::A2R10G10B10_UNORM_PACK32,
    VkFormat::A2R10G10B10_SNORM_PACK32,
    VkFormat::A2R10G10B10_UINT_PACK32,
    VkFormat::A2R10G10B10_SINT_PACK32,
    VkFormat::A2B10G10R10_UNORM_PACK32,
    VkFormat::A2B10G10R10_SNORM_PACK32,
    VkFormat::A2B10G10R10_UINT_PACK32,
    VkFormat::A2B10G10R10_SINT_PACK32,
    VkFormat::R16_UNORM,
    VkFormat::R16_SNORM,
    VkFormat::R16_UINT,
    VkFormat::R16_SINT,
    VkFormat::R16_SFLOAT,
    VkFormat::R16G16_UNORM,
    VkFormat::R16G16_SNORM,
    VkFormat::R16G16_UINT,
    VkFormat::R16G16_SINT,
    VkFormat::R16G16_SFLOAT,
    VkFormat::R16G16B16_UNORM,
    VkFormat::R16G16B16_SNORM,
    VkFormat::R16G16B16_UINT,
    VkFormat::R16G16B16_SINT,
    VkFormat::R16G16B16_SFLOAT,
    VkFormat::R16G16B16A16_UNORM,
    VkFormat::R16G16B16A16_SNORM,
    VkFormat::R16G16B16A16_UINT,
    VkFormat::R16G16B16A16_SINT,
    VkFormat::R16G16B16A16_SFLOAT,
    VkFormat::R32_UINT,
    VkFormat::R32_SINT,
    VkFormat::R32_SFLOAT,
    VkFormat::R32G32_UINT,
    VkFormat::R32G32_SINT,
    VkFormat::R32G32_SFLOAT,
    VkFormat::R32G32B32_UINT,
    VkFormat::R32G32B32_SINT,
    VkFormat::R32G32B32_SFLOAT,
    VkFormat::R32G32B32A32_UINT,
    VkFormat::R32G32B32A32_SINT,
    VkFormat::R32G32B32A32_SFLOAT,
    VkFormat::R64_UINT,
    VkFormat::R64_SINT,
    VkFormat::R64_SFLOAT,
    VkFormat::R64G64_UINT,
    VkFormat::R64G64_SINT,
    VkFormat::R64G64_SFLOAT,
    VkFormat::R64G64B64_UINT,
    VkFormat::R64G64B64_SINT,
    VkFormat::R64G64B64_SFLOAT,
    VkFormat::R64G64B64A64_UINT,
    VkFormat::R64G64B64A64_SINT,
    VkFormat::R64G64B64A64_SFLOAT,
    VkFormat::B10G11R11_UFLOAT_PACK32,
    VkFormat::E5B9G9R9_UFLOAT_PACK32,
    VkFormat::D16_UNORM,
    VkFormat::X8_D24_UNORM_PACK32,
    VkFormat::D32_SFLOAT,
    VkFormat::S8_UINT,
    VkFormat::D16_UNORM_S8_UINT,
    VkFormat::D24_UNORM_S8_UINT,
    VkFormat::D32_SFLOAT_S8_UINT,
    VkFormat::BC1_RGB_UNORM_BLOCK,
    VkFormat::BC1_RGB_SRGB_BLOCK,
    VkFormat::BC1_RGBA_UNORM_BLOCK,
    VkFormat::BC1_RGBA_SRGB_BLOCK,
    VkFormat::BC2_UNORM_BLOCK,
    VkFormat::BC2_SRGB_BLOCK,
    VkFormat::BC3_UNORM_BLOCK,
    VkFormat::BC3_SRGB_BLOCK,
    VkFormat::BC4_UNORM_BLOCK,
    VkFormat::BC4_SNORM_BLOCK,
    VkFormat::BC5_UNORM_BLOCK,
    VkFormat::BC5_SNORM_BLOCK,
    VkFormat::BC6H_UFLOAT_BLOCK,
    VkFormat::BC6H_SFLOAT_BLOCK,
    VkFormat::BC7_UNORM_BLOCK,
    VkFormat::BC7_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK,
    VkFormat::EAC_R11_UNORM_BLOCK,
    VkFormat::EAC_R11_SNORM_BLOCK,
    VkFormat::EAC_R11G11_UNORM_BLOCK,
    VkFormat::EAC_R11G11_SNORM_BLOCK,
    VkFormat::ASTC_4X4_UNORM_BLOCK,
    VkFormat::ASTC_4X4_SRGB_BLOCK,
    VkFormat::ASTC_5X4_UNORM_BLOCK,
    VkFormat::ASTC_5X4_SRGB_BLOCK,
    VkFormat::ASTC_5X5_UNORM_BLOCK,
    VkFormat::ASTC_5X5_SRGB_BLOCK,
    VkFormat::ASTC_6X5_UNORM_BLOCK,
    VkFormat::ASTC_6X5_SRGB_BLOCK,
    VkFormat::ASTC_6X6_UNORM_BLOCK,
    VkFormat::ASTC_6X6_SRGB_BLOCK,
    VkFormat::ASTC_8X5_UNORM_BLOCK,
    VkFormat::ASTC_8X5_SRGB_BLOCK,
    VkFormat::ASTC_8X6_UNORM_BLOCK,
    VkFormat::ASTC_8X6_SRGB_BLOCK,
    VkFormat::ASTC_8X8_UNORM_BLOCK,
    VkFormat::ASTC_8X8_SRGB_BLOCK,
    VkFormat::ASTC_10X5_UNORM_BLOCK,
    VkFormat::ASTC_10X5_SRGB_BLOCK,
    VkFormat::ASTC_10X6_UNORM_BLOCK,
    VkFormat::ASTC_10X6_SRGB_BLOCK,
    VkFormat::ASTC_10X8_UNORM_BLOCK,
    VkFormat::ASTC_10X8_SRGB_BLOCK,
    VkFormat::ASTC_10X10_UNORM_BLOCK,
    VkFormat::ASTC_10X10_SRGB_BLOCK,
    VkFormat::ASTC_12X10_UNORM_BLOCK,
    VkFormat::ASTC_12X10_SRGB_BLOCK,
    VkFormat::ASTC_12X12_UNORM_BLOCK,
    VkFormat::ASTC_12X12_SRGB_BLOCK,
    VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT,
    VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG,
];

///
/// A list of all formats supported by our KTX2 implementation
///
pub const SUPPORTED_FORMATS: [VkFormat; 171] = [
    VkFormat::R4G4_UNORM_PACK8,
    VkFormat::R4G4B4A4_UNORM_PACK16,
    VkFormat::B4G4R4A4_UNORM_PACK16,
    VkFormat::R5G6B5_UNORM_PACK16,
    VkFormat::B5G6R5_UNORM_PACK16,
    VkFormat::R5G5B5A1_UNORM_PACK16,
    VkFormat::B5G5R5A1_UNORM_PACK16,
    VkFormat::A1R5G5B5_UNORM_PACK16,
    VkFormat::R8_UNORM,
    VkFormat::R8_SNORM,
    VkFormat::R8_UINT,
    VkFormat::R8_SINT,
    VkFormat::R8_SRGB,
    VkFormat::R8G8_UNORM,
    VkFormat::R8G8_SNORM,
    VkFormat::R8G8_UINT,
    VkFormat::R8G8_SINT,
    VkFormat::R8G8_SRGB,
    VkFormat::R8G8B8_UNORM,
    VkFormat::R8G8B8_SNORM,
    VkFormat::R8G8B8_UINT,
    VkFormat::R8G8B8_SINT,
    VkFormat::R8G8B8_SRGB,
    VkFormat::B8G8R8_UNORM,
    VkFormat::B8G8R8_SNORM,
    VkFormat::B8G8R8_UINT,
    VkFormat::B8G8R8_SINT,
    VkFormat::B8G8R8_SRGB,
    VkFormat::R8G8B8A8_UNORM,
    VkFormat::R8G8B8A8_SNORM,
    VkFormat::R8G8B8A8_UINT,
    VkFormat::R8G8B8A8_SINT,
    VkFormat::R8G8B8A8_SRGB,
    VkFormat::B8G8R8A8_UNORM,
    VkFormat::B8G8R8A8_SNORM,
    VkFormat::B8G8R8A8_UINT,
    VkFormat::B8G8R8A8_SINT,
    VkFormat::B8G8R8A8_SRGB,
    VkFormat::A2R10G10B10_UNORM_PACK32,
    VkFormat::A2R10G10B10_SNORM_PACK32,
    VkFormat::A2R10G10B10_UINT_PACK32,
    VkFormat::A2R10G10B10_SINT_PACK32,
    VkFormat::A2B10G10R10_UNORM_PACK32,
    VkFormat::A2B10G10R10_SNORM_PACK32,
    VkFormat::A2B10G10R10_UINT_PACK32,
    VkFormat::A2B10G10R10_SINT_PACK32,
    VkFormat::R16_UNORM,
    VkFormat::R16_SNORM,
    VkFormat::R16_UINT,
    VkFormat::R16_SINT,
    VkFormat::R16_SFLOAT,
    VkFormat::R16G16_UNORM,
    VkFormat::R16G16_SNORM,
    VkFormat::R16G16_UINT,
    VkFormat::R16G16_SINT,
    VkFormat::R16G16_SFLOAT,
    VkFormat::R16G16B16_UNORM,
    VkFormat::R16G16B16_SNORM,
    VkFormat::R16G16B16_UINT,
    VkFormat::R16G16B16_SINT,
    VkFormat::R16G16B16_SFLOAT,
    VkFormat::R16G16B16A16_UNORM,
    VkFormat::R16G16B16A16_SNORM,
    VkFormat::R16G16B16A16_UINT,
    VkFormat::R16G16B16A16_SINT,
    VkFormat::R16G16B16A16_SFLOAT,
    VkFormat::R32_UINT,
    VkFormat::R32_SINT,
    VkFormat::R32_SFLOAT,
    VkFormat::R32G32_UINT,
    VkFormat::R32G32_SINT,
    VkFormat::R32G32_SFLOAT,
    VkFormat::R32G32B32_UINT,
    VkFormat::R32G32B32_SINT,
    VkFormat::R32G32B32_SFLOAT,
    VkFormat::R32G32B32A32_UINT,
    VkFormat::R32G32B32A32_SINT,
    VkFormat::R32G32B32A32_SFLOAT,
    VkFormat::R64_UINT,
    VkFormat::R64_SINT,
    VkFormat::R64_SFLOAT,
    VkFormat::R64G64_UINT,
    VkFormat::R64G64_SINT,
    VkFormat::R64G64_SFLOAT,
    VkFormat::R64G64B64_UINT,
    VkFormat::R64G64B64_SINT,
    VkFormat::R64G64B64_SFLOAT,
    VkFormat::R64G64B64A64_UINT,
    VkFormat::R64G64B64A64_SINT,
    VkFormat::R64G64B64A64_SFLOAT,
    VkFormat::B10G11R11_UFLOAT_PACK32,
    VkFormat::E5B9G9R9_UFLOAT_PACK32,
    VkFormat::D16_UNORM,
    VkFormat::D32_SFLOAT,
    VkFormat::S8_UINT,
    VkFormat::BC1_RGB_UNORM_BLOCK,
    VkFormat::BC1_RGB_SRGB_BLOCK,
    VkFormat::BC1_RGBA_UNORM_BLOCK,
    VkFormat::BC1_RGBA_SRGB_BLOCK,
    VkFormat::BC2_UNORM_BLOCK,
    VkFormat::BC2_SRGB_BLOCK,
    VkFormat::BC3_UNORM_BLOCK,
    VkFormat::BC3_SRGB_BLOCK,
    VkFormat::BC4_UNORM_BLOCK,
    VkFormat::BC4_SNORM_BLOCK,
    VkFormat::BC5_UNORM_BLOCK,
    VkFormat::BC5_SNORM_BLOCK,
    VkFormat::BC6H_UFLOAT_BLOCK,
    VkFormat::BC6H_SFLOAT_BLOCK,
    VkFormat::BC7_UNORM_BLOCK,
    VkFormat::BC7_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK,
    VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK,
    VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK,
    VkFormat::EAC_R11_UNORM_BLOCK,
    VkFormat::EAC_R11_SNORM_BLOCK,
    VkFormat::EAC_R11G11_UNORM_BLOCK,
    VkFormat::EAC_R11G11_SNORM_BLOCK,
    VkFormat::ASTC_4X4_UNORM_BLOCK,
    VkFormat::ASTC_4X4_SRGB_BLOCK,
    VkFormat::ASTC_5X4_UNORM_BLOCK,
    VkFormat::ASTC_5X4_SRGB_BLOCK,
    VkFormat::ASTC_5X5_UNORM_BLOCK,
    VkFormat::ASTC_5X5_SRGB_BLOCK,
    VkFormat::ASTC_6X5_UNORM_BLOCK,
    VkFormat::ASTC_6X5_SRGB_BLOCK,
    VkFormat::ASTC_6X6_UNORM_BLOCK,
    VkFormat::ASTC_6X6_SRGB_BLOCK,
    VkFormat::ASTC_8X5_UNORM_BLOCK,
    VkFormat::ASTC_8X5_SRGB_BLOCK,
    VkFormat::ASTC_8X6_UNORM_BLOCK,
    VkFormat::ASTC_8X6_SRGB_BLOCK,
    VkFormat::ASTC_8X8_UNORM_BLOCK,
    VkFormat::ASTC_8X8_SRGB_BLOCK,
    VkFormat::ASTC_10X5_UNORM_BLOCK,
    VkFormat::ASTC_10X5_SRGB_BLOCK,
    VkFormat::ASTC_10X6_UNORM_BLOCK,
    VkFormat::ASTC_10X6_SRGB_BLOCK,
    VkFormat::ASTC_10X8_UNORM_BLOCK,
    VkFormat::ASTC_10X8_SRGB_BLOCK,
    VkFormat::ASTC_10X10_UNORM_BLOCK,
    VkFormat::ASTC_10X10_SRGB_BLOCK,
    VkFormat::ASTC_12X10_UNORM_BLOCK,
    VkFormat::ASTC_12X10_SRGB_BLOCK,
    VkFormat::ASTC_12X12_UNORM_BLOCK,
    VkFormat::ASTC_12X12_SRGB_BLOCK,
    VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT,
    VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT,
    VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG,
    VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG,
    VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG,
];
