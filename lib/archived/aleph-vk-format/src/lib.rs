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

//!
//! This crate provides a simple, universal way to deal with the `VkFormat` enum defined by the
//! Vulkan spec. Prior to this crate, if someone wanted to handle the `VkFormat` enum and ONLY the
//! `VkFormat` enum it was still required to pull in a dependency on one of the available Vulkan
//! binding crates. Depending on one would make using another more inconvenient and would require
//! building both bindings crates.
//!
//! This crate was made to solve this problem by quite literally copy pasting the `VkFormat`
//! definition from the `erupt` into this crate and manually tweaking the definition.
//!
//! This crate also provides a set of utilities to aid pattern matching against different types of
//! formats. This includes checks for if a format is SNORM, SFLOAT, is signed, etc as well as block
//! size for compressed formats.
//!

#[cfg(test)]
mod tests;

///
/// Definitions of all vulkan formats.
///
/// # Attribution
///
/// This is copy pasted directly from the `erupt` auto generated vulkan bindings with some quick
/// manual clean ups. Editing this manually can be done, but dont. Just update it from the generated
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

    // ASTC compressed formats (VK_EXT_texture_compression_astc_hdr)
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

    // PVRTC compressed formats (VK_IMG_format_pvrtc)
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
}

impl VkFormat {
    ///
    /// If this is a block format
    ///
    #[inline]
    pub const fn is_block_format(self) -> bool {
        self.is_bcn() || self.is_etc2() || self.is_eac() || self.is_astc() || self.is_pvrtc()
    }

    ///
    /// Some formats have 1 bit (punch-through alpha) so it may be useful to be able to check for
    /// this.
    ///
    /// This returns if the format can at most represent 1 bit of alpha
    ///
    pub const fn is_1bit_alpha(self) -> bool {
        matches!(
            self,
            VkFormat::R5G5B5A1_UNORM_PACK16
                | VkFormat::B5G5R5A1_UNORM_PACK16
                | VkFormat::A1R5G5B5_UNORM_PACK16
                | VkFormat::BC1_RGBA_UNORM_BLOCK
                | VkFormat::BC1_RGBA_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
                | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
        )
    }

    ///
    /// Returns whether this format has an alpha channel
    ///
    pub const fn has_alpha(self) -> bool {
        matches!(
            self,
            VkFormat::R4G4B4A4_UNORM_PACK16
                | VkFormat::B4G4R4A4_UNORM_PACK16
                | VkFormat::R5G5B5A1_UNORM_PACK16
                | VkFormat::B5G5R5A1_UNORM_PACK16
                | VkFormat::A1R5G5B5_UNORM_PACK16
                | VkFormat::R8G8B8A8_UNORM
                | VkFormat::R8G8B8A8_SNORM
                | VkFormat::R8G8B8A8_USCALED
                | VkFormat::R8G8B8A8_SSCALED
                | VkFormat::R8G8B8A8_UINT
                | VkFormat::R8G8B8A8_SINT
                | VkFormat::R8G8B8A8_SRGB
                | VkFormat::B8G8R8A8_UNORM
                | VkFormat::B8G8R8A8_SNORM
                | VkFormat::B8G8R8A8_USCALED
                | VkFormat::B8G8R8A8_SSCALED
                | VkFormat::B8G8R8A8_UINT
                | VkFormat::B8G8R8A8_SINT
                | VkFormat::B8G8R8A8_SRGB
                | VkFormat::A8B8G8R8_UNORM_PACK32
                | VkFormat::A8B8G8R8_SNORM_PACK32
                | VkFormat::A8B8G8R8_USCALED_PACK32
                | VkFormat::A8B8G8R8_SSCALED_PACK32
                | VkFormat::A8B8G8R8_UINT_PACK32
                | VkFormat::A8B8G8R8_SINT_PACK32
                | VkFormat::A8B8G8R8_SRGB_PACK32
                | VkFormat::A2R10G10B10_UNORM_PACK32
                | VkFormat::A2R10G10B10_SNORM_PACK32
                | VkFormat::A2R10G10B10_USCALED_PACK32
                | VkFormat::A2R10G10B10_SSCALED_PACK32
                | VkFormat::A2R10G10B10_UINT_PACK32
                | VkFormat::A2R10G10B10_SINT_PACK32
                | VkFormat::A2B10G10R10_UNORM_PACK32
                | VkFormat::A2B10G10R10_SNORM_PACK32
                | VkFormat::A2B10G10R10_USCALED_PACK32
                | VkFormat::A2B10G10R10_SSCALED_PACK32
                | VkFormat::A2B10G10R10_UINT_PACK32
                | VkFormat::A2B10G10R10_SINT_PACK32
                | VkFormat::R16G16B16A16_UNORM
                | VkFormat::R16G16B16A16_SNORM
                | VkFormat::R16G16B16A16_USCALED
                | VkFormat::R16G16B16A16_SSCALED
                | VkFormat::R16G16B16A16_UINT
                | VkFormat::R16G16B16A16_SINT
                | VkFormat::R16G16B16A16_SFLOAT
                | VkFormat::R32G32B32A32_UINT
                | VkFormat::R32G32B32A32_SINT
                | VkFormat::R32G32B32A32_SFLOAT
                | VkFormat::R64G64B64A64_UINT
                | VkFormat::R64G64B64A64_SINT
                | VkFormat::R64G64B64A64_SFLOAT
                | VkFormat::BC1_RGBA_UNORM_BLOCK
                | VkFormat::BC1_RGBA_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
                | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK
                | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK
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
                | VkFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16
                | VkFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16
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
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// Returns whether this format is any one of the BCn compressed formats.
    ///
    /// # Info
    ///
    /// This function is equivalent to the logical OR of all of the following functions
    ///
    /// - Self::is_bc1
    /// - Self::is_bc2
    /// - Self::is_bc3
    /// - Self::is_bc4
    /// - Self::is_bc5
    /// - Self::is_bc6h
    /// - Self::is_bc7
    ///
    #[inline]
    pub const fn is_bcn(self) -> bool {
        self.is_bc1()
            || self.is_bc2()
            || self.is_bc3()
            || self.is_bc4()
            || self.is_bc5()
            || self.is_bc6h()
            || self.is_bc7()
    }

    ///
    /// If this format is any one of the possible BC1 compressed image formats
    ///
    #[inline]
    pub const fn is_bc1(self) -> bool {
        match self {
            VkFormat::BC1_RGB_SRGB_BLOCK | VkFormat::BC1_RGB_UNORM_BLOCK => true,
            _ => self.is_bc1_alpha(),
        }
    }

    ///
    /// If this format is any one of the BC1 compressed image formats with cutout alpha
    ///
    #[inline]
    pub const fn is_bc1_alpha(self) -> bool {
        matches!(
            self,
            VkFormat::BC1_RGBA_SRGB_BLOCK | VkFormat::BC1_RGBA_UNORM_BLOCK
        )
    }

    ///
    /// If this format is any one of the possible BC2 compressed image formats
    ///
    #[inline]
    pub const fn is_bc2(self) -> bool {
        matches!(self, VkFormat::BC2_SRGB_BLOCK | VkFormat::BC2_UNORM_BLOCK)
    }

    ///
    /// If this format is any one of the possible BC3 compressed image formats
    ///
    #[inline]
    pub const fn is_bc3(self) -> bool {
        matches!(self, VkFormat::BC3_SRGB_BLOCK | VkFormat::BC3_UNORM_BLOCK)
    }

    ///
    /// If this format is any one of the possible BC4 compressed image formats
    ///
    #[inline]
    pub const fn is_bc4(self) -> bool {
        matches!(self, VkFormat::BC4_SNORM_BLOCK | VkFormat::BC4_UNORM_BLOCK)
    }

    ///
    /// If this format is any one of the possible BC5 compressed image formats
    ///
    #[inline]
    pub const fn is_bc5(self) -> bool {
        matches!(self, VkFormat::BC5_SNORM_BLOCK | VkFormat::BC5_UNORM_BLOCK)
    }

    ///
    /// If this format is any one of the possible BC6H compressed image formats
    ///
    #[inline]
    pub const fn is_bc6h(self) -> bool {
        matches!(
            self,
            VkFormat::BC6H_SFLOAT_BLOCK | VkFormat::BC6H_UFLOAT_BLOCK
        )
    }

    ///
    /// If this format is any one of the possible BC7 compressed image formats
    ///
    #[inline]
    pub const fn is_bc7(self) -> bool {
        matches!(self, VkFormat::BC7_SRGB_BLOCK | VkFormat::BC7_UNORM_BLOCK)
    }

    ///
    /// If this format is any one of the possible ETC2 compressed image formats
    ///
    #[inline]
    pub const fn is_etc2(self) -> bool {
        matches!(
            self,
            VkFormat::ETC2_R8G8B8_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8_UNORM_BLOCK
                | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A1_UNORM_BLOCK
                | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A8_UNORM_BLOCK
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 4
    ///
    #[inline]
    pub const fn is_astc_block_width_4(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_4X4_UNORM_BLOCK
                | VkFormat::ASTC_4X4_SRGB_BLOCK
                | VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 5
    ///
    #[inline]
    pub const fn is_astc_block_width_5(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_5X4_UNORM_BLOCK
                | VkFormat::ASTC_5X4_SRGB_BLOCK
                | VkFormat::ASTC_5X5_UNORM_BLOCK
                | VkFormat::ASTC_5X5_SRGB_BLOCK
                | VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 6
    ///
    #[inline]
    pub const fn is_astc_block_width_6(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_6X5_UNORM_BLOCK
                | VkFormat::ASTC_6X5_SRGB_BLOCK
                | VkFormat::ASTC_6X6_UNORM_BLOCK
                | VkFormat::ASTC_6X6_SRGB_BLOCK
                | VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 8
    ///
    #[inline]
    pub const fn is_astc_block_width_8(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_8X5_UNORM_BLOCK
                | VkFormat::ASTC_8X5_SRGB_BLOCK
                | VkFormat::ASTC_8X6_UNORM_BLOCK
                | VkFormat::ASTC_8X6_SRGB_BLOCK
                | VkFormat::ASTC_8X8_UNORM_BLOCK
                | VkFormat::ASTC_8X8_SRGB_BLOCK
                | VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 10
    ///
    #[inline]
    pub const fn is_astc_block_width_10(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_10X5_UNORM_BLOCK
                | VkFormat::ASTC_10X5_SRGB_BLOCK
                | VkFormat::ASTC_10X6_UNORM_BLOCK
                | VkFormat::ASTC_10X6_SRGB_BLOCK
                | VkFormat::ASTC_10X8_UNORM_BLOCK
                | VkFormat::ASTC_10X8_SRGB_BLOCK
                | VkFormat::ASTC_10X10_UNORM_BLOCK
                | VkFormat::ASTC_10X10_SRGB_BLOCK
                | VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block width
    /// of 12
    ///
    #[inline]
    pub const fn is_astc_block_width_12(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_12X10_UNORM_BLOCK
                | VkFormat::ASTC_12X10_SRGB_BLOCK
                | VkFormat::ASTC_12X12_UNORM_BLOCK
                | VkFormat::ASTC_12X12_SRGB_BLOCK
                | VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 4
    ///
    #[inline]
    pub const fn is_astc_block_height_4(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_4X4_UNORM_BLOCK
                | VkFormat::ASTC_4X4_SRGB_BLOCK
                | VkFormat::ASTC_5X4_UNORM_BLOCK
                | VkFormat::ASTC_5X4_SRGB_BLOCK
                | VkFormat::ASTC_4X4_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_5X4_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 5
    ///
    #[inline]
    pub const fn is_astc_block_height_5(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_5X5_UNORM_BLOCK
                | VkFormat::ASTC_5X5_SRGB_BLOCK
                | VkFormat::ASTC_6X5_UNORM_BLOCK
                | VkFormat::ASTC_6X5_SRGB_BLOCK
                | VkFormat::ASTC_8X5_UNORM_BLOCK
                | VkFormat::ASTC_8X5_SRGB_BLOCK
                | VkFormat::ASTC_10X5_UNORM_BLOCK
                | VkFormat::ASTC_10X5_SRGB_BLOCK
                | VkFormat::ASTC_5X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_6X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_8X5_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X5_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 6
    ///
    #[inline]
    pub const fn is_astc_block_height_6(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_6X6_UNORM_BLOCK
                | VkFormat::ASTC_6X6_SRGB_BLOCK
                | VkFormat::ASTC_8X6_UNORM_BLOCK
                | VkFormat::ASTC_8X6_SRGB_BLOCK
                | VkFormat::ASTC_10X6_UNORM_BLOCK
                | VkFormat::ASTC_10X6_SRGB_BLOCK
                | VkFormat::ASTC_6X6_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_8X6_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X6_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 8
    ///
    #[inline]
    pub const fn is_astc_block_height_8(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_8X8_UNORM_BLOCK
                | VkFormat::ASTC_8X8_SRGB_BLOCK
                | VkFormat::ASTC_10X8_UNORM_BLOCK
                | VkFormat::ASTC_10X8_SRGB_BLOCK
                | VkFormat::ASTC_8X8_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_10X8_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 10
    ///
    #[inline]
    pub const fn is_astc_block_height_10(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_10X10_UNORM_BLOCK
                | VkFormat::ASTC_10X10_SRGB_BLOCK
                | VkFormat::ASTC_12X10_UNORM_BLOCK
                | VkFormat::ASTC_12X10_SRGB_BLOCK
                | VkFormat::ASTC_10X10_SFLOAT_BLOCK_EXT
                | VkFormat::ASTC_12X10_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats with a block height
    /// of 12
    ///
    #[inline]
    pub const fn is_astc_block_height_12(self) -> bool {
        matches!(
            self,
            VkFormat::ASTC_12X12_UNORM_BLOCK
                | VkFormat::ASTC_12X12_SRGB_BLOCK
                | VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible ASTC compressed image formats
    ///
    #[inline]
    pub const fn is_astc(self) -> bool {
        matches!(
            self,
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
                | VkFormat::ASTC_12X12_SFLOAT_BLOCK_EXT
        )
    }

    ///
    /// If this format is any one of the possible PVRTC compressed image formats
    ///
    #[inline]
    pub const fn is_pvrtc(self) -> bool {
        self.is_pvrtc1() || self.is_pvrtc2()
    }

    ///
    /// If this format is any one of the possible PVRTC1 compressed image formats
    ///
    #[inline]
    pub const fn is_pvrtc1(self) -> bool {
        matches!(
            self,
            VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// If this format is any one of the possible PVRTC2 compressed image formats
    ///
    #[inline]
    pub const fn is_pvrtc2(self) -> bool {
        matches!(
            self,
            VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// If this format is any one of the possible PVRTC 2bpp formats (2 bits per pixel)
    ///
    #[inline]
    pub const fn is_pvrtc_2bpp(self) -> bool {
        matches!(
            self,
            VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// If this format is any one of the possible PVRTC 4bpp formats (4 bits per pixel)
    ///
    #[inline]
    pub const fn is_pvrtc_4bpp(self) -> bool {
        matches!(
            self,
            VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// If this format is any one of the possible EAC compressed image formats
    ///
    #[inline]
    pub const fn is_eac(self) -> bool {
        matches!(
            self,
            VkFormat::EAC_R11_SNORM_BLOCK
                | VkFormat::EAC_R11_UNORM_BLOCK
                | VkFormat::EAC_R11G11_SNORM_BLOCK
                | VkFormat::EAC_R11G11_UNORM_BLOCK
        )
    }

    ///
    /// Returns the pixel width of a "block" for this image format
    ///
    /// # Info
    ///
    /// This will be one for non block compressed formats as a "non blocked" can be described as
    /// a blocked format with a 1x1 block size.
    ///
    /// This will be >1 for any compressed formats.
    ///
    #[inline]
    pub const fn block_width(self) -> u32 {
        if self.is_bcn() || self.is_etc2() || self.is_eac() {
            4
        } else if self.is_pvrtc_2bpp() {
            8
        } else if self.is_pvrtc_4bpp() || self.is_astc_block_width_4() {
            4
        } else if self.is_astc_block_width_5() {
            5
        } else if self.is_astc_block_width_6() {
            6
        } else if self.is_astc_block_width_8() {
            8
        } else if self.is_astc_block_width_10() {
            10
        } else if self.is_astc_block_width_12() {
            12
        } else {
            1
        }
    }

    ///
    /// Returns the pixel height of a "block" for this image format
    ///
    /// # Info
    ///
    /// This will be one for non block compressed formats as a "non blocked" can be described as
    /// a blocked format with a 1x1 block size.
    ///
    /// This will be >1 for any compressed formats.
    ///
    #[inline]
    pub const fn block_height(self) -> u32 {
        if self.is_bcn()
            || self.is_etc2()
            || self.is_eac()
            || self.is_pvrtc()
            || self.is_astc_block_height_4()
        {
            4
        } else if self.is_astc_block_height_5() {
            5
        } else if self.is_astc_block_height_6() {
            6
        } else if self.is_astc_block_height_8() {
            8
        } else if self.is_astc_block_height_10() {
            10
        } else if self.is_astc_block_height_12() {
            12
        } else {
            1
        }
    }

    ///
    /// Returns the pixel depth of a "block" for this image format
    ///
    /// # Info
    ///
    /// There are 3D block compressed ASTC formats, but they aren't enumerated by Vulkan at the time
    /// of this code being written so we just return 1 here.
    ///
    #[inline]
    pub const fn block_depth(self) -> u32 {
        1
    }

    ///
    /// Returns whether the format is one of the `SRGB` formats
    ///
    #[inline]
    pub const fn is_srgb(self) -> bool {
        matches!(
            self,
            VkFormat::R8_SRGB
                | VkFormat::R8G8_SRGB
                | VkFormat::R8G8B8_SRGB
                | VkFormat::B8G8R8_SRGB
                | VkFormat::R8G8B8A8_SRGB
                | VkFormat::B8G8R8A8_SRGB
                | VkFormat::A8B8G8R8_SRGB_PACK32
                | VkFormat::BC1_RGB_SRGB_BLOCK
                | VkFormat::BC1_RGBA_SRGB_BLOCK
                | VkFormat::BC2_SRGB_BLOCK
                | VkFormat::ASTC_4X4_SRGB_BLOCK
                | VkFormat::ASTC_8X6_SRGB_BLOCK
                | VkFormat::ASTC_5X4_SRGB_BLOCK
                | VkFormat::ASTC_5X5_SRGB_BLOCK
                | VkFormat::ASTC_6X5_SRGB_BLOCK
                | VkFormat::ASTC_6X6_SRGB_BLOCK
                | VkFormat::ASTC_8X5_SRGB_BLOCK
                | VkFormat::ASTC_8X8_SRGB_BLOCK
                | VkFormat::ASTC_10X5_SRGB_BLOCK
                | VkFormat::ASTC_10X6_SRGB_BLOCK
                | VkFormat::ASTC_10X8_SRGB_BLOCK
                | VkFormat::ASTC_10X10_SRGB_BLOCK
                | VkFormat::ASTC_12X10_SRGB_BLOCK
                | VkFormat::ASTC_12X12_SRGB_BLOCK
                | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
                | VkFormat::BC3_SRGB_BLOCK
                | VkFormat::BC7_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A1_SRGB_BLOCK
                | VkFormat::ETC2_R8G8B8A8_SRGB_BLOCK
        )
    }

    ///
    /// Does this format have a depth component
    ///
    #[inline]
    pub const fn is_depth_format(self) -> bool {
        matches!(
            self,
            VkFormat::D32_SFLOAT
                | VkFormat::D32_SFLOAT_S8_UINT
                | VkFormat::D24_UNORM_S8_UINT
                | VkFormat::D16_UNORM_S8_UINT
                | VkFormat::D16_UNORM
                | VkFormat::X8_D24_UNORM_PACK32
        )
    }

    ///
    /// Does this format have a stencil component
    ///
    pub const fn is_stencil_format(self) -> bool {
        matches!(
            self,
            VkFormat::S8_UINT
                | VkFormat::D32_SFLOAT_S8_UINT
                | VkFormat::D24_UNORM_S8_UINT
                | VkFormat::D16_UNORM_S8_UINT
        )
    }

    ///
    /// If this is a signed format (can represent negative values: SNORM, SINT, SFLOAT, SSCALED)
    ///
    #[inline]
    pub const fn is_signed(self) -> bool {
        matches!(
            self,
            VkFormat::R8_SNORM
                | VkFormat::R8_SSCALED
                | VkFormat::R8_SINT
                | VkFormat::R8G8_SNORM
                | VkFormat::R8G8_SSCALED
                | VkFormat::R8G8_SINT
                | VkFormat::R8G8B8_SNORM
                | VkFormat::R8G8B8_SSCALED
                | VkFormat::R8G8B8_SINT
                | VkFormat::B8G8R8_SNORM
                | VkFormat::B8G8R8_SSCALED
                | VkFormat::B8G8R8_SINT
                | VkFormat::R8G8B8A8_SNORM
                | VkFormat::R8G8B8A8_SSCALED
                | VkFormat::R8G8B8A8_SINT
                | VkFormat::B8G8R8A8_SNORM
                | VkFormat::B8G8R8A8_SSCALED
                | VkFormat::B8G8R8A8_SINT
                | VkFormat::A8B8G8R8_SNORM_PACK32
                | VkFormat::A8B8G8R8_SSCALED_PACK32
                | VkFormat::A8B8G8R8_SINT_PACK32
                | VkFormat::A2R10G10B10_SNORM_PACK32
                | VkFormat::A2R10G10B10_SSCALED_PACK32
                | VkFormat::A2R10G10B10_SINT_PACK32
                | VkFormat::A2B10G10R10_SNORM_PACK32
                | VkFormat::A2B10G10R10_SSCALED_PACK32
                | VkFormat::A2B10G10R10_SINT_PACK32
                | VkFormat::R16_SNORM
                | VkFormat::R16_SSCALED
                | VkFormat::R16_SINT
                | VkFormat::R16_SFLOAT
                | VkFormat::R16G16_SNORM
                | VkFormat::R16G16_SSCALED
                | VkFormat::R16G16_SINT
                | VkFormat::R16G16_SFLOAT
                | VkFormat::R16G16B16_SNORM
                | VkFormat::R16G16B16_SSCALED
                | VkFormat::R16G16B16_SINT
                | VkFormat::R16G16B16_SFLOAT
                | VkFormat::R16G16B16A16_SNORM
                | VkFormat::R16G16B16A16_SSCALED
                | VkFormat::R16G16B16A16_SINT
                | VkFormat::R16G16B16A16_SFLOAT
                | VkFormat::R32_SINT
                | VkFormat::R32_SFLOAT
                | VkFormat::R32G32_SINT
                | VkFormat::R32G32_SFLOAT
                | VkFormat::R32G32B32_SINT
                | VkFormat::R32G32B32_SFLOAT
                | VkFormat::R32G32B32A32_SINT
                | VkFormat::R32G32B32A32_SFLOAT
                | VkFormat::R64_SINT
                | VkFormat::R64_SFLOAT
                | VkFormat::R64G64_SINT
                | VkFormat::R64G64_SFLOAT
                | VkFormat::R64G64B64_SINT
                | VkFormat::R64G64B64_SFLOAT
                | VkFormat::R64G64B64A64_SINT
                | VkFormat::R64G64B64A64_SFLOAT
                | VkFormat::D32_SFLOAT
                | VkFormat::D32_SFLOAT_S8_UINT
                | VkFormat::BC4_SNORM_BLOCK
                | VkFormat::BC5_SNORM_BLOCK
                | VkFormat::BC6H_SFLOAT_BLOCK
                | VkFormat::EAC_R11_SNORM_BLOCK
                | VkFormat::EAC_R11G11_SNORM_BLOCK
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
        )
    }

    ///
    /// Is this format one of the floating point formats (SFLOAT, UFLOAT)
    ///
    #[inline]
    pub const fn is_floating_point(self) -> bool {
        matches!(
            self,
            VkFormat::R16_SFLOAT
                | VkFormat::R16G16_SFLOAT
                | VkFormat::R16G16B16_SFLOAT
                | VkFormat::R16G16B16A16_SFLOAT
                | VkFormat::R32_SFLOAT
                | VkFormat::R32G32_SFLOAT
                | VkFormat::R32G32B32_SFLOAT
                | VkFormat::R32G32B32A32_SFLOAT
                | VkFormat::R64_SFLOAT
                | VkFormat::R64G64_SFLOAT
                | VkFormat::R64G64B64_SFLOAT
                | VkFormat::R64G64B64A64_SFLOAT
                | VkFormat::B10G11R11_UFLOAT_PACK32
                | VkFormat::E5B9G9R9_UFLOAT_PACK32
                | VkFormat::D32_SFLOAT
                | VkFormat::D32_SFLOAT_S8_UINT
                | VkFormat::BC6H_UFLOAT_BLOCK
                | VkFormat::BC6H_SFLOAT_BLOCK
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
        )
    }

    ///
    /// Is this format one of the normalized formats (UNORM, SNORM)
    ///
    #[inline]
    pub const fn is_normalized(self) -> bool {
        matches!(
            self,
            VkFormat::R4G4_UNORM_PACK8
                | VkFormat::R4G4B4A4_UNORM_PACK16
                | VkFormat::B4G4R4A4_UNORM_PACK16
                | VkFormat::R5G6B5_UNORM_PACK16
                | VkFormat::B5G6R5_UNORM_PACK16
                | VkFormat::R5G5B5A1_UNORM_PACK16
                | VkFormat::B5G5R5A1_UNORM_PACK16
                | VkFormat::A1R5G5B5_UNORM_PACK16
                | VkFormat::R8_UNORM
                | VkFormat::R8_SNORM
                | VkFormat::R8_SRGB
                | VkFormat::R8G8_UNORM
                | VkFormat::R8G8_SNORM
                | VkFormat::R8G8_SRGB
                | VkFormat::R8G8B8_UNORM
                | VkFormat::R8G8B8_SNORM
                | VkFormat::R8G8B8_SRGB
                | VkFormat::B8G8R8_UNORM
                | VkFormat::B8G8R8_SNORM
                | VkFormat::B8G8R8_SRGB
                | VkFormat::R8G8B8A8_UNORM
                | VkFormat::R8G8B8A8_SNORM
                | VkFormat::R8G8B8A8_SRGB
                | VkFormat::B8G8R8A8_UNORM
                | VkFormat::B8G8R8A8_SNORM
                | VkFormat::B8G8R8A8_SRGB
                | VkFormat::A8B8G8R8_UNORM_PACK32
                | VkFormat::A8B8G8R8_SNORM_PACK32
                | VkFormat::A8B8G8R8_SRGB_PACK32
                | VkFormat::A2R10G10B10_UNORM_PACK32
                | VkFormat::A2R10G10B10_SNORM_PACK32
                | VkFormat::A2B10G10R10_UNORM_PACK32
                | VkFormat::A2B10G10R10_SNORM_PACK32
                | VkFormat::R16_UNORM
                | VkFormat::R16_SNORM
                | VkFormat::R16G16_UNORM
                | VkFormat::R16G16_SNORM
                | VkFormat::R16G16B16_UNORM
                | VkFormat::R16G16B16_SNORM
                | VkFormat::R16G16B16A16_UNORM
                | VkFormat::R16G16B16A16_SNORM
                | VkFormat::D16_UNORM
                | VkFormat::X8_D24_UNORM_PACK32
                | VkFormat::D16_UNORM_S8_UINT
                | VkFormat::D24_UNORM_S8_UINT
                | VkFormat::BC1_RGB_UNORM_BLOCK
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
                | VkFormat::G16_B16_R16_3PLANE_444_UNORM
                | VkFormat::PVRTC1_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_UNORM_BLOCK_IMG
                | VkFormat::PVRTC1_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC1_4BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_2BPP_SRGB_BLOCK_IMG
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        )
    }

    ///
    /// Returns whether this format is known by the implementation.
    ///
    /// # Info
    ///
    /// This library may end up compiled into binary that is supposed to consume `VkFormat` values
    /// provided by other programs. As a result it may be possible for `VkFormat` values from newer
    /// versions of the spec to be passed to a version of this library embedded in some binary
    /// somewhere.
    ///
    /// Newer Vulkan versions may define new formats that didn't exist when a particular version of
    /// this crate was written. The value would be a valid Vulkan format but it would still be
    /// treated as an unknown value. It would cause every utility function provided by this crate to
    /// return nonsensical results that have no bearing on the actual semantics of the format.
    ///
    /// To that end, this function is provided to allow for a user to check if the format is known
    /// by the version of `aleph-vk-format` they are using to defend against potential errors.
    ///
    /// This function will return `true` for **ALL** formats enumerated by the particular version
    /// of this crate that is being depended on.
    ///
    #[inline]
    pub const fn is_known(self) -> bool {
        matches!(
            self,
            VkFormat::UNDEFINED
                | VkFormat::R4G4_UNORM_PACK8
                | VkFormat::R4G4B4A4_UNORM_PACK16
                | VkFormat::B4G4R4A4_UNORM_PACK16
                | VkFormat::R5G6B5_UNORM_PACK16
                | VkFormat::B5G6R5_UNORM_PACK16
                | VkFormat::R5G5B5A1_UNORM_PACK16
                | VkFormat::B5G5R5A1_UNORM_PACK16
                | VkFormat::A1R5G5B5_UNORM_PACK16
                | VkFormat::R8_UNORM
                | VkFormat::R8_SNORM
                | VkFormat::R8_USCALED
                | VkFormat::R8_SSCALED
                | VkFormat::R8_UINT
                | VkFormat::R8_SINT
                | VkFormat::R8_SRGB
                | VkFormat::R8G8_UNORM
                | VkFormat::R8G8_SNORM
                | VkFormat::R8G8_USCALED
                | VkFormat::R8G8_SSCALED
                | VkFormat::R8G8_UINT
                | VkFormat::R8G8_SINT
                | VkFormat::R8G8_SRGB
                | VkFormat::R8G8B8_UNORM
                | VkFormat::R8G8B8_SNORM
                | VkFormat::R8G8B8_USCALED
                | VkFormat::R8G8B8_SSCALED
                | VkFormat::R8G8B8_UINT
                | VkFormat::R8G8B8_SINT
                | VkFormat::R8G8B8_SRGB
                | VkFormat::B8G8R8_UNORM
                | VkFormat::B8G8R8_SNORM
                | VkFormat::B8G8R8_USCALED
                | VkFormat::B8G8R8_SSCALED
                | VkFormat::B8G8R8_UINT
                | VkFormat::B8G8R8_SINT
                | VkFormat::B8G8R8_SRGB
                | VkFormat::R8G8B8A8_UNORM
                | VkFormat::R8G8B8A8_SNORM
                | VkFormat::R8G8B8A8_USCALED
                | VkFormat::R8G8B8A8_SSCALED
                | VkFormat::R8G8B8A8_UINT
                | VkFormat::R8G8B8A8_SINT
                | VkFormat::R8G8B8A8_SRGB
                | VkFormat::B8G8R8A8_UNORM
                | VkFormat::B8G8R8A8_SNORM
                | VkFormat::B8G8R8A8_USCALED
                | VkFormat::B8G8R8A8_SSCALED
                | VkFormat::B8G8R8A8_UINT
                | VkFormat::B8G8R8A8_SINT
                | VkFormat::B8G8R8A8_SRGB
                | VkFormat::A8B8G8R8_UNORM_PACK32
                | VkFormat::A8B8G8R8_SNORM_PACK32
                | VkFormat::A8B8G8R8_USCALED_PACK32
                | VkFormat::A8B8G8R8_SSCALED_PACK32
                | VkFormat::A8B8G8R8_UINT_PACK32
                | VkFormat::A8B8G8R8_SINT_PACK32
                | VkFormat::A8B8G8R8_SRGB_PACK32
                | VkFormat::A2R10G10B10_UNORM_PACK32
                | VkFormat::A2R10G10B10_SNORM_PACK32
                | VkFormat::A2R10G10B10_USCALED_PACK32
                | VkFormat::A2R10G10B10_SSCALED_PACK32
                | VkFormat::A2R10G10B10_UINT_PACK32
                | VkFormat::A2R10G10B10_SINT_PACK32
                | VkFormat::A2B10G10R10_UNORM_PACK32
                | VkFormat::A2B10G10R10_SNORM_PACK32
                | VkFormat::A2B10G10R10_USCALED_PACK32
                | VkFormat::A2B10G10R10_SSCALED_PACK32
                | VkFormat::A2B10G10R10_UINT_PACK32
                | VkFormat::A2B10G10R10_SINT_PACK32
                | VkFormat::R16_UNORM
                | VkFormat::R16_SNORM
                | VkFormat::R16_USCALED
                | VkFormat::R16_SSCALED
                | VkFormat::R16_UINT
                | VkFormat::R16_SINT
                | VkFormat::R16_SFLOAT
                | VkFormat::R16G16_UNORM
                | VkFormat::R16G16_SNORM
                | VkFormat::R16G16_USCALED
                | VkFormat::R16G16_SSCALED
                | VkFormat::R16G16_UINT
                | VkFormat::R16G16_SINT
                | VkFormat::R16G16_SFLOAT
                | VkFormat::R16G16B16_UNORM
                | VkFormat::R16G16B16_SNORM
                | VkFormat::R16G16B16_USCALED
                | VkFormat::R16G16B16_SSCALED
                | VkFormat::R16G16B16_UINT
                | VkFormat::R16G16B16_SINT
                | VkFormat::R16G16B16_SFLOAT
                | VkFormat::R16G16B16A16_UNORM
                | VkFormat::R16G16B16A16_SNORM
                | VkFormat::R16G16B16A16_USCALED
                | VkFormat::R16G16B16A16_SSCALED
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
                | VkFormat::B10G11R11_UFLOAT_PACK32
                | VkFormat::E5B9G9R9_UFLOAT_PACK32
                | VkFormat::D16_UNORM
                | VkFormat::X8_D24_UNORM_PACK32
                | VkFormat::D32_SFLOAT
                | VkFormat::S8_UINT
                | VkFormat::D16_UNORM_S8_UINT
                | VkFormat::D24_UNORM_S8_UINT
                | VkFormat::D32_SFLOAT_S8_UINT
                | VkFormat::BC1_RGB_UNORM_BLOCK
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
                | VkFormat::G16_B16_R16_3PLANE_444_UNORM
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
                | VkFormat::PVRTC2_4BPP_SRGB_BLOCK_IMG
        )
    }
}

impl std::fmt::Debug for VkFormat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match *self {
            Self::UNDEFINED => "UNDEFINED",
            Self::R4G4_UNORM_PACK8 => "R4G4_UNORM_PACK8",
            Self::R4G4B4A4_UNORM_PACK16 => "R4G4B4A4_UNORM_PACK16",
            Self::B4G4R4A4_UNORM_PACK16 => "B4G4R4A4_UNORM_PACK16",
            Self::R5G6B5_UNORM_PACK16 => "R5G6B5_UNORM_PACK16",
            Self::B5G6R5_UNORM_PACK16 => "B5G6R5_UNORM_PACK16",
            Self::R5G5B5A1_UNORM_PACK16 => "R5G5B5A1_UNORM_PACK16",
            Self::B5G5R5A1_UNORM_PACK16 => "B5G5R5A1_UNORM_PACK16",
            Self::A1R5G5B5_UNORM_PACK16 => "A1R5G5B5_UNORM_PACK16",
            Self::R8_UNORM => "R8_UNORM",
            Self::R8_SNORM => "R8_SNORM",
            Self::R8_USCALED => "R8_USCALED",
            Self::R8_SSCALED => "R8_SSCALED",
            Self::R8_UINT => "R8_UINT",
            Self::R8_SINT => "R8_SINT",
            Self::R8_SRGB => "R8_SRGB",
            Self::R8G8_UNORM => "R8G8_UNORM",
            Self::R8G8_SNORM => "R8G8_SNORM",
            Self::R8G8_USCALED => "R8G8_USCALED",
            Self::R8G8_SSCALED => "R8G8_SSCALED",
            Self::R8G8_UINT => "R8G8_UINT",
            Self::R8G8_SINT => "R8G8_SINT",
            Self::R8G8_SRGB => "R8G8_SRGB",
            Self::R8G8B8_UNORM => "R8G8B8_UNORM",
            Self::R8G8B8_SNORM => "R8G8B8_SNORM",
            Self::R8G8B8_USCALED => "R8G8B8_USCALED",
            Self::R8G8B8_SSCALED => "R8G8B8_SSCALED",
            Self::R8G8B8_UINT => "R8G8B8_UINT",
            Self::R8G8B8_SINT => "R8G8B8_SINT",
            Self::R8G8B8_SRGB => "R8G8B8_SRGB",
            Self::B8G8R8_UNORM => "B8G8R8_UNORM",
            Self::B8G8R8_SNORM => "B8G8R8_SNORM",
            Self::B8G8R8_USCALED => "B8G8R8_USCALED",
            Self::B8G8R8_SSCALED => "B8G8R8_SSCALED",
            Self::B8G8R8_UINT => "B8G8R8_UINT",
            Self::B8G8R8_SINT => "B8G8R8_SINT",
            Self::B8G8R8_SRGB => "B8G8R8_SRGB",
            Self::R8G8B8A8_UNORM => "R8G8B8A8_UNORM",
            Self::R8G8B8A8_SNORM => "R8G8B8A8_SNORM",
            Self::R8G8B8A8_USCALED => "R8G8B8A8_USCALED",
            Self::R8G8B8A8_SSCALED => "R8G8B8A8_SSCALED",
            Self::R8G8B8A8_UINT => "R8G8B8A8_UINT",
            Self::R8G8B8A8_SINT => "R8G8B8A8_SINT",
            Self::R8G8B8A8_SRGB => "R8G8B8A8_SRGB",
            Self::B8G8R8A8_UNORM => "B8G8R8A8_UNORM",
            Self::B8G8R8A8_SNORM => "B8G8R8A8_SNORM",
            Self::B8G8R8A8_USCALED => "B8G8R8A8_USCALED",
            Self::B8G8R8A8_SSCALED => "B8G8R8A8_SSCALED",
            Self::B8G8R8A8_UINT => "B8G8R8A8_UINT",
            Self::B8G8R8A8_SINT => "B8G8R8A8_SINT",
            Self::B8G8R8A8_SRGB => "B8G8R8A8_SRGB",
            Self::A8B8G8R8_UNORM_PACK32 => "A8B8G8R8_UNORM_PACK32",
            Self::A8B8G8R8_SNORM_PACK32 => "A8B8G8R8_SNORM_PACK32",
            Self::A8B8G8R8_USCALED_PACK32 => "A8B8G8R8_USCALED_PACK32",
            Self::A8B8G8R8_SSCALED_PACK32 => "A8B8G8R8_SSCALED_PACK32",
            Self::A8B8G8R8_UINT_PACK32 => "A8B8G8R8_UINT_PACK32",
            Self::A8B8G8R8_SINT_PACK32 => "A8B8G8R8_SINT_PACK32",
            Self::A8B8G8R8_SRGB_PACK32 => "A8B8G8R8_SRGB_PACK32",
            Self::A2R10G10B10_UNORM_PACK32 => "A2R10G10B10_UNORM_PACK32",
            Self::A2R10G10B10_SNORM_PACK32 => "A2R10G10B10_SNORM_PACK32",
            Self::A2R10G10B10_USCALED_PACK32 => "A2R10G10B10_USCALED_PACK32",
            Self::A2R10G10B10_SSCALED_PACK32 => "A2R10G10B10_SSCALED_PACK32",
            Self::A2R10G10B10_UINT_PACK32 => "A2R10G10B10_UINT_PACK32",
            Self::A2R10G10B10_SINT_PACK32 => "A2R10G10B10_SINT_PACK32",
            Self::A2B10G10R10_UNORM_PACK32 => "A2B10G10R10_UNORM_PACK32",
            Self::A2B10G10R10_SNORM_PACK32 => "A2B10G10R10_SNORM_PACK32",
            Self::A2B10G10R10_USCALED_PACK32 => "A2B10G10R10_USCALED_PACK32",
            Self::A2B10G10R10_SSCALED_PACK32 => "A2B10G10R10_SSCALED_PACK32",
            Self::A2B10G10R10_UINT_PACK32 => "A2B10G10R10_UINT_PACK32",
            Self::A2B10G10R10_SINT_PACK32 => "A2B10G10R10_SINT_PACK32",
            Self::R16_UNORM => "R16_UNORM",
            Self::R16_SNORM => "R16_SNORM",
            Self::R16_USCALED => "R16_USCALED",
            Self::R16_SSCALED => "R16_SSCALED",
            Self::R16_UINT => "R16_UINT",
            Self::R16_SINT => "R16_SINT",
            Self::R16_SFLOAT => "R16_SFLOAT",
            Self::R16G16_UNORM => "R16G16_UNORM",
            Self::R16G16_SNORM => "R16G16_SNORM",
            Self::R16G16_USCALED => "R16G16_USCALED",
            Self::R16G16_SSCALED => "R16G16_SSCALED",
            Self::R16G16_UINT => "R16G16_UINT",
            Self::R16G16_SINT => "R16G16_SINT",
            Self::R16G16_SFLOAT => "R16G16_SFLOAT",
            Self::R16G16B16_UNORM => "R16G16B16_UNORM",
            Self::R16G16B16_SNORM => "R16G16B16_SNORM",
            Self::R16G16B16_USCALED => "R16G16B16_USCALED",
            Self::R16G16B16_SSCALED => "R16G16B16_SSCALED",
            Self::R16G16B16_UINT => "R16G16B16_UINT",
            Self::R16G16B16_SINT => "R16G16B16_SINT",
            Self::R16G16B16_SFLOAT => "R16G16B16_SFLOAT",
            Self::R16G16B16A16_UNORM => "R16G16B16A16_UNORM",
            Self::R16G16B16A16_SNORM => "R16G16B16A16_SNORM",
            Self::R16G16B16A16_USCALED => "R16G16B16A16_USCALED",
            Self::R16G16B16A16_SSCALED => "R16G16B16A16_SSCALED",
            Self::R16G16B16A16_UINT => "R16G16B16A16_UINT",
            Self::R16G16B16A16_SINT => "R16G16B16A16_SINT",
            Self::R16G16B16A16_SFLOAT => "R16G16B16A16_SFLOAT",
            Self::R32_UINT => "R32_UINT",
            Self::R32_SINT => "R32_SINT",
            Self::R32_SFLOAT => "R32_SFLOAT",
            Self::R32G32_UINT => "R32G32_UINT",
            Self::R32G32_SINT => "R32G32_SINT",
            Self::R32G32_SFLOAT => "R32G32_SFLOAT",
            Self::R32G32B32_UINT => "R32G32B32_UINT",
            Self::R32G32B32_SINT => "R32G32B32_SINT",
            Self::R32G32B32_SFLOAT => "R32G32B32_SFLOAT",
            Self::R32G32B32A32_UINT => "R32G32B32A32_UINT",
            Self::R32G32B32A32_SINT => "R32G32B32A32_SINT",
            Self::R32G32B32A32_SFLOAT => "R32G32B32A32_SFLOAT",
            Self::R64_UINT => "R64_UINT",
            Self::R64_SINT => "R64_SINT",
            Self::R64_SFLOAT => "R64_SFLOAT",
            Self::R64G64_UINT => "R64G64_UINT",
            Self::R64G64_SINT => "R64G64_SINT",
            Self::R64G64_SFLOAT => "R64G64_SFLOAT",
            Self::R64G64B64_UINT => "R64G64B64_UINT",
            Self::R64G64B64_SINT => "R64G64B64_SINT",
            Self::R64G64B64_SFLOAT => "R64G64B64_SFLOAT",
            Self::R64G64B64A64_UINT => "R64G64B64A64_UINT",
            Self::R64G64B64A64_SINT => "R64G64B64A64_SINT",
            Self::R64G64B64A64_SFLOAT => "R64G64B64A64_SFLOAT",
            Self::B10G11R11_UFLOAT_PACK32 => "B10G11R11_UFLOAT_PACK32",
            Self::E5B9G9R9_UFLOAT_PACK32 => "E5B9G9R9_UFLOAT_PACK32",
            Self::D16_UNORM => "D16_UNORM",
            Self::X8_D24_UNORM_PACK32 => "X8_D24_UNORM_PACK32",
            Self::D32_SFLOAT => "D32_SFLOAT",
            Self::S8_UINT => "S8_UINT",
            Self::D16_UNORM_S8_UINT => "D16_UNORM_S8_UINT",
            Self::D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT",
            Self::D32_SFLOAT_S8_UINT => "D32_SFLOAT_S8_UINT",
            Self::BC1_RGB_UNORM_BLOCK => "BC1_RGB_UNORM_BLOCK",
            Self::BC1_RGB_SRGB_BLOCK => "BC1_RGB_SRGB_BLOCK",
            Self::BC1_RGBA_UNORM_BLOCK => "BC1_RGBA_UNORM_BLOCK",
            Self::BC1_RGBA_SRGB_BLOCK => "BC1_RGBA_SRGB_BLOCK",
            Self::BC2_UNORM_BLOCK => "BC2_UNORM_BLOCK",
            Self::BC2_SRGB_BLOCK => "BC2_SRGB_BLOCK",
            Self::BC3_UNORM_BLOCK => "BC3_UNORM_BLOCK",
            Self::BC3_SRGB_BLOCK => "BC3_SRGB_BLOCK",
            Self::BC4_UNORM_BLOCK => "BC4_UNORM_BLOCK",
            Self::BC4_SNORM_BLOCK => "BC4_SNORM_BLOCK",
            Self::BC5_UNORM_BLOCK => "BC5_UNORM_BLOCK",
            Self::BC5_SNORM_BLOCK => "BC5_SNORM_BLOCK",
            Self::BC6H_UFLOAT_BLOCK => "BC6H_UFLOAT_BLOCK",
            Self::BC6H_SFLOAT_BLOCK => "BC6H_SFLOAT_BLOCK",
            Self::BC7_UNORM_BLOCK => "BC7_UNORM_BLOCK",
            Self::BC7_SRGB_BLOCK => "BC7_SRGB_BLOCK",
            Self::ETC2_R8G8B8_UNORM_BLOCK => "ETC2_R8G8B8_UNORM_BLOCK",
            Self::ETC2_R8G8B8_SRGB_BLOCK => "ETC2_R8G8B8_SRGB_BLOCK",
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => "ETC2_R8G8B8A1_UNORM_BLOCK",
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => "ETC2_R8G8B8A1_SRGB_BLOCK",
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => "ETC2_R8G8B8A8_UNORM_BLOCK",
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => "ETC2_R8G8B8A8_SRGB_BLOCK",
            Self::EAC_R11_UNORM_BLOCK => "EAC_R11_UNORM_BLOCK",
            Self::EAC_R11_SNORM_BLOCK => "EAC_R11_SNORM_BLOCK",
            Self::EAC_R11G11_UNORM_BLOCK => "EAC_R11G11_UNORM_BLOCK",
            Self::EAC_R11G11_SNORM_BLOCK => "EAC_R11G11_SNORM_BLOCK",
            Self::ASTC_4X4_UNORM_BLOCK => "ASTC_4X4_UNORM_BLOCK",
            Self::ASTC_4X4_SRGB_BLOCK => "ASTC_4X4_SRGB_BLOCK",
            Self::ASTC_5X4_UNORM_BLOCK => "ASTC_5X4_UNORM_BLOCK",
            Self::ASTC_5X4_SRGB_BLOCK => "ASTC_5X4_SRGB_BLOCK",
            Self::ASTC_5X5_UNORM_BLOCK => "ASTC_5X5_UNORM_BLOCK",
            Self::ASTC_5X5_SRGB_BLOCK => "ASTC_5X5_SRGB_BLOCK",
            Self::ASTC_6X5_UNORM_BLOCK => "ASTC_6X5_UNORM_BLOCK",
            Self::ASTC_6X5_SRGB_BLOCK => "ASTC_6X5_SRGB_BLOCK",
            Self::ASTC_6X6_UNORM_BLOCK => "ASTC_6X6_UNORM_BLOCK",
            Self::ASTC_6X6_SRGB_BLOCK => "ASTC_6X6_SRGB_BLOCK",
            Self::ASTC_8X5_UNORM_BLOCK => "ASTC_8X5_UNORM_BLOCK",
            Self::ASTC_8X5_SRGB_BLOCK => "ASTC_8X5_SRGB_BLOCK",
            Self::ASTC_8X6_UNORM_BLOCK => "ASTC_8X6_UNORM_BLOCK",
            Self::ASTC_8X6_SRGB_BLOCK => "ASTC_8X6_SRGB_BLOCK",
            Self::ASTC_8X8_UNORM_BLOCK => "ASTC_8X8_UNORM_BLOCK",
            Self::ASTC_8X8_SRGB_BLOCK => "ASTC_8X8_SRGB_BLOCK",
            Self::ASTC_10X5_UNORM_BLOCK => "ASTC_10X5_UNORM_BLOCK",
            Self::ASTC_10X5_SRGB_BLOCK => "ASTC_10X5_SRGB_BLOCK",
            Self::ASTC_10X6_UNORM_BLOCK => "ASTC_10X6_UNORM_BLOCK",
            Self::ASTC_10X6_SRGB_BLOCK => "ASTC_10X6_SRGB_BLOCK",
            Self::ASTC_10X8_UNORM_BLOCK => "ASTC_10X8_UNORM_BLOCK",
            Self::ASTC_10X8_SRGB_BLOCK => "ASTC_10X8_SRGB_BLOCK",
            Self::ASTC_10X10_UNORM_BLOCK => "ASTC_10X10_UNORM_BLOCK",
            Self::ASTC_10X10_SRGB_BLOCK => "ASTC_10X10_SRGB_BLOCK",
            Self::ASTC_12X10_UNORM_BLOCK => "ASTC_12X10_UNORM_BLOCK",
            Self::ASTC_12X10_SRGB_BLOCK => "ASTC_12X10_SRGB_BLOCK",
            Self::ASTC_12X12_UNORM_BLOCK => "ASTC_12X12_UNORM_BLOCK",
            Self::ASTC_12X12_SRGB_BLOCK => "ASTC_12X12_SRGB_BLOCK",
            Self::G8B8G8R8_422_UNORM => "G8B8G8R8_422_UNORM",
            Self::B8G8R8G8_422_UNORM => "B8G8R8G8_422_UNORM",
            Self::G8_B8_R8_3PLANE_420_UNORM => "G8_B8_R8_3PLANE_420_UNORM",
            Self::G8_B8R8_2PLANE_420_UNORM => "G8_B8R8_2PLANE_420_UNORM",
            Self::G8_B8_R8_3PLANE_422_UNORM => "G8_B8_R8_3PLANE_422_UNORM",
            Self::G8_B8R8_2PLANE_422_UNORM => "G8_B8R8_2PLANE_422_UNORM",
            Self::G8_B8_R8_3PLANE_444_UNORM => "G8_B8_R8_3PLANE_444_UNORM",
            Self::R10X6_UNORM_PACK16 => "R10X6_UNORM_PACK16",
            Self::R10X6G10X6_UNORM_2PACK16 => "R10X6G10X6_UNORM_2PACK16",
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "R10X6G10X6B10X6A10X6_UNORM_4PACK16",
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16"
            }
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16"
            }
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16"
            }
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16"
            }
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16"
            }
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16"
            }
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16"
            }
            Self::R12X4_UNORM_PACK16 => "R12X4_UNORM_PACK16",
            Self::R12X4G12X4_UNORM_2PACK16 => "R12X4G12X4_UNORM_2PACK16",
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "R12X4G12X4B12X4A12X4_UNORM_4PACK16",
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16"
            }
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16"
            }
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16"
            }
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16"
            }
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16"
            }
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16"
            }
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16"
            }
            Self::G16B16G16R16_422_UNORM => "G16B16G16R16_422_UNORM",
            Self::B16G16R16G16_422_UNORM => "B16G16R16G16_422_UNORM",
            Self::G16_B16_R16_3PLANE_420_UNORM => "G16_B16_R16_3PLANE_420_UNORM",
            Self::G16_B16R16_2PLANE_420_UNORM => "G16_B16R16_2PLANE_420_UNORM",
            Self::G16_B16_R16_3PLANE_422_UNORM => "G16_B16_R16_3PLANE_422_UNORM",
            Self::G16_B16R16_2PLANE_422_UNORM => "G16_B16R16_2PLANE_422_UNORM",
            Self::G16_B16_R16_3PLANE_444_UNORM => "G16_B16_R16_3PLANE_444_UNORM",
            Self::ASTC_4X4_SFLOAT_BLOCK_EXT => "ASTC_4X4_SFLOAT_BLOCK_EXT",
            Self::ASTC_5X4_SFLOAT_BLOCK_EXT => "ASTC_5X4_SFLOAT_BLOCK_EXT",
            Self::ASTC_5X5_SFLOAT_BLOCK_EXT => "ASTC_5X5_SFLOAT_BLOCK_EXT",
            Self::ASTC_6X5_SFLOAT_BLOCK_EXT => "ASTC_6X5_SFLOAT_BLOCK_EXT",
            Self::ASTC_6X6_SFLOAT_BLOCK_EXT => "ASTC_6X6_SFLOAT_BLOCK_EXT",
            Self::ASTC_8X5_SFLOAT_BLOCK_EXT => "ASTC_8X5_SFLOAT_BLOCK_EXT",
            Self::ASTC_8X6_SFLOAT_BLOCK_EXT => "ASTC_8X6_SFLOAT_BLOCK_EXT",
            Self::ASTC_8X8_SFLOAT_BLOCK_EXT => "ASTC_8X8_SFLOAT_BLOCK_EXT",
            Self::ASTC_10X5_SFLOAT_BLOCK_EXT => "ASTC_10X5_SFLOAT_BLOCK_EXT",
            Self::ASTC_10X6_SFLOAT_BLOCK_EXT => "ASTC_10X6_SFLOAT_BLOCK_EXT",
            Self::ASTC_10X8_SFLOAT_BLOCK_EXT => "ASTC_10X8_SFLOAT_BLOCK_EXT",
            Self::ASTC_10X10_SFLOAT_BLOCK_EXT => "ASTC_10X10_SFLOAT_BLOCK_EXT",
            Self::ASTC_12X10_SFLOAT_BLOCK_EXT => "ASTC_12X10_SFLOAT_BLOCK_EXT",
            Self::ASTC_12X12_SFLOAT_BLOCK_EXT => "ASTC_12X12_SFLOAT_BLOCK_EXT",
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => "PVRTC1_2BPP_UNORM_BLOCK_IMG",
            Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => "PVRTC1_4BPP_UNORM_BLOCK_IMG",
            Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => "PVRTC2_2BPP_UNORM_BLOCK_IMG",
            Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => "PVRTC2_4BPP_UNORM_BLOCK_IMG",
            Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => "PVRTC1_2BPP_SRGB_BLOCK_IMG",
            Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => "PVRTC1_4BPP_SRGB_BLOCK_IMG",
            Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => "PVRTC2_2BPP_SRGB_BLOCK_IMG",
            Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => "PVRTC2_4BPP_SRGB_BLOCK_IMG",
            _ => "(unknown)",
        })
    }
}

///
/// A list of all formats that are enumerated by this crate
///
pub const ALL_FORMATS: [VkFormat; 241] = [
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
    VkFormat::R8_USCALED,
    VkFormat::R8_SSCALED,
    VkFormat::R8_UINT,
    VkFormat::R8_SINT,
    VkFormat::R8_SRGB,
    VkFormat::R8G8_UNORM,
    VkFormat::R8G8_SNORM,
    VkFormat::R8G8_USCALED,
    VkFormat::R8G8_SSCALED,
    VkFormat::R8G8_UINT,
    VkFormat::R8G8_SINT,
    VkFormat::R8G8_SRGB,
    VkFormat::R8G8B8_UNORM,
    VkFormat::R8G8B8_SNORM,
    VkFormat::R8G8B8_USCALED,
    VkFormat::R8G8B8_SSCALED,
    VkFormat::R8G8B8_UINT,
    VkFormat::R8G8B8_SINT,
    VkFormat::R8G8B8_SRGB,
    VkFormat::B8G8R8_UNORM,
    VkFormat::B8G8R8_SNORM,
    VkFormat::B8G8R8_USCALED,
    VkFormat::B8G8R8_SSCALED,
    VkFormat::B8G8R8_UINT,
    VkFormat::B8G8R8_SINT,
    VkFormat::B8G8R8_SRGB,
    VkFormat::R8G8B8A8_UNORM,
    VkFormat::R8G8B8A8_SNORM,
    VkFormat::R8G8B8A8_USCALED,
    VkFormat::R8G8B8A8_SSCALED,
    VkFormat::R8G8B8A8_UINT,
    VkFormat::R8G8B8A8_SINT,
    VkFormat::R8G8B8A8_SRGB,
    VkFormat::B8G8R8A8_UNORM,
    VkFormat::B8G8R8A8_SNORM,
    VkFormat::B8G8R8A8_USCALED,
    VkFormat::B8G8R8A8_SSCALED,
    VkFormat::B8G8R8A8_UINT,
    VkFormat::B8G8R8A8_SINT,
    VkFormat::B8G8R8A8_SRGB,
    VkFormat::A8B8G8R8_UNORM_PACK32,
    VkFormat::A8B8G8R8_SNORM_PACK32,
    VkFormat::A8B8G8R8_USCALED_PACK32,
    VkFormat::A8B8G8R8_SSCALED_PACK32,
    VkFormat::A8B8G8R8_UINT_PACK32,
    VkFormat::A8B8G8R8_SINT_PACK32,
    VkFormat::A8B8G8R8_SRGB_PACK32,
    VkFormat::A2R10G10B10_UNORM_PACK32,
    VkFormat::A2R10G10B10_SNORM_PACK32,
    VkFormat::A2R10G10B10_USCALED_PACK32,
    VkFormat::A2R10G10B10_SSCALED_PACK32,
    VkFormat::A2R10G10B10_UINT_PACK32,
    VkFormat::A2R10G10B10_SINT_PACK32,
    VkFormat::A2B10G10R10_UNORM_PACK32,
    VkFormat::A2B10G10R10_SNORM_PACK32,
    VkFormat::A2B10G10R10_USCALED_PACK32,
    VkFormat::A2B10G10R10_SSCALED_PACK32,
    VkFormat::A2B10G10R10_UINT_PACK32,
    VkFormat::A2B10G10R10_SINT_PACK32,
    VkFormat::R16_UNORM,
    VkFormat::R16_SNORM,
    VkFormat::R16_USCALED,
    VkFormat::R16_SSCALED,
    VkFormat::R16_UINT,
    VkFormat::R16_SINT,
    VkFormat::R16_SFLOAT,
    VkFormat::R16G16_UNORM,
    VkFormat::R16G16_SNORM,
    VkFormat::R16G16_USCALED,
    VkFormat::R16G16_SSCALED,
    VkFormat::R16G16_UINT,
    VkFormat::R16G16_SINT,
    VkFormat::R16G16_SFLOAT,
    VkFormat::R16G16B16_UNORM,
    VkFormat::R16G16B16_SNORM,
    VkFormat::R16G16B16_USCALED,
    VkFormat::R16G16B16_SSCALED,
    VkFormat::R16G16B16_UINT,
    VkFormat::R16G16B16_SINT,
    VkFormat::R16G16B16_SFLOAT,
    VkFormat::R16G16B16A16_UNORM,
    VkFormat::R16G16B16A16_SNORM,
    VkFormat::R16G16B16A16_USCALED,
    VkFormat::R16G16B16A16_SSCALED,
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
    VkFormat::G8B8G8R8_422_UNORM,
    VkFormat::B8G8R8G8_422_UNORM,
    VkFormat::G8_B8_R8_3PLANE_420_UNORM,
    VkFormat::G8_B8R8_2PLANE_420_UNORM,
    VkFormat::G8_B8_R8_3PLANE_422_UNORM,
    VkFormat::G8_B8R8_2PLANE_422_UNORM,
    VkFormat::G8_B8_R8_3PLANE_444_UNORM,
    VkFormat::R10X6_UNORM_PACK16,
    VkFormat::R10X6G10X6_UNORM_2PACK16,
    VkFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16,
    VkFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16,
    VkFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16,
    VkFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    VkFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    VkFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    VkFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    VkFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
    VkFormat::R12X4_UNORM_PACK16,
    VkFormat::R12X4G12X4_UNORM_2PACK16,
    VkFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16,
    VkFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16,
    VkFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16,
    VkFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    VkFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    VkFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    VkFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    VkFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
    VkFormat::G16B16G16R16_422_UNORM,
    VkFormat::B16G16R16G16_422_UNORM,
    VkFormat::G16_B16_R16_3PLANE_420_UNORM,
    VkFormat::G16_B16R16_2PLANE_420_UNORM,
    VkFormat::G16_B16_R16_3PLANE_422_UNORM,
    VkFormat::G16_B16R16_2PLANE_422_UNORM,
    VkFormat::G16_B16_R16_3PLANE_444_UNORM,
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
