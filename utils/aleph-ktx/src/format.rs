//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Definitions of all vulkan formats.
///
/// # Attribution
///
/// This is copy pasted directly from the `erupt` auto generated vulkan bindings with some quick
/// manual clean ups. Editing this manually can be dont, but dont. Just update it from the generated
/// bindings as needed. This way we don't have to have a cargo dependency on any particular bindings
/// and we can be agnostic of whether someone is using `ash` or `erupt` or their own bindings.
///
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VkFormat(pub u32);

impl VkFormat {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
}

impl VkFormat {
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
}

///
/// ASTC compressed formats (VK_EXT_texture_compression_astc_hdr)
///
impl VkFormat {
    pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self(1000066000);
    pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self(1000066001);
    pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self(1000066002);
    pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self(1000066003);
    pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self(1000066004);
    pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self(1000066005);
    pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self(1000066006);
    pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self(1000066007);
    pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self(1000066008);
    pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self(1000066009);
    pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self(1000066010);
    pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self(1000066011);
    pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self(1000066012);
    pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self(1000066013);
}

///
/// PVRTC compressed formats (VK_IMG_format_pvrtc)
///
impl VkFormat {
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
}

///
/// YcBcR formats (VK_KHR_sampler_ycbcr_conversion)
///
impl VkFormat {
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
        Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
        Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
        Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
        Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
        Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
        Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
}

impl VkFormat {
    ///
    /// If this is a block format
    ///
    pub fn is_block_format(self) -> bool {
        match self {
            VkFormat::BC1_RGB_UNORM_BLOCK
            | VkFormat::BC1_RGB_SRGB_BLOCK
            | VkFormat::BC1_RGBA_UNORM_BLOCK
            | VkFormat::BC1_RGBA_SRGB_BLOCK
            | VkFormat::BC2_UNORM_BLOCK
            | VkFormat::BC2_SRGB_BLOCK
            | VkFormat::BC3_UNORM_BLOCK
            | VkFormat::BC3_SRGB_BLOCK
            | VkFormat::BC4_UNORM_BLOCK
            | VkFormat::BC4_SNORM_BLOCK
            | VkFormat::BC5_UNORM_BLOCK
            | VkFormat::BC5_SNORM_BLOCK
            | VkFormat::BC6H_UFLOAT_BLOCK
            | VkFormat::BC6H_SFLOAT_BLOCK
            | VkFormat::BC7_UNORM_BLOCK
            | VkFormat::BC7_SRGB_BLOCK
            | VkFormat::ETC2_R8G8B8_UNORM_BLOCK
            | VkFormat::ETC2_R8G8B8_SRGB_BLOCK
            | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
            | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
            | VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK
            | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK
            | VkFormat::EAC_R11_UNORM_BLOCK
            | VkFormat::EAC_R11_SNORM_BLOCK
            | VkFormat::EAC_R11G11_UNORM_BLOCK
            | VkFormat::EAC_R11G11_SNORM_BLOCK
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
            | VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
            | VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
            | VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
            | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
            | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
            | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
            | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
            | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG => true,
            _ => false,
        }
    }

    ///
    /// If this format has been marked explicitly as unsupported by our implementation
    ///
    pub fn is_unsupported(self) -> bool {
        match self {
            VkFormat::UNDEFINED
            | VkFormat::D16_UNORM_S8_UINT
            | VkFormat::D24_UNORM_S8_UINT
            | VkFormat::X8_D24_UNORM_PACK32
            | VkFormat::D32_SFLOAT_S8_UINT => true,
            _ => false,
        }
    }

    ///
    /// If this format has been marked explicitly as unsupported by our implementation
    ///
    pub fn is_depth_format(self) -> bool {
        match self {
            VkFormat::D32_SFLOAT
            | VkFormat::D32_SFLOAT_S8_UINT
            | VkFormat::D24_UNORM_S8_UINT
            | VkFormat::D16_UNORM_S8_UINT
            | VkFormat::D16_UNORM
            | VkFormat::X8_D24_UNORM_PACK32 => true,
            _ => false,
        }
    }

    ///
    /// If this format has been marked as prohibited by the KTX 2.0 spec
    ///
    pub fn is_prohibited(self) -> bool {
        match self {
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
    /// Returns whether this format is known by the implementation.
    ///
    /// In the event that newer versions of the Vulkan spec introduce new formats that can be stored
    /// inside of a KTX file, it could lead to `aleph-ktx` being used on KTX files that hold formats
    /// that didn't exist when `aleph-ktx` was written (or when an older `aleph-ktx` version was
    /// compiled into an executable).
    ///
    /// To handle this possibility, this function will return if the format is known by the library.
    ///
    /// This shouldn't cause any errors when reading a KTX file but may be an issue when trying to
    /// output a KTX file.
    ///
    pub fn is_known(self) -> bool {
        match self {
            VkFormat::UNDEFINED => true,
            VkFormat::R4G4_UNORM_PACK8 => true,
            VkFormat::R4G4B4A4_UNORM_PACK16 => true,
            VkFormat::B4G4R4A4_UNORM_PACK16 => true,
            VkFormat::R5G6B5_UNORM_PACK16 => true,
            VkFormat::B5G6R5_UNORM_PACK16 => true,
            VkFormat::R5G5B5A1_UNORM_PACK16 => true,
            VkFormat::B5G5R5A1_UNORM_PACK16 => true,
            VkFormat::A1R5G5B5_UNORM_PACK16 => true,
            VkFormat::R8_UNORM => true,
            VkFormat::R8_SNORM => true,
            VkFormat::R8_USCALED => true,
            VkFormat::R8_SSCALED => true,
            VkFormat::R8_UINT => true,
            VkFormat::R8_SINT => true,
            VkFormat::R8_SRGB => true,
            VkFormat::R8G8_UNORM => true,
            VkFormat::R8G8_SNORM => true,
            VkFormat::R8G8_USCALED => true,
            VkFormat::R8G8_SSCALED => true,
            VkFormat::R8G8_UINT => true,
            VkFormat::R8G8_SINT => true,
            VkFormat::R8G8_SRGB => true,
            VkFormat::R8G8B8_UNORM => true,
            VkFormat::R8G8B8_SNORM => true,
            VkFormat::R8G8B8_USCALED => true,
            VkFormat::R8G8B8_SSCALED => true,
            VkFormat::R8G8B8_UINT => true,
            VkFormat::R8G8B8_SINT => true,
            VkFormat::R8G8B8_SRGB => true,
            VkFormat::B8G8R8_UNORM => true,
            VkFormat::B8G8R8_SNORM => true,
            VkFormat::B8G8R8_USCALED => true,
            VkFormat::B8G8R8_SSCALED => true,
            VkFormat::B8G8R8_UINT => true,
            VkFormat::B8G8R8_SINT => true,
            VkFormat::B8G8R8_SRGB => true,
            VkFormat::R8G8B8A8_UNORM => true,
            VkFormat::R8G8B8A8_SNORM => true,
            VkFormat::R8G8B8A8_USCALED => true,
            VkFormat::R8G8B8A8_SSCALED => true,
            VkFormat::R8G8B8A8_UINT => true,
            VkFormat::R8G8B8A8_SINT => true,
            VkFormat::R8G8B8A8_SRGB => true,
            VkFormat::B8G8R8A8_UNORM => true,
            VkFormat::B8G8R8A8_SNORM => true,
            VkFormat::B8G8R8A8_USCALED => true,
            VkFormat::B8G8R8A8_SSCALED => true,
            VkFormat::B8G8R8A8_UINT => true,
            VkFormat::B8G8R8A8_SINT => true,
            VkFormat::B8G8R8A8_SRGB => true,
            VkFormat::A8B8G8R8_UNORM_PACK32 => true,
            VkFormat::A8B8G8R8_SNORM_PACK32 => true,
            VkFormat::A8B8G8R8_USCALED_PACK32 => true,
            VkFormat::A8B8G8R8_SSCALED_PACK32 => true,
            VkFormat::A8B8G8R8_UINT_PACK32 => true,
            VkFormat::A8B8G8R8_SINT_PACK32 => true,
            VkFormat::A8B8G8R8_SRGB_PACK32 => true,
            VkFormat::A2R10G10B10_UNORM_PACK32 => true,
            VkFormat::A2R10G10B10_SNORM_PACK32 => true,
            VkFormat::A2R10G10B10_USCALED_PACK32 => true,
            VkFormat::A2R10G10B10_SSCALED_PACK32 => true,
            VkFormat::A2R10G10B10_UINT_PACK32 => true,
            VkFormat::A2R10G10B10_SINT_PACK32 => true,
            VkFormat::A2B10G10R10_UNORM_PACK32 => true,
            VkFormat::A2B10G10R10_SNORM_PACK32 => true,
            VkFormat::A2B10G10R10_USCALED_PACK32 => true,
            VkFormat::A2B10G10R10_SSCALED_PACK32 => true,
            VkFormat::A2B10G10R10_UINT_PACK32 => true,
            VkFormat::A2B10G10R10_SINT_PACK32 => true,
            VkFormat::R16_UNORM => true,
            VkFormat::R16_SNORM => true,
            VkFormat::R16_USCALED => true,
            VkFormat::R16_SSCALED => true,
            VkFormat::R16_UINT => true,
            VkFormat::R16_SINT => true,
            VkFormat::R16_SFLOAT => true,
            VkFormat::R16G16_UNORM => true,
            VkFormat::R16G16_SNORM => true,
            VkFormat::R16G16_USCALED => true,
            VkFormat::R16G16_SSCALED => true,
            VkFormat::R16G16_UINT => true,
            VkFormat::R16G16_SINT => true,
            VkFormat::R16G16_SFLOAT => true,
            VkFormat::R16G16B16_UNORM => true,
            VkFormat::R16G16B16_SNORM => true,
            VkFormat::R16G16B16_USCALED => true,
            VkFormat::R16G16B16_SSCALED => true,
            VkFormat::R16G16B16_UINT => true,
            VkFormat::R16G16B16_SINT => true,
            VkFormat::R16G16B16_SFLOAT => true,
            VkFormat::R16G16B16A16_UNORM => true,
            VkFormat::R16G16B16A16_SNORM => true,
            VkFormat::R16G16B16A16_USCALED => true,
            VkFormat::R16G16B16A16_SSCALED => true,
            VkFormat::R16G16B16A16_UINT => true,
            VkFormat::R16G16B16A16_SINT => true,
            VkFormat::R16G16B16A16_SFLOAT => true,
            VkFormat::R32_UINT => true,
            VkFormat::R32_SINT => true,
            VkFormat::R32_SFLOAT => true,
            VkFormat::R32G32_UINT => true,
            VkFormat::R32G32_SINT => true,
            VkFormat::R32G32_SFLOAT => true,
            VkFormat::R32G32B32_UINT => true,
            VkFormat::R32G32B32_SINT => true,
            VkFormat::R32G32B32_SFLOAT => true,
            VkFormat::R32G32B32A32_UINT => true,
            VkFormat::R32G32B32A32_SINT => true,
            VkFormat::R32G32B32A32_SFLOAT => true,
            VkFormat::R64_UINT => true,
            VkFormat::R64_SINT => true,
            VkFormat::R64_SFLOAT => true,
            VkFormat::R64G64_UINT => true,
            VkFormat::R64G64_SINT => true,
            VkFormat::R64G64_SFLOAT => true,
            VkFormat::R64G64B64_UINT => true,
            VkFormat::R64G64B64_SINT => true,
            VkFormat::R64G64B64_SFLOAT => true,
            VkFormat::R64G64B64A64_UINT => true,
            VkFormat::R64G64B64A64_SINT => true,
            VkFormat::R64G64B64A64_SFLOAT => true,
            VkFormat::B10G11R11_UFLOAT_PACK32 => true,
            VkFormat::E5B9G9R9_UFLOAT_PACK32 => true,
            VkFormat::D16_UNORM => true,
            VkFormat::X8_D24_UNORM_PACK32 => true,
            VkFormat::D32_SFLOAT => true,
            VkFormat::S8_UINT => true,
            VkFormat::D16_UNORM_S8_UINT => true,
            VkFormat::D24_UNORM_S8_UINT => true,
            VkFormat::D32_SFLOAT_S8_UINT => true,
            VkFormat::BC1_RGB_UNORM_BLOCK => true,
            VkFormat::BC1_RGB_SRGB_BLOCK => true,
            VkFormat::BC1_RGBA_UNORM_BLOCK => true,
            VkFormat::BC1_RGBA_SRGB_BLOCK => true,
            VkFormat::BC2_UNORM_BLOCK => true,
            VkFormat::BC2_SRGB_BLOCK => true,
            VkFormat::BC3_UNORM_BLOCK => true,
            VkFormat::BC3_SRGB_BLOCK => true,
            VkFormat::BC4_UNORM_BLOCK => true,
            VkFormat::BC4_SNORM_BLOCK => true,
            VkFormat::BC5_UNORM_BLOCK => true,
            VkFormat::BC5_SNORM_BLOCK => true,
            VkFormat::BC6H_UFLOAT_BLOCK => true,
            VkFormat::BC6H_SFLOAT_BLOCK => true,
            VkFormat::BC7_UNORM_BLOCK => true,
            VkFormat::BC7_SRGB_BLOCK => true,
            VkFormat::ETC2_R8G8B8_UNORM_BLOCK => true,
            VkFormat::ETC2_R8G8B8_SRGB_BLOCK => true,
            VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK => true,
            VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK => true,
            VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK => true,
            VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK => true,
            VkFormat::EAC_R11_UNORM_BLOCK => true,
            VkFormat::EAC_R11_SNORM_BLOCK => true,
            VkFormat::EAC_R11G11_UNORM_BLOCK => true,
            VkFormat::EAC_R11G11_SNORM_BLOCK => true,
            VkFormat::ASTC_4X4_UNORM_BLOCK => true,
            VkFormat::ASTC_4X4_SRGB_BLOCK => true,
            VkFormat::ASTC_5X4_UNORM_BLOCK => true,
            VkFormat::ASTC_5X4_SRGB_BLOCK => true,
            VkFormat::ASTC_5X5_UNORM_BLOCK => true,
            VkFormat::ASTC_5X5_SRGB_BLOCK => true,
            VkFormat::ASTC_6X5_UNORM_BLOCK => true,
            VkFormat::ASTC_6X5_SRGB_BLOCK => true,
            VkFormat::ASTC_6X6_UNORM_BLOCK => true,
            VkFormat::ASTC_6X6_SRGB_BLOCK => true,
            VkFormat::ASTC_8X5_UNORM_BLOCK => true,
            VkFormat::ASTC_8X5_SRGB_BLOCK => true,
            VkFormat::ASTC_8X6_UNORM_BLOCK => true,
            VkFormat::ASTC_8X6_SRGB_BLOCK => true,
            VkFormat::ASTC_8X8_UNORM_BLOCK => true,
            VkFormat::ASTC_8X8_SRGB_BLOCK => true,
            VkFormat::ASTC_10X5_UNORM_BLOCK => true,
            VkFormat::ASTC_10X5_SRGB_BLOCK => true,
            VkFormat::ASTC_10X6_UNORM_BLOCK => true,
            VkFormat::ASTC_10X6_SRGB_BLOCK => true,
            VkFormat::ASTC_10X8_UNORM_BLOCK => true,
            VkFormat::ASTC_10X8_SRGB_BLOCK => true,
            VkFormat::ASTC_10X10_UNORM_BLOCK => true,
            VkFormat::ASTC_10X10_SRGB_BLOCK => true,
            VkFormat::ASTC_12X10_UNORM_BLOCK => true,
            VkFormat::ASTC_12X10_SRGB_BLOCK => true,
            VkFormat::ASTC_12X12_UNORM_BLOCK => true,
            VkFormat::ASTC_12X12_SRGB_BLOCK => true,
            VkFormat::G8B8G8R8_422_UNORM => true,
            VkFormat::B8G8R8G8_422_UNORM => true,
            VkFormat::G8_B8_R8_3PLANE_420_UNORM => true,
            VkFormat::G8_B8R8_2PLANE_420_UNORM => true,
            VkFormat::G8_B8_R8_3PLANE_422_UNORM => true,
            VkFormat::G8_B8R8_2PLANE_422_UNORM => true,
            VkFormat::G8_B8_R8_3PLANE_444_UNORM => true,
            VkFormat::R10X6_UNORM_PACK16 => true,
            VkFormat::R10X6G10X6_UNORM_2PACK16 => true,
            VkFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => true,
            VkFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => true,
            VkFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => true,
            VkFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => true,
            VkFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => true,
            VkFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => true,
            VkFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => true,
            VkFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => true,
            VkFormat::R12X4_UNORM_PACK16 => true,
            VkFormat::R12X4G12X4_UNORM_2PACK16 => true,
            VkFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => true,
            VkFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => true,
            VkFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => true,
            VkFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => true,
            VkFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => true,
            VkFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => true,
            VkFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => true,
            VkFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => true,
            VkFormat::G16B16G16R16_422_UNORM => true,
            VkFormat::B16G16R16G16_422_UNORM => true,
            VkFormat::G16_B16_R16_3PLANE_420_UNORM => true,
            VkFormat::G16_B16R16_2PLANE_420_UNORM => true,
            VkFormat::G16_B16_R16_3PLANE_422_UNORM => true,
            VkFormat::G16_B16R16_2PLANE_422_UNORM => true,
            VkFormat::G16_B16_R16_3PLANE_444_UNORM => true,
            VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT => true,
            VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT => true,
            VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG => true,
            VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG => true,
            VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG => true,
            VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG => true,
            VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG => true,
            VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG => true,
            VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG => true,
            VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for VkFormat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::UNDEFINED => "UNDEFINED",
            &Self::R4G4_UNORM_PACK8 => "R4G4_UNORM_PACK8",
            &Self::R4G4B4A4_UNORM_PACK16 => "R4G4B4A4_UNORM_PACK16",
            &Self::B4G4R4A4_UNORM_PACK16 => "B4G4R4A4_UNORM_PACK16",
            &Self::R5G6B5_UNORM_PACK16 => "R5G6B5_UNORM_PACK16",
            &Self::B5G6R5_UNORM_PACK16 => "B5G6R5_UNORM_PACK16",
            &Self::R5G5B5A1_UNORM_PACK16 => "R5G5B5A1_UNORM_PACK16",
            &Self::B5G5R5A1_UNORM_PACK16 => "B5G5R5A1_UNORM_PACK16",
            &Self::A1R5G5B5_UNORM_PACK16 => "A1R5G5B5_UNORM_PACK16",
            &Self::R8_UNORM => "R8_UNORM",
            &Self::R8_SNORM => "R8_SNORM",
            &Self::R8_USCALED => "R8_USCALED",
            &Self::R8_SSCALED => "R8_SSCALED",
            &Self::R8_UINT => "R8_UINT",
            &Self::R8_SINT => "R8_SINT",
            &Self::R8_SRGB => "R8_SRGB",
            &Self::R8G8_UNORM => "R8G8_UNORM",
            &Self::R8G8_SNORM => "R8G8_SNORM",
            &Self::R8G8_USCALED => "R8G8_USCALED",
            &Self::R8G8_SSCALED => "R8G8_SSCALED",
            &Self::R8G8_UINT => "R8G8_UINT",
            &Self::R8G8_SINT => "R8G8_SINT",
            &Self::R8G8_SRGB => "R8G8_SRGB",
            &Self::R8G8B8_UNORM => "R8G8B8_UNORM",
            &Self::R8G8B8_SNORM => "R8G8B8_SNORM",
            &Self::R8G8B8_USCALED => "R8G8B8_USCALED",
            &Self::R8G8B8_SSCALED => "R8G8B8_SSCALED",
            &Self::R8G8B8_UINT => "R8G8B8_UINT",
            &Self::R8G8B8_SINT => "R8G8B8_SINT",
            &Self::R8G8B8_SRGB => "R8G8B8_SRGB",
            &Self::B8G8R8_UNORM => "B8G8R8_UNORM",
            &Self::B8G8R8_SNORM => "B8G8R8_SNORM",
            &Self::B8G8R8_USCALED => "B8G8R8_USCALED",
            &Self::B8G8R8_SSCALED => "B8G8R8_SSCALED",
            &Self::B8G8R8_UINT => "B8G8R8_UINT",
            &Self::B8G8R8_SINT => "B8G8R8_SINT",
            &Self::B8G8R8_SRGB => "B8G8R8_SRGB",
            &Self::R8G8B8A8_UNORM => "R8G8B8A8_UNORM",
            &Self::R8G8B8A8_SNORM => "R8G8B8A8_SNORM",
            &Self::R8G8B8A8_USCALED => "R8G8B8A8_USCALED",
            &Self::R8G8B8A8_SSCALED => "R8G8B8A8_SSCALED",
            &Self::R8G8B8A8_UINT => "R8G8B8A8_UINT",
            &Self::R8G8B8A8_SINT => "R8G8B8A8_SINT",
            &Self::R8G8B8A8_SRGB => "R8G8B8A8_SRGB",
            &Self::B8G8R8A8_UNORM => "B8G8R8A8_UNORM",
            &Self::B8G8R8A8_SNORM => "B8G8R8A8_SNORM",
            &Self::B8G8R8A8_USCALED => "B8G8R8A8_USCALED",
            &Self::B8G8R8A8_SSCALED => "B8G8R8A8_SSCALED",
            &Self::B8G8R8A8_UINT => "B8G8R8A8_UINT",
            &Self::B8G8R8A8_SINT => "B8G8R8A8_SINT",
            &Self::B8G8R8A8_SRGB => "B8G8R8A8_SRGB",
            &Self::A8B8G8R8_UNORM_PACK32 => "A8B8G8R8_UNORM_PACK32",
            &Self::A8B8G8R8_SNORM_PACK32 => "A8B8G8R8_SNORM_PACK32",
            &Self::A8B8G8R8_USCALED_PACK32 => "A8B8G8R8_USCALED_PACK32",
            &Self::A8B8G8R8_SSCALED_PACK32 => "A8B8G8R8_SSCALED_PACK32",
            &Self::A8B8G8R8_UINT_PACK32 => "A8B8G8R8_UINT_PACK32",
            &Self::A8B8G8R8_SINT_PACK32 => "A8B8G8R8_SINT_PACK32",
            &Self::A8B8G8R8_SRGB_PACK32 => "A8B8G8R8_SRGB_PACK32",
            &Self::A2R10G10B10_UNORM_PACK32 => "A2R10G10B10_UNORM_PACK32",
            &Self::A2R10G10B10_SNORM_PACK32 => "A2R10G10B10_SNORM_PACK32",
            &Self::A2R10G10B10_USCALED_PACK32 => "A2R10G10B10_USCALED_PACK32",
            &Self::A2R10G10B10_SSCALED_PACK32 => "A2R10G10B10_SSCALED_PACK32",
            &Self::A2R10G10B10_UINT_PACK32 => "A2R10G10B10_UINT_PACK32",
            &Self::A2R10G10B10_SINT_PACK32 => "A2R10G10B10_SINT_PACK32",
            &Self::A2B10G10R10_UNORM_PACK32 => "A2B10G10R10_UNORM_PACK32",
            &Self::A2B10G10R10_SNORM_PACK32 => "A2B10G10R10_SNORM_PACK32",
            &Self::A2B10G10R10_USCALED_PACK32 => "A2B10G10R10_USCALED_PACK32",
            &Self::A2B10G10R10_SSCALED_PACK32 => "A2B10G10R10_SSCALED_PACK32",
            &Self::A2B10G10R10_UINT_PACK32 => "A2B10G10R10_UINT_PACK32",
            &Self::A2B10G10R10_SINT_PACK32 => "A2B10G10R10_SINT_PACK32",
            &Self::R16_UNORM => "R16_UNORM",
            &Self::R16_SNORM => "R16_SNORM",
            &Self::R16_USCALED => "R16_USCALED",
            &Self::R16_SSCALED => "R16_SSCALED",
            &Self::R16_UINT => "R16_UINT",
            &Self::R16_SINT => "R16_SINT",
            &Self::R16_SFLOAT => "R16_SFLOAT",
            &Self::R16G16_UNORM => "R16G16_UNORM",
            &Self::R16G16_SNORM => "R16G16_SNORM",
            &Self::R16G16_USCALED => "R16G16_USCALED",
            &Self::R16G16_SSCALED => "R16G16_SSCALED",
            &Self::R16G16_UINT => "R16G16_UINT",
            &Self::R16G16_SINT => "R16G16_SINT",
            &Self::R16G16_SFLOAT => "R16G16_SFLOAT",
            &Self::R16G16B16_UNORM => "R16G16B16_UNORM",
            &Self::R16G16B16_SNORM => "R16G16B16_SNORM",
            &Self::R16G16B16_USCALED => "R16G16B16_USCALED",
            &Self::R16G16B16_SSCALED => "R16G16B16_SSCALED",
            &Self::R16G16B16_UINT => "R16G16B16_UINT",
            &Self::R16G16B16_SINT => "R16G16B16_SINT",
            &Self::R16G16B16_SFLOAT => "R16G16B16_SFLOAT",
            &Self::R16G16B16A16_UNORM => "R16G16B16A16_UNORM",
            &Self::R16G16B16A16_SNORM => "R16G16B16A16_SNORM",
            &Self::R16G16B16A16_USCALED => "R16G16B16A16_USCALED",
            &Self::R16G16B16A16_SSCALED => "R16G16B16A16_SSCALED",
            &Self::R16G16B16A16_UINT => "R16G16B16A16_UINT",
            &Self::R16G16B16A16_SINT => "R16G16B16A16_SINT",
            &Self::R16G16B16A16_SFLOAT => "R16G16B16A16_SFLOAT",
            &Self::R32_UINT => "R32_UINT",
            &Self::R32_SINT => "R32_SINT",
            &Self::R32_SFLOAT => "R32_SFLOAT",
            &Self::R32G32_UINT => "R32G32_UINT",
            &Self::R32G32_SINT => "R32G32_SINT",
            &Self::R32G32_SFLOAT => "R32G32_SFLOAT",
            &Self::R32G32B32_UINT => "R32G32B32_UINT",
            &Self::R32G32B32_SINT => "R32G32B32_SINT",
            &Self::R32G32B32_SFLOAT => "R32G32B32_SFLOAT",
            &Self::R32G32B32A32_UINT => "R32G32B32A32_UINT",
            &Self::R32G32B32A32_SINT => "R32G32B32A32_SINT",
            &Self::R32G32B32A32_SFLOAT => "R32G32B32A32_SFLOAT",
            &Self::R64_UINT => "R64_UINT",
            &Self::R64_SINT => "R64_SINT",
            &Self::R64_SFLOAT => "R64_SFLOAT",
            &Self::R64G64_UINT => "R64G64_UINT",
            &Self::R64G64_SINT => "R64G64_SINT",
            &Self::R64G64_SFLOAT => "R64G64_SFLOAT",
            &Self::R64G64B64_UINT => "R64G64B64_UINT",
            &Self::R64G64B64_SINT => "R64G64B64_SINT",
            &Self::R64G64B64_SFLOAT => "R64G64B64_SFLOAT",
            &Self::R64G64B64A64_UINT => "R64G64B64A64_UINT",
            &Self::R64G64B64A64_SINT => "R64G64B64A64_SINT",
            &Self::R64G64B64A64_SFLOAT => "R64G64B64A64_SFLOAT",
            &Self::B10G11R11_UFLOAT_PACK32 => "B10G11R11_UFLOAT_PACK32",
            &Self::E5B9G9R9_UFLOAT_PACK32 => "E5B9G9R9_UFLOAT_PACK32",
            &Self::D16_UNORM => "D16_UNORM",
            &Self::X8_D24_UNORM_PACK32 => "X8_D24_UNORM_PACK32",
            &Self::D32_SFLOAT => "D32_SFLOAT",
            &Self::S8_UINT => "S8_UINT",
            &Self::D16_UNORM_S8_UINT => "D16_UNORM_S8_UINT",
            &Self::D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT",
            &Self::D32_SFLOAT_S8_UINT => "D32_SFLOAT_S8_UINT",
            &Self::BC1_RGB_UNORM_BLOCK => "BC1_RGB_UNORM_BLOCK",
            &Self::BC1_RGB_SRGB_BLOCK => "BC1_RGB_SRGB_BLOCK",
            &Self::BC1_RGBA_UNORM_BLOCK => "BC1_RGBA_UNORM_BLOCK",
            &Self::BC1_RGBA_SRGB_BLOCK => "BC1_RGBA_SRGB_BLOCK",
            &Self::BC2_UNORM_BLOCK => "BC2_UNORM_BLOCK",
            &Self::BC2_SRGB_BLOCK => "BC2_SRGB_BLOCK",
            &Self::BC3_UNORM_BLOCK => "BC3_UNORM_BLOCK",
            &Self::BC3_SRGB_BLOCK => "BC3_SRGB_BLOCK",
            &Self::BC4_UNORM_BLOCK => "BC4_UNORM_BLOCK",
            &Self::BC4_SNORM_BLOCK => "BC4_SNORM_BLOCK",
            &Self::BC5_UNORM_BLOCK => "BC5_UNORM_BLOCK",
            &Self::BC5_SNORM_BLOCK => "BC5_SNORM_BLOCK",
            &Self::BC6H_UFLOAT_BLOCK => "BC6H_UFLOAT_BLOCK",
            &Self::BC6H_SFLOAT_BLOCK => "BC6H_SFLOAT_BLOCK",
            &Self::BC7_UNORM_BLOCK => "BC7_UNORM_BLOCK",
            &Self::BC7_SRGB_BLOCK => "BC7_SRGB_BLOCK",
            &Self::ETC2_R8G8B8_UNORM_BLOCK => "ETC2_R8G8B8_UNORM_BLOCK",
            &Self::ETC2_R8G8B8_SRGB_BLOCK => "ETC2_R8G8B8_SRGB_BLOCK",
            &Self::ETC2_R8G8B8A1_UNORM_BLOCK => "ETC2_R8G8B8A1_UNORM_BLOCK",
            &Self::ETC2_R8G8B8A1_SRGB_BLOCK => "ETC2_R8G8B8A1_SRGB_BLOCK",
            &Self::ETC2_R8G8B8A8_UNORM_BLOCK => "ETC2_R8G8B8A8_UNORM_BLOCK",
            &Self::ETC2_R8G8B8A8_SRGB_BLOCK => "ETC2_R8G8B8A8_SRGB_BLOCK",
            &Self::EAC_R11_UNORM_BLOCK => "EAC_R11_UNORM_BLOCK",
            &Self::EAC_R11_SNORM_BLOCK => "EAC_R11_SNORM_BLOCK",
            &Self::EAC_R11G11_UNORM_BLOCK => "EAC_R11G11_UNORM_BLOCK",
            &Self::EAC_R11G11_SNORM_BLOCK => "EAC_R11G11_SNORM_BLOCK",
            &Self::ASTC_4X4_UNORM_BLOCK => "ASTC_4X4_UNORM_BLOCK",
            &Self::ASTC_4X4_SRGB_BLOCK => "ASTC_4X4_SRGB_BLOCK",
            &Self::ASTC_5X4_UNORM_BLOCK => "ASTC_5X4_UNORM_BLOCK",
            &Self::ASTC_5X4_SRGB_BLOCK => "ASTC_5X4_SRGB_BLOCK",
            &Self::ASTC_5X5_UNORM_BLOCK => "ASTC_5X5_UNORM_BLOCK",
            &Self::ASTC_5X5_SRGB_BLOCK => "ASTC_5X5_SRGB_BLOCK",
            &Self::ASTC_6X5_UNORM_BLOCK => "ASTC_6X5_UNORM_BLOCK",
            &Self::ASTC_6X5_SRGB_BLOCK => "ASTC_6X5_SRGB_BLOCK",
            &Self::ASTC_6X6_UNORM_BLOCK => "ASTC_6X6_UNORM_BLOCK",
            &Self::ASTC_6X6_SRGB_BLOCK => "ASTC_6X6_SRGB_BLOCK",
            &Self::ASTC_8X5_UNORM_BLOCK => "ASTC_8X5_UNORM_BLOCK",
            &Self::ASTC_8X5_SRGB_BLOCK => "ASTC_8X5_SRGB_BLOCK",
            &Self::ASTC_8X6_UNORM_BLOCK => "ASTC_8X6_UNORM_BLOCK",
            &Self::ASTC_8X6_SRGB_BLOCK => "ASTC_8X6_SRGB_BLOCK",
            &Self::ASTC_8X8_UNORM_BLOCK => "ASTC_8X8_UNORM_BLOCK",
            &Self::ASTC_8X8_SRGB_BLOCK => "ASTC_8X8_SRGB_BLOCK",
            &Self::ASTC_10X5_UNORM_BLOCK => "ASTC_10X5_UNORM_BLOCK",
            &Self::ASTC_10X5_SRGB_BLOCK => "ASTC_10X5_SRGB_BLOCK",
            &Self::ASTC_10X6_UNORM_BLOCK => "ASTC_10X6_UNORM_BLOCK",
            &Self::ASTC_10X6_SRGB_BLOCK => "ASTC_10X6_SRGB_BLOCK",
            &Self::ASTC_10X8_UNORM_BLOCK => "ASTC_10X8_UNORM_BLOCK",
            &Self::ASTC_10X8_SRGB_BLOCK => "ASTC_10X8_SRGB_BLOCK",
            &Self::ASTC_10X10_UNORM_BLOCK => "ASTC_10X10_UNORM_BLOCK",
            &Self::ASTC_10X10_SRGB_BLOCK => "ASTC_10X10_SRGB_BLOCK",
            &Self::ASTC_12X10_UNORM_BLOCK => "ASTC_12X10_UNORM_BLOCK",
            &Self::ASTC_12X10_SRGB_BLOCK => "ASTC_12X10_SRGB_BLOCK",
            &Self::ASTC_12X12_UNORM_BLOCK => "ASTC_12X12_UNORM_BLOCK",
            &Self::ASTC_12X12_SRGB_BLOCK => "ASTC_12X12_SRGB_BLOCK",
            &Self::G8B8G8R8_422_UNORM => "G8B8G8R8_422_UNORM",
            &Self::B8G8R8G8_422_UNORM => "B8G8R8G8_422_UNORM",
            &Self::G8_B8_R8_3PLANE_420_UNORM => "G8_B8_R8_3PLANE_420_UNORM",
            &Self::G8_B8R8_2PLANE_420_UNORM => "G8_B8R8_2PLANE_420_UNORM",
            &Self::G8_B8_R8_3PLANE_422_UNORM => "G8_B8_R8_3PLANE_422_UNORM",
            &Self::G8_B8R8_2PLANE_422_UNORM => "G8_B8R8_2PLANE_422_UNORM",
            &Self::G8_B8_R8_3PLANE_444_UNORM => "G8_B8_R8_3PLANE_444_UNORM",
            &Self::R10X6_UNORM_PACK16 => "R10X6_UNORM_PACK16",
            &Self::R10X6G10X6_UNORM_2PACK16 => "R10X6G10X6_UNORM_2PACK16",
            &Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "R10X6G10X6B10X6A10X6_UNORM_4PACK16",
            &Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16"
            }
            &Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16"
            }
            &Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16"
            }
            &Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16"
            }
            &Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16"
            }
            &Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16"
            }
            &Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16"
            }
            &Self::R12X4_UNORM_PACK16 => "R12X4_UNORM_PACK16",
            &Self::R12X4G12X4_UNORM_2PACK16 => "R12X4G12X4_UNORM_2PACK16",
            &Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "R12X4G12X4B12X4A12X4_UNORM_4PACK16",
            &Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16"
            }
            &Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16"
            }
            &Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16"
            }
            &Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16"
            }
            &Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16"
            }
            &Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16"
            }
            &Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16"
            }
            &Self::G16B16G16R16_422_UNORM => "G16B16G16R16_422_UNORM",
            &Self::B16G16R16G16_422_UNORM => "B16G16R16G16_422_UNORM",
            &Self::G16_B16_R16_3PLANE_420_UNORM => "G16_B16_R16_3PLANE_420_UNORM",
            &Self::G16_B16R16_2PLANE_420_UNORM => "G16_B16R16_2PLANE_420_UNORM",
            &Self::G16_B16_R16_3PLANE_422_UNORM => "G16_B16_R16_3PLANE_422_UNORM",
            &Self::G16_B16R16_2PLANE_422_UNORM => "G16_B16R16_2PLANE_422_UNORM",
            &Self::G16_B16_R16_3PLANE_444_UNORM => "G16_B16_R16_3PLANE_444_UNORM",
            &Self::ASTC_4X4_SFLOAT_BLOCK_EXT => "ASTC_4X4_SFLOAT_BLOCK_EXT",
            &Self::ASTC_5X4_SFLOAT_BLOCK_EXT => "ASTC_5X4_SFLOAT_BLOCK_EXT",
            &Self::ASTC_5X5_SFLOAT_BLOCK_EXT => "ASTC_5X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_6X5_SFLOAT_BLOCK_EXT => "ASTC_6X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_6X6_SFLOAT_BLOCK_EXT => "ASTC_6X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X5_SFLOAT_BLOCK_EXT => "ASTC_8X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X6_SFLOAT_BLOCK_EXT => "ASTC_8X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X8_SFLOAT_BLOCK_EXT => "ASTC_8X8_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X5_SFLOAT_BLOCK_EXT => "ASTC_10X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X6_SFLOAT_BLOCK_EXT => "ASTC_10X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X8_SFLOAT_BLOCK_EXT => "ASTC_10X8_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X10_SFLOAT_BLOCK_EXT => "ASTC_10X10_SFLOAT_BLOCK_EXT",
            &Self::ASTC_12X10_SFLOAT_BLOCK_EXT => "ASTC_12X10_SFLOAT_BLOCK_EXT",
            &Self::ASTC_12X12_SFLOAT_BLOCK_EXT => "ASTC_12X12_SFLOAT_BLOCK_EXT",
            &Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => "PVRTC1_2BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => "PVRTC1_4BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => "PVRTC2_2BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => "PVRTC2_4BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => "PVRTC1_2BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => "PVRTC1_4BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => "PVRTC2_2BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => "PVRTC2_4BPP_SRGB_BLOCK_IMG",
            _ => "(unknown)",
        })
    }
}
