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
