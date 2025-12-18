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

use crate::*;

#[doc(hidden)]
macro_rules! new_linear_metadata {
    ($f: expr, $n: expr, $e_size: expr, $encoding: expr) => {
        FormatMetadata {
            name: $n,
            bytes_per_element: $e_size,
            is_srgb: false,
            encoding: $encoding,
            has_depth: false,
            has_stencil: false,
            srgb_format: $f,
            linear_format: $f,
            compatible_views: &[$f],
            aspect_mask: TextureAspect::COLOR,
            block_dimensions: (1, 1, 1),
        }
    };
    ($f: expr, $n: expr, $e_size: expr, $encoding: expr, $s_f: expr) => {
        FormatMetadata {
            name: $n,
            bytes_per_element: $e_size,
            is_srgb: false,
            encoding: $encoding,
            has_depth: false,
            has_stencil: false,
            srgb_format: $s_f,
            linear_format: $f,
            compatible_views: &[$f, $s_f],
            aspect_mask: $crate::TextureAspect::COLOR,
            block_dimensions: (1, 1, 1),
        }
    };
}

#[doc(hidden)]
macro_rules! new_srgb_metadata {
    ($f: expr, $n: expr, $e_size: expr, $encoding: expr, $l_f: expr) => {
        FormatMetadata {
            name: $n,
            bytes_per_element: $e_size,
            is_srgb: true,
            encoding: $encoding,
            has_depth: false,
            has_stencil: false,
            srgb_format: $f,
            linear_format: $l_f,
            compatible_views: &[$l_f, $f],
            aspect_mask: $crate::TextureAspect::COLOR,
            block_dimensions: (1, 1, 1),
        }
    };
}

#[derive(Clone, Debug)]
pub struct FormatMetadata {
    pub name: &'static str,
    pub bytes_per_element: u8,
    pub is_srgb: bool,
    pub encoding: FormatChannelEncoding,
    pub has_depth: bool,
    pub has_stencil: bool,
    pub srgb_format: Format,
    pub linear_format: Format,
    pub compatible_views: &'static [Format],
    pub aspect_mask: TextureAspect,
    pub block_dimensions: (u8, u8, u8),
}

#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FormatChannelEncoding {
    Unorm,
    Snorm,
    Uint,
    Sint,
    Float,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Format {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Unorm,
    R16Snorm,
    R16Float,
    R32Uint,
    R32Sint,
    R32Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    Rg16Uint,
    Rg16Sint,
    Rg16Unorm,
    Rg16Snorm,
    Rg16Float,
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgb32Uint,
    Rgb32Sint,
    Rgb32Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb10a2Unorm,
    Rg11b10Float,
    Rgb9e5Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Unorm,
    Rgba16Snorm,
    Rgba16Float,
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,
    Depth32Float,
    Depth32FloatStencil8,
    Depth24Stencil8,
}

impl Default for Format {
    #[inline(always)]
    fn default() -> Self {
        Self::R8Unorm
    }
}

impl Format {
    #[rustfmt::skip]
    pub const fn metadata(&self) -> FormatMetadata {
        match *self {
            Format::R8Unorm => new_linear_metadata!(Format::R8Unorm, "R8Unorm", 1, FormatChannelEncoding::Unorm),
            Format::R8Snorm => new_linear_metadata!(Format::R8Snorm, "R8Snorm", 1, FormatChannelEncoding::Snorm),
            Format::R8Uint => new_linear_metadata!(Format::R8Uint, "R8Uint", 1, FormatChannelEncoding::Uint),
            Format::R8Sint => new_linear_metadata!(Format::R8Sint, "R8Sint", 1, FormatChannelEncoding::Sint),
            Format::R16Uint => new_linear_metadata!(Format::R16Uint, "R16Uint", 2, FormatChannelEncoding::Uint),
            Format::R16Sint => new_linear_metadata!(Format::R16Sint, "R16Sint", 2, FormatChannelEncoding::Sint),
            Format::R16Unorm => new_linear_metadata!(Format::R16Unorm, "R16Unorm", 2, FormatChannelEncoding::Unorm),
            Format::R16Snorm => new_linear_metadata!(Format::R16Snorm, "R16Snorm", 2, FormatChannelEncoding::Snorm),
            Format::R16Float => new_linear_metadata!(Format::R16Float, "R16Float", 2, FormatChannelEncoding::Float),
            Format::R32Uint => new_linear_metadata!(Format::R32Uint, "R32Uint", 4, FormatChannelEncoding::Uint),
            Format::R32Sint => new_linear_metadata!(Format::R32Sint, "R32Sint", 4, FormatChannelEncoding::Sint),
            Format::R32Float => new_linear_metadata!(Format::R32Float, "R32Float", 4, FormatChannelEncoding::Float),
            Format::Rg8Unorm => new_linear_metadata!(Format::Rg8Unorm, "Rg8Unorm", 2, FormatChannelEncoding::Unorm),
            Format::Rg8Snorm => new_linear_metadata!(Format::Rg8Snorm, "Rg8Snorm", 2, FormatChannelEncoding::Snorm),
            Format::Rg8Uint => new_linear_metadata!(Format::Rg8Uint, "Rg8Uint", 2, FormatChannelEncoding::Uint),
            Format::Rg8Sint => new_linear_metadata!(Format::Rg8Sint, "Rg8Sint", 2, FormatChannelEncoding::Sint),
            Format::Rg16Uint => new_linear_metadata!(Format::Rg16Uint, "Rg16Uint", 4, FormatChannelEncoding::Uint),
            Format::Rg16Sint => new_linear_metadata!(Format::Rg16Sint, "Rg16Sint", 4, FormatChannelEncoding::Sint),
            Format::Rg16Unorm => new_linear_metadata!(Format::Rg16Unorm, "Rg16Unorm", 4, FormatChannelEncoding::Unorm),
            Format::Rg16Snorm => new_linear_metadata!(Format::Rg16Snorm, "Rg16Snorm", 4, FormatChannelEncoding::Snorm),
            Format::Rg16Float => new_linear_metadata!(Format::Rg16Float, "Rg16Float", 4, FormatChannelEncoding::Float),
            Format::Rg32Uint => new_linear_metadata!(Format::Rg32Uint, "Rg32Uint", 8, FormatChannelEncoding::Uint),
            Format::Rg32Sint => new_linear_metadata!(Format::Rg32Sint, "Rg32Sint", 8, FormatChannelEncoding::Sint),
            Format::Rg32Float => new_linear_metadata!(Format::Rg32Float, "Rg32Float", 8, FormatChannelEncoding::Float),
            Format::Rgb32Uint => new_linear_metadata!(Format::Rgb32Uint, "Rgb32Uint", 12, FormatChannelEncoding::Uint),
            Format::Rgb32Sint => new_linear_metadata!(Format::Rgb32Sint, "Rgb32Sint", 12, FormatChannelEncoding::Sint),
            Format::Rgb32Float => new_linear_metadata!(Format::Rgb32Float, "Rgb32Float", 12, FormatChannelEncoding::Float),
            Format::Rgba8Unorm => new_linear_metadata!(Format::Rgba8Unorm, "Rgba8Unorm", 4, FormatChannelEncoding::Unorm, Format::Rgba8UnormSrgb),
            Format::Rgba8UnormSrgb => new_srgb_metadata!(Format::Rgba8UnormSrgb, "Rgba8UnormSrgb", 4, FormatChannelEncoding::Unorm, Format::Rgba8Unorm),
            Format::Rgba8Snorm => new_linear_metadata!(Format::Rgba8Snorm, "Rgba8Snorm", 4, FormatChannelEncoding::Snorm),
            Format::Rgba8Uint => new_linear_metadata!(Format::Rgba8Uint, "Rgba8Uint", 4, FormatChannelEncoding::Uint),
            Format::Rgba8Sint => new_linear_metadata!(Format::Rgba8Sint, "Rgba8Sint", 4, FormatChannelEncoding::Sint),
            Format::Bgra8Unorm => new_linear_metadata!(Format::Bgra8Unorm, "Bgra8Unorm", 4, FormatChannelEncoding::Unorm, Format::Bgra8UnormSrgb),
            Format::Bgra8UnormSrgb => new_srgb_metadata!(Format::Bgra8UnormSrgb, "Bgra8UnormSrgb", 4, FormatChannelEncoding::Unorm, Format::Bgra8Unorm),
            Format::Rgb10a2Unorm => new_linear_metadata!(Format::Rgb10a2Unorm, "Rgb10a2Unorm", 4, FormatChannelEncoding::Unorm),
            Format::Rg11b10Float => new_linear_metadata!(Format::Rg11b10Float, "Rg11b10Float", 4, FormatChannelEncoding::Float),
            Format::Rgb9e5Float => new_linear_metadata!(Format::Rgb9e5Float, "Rgb9e5Float", 4, FormatChannelEncoding::Float),
            Format::Rgba16Uint => new_linear_metadata!(Format::Rgba16Uint, "Rgba16Uint", 8, FormatChannelEncoding::Uint),
            Format::Rgba16Sint => new_linear_metadata!(Format::Rgba16Sint, "Rgba16Sint", 8, FormatChannelEncoding::Sint),
            Format::Rgba16Unorm => new_linear_metadata!(Format::Rgba16Unorm, "Rgba16Unorm", 8, FormatChannelEncoding::Unorm),
            Format::Rgba16Snorm => new_linear_metadata!(Format::Rgba16Snorm, "Rgba16Snorm", 8, FormatChannelEncoding::Snorm),
            Format::Rgba16Float => new_linear_metadata!(Format::Rgba16Float, "Rgba16Float", 8, FormatChannelEncoding::Float),
            Format::Rgba32Uint => new_linear_metadata!(Format::Rgba32Uint, "Rgba32Uint", 16, FormatChannelEncoding::Uint),
            Format::Rgba32Sint => new_linear_metadata!(Format::Rgba32Sint, "Rgba32Sint", 16, FormatChannelEncoding::Sint),
            Format::Rgba32Float => new_linear_metadata!(Format::Rgba32Float, "Rgba32Float", 16, FormatChannelEncoding::Float),
            Format::Depth32Float => FormatMetadata {
                name: "Depth32Float",
                bytes_per_element: 4,
                is_srgb: false,
                encoding: FormatChannelEncoding::Float,
                has_depth: true,
                has_stencil: false,
                srgb_format: Format::Depth32Float,
                linear_format: Format::Depth32Float,
                compatible_views: &[Format::Depth32Float],
                aspect_mask: TextureAspect::DEPTH,
                block_dimensions: (1, 1, 1),
            },
            Format::Depth32FloatStencil8 => FormatMetadata {
                name: "Depth32FloatStencil8",
                bytes_per_element: 8,
                is_srgb: false,
                encoding: FormatChannelEncoding::Float,
                has_depth: true,
                has_stencil: true,
                srgb_format: Format::Depth32FloatStencil8,
                linear_format: Format::Depth32FloatStencil8,
                compatible_views: &[Format::Depth32FloatStencil8],
                aspect_mask: TextureAspect::DEPTH_STENCIL,
                block_dimensions: (1, 1, 1),
            },
            Format::Depth24Stencil8 => FormatMetadata {
                name: "Depth24Stencil8",
                bytes_per_element: 4,
                is_srgb: false,
                encoding: FormatChannelEncoding::Unorm,
                has_depth: true,
                has_stencil: true,
                srgb_format: Format::Depth24Stencil8,
                linear_format: Format::Depth24Stencil8,
                compatible_views: &[Format::Depth24Stencil8],
                aspect_mask: TextureAspect::DEPTH_STENCIL,
                block_dimensions: (1, 1, 1),
            },
        }
    }

    /// Returns the Linear version of a format if it's an SRGB format. If the format is already
    /// linear it simply returns itself.
    pub const fn compatible_view_formats(&self) -> &'static [Format] {
        self.metadata().compatible_views
    }

    /// Returns the Linear version of a format if it's an SRGB format. If the format is already
    /// linear it simply returns itself.
    pub const fn to_non_srgb(&self) -> Self {
        self.metadata().linear_format
    }

    /// Converts the given format into its SRGB form. (i.e. Format::Rgba8Unorm ->
    /// Format::Rgba8UnormSrgb).
    ///
    /// If the format is already an SRGB format this function returns the same SRGB format.
    ///
    /// If the format does not have a matching SRGB format then it returns the same format as was
    /// passed in.
    pub const fn to_srgb(&self) -> Self {
        self.metadata().srgb_format
    }

    /// Returns whether the format is an SRGB format
    pub const fn is_srgb(&self) -> bool {
        self.metadata().is_srgb
    }

    /// Returns whether the format has an alterative, storage compatible format that encodes in the
    /// SRGB space instead of linear RGB.
    ///
    /// This is useful for detecting when the format allows viewing as both SRGB or Linear depending
    /// on the format you provide to the SRV.
    pub const fn has_srgb_format(&self) -> bool {
        let m = self.metadata();

        // Cast to u8 to workaround const fn limits
        m.srgb_format as u8 != m.linear_format as u8
    }

    /// Returns whether the format is a depth texture format
    pub const fn is_depth(&self) -> bool {
        // TODO: we need to revamp our depth metadata reporting
        let m = self.metadata();
        m.has_depth && !m.has_stencil
    }

    /// Returns whether the format is a stencil texture format
    pub const fn is_stencil(&self) -> bool {
        self.metadata().has_stencil
    }

    /// Returns whether the format is a depth/stencil texture format
    pub const fn is_depth_stencil(&self) -> bool {
        let m = self.metadata();
        m.has_depth || m.has_stencil
    }

    /// Returns whether the format is a float format
    pub const fn is_float(&self) -> bool {
        matches!(self.metadata().encoding, FormatChannelEncoding::Float)
    }

    /// Returns whether the format is a signed-int format
    pub const fn is_sint(&self) -> bool {
        matches!(self.metadata().encoding, FormatChannelEncoding::Sint)
    }

    /// Returns whether the format is an unsigned-int format
    pub const fn is_uint(&self) -> bool {
        matches!(self.metadata().encoding, FormatChannelEncoding::Uint)
    }

    /// Returns whether the format is a signed-normalized-int format
    pub const fn is_snorm(&self) -> bool {
        matches!(self.metadata().encoding, FormatChannelEncoding::Snorm)
    }

    /// Returns whether the format is an unsigned-normalized-int format
    pub const fn is_unorm(&self) -> bool {
        matches!(self.metadata().encoding, FormatChannelEncoding::Unorm)
    }

    /// Returns the number of bytes the format consumes per individual element.
    ///
    /// For standard formats this will return the number of bytes per texel, for block formats this
    /// will return the number of bytes per block (block formats smallest 'element' is a single
    /// block).
    pub const fn bytes_per_element(&self) -> u32 {
        self.metadata().bytes_per_element as u32
    }

    /// Returns the dimensions of a block for the given format.
    pub const fn block_dimensions(&self) -> (u8, u8, u8) {
        self.metadata().block_dimensions
    }

    pub const fn has_aspect(&self, aspect: TextureCopyAspect) -> bool {
        self.aspect_mask().contains(aspect.as_flag())
    }

    pub const fn is_aspect_compatible(&self, aspect: TextureAspect) -> bool {
        self.aspect_mask().contains(aspect)
    }

    pub const fn aspect_mask(&self) -> TextureAspect {
        self.metadata().aspect_mask
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.metadata().name)
    }
}

/// Enumeration of all available _vertex_ formats. These are distinct from pixel formats as they
/// are generally a distinct set of formats. There are many valid vertex formats that are not valid
/// pixel formats, and vice versa.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum VertexFormat {
    UChar,
    UChar2,
    UChar3,
    UChar4,
    Char,
    Char2,
    Char3,
    Char4,
    UCharNormalized,
    UChar2Normalized,
    UChar3Normalized,
    UChar4Normalized,
    UChar4NormalizedBgra,
    CharNormalized,
    Char2Normalized,
    Char3Normalized,
    Char4Normalized,
    UShort,
    UShort2,
    UShort3,
    UShort4,
    Short,
    Short2,
    Short3,
    Short4,
    UShortNormalized,
    UShort2Normalized,
    UShort3Normalized,
    UShort4Normalized,
    ShortNormalized,
    Short2Normalized,
    Short3Normalized,
    Short4Normalized,
    Half,
    Half2,
    Half3,
    Half4,
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
    UInt,
    UInt2,
    UInt3,
    UInt4,
    Int1010102Normalized,
    UInt1010102Normalized,
    FloatRG11B10,
    FloatRGB9E5,
}

impl Default for VertexFormat {
    #[inline]
    fn default() -> Self {
        Self::UChar
    }
}

impl std::fmt::Display for VertexFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            VertexFormat::UChar => "UChar",
            VertexFormat::UChar2 => "UChar2",
            VertexFormat::UChar3 => "UChar3",
            VertexFormat::UChar4 => "UChar4",
            VertexFormat::Char => "Char",
            VertexFormat::Char2 => "Char2",
            VertexFormat::Char3 => "Char3",
            VertexFormat::Char4 => "Char4",
            VertexFormat::UCharNormalized => "UCharNormalized",
            VertexFormat::UChar2Normalized => "UChar2Normalized",
            VertexFormat::UChar3Normalized => "UChar3Normalized",
            VertexFormat::UChar4Normalized => "UChar4Normalized",
            VertexFormat::UChar4NormalizedBgra => "UChar4NormalizedBgra",
            VertexFormat::CharNormalized => "CharNormalized",
            VertexFormat::Char2Normalized => "Char2Normalized",
            VertexFormat::Char3Normalized => "Char3Normalized",
            VertexFormat::Char4Normalized => "Char4Normalized",
            VertexFormat::UShort => "UShort",
            VertexFormat::UShort2 => "UShort2",
            VertexFormat::UShort3 => "UShort3",
            VertexFormat::UShort4 => "UShort4",
            VertexFormat::Short => "Short",
            VertexFormat::Short2 => "Short2",
            VertexFormat::Short3 => "Short3",
            VertexFormat::Short4 => "Short4",
            VertexFormat::UShortNormalized => "UShortNormalized",
            VertexFormat::UShort2Normalized => "UShort2Normalized",
            VertexFormat::UShort3Normalized => "UShort3Normalized",
            VertexFormat::UShort4Normalized => "UShort4Normalized",
            VertexFormat::ShortNormalized => "ShortNormalized",
            VertexFormat::Short2Normalized => "Short2Normalized",
            VertexFormat::Short3Normalized => "Short3Normalized",
            VertexFormat::Short4Normalized => "Short4Normalized",
            VertexFormat::Half => "Half",
            VertexFormat::Half2 => "Half2",
            VertexFormat::Half3 => "Half3",
            VertexFormat::Half4 => "Half4",
            VertexFormat::Float => "Float",
            VertexFormat::Float2 => "Float2",
            VertexFormat::Float3 => "Float3",
            VertexFormat::Float4 => "Float4",
            VertexFormat::Int => "Int",
            VertexFormat::Int2 => "Int2",
            VertexFormat::Int3 => "Int3",
            VertexFormat::Int4 => "Int4",
            VertexFormat::UInt => "UInt",
            VertexFormat::UInt2 => "UInt2",
            VertexFormat::UInt3 => "UInt3",
            VertexFormat::UInt4 => "UInt4",
            VertexFormat::Int1010102Normalized => "Int1010102Normalized",
            VertexFormat::UInt1010102Normalized => "UInt1010102Normalized",
            VertexFormat::FloatRG11B10 => "FloatRG11B10",
            VertexFormat::FloatRGB9E5 => "FloatRGB9E5",
        };
        f.write_str(string)
    }
}
