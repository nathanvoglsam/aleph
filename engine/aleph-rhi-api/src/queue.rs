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

use std::cell::Cell;

use aleph_any::{AnyArc, IAny};
use thiserror::Error;

use crate::*;

pub trait IQueue: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IQueue);

    /// Returns the set of per-queue properties associated with this queue.
    fn queue_properties(&self) -> QueueProperties;

    /// Triggers a garbage collection cycle. This will walk the list of known in-flight command
    /// lists and release any that are now fully retired on the queue. Any resources that the
    /// command list is extending the lifetime for will also have their reference count decremented.
    ///
    /// This is expected to be called once per-frame. This provides a well-known API that
    /// encapsulates the CPU work associated with collecting and releasing in-flight resources.
    ///
    /// It is possible, and encouraged, to call and punt this onto a task thread. Each queue can be
    /// collected on separate threads, spreading the work across multiple cores. The calls are
    /// non-blocking and thread-safe. They could trivially be handled as fire-and-forget rayon
    /// tasks, for example.
    ///
    /// Triggers a non blocking garbage collection cycle. This must be called for resources used in
    /// command lists to be freed. It is recommended to call this at least once per frame.
    ///
    /// # Warning
    ///
    /// Not calling this function *will* cause problems. RHI implementations may (and *do*) use
    /// fixed-sized buffers for tracking in-flight work. Failing to call this function means you
    /// will overflow the internal buffers after a few frames of queue submissions and panic, or
    /// just leak memory.
    fn garbage_collect(&self);

    /// Block the calling thread until the queue is flushed of work. This is similar to
    /// vkQueueWaitIdle.
    fn wait_idle(&self);

    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the command lists submitted to the GPU
    /// contain a valid command stream.
    ///
    unsafe fn submit(&self, desc: &QueueSubmitDesc) -> Result<(), QueueSubmitError>;

    ///
    /// Enqueues a 'present' operation onto the queue for the given [ISwapChain].
    ///
    /// The image to be presented is the most recently acquired image from the swap chain.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the image that is being presented will be
    /// in the required resource state for presentation by the time this operation will be executed
    /// on the GPU timeline.
    ///
    unsafe fn present(&self, swap_image: AnyArc<dyn ISwapImage>) -> Result<(), QueuePresentError>;
}

#[derive(Clone, Debug)]
pub struct QueueProperties {
    /// The minimum offset alignment and smallest extent supported for image transfer operations.
    ///
    /// This effects the supported offset and extents for image transfer operations. The values
    /// (x, y, z) specify the minimum extent on the corresponding axis that is supported on the
    /// associated queue. The (x, y, z) values also specify the alignment for the offset values on
    /// the corresponding axis.
    ///
    /// Each extent axis must be a multiple of the corresponding value, unless the extent would copy
    /// beyond the bounds of the image. In this case the extent can be clamped so the region doesn't
    /// access outside of the image.
    ///
    /// The special case (0, 0, 0) value denotes that there is no granularity restriction and any
    /// offset and extent can be used (other rules still withstanding).
    ///
    /// # Details
    ///
    /// This directly maps to the Vulkan `minImageTransferGranularity` queue property. D3D12 has
    /// no such concept and so will always report (0, 0, 0). For more specific documentation see
    /// the Vulkan documentation for `VkQueueFamilyProperties`.
    pub min_image_transfer_granularity: Extent3D,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum QueueType {
    General,
    Compute,
    Transfer,
}

impl Default for QueueType {
    #[inline(always)]
    fn default() -> Self {
        Self::General
    }
}

impl std::fmt::Display for QueueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueType::General => f.write_str("General"),
            QueueType::Compute => f.write_str("Compute"),
            QueueType::Transfer => f.write_str("Transfer"),
        }
    }
}

#[derive(Clone)]
pub struct QueueSubmitDesc<'a> {
    /// A list of the command lists that are to be submitted in this batch
    pub command_lists: &'a [Cell<Option<Box<dyn ICommandList>>>],

    /// A list of semaphores that will block the execution of the batch until all semaphores in the
    /// list are signaled.
    pub wait_semaphores: &'a [&'a SemaphoreHandle],

    /// A list of semaphores that will be signaled once all command lists in the batch have
    /// completed executing.
    pub signal_semaphores: &'a [&'a SemaphoreHandle],

    /// A fence that will be signaled once all command lists in the batch have completed executing.
    pub fence: Option<&'a FenceHandle>,

    /// The acquired swap chain image to associate this queue submission with.
    ///
    /// This work submission will synchronize with the swap chain that the given image is acquired
    /// from. The eventual present operation for the swap image will also synchronize against all
    /// the command lists in this submission.
    pub swap_image: Option<&'a dyn ISwapImage>,
}

impl<'a> QueueSubmitDesc<'a> {
    /// Constructs a new, empty [QueueSubmitDesc]
    pub const fn new() -> Self {
        Self {
            command_lists: &[],
            wait_semaphores: &[],
            signal_semaphores: &[],
            fence: None,
            swap_image: None,
        }
    }

    /// Takes the given desc and returns it with [QueueSubmitDesc::command_lists] set to the given
    /// parameter
    pub const fn with_lists(
        mut self,
        command_lists: &'a [Cell<Option<Box<dyn ICommandList>>>],
    ) -> Self {
        self.command_lists = command_lists;
        self
    }

    /// Takes the given desc and returns it with [QueueSubmitDesc::wait_semaphores] set to the given
    /// parameter
    pub const fn with_wait_semaphores(
        mut self,
        wait_semaphores: &'a [&'a SemaphoreHandle],
    ) -> Self {
        self.wait_semaphores = wait_semaphores;
        self
    }

    /// Takes the given desc and returns it with [QueueSubmitDesc::signal_semaphores] set to the
    /// given parameter
    pub const fn with_signal_semaphores(
        mut self,
        signal_semaphores: &'a [&'a SemaphoreHandle],
    ) -> Self {
        self.signal_semaphores = signal_semaphores;
        self
    }

    /// Takes the given desc and returns it with [QueueSubmitDesc::fence] set to the given parameter
    pub const fn with_fence(mut self, fence: &'a FenceHandle) -> Self {
        self.fence = Some(fence);
        self
    }

    /// Takes the given desc and returns it with [QueueSubmitDesc::swap_image] set to the given
    /// parameter
    pub const fn with_swap_image(mut self, swap_image: &'a dyn ISwapImage) -> Self {
        self.swap_image = Some(swap_image);
        self
    }
}

impl<'a> Default for QueueSubmitDesc<'a> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum QueueSubmitError {
    #[error("A command list in the submission was not in the correct state for submission.")]
    InvalidCommandListState,

    #[error("The queue does not support submitting '{0}' commands")]
    InvalidEncoderType(QueueType),

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(QueueSubmitError);

#[derive(Error, Debug)]
pub enum QueuePresentError {
    #[error("The queue '{0}' does not support presentation to the requested swap chain")]
    QueuePresentationNotSupported(QueueType),

    /// This 'error' is a soft failure case for [IQueue::present]. In some cases it is possible for
    /// the swapchain to be placed in a state where it does not fully match the underlying surface
    /// being rendered to. For example, when the window is resized but the surface isn't lost. This
    /// can happen on composited platforms where they stretch/squash the swap images into the real
    /// surface.
    ///
    /// This is not a hard error, and it is perfectly valid to continue using and presenting to a
    /// sub-optimal swapchain. It is, however, recommended that the swapchain be rebuilt to
    /// correctly match the underlying surface again. This error variant flags the sub-optimal case
    /// for the caller to handle.
    #[error("The swapchain is sub-optimal for the surface and should be rebuilt")]
    SubOptimal,

    #[error("The swap chain is out of date and needs to be rebuilt")]
    OutOfDate,

    /// This error occurs when the surface backing a swap chain has become permanently lost to the
    /// RHI and can no longer be used. The swap chain, and the surface it was created from, are now
    /// 'dead' and must not be accessed.
    #[error("The surface has been permanently lost")]
    SurfaceLost,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(QueuePresentError);
