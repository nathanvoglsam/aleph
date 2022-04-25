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

use crate::gpu::{INamedObject, ResourceStates};
use any::{AnyArc, IAny};
use std::any::Any;
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub trait ITexture: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(ITexture);

    fn desc(&self) -> &TextureDesc;
}

pub trait IAcquiredTexture: IAny + Send + 'static {
    fn image(&self) -> &dyn ITexture;
}

/// Description object used for creating a new texture.
#[derive(Clone, PartialEq, Debug)]
pub struct TextureDesc {
    /// The width of the texture
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: TextureFormat,

    /// The dimensionality of the texture.
    ///
    /// Declares whether the texture should be a 1D, 2D, 3D or cube texture.
    pub dimension: TextureDimension,

    /// The initial resource state the texture will take
    pub initial_state: ResourceStates,

    /// An optional clear value that will be 'optimal' for the underlying implementation.
    pub clear_value: Option<OptimalClearValue>,

    /// Number of image array elements.
    ///
    /// A value of '1' means to create a regular, non-array texture. Setting this to a value >1
    /// declares the texture as a texture array.
    pub array_size: u32,

    /// Number of mip levels.
    pub mip_levels: u32,

    /// Sample count, for MSAA texture.
    ///
    /// A value of '1' means a regular, non MSAA texture. This value must always be a power of two.
    /// Setting this to a value >1 declares the texture as an MSAA texture.
    pub sample_count: u32,

    /// Sample quality, for MSAA texture
    pub sample_quality: u32,

    /// Enables the texture to be used with unordered access (unordered access view, storage
    /// texture)
    pub allow_unordered_access: bool,

    /// Enables the texture to be used as a face for a cube map
    pub allow_cube_face: bool,

    /// Enables the texture to be used as a render target
    pub is_render_target: bool,
}

impl Default for TextureDesc {
    #[inline]
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth: 1,
            format: TextureFormat::R8Unorm,
            dimension: TextureDimension::Texture2D,
            initial_state: ResourceStates::UNKNOWN,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            allow_unordered_access: false,
            allow_cube_face: false,
            is_render_target: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureSubResourceSet {
    pub base_mip_level: u32,
    pub num_mip_levels: u32,
    pub base_array_slice: u32,
    pub num_array_slices: u32,
}

impl Default for TextureSubResourceSet {
    fn default() -> Self {
        Self {
            base_mip_level: 0,
            num_mip_levels: 1,
            base_array_slice: 0,
            num_array_slices: 1,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureFormat {
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Unorm,
    R16Snorm,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Unorm,
    Rg16Snorm,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb10a2Unorm,
    Rg11b10Float,
    Rg32Uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Unorm,
    Rgba16Snorm,
    Rgba16Float,
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,
    Depth32Float,
    Depth24Stencil8,
}

impl TextureFormat {
    /// Returns whether the format is a depth texture format
    pub fn is_depth(&self) -> bool {
        matches!(self, Self::Depth32Float)
    }

    /// Returns whether the format is a stencil texture format
    pub fn is_stencil(&self) -> bool {
        matches!(self, Self::Depth24Stencil8)
    }

    /// Returns whether the format is a depth/stencil texture format
    pub fn is_depth_stencil(&self) -> bool {
        matches!(self, Self::Depth32Float | Self::Depth24Stencil8)
    }

    /// Returns whether the format is a float format
    pub fn is_float(&self) -> bool {
        matches!(
            self,
            Self::R16Float
                | Self::R32Float
                | Self::Rg16Float
                | Self::Rg11b10Float
                | Self::Rg32Float
                | Self::Rgba16Float
                | Self::Rgba32Float
                | Self::Depth32Float
        )
    }

    /// Returns whether the format is a signed-int format
    pub fn is_sint(&self) -> bool {
        matches!(
            self,
            Self::R8Sint
                | Self::R16Sint
                | Self::Rg8Sint
                | Self::R32Sint
                | Self::Rg16Sint
                | Self::Rgba8Sint
                | Self::Rg32Sint
                | Self::Rgba16Sint
                | Self::Rgba32Sint
        )
    }

    /// Returns whether the format is an unsigned-int format
    pub fn is_uint(&self) -> bool {
        matches!(
            self,
            Self::R8Uint
                | Self::R16Uint
                | Self::Rg8Uint
                | Self::R32Uint
                | Self::Rg16Uint
                | Self::Rgba8Uint
                | Self::Rg32Uint
                | Self::Rgba16Uint
                | Self::Rgba32Uint
        )
    }

    /// Returns whether the format is a signed-normalized-int format
    pub fn is_snorm(&self) -> bool {
        matches!(
            self,
            Self::R8Snorm
                | Self::R16Snorm
                | Self::Rg8Snorm
                | Self::Rg16Snorm
                | Self::Rgba8Snorm
                | Self::Rgba16Snorm
        )
    }

    /// Returns whether the format is an unsigned-normalized-int format
    pub fn is_unorm(&self) -> bool {
        matches!(
            self,
            Self::R8Unorm
                | Self::R16Unorm
                | Self::Rg8Unorm
                | Self::Rg16Unorm
                | Self::Rgba8Unorm
                | Self::Rgba8UnormSrgb
                | Self::Bgra8Unorm
                | Self::Bgra8UnormSrgb
                | Self::Rgb10a2Unorm
                | Self::Rgba16Unorm
        )
    }
}

impl Display for TextureFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureFormat::R8Unorm => f.write_str("TextureFormat::R8Unorm"),
            TextureFormat::R8Snorm => f.write_str("TextureFormat::R8Snorm"),
            TextureFormat::R8Uint => f.write_str("TextureFormat::R8Uint"),
            TextureFormat::R8Sint => f.write_str("TextureFormat::R8Sint"),
            TextureFormat::R16Uint => f.write_str("TextureFormat::R16Uint"),
            TextureFormat::R16Sint => f.write_str("TextureFormat::R16Sint"),
            TextureFormat::R16Unorm => f.write_str("TextureFormat::R16Unorm"),
            TextureFormat::R16Snorm => f.write_str("TextureFormat::R16Snorm"),
            TextureFormat::R16Float => f.write_str("TextureFormat::R16Float"),
            TextureFormat::Rg8Unorm => f.write_str("TextureFormat::Rg8Unorm"),
            TextureFormat::Rg8Snorm => f.write_str("TextureFormat::Rg8Snorm"),
            TextureFormat::Rg8Uint => f.write_str("TextureFormat::Rg8Uint"),
            TextureFormat::Rg8Sint => f.write_str("TextureFormat::Rg8Sint"),
            TextureFormat::R32Uint => f.write_str("TextureFormat::R32Uint"),
            TextureFormat::R32Sint => f.write_str("TextureFormat::R32Sint"),
            TextureFormat::R32Float => f.write_str("TextureFormat::R32Float"),
            TextureFormat::Rg16Uint => f.write_str("TextureFormat::Rg16Uint"),
            TextureFormat::Rg16Sint => f.write_str("TextureFormat::Rg16Sint"),
            TextureFormat::Rg16Unorm => f.write_str("TextureFormat::Rg16Unorm"),
            TextureFormat::Rg16Snorm => f.write_str("TextureFormat::Rg16Snorm"),
            TextureFormat::Rg16Float => f.write_str("TextureFormat::Rg16Float"),
            TextureFormat::Rgba8Unorm => f.write_str("TextureFormat::Rgba8Unorm"),
            TextureFormat::Rgba8UnormSrgb => f.write_str("TextureFormat::Rgba8UnormSrgb"),
            TextureFormat::Rgba8Snorm => f.write_str("TextureFormat::Rgba8Snorm"),
            TextureFormat::Rgba8Uint => f.write_str("TextureFormat::Rgba8Uint"),
            TextureFormat::Rgba8Sint => f.write_str("TextureFormat::Rgba8Sint"),
            TextureFormat::Bgra8Unorm => f.write_str("TextureFormat::Bgra8Unorm"),
            TextureFormat::Bgra8UnormSrgb => f.write_str("TextureFormat::Bgra8UnormSrgb"),
            TextureFormat::Rgb10a2Unorm => f.write_str("TextureFormat::Rgb10a2Unorm"),
            TextureFormat::Rg11b10Float => f.write_str("TextureFormat::Rg11b10Float"),
            TextureFormat::Rg32Uint => f.write_str("TextureFormat::Rg32Uint"),
            TextureFormat::Rg32Sint => f.write_str("TextureFormat::Rg32Sint"),
            TextureFormat::Rg32Float => f.write_str("TextureFormat::Rg32Float"),
            TextureFormat::Rgba16Uint => f.write_str("TextureFormat::Rgba16Uint"),
            TextureFormat::Rgba16Sint => f.write_str("TextureFormat::Rgba16Sint"),
            TextureFormat::Rgba16Unorm => f.write_str("TextureFormat::Rgba16Unorm"),
            TextureFormat::Rgba16Snorm => f.write_str("TextureFormat::Rgba16Snorm"),
            TextureFormat::Rgba16Float => f.write_str("TextureFormat::Rgba16Float"),
            TextureFormat::Rgba32Uint => f.write_str("TextureFormat::Rgba32Uint"),
            TextureFormat::Rgba32Sint => f.write_str("TextureFormat::Rgba32Sint"),
            TextureFormat::Rgba32Float => f.write_str("TextureFormat::Rgba32Float"),
            TextureFormat::Depth32Float => f.write_str("TextureFormat::Depth32Float"),
            TextureFormat::Depth24Stencil8 => f.write_str("TextureFormat::Depth24Stencil8"),
        }
    }
}

/// Enumeration about all major texture types.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureDimension {
    /// One dimensional texture. Logically similar to a 2D image with a height of 1
    Texture1D,

    /// A standard 2D texture.
    Texture2D,

    /// A 3D volume texture.
    Texture3D,
}

/// An enumeration of all possible input types for initializing a texture's optimal clear color
/// value
#[derive(Clone, Debug, PartialEq)]
pub enum OptimalClearValue {
    /// A full 4-channel f32 colour
    ColorF32 { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    ColorInt(u32),

    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),
}

impl From<u32> for OptimalClearValue {
    fn from(v: u32) -> Self {
        Self::ColorInt(v)
    }
}

impl Display for OptimalClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimalClearValue::ColorF32 { r, g, b, a } => {
                write!(f, "OptimalClearValue::ColorF32({}, {}, {}, {})", r, g, b, a)
            }
            OptimalClearValue::ColorInt(v) => {
                write!(f, "OptimalClearValue::ColorInt({:X})", *v)
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                write!(f, "OptimalClearValue::DepthStencil({}, {})", depth, stencil)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TextureCreateError {
    #[error("Requested texture width '{0}' is invalid")]
    InvalidWidth(u32),

    #[error("Requested texture height '{0}' is invalid")]
    InvalidHeight(u32),

    #[error("Requested texture depth '{0}' is invalid")]
    InvalidDepth(u32),

    #[error("Requested texture array size '{0}' is invalid")]
    InvalidArraySize(u32),

    #[error("Requested texture mip level count '{0}' is invalid")]
    InvalidMipLevelCount(u32),

    #[error("Requested sample count '{0}' is invalid")]
    InvalidSampleCount(u32),

    #[error("Requested optimal clear value '{0}' is invalid")]
    InvalidOptimalClearValue(OptimalClearValue),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
