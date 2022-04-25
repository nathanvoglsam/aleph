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

use crate::gpu::{IAcquiredTexture, INamedObject, QueueType, TextureFormat};
use any::{AnyArc, IAny};
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub trait ISwapChain: INamedObject + IAny + 'static {
    any_arc_trait_utils_decl!(ISwapChain);

    /// Returns whether support operations are supported on the given queue.
    fn present_supported_on_queue(&self, queue: QueueType) -> bool;

    /// Force a resize of the swap chain. Will block until the swap chain is no longer in use before
    /// performing the resize operation.
    fn queue_resize(&self, width: u32, height: u32);

    /// Returns a [SwapChainConfiguration] that describes the state of the swap chain at the time
    /// of the function being called.
    ///
    /// The state may change after this function is called. If a rebuild was needed internally in
    /// [ISwapChain::acquire_image] then the size may be different once the
    /// [ISwapChain::acquire_image] call returns.
    fn get_config(&self) -> SwapChainConfiguration;

    /// Acquire an image from the swap chain for use with rendering
    fn acquire_image(&self) -> Result<Box<dyn IAcquiredTexture>, AcquireImageError>;
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SwapChainConfiguration {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: PresentationMode,
    pub preferred_queue: QueueType,
}

impl Default for SwapChainConfiguration {
    #[inline]
    fn default() -> Self {
        Self {
            format: TextureFormat::Bgra8UnormSrgb,
            width: 0,
            height: 0,
            present_mode: PresentationMode::Fifo,
            preferred_queue: QueueType::General,
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
    ///
    /// This error occurs when a queued resize operation was attempted to be resolved before
    /// acquiring and returning an image handle, but the resize operation could not complete.
    ///
    /// This does not flag when the actual GAPI calls for resizing or recreating the swap chain
    /// fails, rather this failure occurs when the wrapper API requirements for resize operations
    /// are not met and the resize could not be completed.
    ///
    /// A resize operation can only occur if there are no swap textures in use on the GPU and there
    /// are no images acquired by the API consumer. When resizing the GPU queues will be flushed so
    /// it is easy to ensure the first condition by managing your image acquires.
    ///
    /// It is the caller's job to ensure it is possible for the resize operation to complete.
    ///
    #[error("A resize operation that was queued failed to complete")]
    QueuedResizeFailed,

    ///
    /// This error occurs when the swap image has already been acquired and an API consumer attempts
    /// to acquire the image again.
    ///
    /// It is the caller's job to manage image acquisitions to avoid triggering this.
    ///
    #[error("No swap chain images are available to acquire")]
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
