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

use aleph_any::{AnyArc, IAny};
use thiserror::Error;

use crate::*;

pub trait ISwapChain: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(ISwapChain);

    /// Returns whether support operations are supported on the given queue.
    fn present_supported_on_queue(&self, queue: QueueType) -> bool;

    /// Returns a [SwapChainConfiguration] that describes the state of the swap chain at the time
    /// of the function being called.
    ///
    /// The state may change after this function is called. If a rebuild was needed internally in
    /// [ISwapChain::acquire_next_image] then the size may be different once the
    /// [ISwapChain::acquire_next_image] call returns.
    fn get_config(&self) -> SwapChainConfiguration;

    /// Performs a swap chain rebuild operation, recreating the swap images while remaining attached
    /// to the underlying surface. An optional new size hint can be specified to provide resize the
    /// back-buffers.
    ///
    /// This is important and enables several pieces of functionality:
    /// - Firstly, it allows resizing the swap chain images when the size of the surface has
    ///   changed.
    /// - It allows rebuilding the swap images on a fullscreen/windowed transition which is required
    ///   on some platforms to trigger fullscreen optimizations (D3D12)
    /// - It allows a rebuild for cases where it is required, such as when the swap chain has become
    ///   out of date for the associated surface and must be rebuilt before it can be used again.
    ///
    /// # Info
    ///
    /// This function will trigger a full device sync and flush ([IDevice::wait_idle]) in order to
    /// drain the GPU of any in-flight work referencing the swap images. It will also assert that
    /// the user has dropped all references, panicking if the user has failed to meet this
    /// requirement.
    ///
    /// Once a thread has entered [ISwapChain::rebuild] any remaining views in descriptor sets are
    /// considered dangling and are no longer valid to use. None of the swap images can be in-use on
    /// a queue when this function is called. No further work can be queued referencing the old swap
    /// textures the instant any thread enters [ISwapChain::rebuild].
    ///
    /// # Full Sync and Flush
    ///
    /// It is prudent to explain why a full device flush is used here, as this has major performance
    /// implications. We make the decision that forcefully stalling and draining the GPU of work
    /// here is the correct choice for two reasons.
    ///
    /// - Implementation safety and simplicity.
    /// - The performance impact is not important.
    ///
    /// Forcing a full flush means implementations don't have to do any special tracking on the GPU
    /// timeline for GPU resources. They can simply drain the work and expect the caller to not
    /// queue any more work using the old swap textures *after* calling [ISwapChain::rebuild].
    ///
    /// The performance cost for doing this is not important as [ISwapChain::rebuild] will be called
    /// exceedingly rarely in only a few circumstances in any real app, namely:
    /// - Fullscreen transitions
    /// - Window resizing
    ///
    /// These operations are already *very* slow and are irrelevant to the performance of a running
    /// game. The additional cost will not be noticed and the benefit is worth the extra trade.
    fn rebuild(
        &self,
        new_size: Option<Extent2D>,
    ) -> Result<SwapChainConfiguration, SwapChainRebuildError>;

    /// Acquire an image from the swap chain for use with rendering
    ///
    /// # Safety
    ///
    /// TODO: Safety docs
    unsafe fn acquire_next_image(&self) -> Result<AcquiredImage, ImageAcquireError>;
}

/// Represents a handle to an acquired swap chain image.
///
/// A [`ISwapImage`] object is acquired from a swap chain using [`ISwapChain::acquire_next_image`].
/// Once acquired the [`TextureHandle`] for the swap image can be queried, and the swap image can
/// have GPU work associated with it at submission time. Once all work that accesses the swap image
/// has been submitted the swap image's presentation can be queued with [`IQueue::present`].
///
/// There can only be a single [`ISwapImage`] object alive from a single [`ISwapChain`] at any point
/// in time. When the [`ISwapImage`] is queued for presentation ownership of the object _must_ be
/// given back to the API. That is, when calling [`IQueue::present`] you must ensure that given
/// swap image handle is the only handle referring to that object (refcount = 1).
///
/// Ownership of the [`TextureHandle`] must also be returned at presentation time. There can be no
/// handles retained that refer to the swap image.
///
/// # Why
///
/// The [`ISwapImage`] object represents the lifetime of an acquired swap chain image, modelled
/// after its own lifetime. You 'acquire' the image from a swap chain, draw to it and associate
/// GPU work with it, and finally return it to the swap chain for presentation. Internally the RHI
/// will perform whatever synchronization is needed to make this possible.
///
/// Swap chain integration on different APIs is wildly different. Some sync implicitly, some
/// explicitly. Some let you hold on to the texture handles, some require they are fully released.
/// It's a mess. This abstraction simplifies the API surface and efficiently abstracts over the
/// different APIs without trading much flexibility.
pub trait ISwapImage: IAny + IGetPlatformInterface + Send + Sync {
    /// Get the texture handle associated with this [`ISwapImage`].
    ///
    /// All references to this texture must be dropped before calling [`IQueue::present`].
    fn texture(&self) -> &TextureHandle;

    /// Returns a [TextureDesc] that describes the texture this [`ISwapImage`] encapsulates.
    fn texture_desc(&self) -> &TextureDesc<'_>;
}

/// Wrapper enum that flags the different success conditions for [`ISwapChain::acquire_next_image`].
pub enum AcquiredImage {
    /// The image was acquired fully with no issues.
    Ok(AnyArc<dyn ISwapImage>),

    /// This 'error' is a soft failure case for [ISwapChain::acquire_next_image]. In some cases it
    /// is possible for the swapchain to be placed in a state where it does not fully match the
    /// underlying surface being rendered too. For example, when the window is resized but the
    /// surface isn't lost. This can happen on composited platforms where they stretch/squash the
    /// swap images into the real surface.
    ///
    /// This is not a hard error, and it is perfectly valid to continue using and presenting to a
    /// sub-optimal swapchain. It is, however, recommended that the swapchain be rebuilt to
    /// correctly match the underlying surface again. This variant flags the sub-optimal case
    /// for the caller to handle.
    SubOptimal(AnyArc<dyn ISwapImage>),
}

impl AcquiredImage {
    /// Discard the [`AcquiredImage`] container and get the image inside.
    #[inline]
    pub fn get(self) -> AnyArc<dyn ISwapImage> {
        match self {
            AcquiredImage::Ok(v) => v,
            AcquiredImage::SubOptimal(v) => v,
        }
    }

    /// Returns if the swap chain is in the 'sub optimal' state. See [`AcquiredImage::SubOptimal`]
    /// for more info.
    pub const fn is_sub_optimal(&self) -> bool {
        matches!(self, AcquiredImage::SubOptimal(_))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PresentationMode {
    Immediate,
    Mailbox,
    Fifo,
}

impl Default for PresentationMode {
    #[inline(always)]
    fn default() -> Self {
        Self::Immediate
    }
}

impl std::fmt::Display for PresentationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PresentationMode::Immediate => f.write_str("Immediate"),
            PresentationMode::Mailbox => f.write_str("Mailbox"),
            PresentationMode::Fifo => f.write_str("Fifo"),
        }
    }
}

/// Specifies the preferred values for a swap chain during creation (when used in
/// [ISurface::create_swap_chain]) or the actual current configuration of the swap chain (when
/// queried with [ISwapChain::get_config]).
///
/// In the creation context, some of these values only specify *preferences* rather than
/// requirements. Specifically:
/// - `width`
/// - `height`
/// - `presentation_mode`
/// - `buffer_count`
/// - `present_queue`
///
/// All of these have complex feature matrices that only a mother could love. Especially on Vulkan.
/// We take the opinionated approach that sane fallbacks should be used in place of front-loading it
/// all on the user.
///
/// This means, in the context of [ISurface::create_swap_chain], the fields in the above list are
/// treated as *hints* rather than *requirements*. The actual configuration is allowed to differ
/// from the request. This allows the implementation to use fallbacks rather than leaving the user
/// to decide with heuristics, and avoids pessimizing platforms that don't have this problem
/// (*cough* D3D12 *cough*).
///
/// In the context of [ISwapChain::get_config] then all fields represent the actual state of the
/// swap chain *at the time it was queried*. This state can (and will) change between calls to
/// [ISwapChain::rebuild].
///
/// Specific cases where the preferences are ignored include:
/// - Windows Vulkan can only use `width` and `height` exactly equal to the window dimensions so it
///   can't meet arbitrary width or height requests.
/// - Not all Vulkan implementations support all present modes so the next closest fallback must be
///   used.
/// - Support for a given buffer count varies, including between presentation modes. Only some
///   combinations are sane (mailbox with anything other than 3 buffers is pointless)
/// - The queue a swap chain can be presented to from is device dependent. We do at least guarantee
///   that you can present from general queues.
///
/// In summary, Vulkan swap chains are a pain and we can't hide it. Sane fallbacks make it a lot
/// more elegant though.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct SwapChainConfiguration {
    /// The texture format of the swap chain images.
    pub format: Format,

    /// The width of the swap chain, in pixels.
    pub width: u32,

    /// The height of the swap chain, in pixels.
    pub height: u32,

    /// The presentation mode of the swap chain.
    pub present_mode: PresentationMode,

    /// The number of back buffers in the swap chain. Valid range 2..=3.
    pub buffer_count: u32,

    /// The queue that can queue present operations for this swap chain.
    pub present_queue: QueueType,
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SwapChainCreateError {
    #[error("The requested image format '{0}' is not supported by the swap chain")]
    UnsupportedFormat(Format),

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

    /// For a detailed explanation see [ImageAcquireError::SurfaceNotAvailable]
    #[error("The surface is currently in a state where it can not be used")]
    SurfaceNotAvailable,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(SwapChainCreateError);

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SwapChainRebuildError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(SwapChainRebuildError);

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ImageAcquireError {
    /// This error occurs when the underlying surface has changed state in a way that the swap-chain
    /// object is no longer compatible with and the swap-chain needs to be rebuilt to represent the
    /// new state. It is impossible to hand out swap-chain images in this case and so the caller
    /// must rebuild the swap-chain before images can be acquired again.
    #[error("The swap chain is out of date and needs to be rebuilt")]
    OutOfDate,

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

    /// This error occurs when the surface backing a swap chain has become permanently lost to the
    /// RHI and can no longer be used. The swap chain, and the surface it was created from, are now
    /// 'dead' and must not be accessed.
    #[error("The surface has been permanently lost")]
    SurfaceLost,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(ImageAcquireError);
