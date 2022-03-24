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

use crate::gpu::ColorRGBA;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// The set of adapter power classes. Primarily used as part of requesting an adapter from the
/// [IContext].
#[derive(Copy, Clone, Debug)]
pub enum AdapterPowerClass {
    /// A low-power adapter refers to the most power efficient GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, low-power refers to the integrated
    /// GPU as it will almost certainly use less power than the discrete GPU.
    LowPower,

    /// A high-power adapter refers to the highest performance GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, high-power refers to the discrete GPU
    /// as it will almost certainly be faster than the integrated GPU (otherwise why would it be
    /// installed in the system?).
    HighPower,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum QueueType {
    General,
    Compute,
    Transfer,
}

impl Display for QueueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueType::General => f.write_str("QueueType::General"),
            QueueType::Compute => f.write_str("QueueType::Compute"),
            QueueType::Transfer => f.write_str("QueueType::Transfer"),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PresentationMode {
    Immediate,
    Mailbox,
    Fifo,
}

impl Display for PresentationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationMode::Immediate => f.write_str("PresentationMode::Immediate"),
            PresentationMode::Mailbox => f.write_str("PresentationMode::Mailbox"),
            PresentationMode::Fifo => f.write_str("PresentationMode::Fifo"),
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

/// Set of errors that can occur when creating an [IContext]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ContextCreateError {
    #[error("A context has already been created by this provider")]
    ContextAlreadyCreated,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

/// Set of errors that can occur when creating an [IDevice]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

/// Set of errors that can occur when creating an [ISurface]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SurfaceCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SwapChainCreateError {
    #[error("The requested image format '{0}' is not supported by the swap chain")]
    UnsupportedFormat(TextureFormat),

    #[error("The requested image usage is not supported by the swap chain")]
    UnsupportedUsage(()),

    #[error("The requested width '{0}' is not supported by the swap chain")]
    UnsupportedWidth(u32),

    #[error("The requested height '{0}' is not supported by the swap chain")]
    UnsupportedHeight(u32),

    #[error("The requested presentation mode '{0}' is not supported by the swap chain")]
    UnsupportedPresentMode(PresentationMode),

    #[error("There is no queue available for the swap chain to be attached to")]
    NoQueueAvailable,

    #[error("The surface is already owned by another existing swap chain")]
    SurfaceAlreadyOwned,

    /// For a detailed explanation see [AcquireImageError::SurfaceNotAvailable]
    #[error("The surface is currently in a state where it can not be used")]
    SurfaceNotAvailable,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AcquireImageError {
    #[error("The swap chain is already acquired")]
    ImageNotAvailable,

    ///
    /// This error is subtle and requires explanation.
    ///
    /// SurfaceNotAvailable will be returned when it is not possible for the backend to create the
    /// underlying swap chain object for the surface at the present time. This is not a failure, the
    /// surface can return to a valid state.
    ///
    /// This is primarily an issue on Vulkan under Windows. On Windows, when a window is minimized
    /// the vkGetPhysicalDeviceSurfaceCapabilitiesKHR call will return a current_extent of (0, 0).
    /// As per the Vulkan spec if current_extent is specified as anything other than
    /// (U32_MAX, U32_MAX) then you must use exactly current_extent when creating the swap chain.
    /// (0, 0) is an invalid value to pass so a minimized window can't have a swap chain attached
    /// to it.
    ///
    /// If the window is minimized then it is impossible to create a swap chain, making it
    /// impossible to hand out images.
    ///
    #[error("The surface is currently in a state where it can not be used")]
    SurfaceNotAvailable,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ShaderCreateError {
    /// This error occurs when the byte size of the shader blob is of an invalid size.
    ///
    /// Invalid sizes include:
    ///     - 0
    ///     - Non multiples of 4 (on Vulkan)
    ///
    /// # Vulkan
    ///
    /// Vulkan consumes SPIR-V as the shader blob. SPIR-V is encoded as a sequence of `u32` values.
    /// It is impossible for a valid SPIR-V binary to have a size that is not a multiple of 4 (the
    /// size of a u32) for this reason.
    #[error("The shader binary size '{0}' is invalid")]
    InvalidInputSize(usize),

    /// This error occurs when a shader binary is provided in a format not supported by the active
    /// backend.
    ///
    /// The `Vulkan` backend can only accept SPIR-V shaders, while the `D3D12` backend can only
    /// accept DXIL shaders.
    #[error("The shader binary is of unsupported format")]
    UnsupportedShaderFormat,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum BufferCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
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

impl From<ColorRGBA> for OptimalClearValue {
    fn from(v: ColorRGBA) -> Self {
        Self::ColorF32 {
            r: v.r,
            g: v.g,
            b: v.b,
            a: v.a,
        }
    }
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

/// An enumeration of all possible input types to a color texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum ColorClearValue {
    /// A full 4-channel f32 colour
    Float { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    Int(u32),
}

impl From<ColorRGBA> for ColorClearValue {
    fn from(v: ColorRGBA) -> Self {
        Self::Float {
            r: v.r,
            g: v.g,
            b: v.b,
            a: v.a,
        }
    }
}

impl From<u32> for ColorClearValue {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl Display for ColorClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorClearValue::Float { r, g, b, a } => {
                write!(f, "ColorClearValue::Float({}, {}, {}, {})", r, g, b, a)
            }
            ColorClearValue::Int(v) => {
                write!(f, "ColorClearValue::Int({:X})", *v)
            }
        }
    }
}

/// An enumeration of all possible input types to a depth/stencil texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum DepthStencilClearValue {
    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),

    /// A floating point value for clearing only depth
    Depth(f32),

    /// A u8 value for clearing only stencil
    Stencil(u8),
}

impl Display for DepthStencilClearValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DepthStencilClearValue::DepthStencil(depth, stencil) => {
                write!(f, "ColorClearValue::Float({}, {})", *depth, *stencil)
            }
            DepthStencilClearValue::Depth(v) => {
                write!(f, "DepthStencilClearValue::Depth({})", *v)
            }
            DepthStencilClearValue::Stencil(v) => {
                write!(f, "DepthStencilClearValue::Stencil({})", *v)
            }
        }
    }
}

/// An enumeration of all individual shader types
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

/// An enumeration of the supported set of shader input types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderBinary<'a> {
    /// This variant encloses a SPIR-V binary. Only supported by the `Vulkan` backend.
    Spirv(&'a [u8]),

    /// This variant encloses a DXIL binary. Only supported by the `D3D12` backend.
    Dxil(&'a [u8]),
}

/// Enumeration of all available backends.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BackendAPI {
    Vulkan,
    D3D12,
}

/// Enumeration of all CPU access modes for resources
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CpuAccessMode {
    /// Resource can not be accessed by the CPU at all (device local)
    None,

    /// Resource can be read by the CPU (read back)
    Read,

    /// Resource can be written by the CPU (upload)
    Write,
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

#[derive(Error, Debug)]
pub enum CommandPoolCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandListCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandListBeginError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandListSubmitError {
    #[error("The queue '{0}' is not available")]
    QueueNotAvailable(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
