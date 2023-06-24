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
use bitflags::bitflags;
use raw_window_handle::HasRawWindowHandle;
use std::any::TypeId;
use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::ptr::NonNull;
use thiserror::Error;

pub const API_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const API_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const API_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

//
// =================================================================================================
// UTILITY MACROS
// =================================================================================================
//

macro_rules! any_arc_trait_utils_decl {
    ($x: path) => {
        /// Returns an `AnyArc` that points to `self`. This is similar to upgrading a weak
        /// reference. We take a non-owning reference `&dyn SomeTrait` and upgrade it to an owning
        /// `AnyArc<dyn SomeTrait>` handle.
        fn upgrade(&self) -> AnyArc<dyn $x>;

        /// Returns the number of strong references to the object.
        ///
        /// A strong reference is an owning handle to the object (`AnyArc`). The object will remain
        /// alive as long as this remains > 0. The object will be dropped when this reaches 0.
        ///
        /// It is only possible to observe a 0 value for `strong_count` through an `AnyWeak`.
        fn strong_count(&self) -> usize;

        /// Returns the number of weak references to the object.
        ///
        /// A weak reference is a non-owning handle to the object (`AnyWeak`). Weak references do
        /// not extend the lifetime of the object itself, only the ref-count block and the memory
        /// allocation that backs it.
        ///
        /// If `strong_count` is 0 and `weak_count` is >0 then the object is no longer accessible as
        /// it will have been dropped.
        ///
        /// It is only possible to observe a 0 value for `weak_count` through an `AnyArc`.
        fn weak_count(&self) -> usize;
    };
}

//
// =================================================================================================
// INTERFACES
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// Misc

///
/// A trait exposed by API objects that allows querying platform specific objects and interfaces
/// for accessing details specific to underlying implementations.
///
/// This allows for intentionally 'leaking' backend objects.
///
pub trait IGetPlatformInterface {
    /// The dynamic interface for looking up an interface/object by type-id. Writes the resulting
    /// object into a prepared place im memory though the provided 'out' pointer. 'out' must provide
    /// valid storage for an object of the requested type.
    ///
    /// It is not recommended to use this interface directly. Instead use this via the
    /// [GetPlatformInterface::query_platform_interface] wrapper.
    ///
    /// # Safety
    ///
    /// The caller has a responsibility to ensure that 'out' points to a valid region of memory that
    /// is of sufficient size, alignment and ownership to initialize a new object of the requested
    /// type into. There is not type safety in this interface, all responsibility lies on the caller
    /// to ensure this.
    ///
    /// The implementation is required to, if the target can be provided, clone or construct a new
    /// object of the expected type at the address given by 'out' and return Some to signify
    /// success. If the object of the requested type *can not be* provided then 'out' must remain
    /// untouched and None must be returned.
    ///
    /// These requirements are important as they allow implementing the interface without any heap
    /// allocations while keeping the [IGetPlatformInterface] trait-object safe. We can't use
    /// generics in object-safe traits so we must do this instead.
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()>;
}

///
/// A wrapper over [IGetPlatformInterface] that provides a type safe interface for using
/// `__query_platform_interface`.
///
pub trait GetPlatformInterface {
    /// A type-safe wrapper over [IGetPlatformInterface::__query_platform_interface] that
    /// automatically handles sending the correct type-id to the dynamic interface and casting back
    /// to the requested type.
    fn query_platform_interface<T: Sized + 'static>(&self) -> Option<T>;
}

impl<T: IGetPlatformInterface + ?Sized> GetPlatformInterface for T {
    #[inline]
    fn query_platform_interface<R: Sized + 'static>(&self) -> Option<R> {
        let mut stack_slot: MaybeUninit<R> = MaybeUninit::uninit();

        // Safety: It is our responsibility to ensure 'out' points to a valid memory region for an
        //         object of type R. We do that via 'stack_slot.
        //
        //         The caller is expected to initialize 'stack_slot' if it has returned 'Some' so
        //         it is safe for us to assume_init in that case.
        unsafe {
            if self
                .__query_platform_interface(TypeId::of::<R>(), stack_slot.as_mut_ptr() as *mut ())
                .is_some()
            {
                Some(MaybeUninit::assume_init(stack_slot))
            } else {
                None
            }
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Context

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving.
pub trait IContext: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IContext);

    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

//
//
// _________________________________________________________________________________________________
// Surface

/// Represents the graphics API's handle to the window or monitor surface. SwapChains are created
/// from surfaces.
///
/// A surface is not tied to a specific [IDevice], it represents an API level handle to a rendering
/// surface. As such [ISurface] is not created by an [IDevice], rather it is created by the
/// [IContext]. An [IDevice] will be selected and created based on its compatibility with an
/// [ISurface].
pub trait ISurface: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(ISurface);

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError>;
}

//
//
// _________________________________________________________________________________________________
// Adapter

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IAdapter);

    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&self) -> AdapterDescription;

    /// Requests an IDevice
    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError>;
}

//
//
// _________________________________________________________________________________________________
// Device

pub trait IDevice: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IDevice);

    /// Triggers a garbage collection cycle across all queues in a single function call. For more
    /// information, see [IQueue::garbage_collect].
    ///
    /// This is simply a utility function that calls the matching function for all available queues.
    ///
    /// # Warning
    ///
    /// It is *not* recommended to use this in a real app as trivial parallelization is left on the
    /// table with this interface. Call [IQueue::garbage_collect] for each queue individually on
    /// separate threads without blocking, the work is completely asynchronous.
    fn garbage_collect(&self);

    /// A utility that will wait for all GPU queues to be idle. For more information, see
    /// [IQueue::wait_idle].
    ///
    /// This is just a utility function that functions as if calling [IQueue::wait_idle] for all
    /// available queues individually.
    fn wait_idle(&self);

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<AnyArc<dyn IGraphicsPipeline>, GraphicsPipelineCreateError>;

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<AnyArc<dyn IComputePipeline>, ComputePipelineCreateError>;

    fn create_shader(
        &self,
        options: &ShaderOptions,
    ) -> Result<AnyArc<dyn IShader>, ShaderCreateError>;

    fn create_descriptor_set_layout(
        &self,
        desc: &DescriptorSetLayoutDesc,
    ) -> Result<AnyArc<dyn IDescriptorSetLayout>, DescriptorSetLayoutCreateError>;

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError>;

    fn create_pipeline_layout(
        &self,
        desc: &PipelineLayoutDesc,
    ) -> Result<AnyArc<dyn IPipelineLayout>, PipelineLayoutCreateError>;

    fn create_buffer(&self, desc: &BufferDesc) -> Result<AnyArc<dyn IBuffer>, BufferCreateError>;

    fn create_texture(
        &self,
        desc: &TextureDesc,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError>;

    fn create_sampler(
        &self,
        desc: &SamplerDesc,
    ) -> Result<AnyArc<dyn ISampler>, SamplerCreateError>;

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError>;

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>>;

    /// Perform the given list of descriptor updates.
    ///
    /// # Safety
    ///
    /// Accesses to the descriptor sets referenced via [DescriptorSetHandle] are not synchronized.
    /// A descriptor write requires mutable (exclusive) access to the individual set. It is unsafe
    /// to call this function on the same [DescriptorSetHandle] from multiple threads without
    /// external synchronization.
    ///
    /// It is unsafe to try and write to a [DescriptorSetHandle] after it has been freed.
    ///
    /// # Warning
    ///
    /// Some implementations may re-use handles, where allocating a new set may return a previously
    /// freed set using the same handle. The implication is that use-after free will not cause
    /// immediate UB or validation errors on the platform API in some cases due to implementation
    /// detail. Instead you will observe 'spooky action at a distance' where two systems think they
    /// own the set, when instead they're sharing one, and they clobber each other's descriptors or
    /// have synchronization issues if they're being shared across threads.
    ///
    /// The take-away here is to be very careful with descriptor sets, buggy usage will be very hard
    /// to debug. Test with as many implementations as you can, especially Vulkan. Most of our
    /// descriptor API is based on Vulkan as it's the 'lowest common denominator', and can be
    /// implemented as thin wrappers to Vulkan. This is useful, being a thin wrapper to Vulkan means
    /// Vulkan's validation layers will also validate our own API if we mirror their semantics as
    /// close as we can.
    ///
    /// D3D12 will be very permissive to errors as D3D12's descriptor model is much less
    /// restrictive.
    unsafe fn update_descriptor_sets(&self, writes: &[DescriptorWriteDesc]);

    /// Constructs a new fence in the default (reset) state.
    fn create_fence(&self) -> Result<AnyArc<dyn IFence>, FenceCreateError>;

    /// Constructs a new semaphore in the default (reset) state.
    fn create_semaphore(&self) -> Result<AnyArc<dyn ISemaphore>, SemaphoreCreateError>;

    /// This function will block the calling thread until the fences are signalled by an operation
    /// some GPU queue.
    ///
    /// - `wait_all` controls whether the call should block until all the fences in the set are
    ///   signalled. if `wait_all` is `false` then the [IDevice::wait_fences] call will return when
    ///   any of the given fences is signaled.
    ///
    /// - `timeout` specifies how long to wait, in milliseconds, before timing out and returning
    ///   from the function. If the timeout time is reached before the wait condition is met then
    ///   the function will return [FenceWaitResult::Timeout]. If `timeout` is equal to `u32::MAX`
    ///   the wait_fences call will block indefinitely and can not timeout.
    ///
    /// # Info
    ///
    /// If the fences are never signalled this function will deadlock
    fn wait_fences(&self, fences: &[&dyn IFence], wait_all: bool, timeout: u32) -> FenceWaitResult;

    /// Polls, and returns, whether the fence has been signalled by the device.
    fn poll_fence(&self, fence: &dyn IFence) -> bool;

    /// Resets all the given fences to the default state, ready to be used again on a queue.
    fn reset_fences(&self, fences: &[&dyn IFence]);

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

//
//
// _________________________________________________________________________________________________
// Semaphore

pub trait ISemaphore: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(ISemaphore);
}

//
//
// _________________________________________________________________________________________________
// Fence

pub trait IFence: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IFence);
}

//
//
// _________________________________________________________________________________________________
// Queue

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
    /// Returns whether the swap chain is currently sub-optimal for the surface being presented to.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the image that is being presented will be
    /// in the required resource state for presentation by the time this operation will be executed
    /// on the GPU timeline.
    ///
    unsafe fn present(&self, desc: &QueuePresentDesc) -> Result<bool, QueuePresentError>;
}

/// Optional extension to [IQueue] that provides various debug utilities, like setting debug markers
/// and events that can be seen inside graphics debuggers.
///
/// This interface must be queried from a queue, and may not always be present. The features needed
/// to implement this interface will only be present on developer machines, so this interface can't
/// be guaranteed to be available. Especially not on client devices.
pub trait IQueueDebug: IQueue {
    ///
    /// Emits an instantaneous 'marker' on this queue, with the given message and message color.
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    fn set_marker(&self, color: Color, message: &str);

    ///
    /// Marks the beginning of a new event on this queue, with the given message and message color.
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    fn begin_event(&self, color: Color, message: &str);

    ///
    /// Marks the end of an event on this queue.
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    fn end_event(&self);
}

//
//
// _________________________________________________________________________________________________
// SwapChain

pub trait ISwapChain: IAny + IGetPlatformInterface {
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

    /// Acquires handles to the swap chain textures and writes them into the given array.
    ///
    /// # Info
    ///
    /// If `images.len()` is > than the number of swap chain images the out-of-range array elements
    /// will be left unchanged.
    ///
    /// If `images.len()` is < than the number of swap chain images then only the first
    /// `images.len()` swap chain images will be returned.
    fn get_images(&self, images: &mut [Option<AnyArc<dyn ITexture>>]);

    /// Acquire an image from the swap chain for use with rendering
    ///
    /// # Safety
    ///
    /// TODO: Safety docs
    unsafe fn acquire_next_image(&self, desc: &AcquireDesc) -> Result<u32, ImageAcquireError>;
}

//
//
// _________________________________________________________________________________________________
// Resources

pub trait IBuffer: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IBuffer);

    /// Returns a [BufferDesc] that describes this [IBuffer]
    fn desc(&self) -> BufferDesc;

    /// Returns a host virtual address pointer to a region of a mappable buffer.
    ///
    /// [IBuffer::map] will map the entire buffer.
    ///
    /// Writes to buffer memory through a mapped pointer won't become available to the device until
    /// after a submission to an [IQueue], or when signalling an event/fence to the GPU. The writes
    /// will only be made available to the device commands when submitted, or when waiting for an
    /// event/fence to be triggered from the CPU.
    fn map(&self) -> Result<NonNull<u8>, ResourceMapError>;

    /// Unmaps the buffers memory, releasing the associated address space range to be reused.
    fn unmap(&self);

    /// Flushes any writes to mapped buffer memory for non `HOST_COHERENT` memory.
    ///
    /// Writes to non `HOST_COHERENT` memory will no be made available to the device until the
    /// written range is flushed with this function.
    ///
    /// This should be combined with an event/fence for writes from the host to become available
    ///
    /// Mapped memory that is considered `HOST_COHERENT` does not need to be flushed.
    fn flush_range(&self, offset: u64, len: u64);

    /// Invalidate the requested region inside the mapped buffer memory for non `HOST_COHERENT`
    /// memory.
    ///
    /// Device writes to non `HOST_COHERENT` mapped memory will not be available to the host until
    /// this function is called for the region to be read.
    ///
    /// Mapped memory that is considered `HOST_COHERENT` does not need to be invalidated.
    fn invalidate_range(&self, offset: u64, len: u64);
}

pub trait ITexture: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(ITexture);

    /// Returns a [TextureDesc] that describes this [ITexture]
    fn desc(&self) -> TextureDesc;

    fn get_view(&self, desc: &ImageViewDesc) -> Result<ImageView, ()>;

    fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()>;

    fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()>;
}

pub trait ISampler: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(ISampler);

    /// Returns a [SamplerDesc] that describes this [ISampler]
    fn desc(&self) -> SamplerDesc;
}

//
//
// _________________________________________________________________________________________________
// Command Encoders

pub trait IGeneralEncoder: IComputeEncoder {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline);

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    );

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    );

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]);

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]);

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]);

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo);

    unsafe fn end_rendering(&mut self);

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    );
}

pub trait IComputeEncoder: ITransferEncoder {
    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &dyn IPipelineLayout,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
    );

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}

pub trait ITransferEncoder: IGetPlatformInterface + Send {
    unsafe fn resource_barrier(
        &mut self,
        memory_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    );

    unsafe fn copy_buffer_regions(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn IBuffer,
        regions: &[BufferCopyRegion],
    );

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn ITexture,
        dst_layout: ImageLayout,
        regions: &[BufferToTextureCopyRegion],
    );

    ///
    /// Emits an instantaneous 'marker' on this command list, with the given message and message
    /// color.
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    ///
    /// # Safety
    ///
    /// TODO investigate
    ///
    unsafe fn set_marker(&mut self, color: Color, message: &str);

    ///
    /// Marks the beginning of a new event on this command list, with the given message and message
    /// color.
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    ///
    /// # Safety
    ///
    /// TODO investigate
    ///
    unsafe fn begin_event(&mut self, color: Color, message: &str);

    ///
    /// Marks the end of an event on this command list
    ///
    /// This function isn't guaranteed to do anything. This function will be a no-op unless a debug
    /// instance is created and the required backend facilities are present (i.e. Vulkan may not
    /// always expose the `VK_EXT_debug_utils` extension).
    ///
    /// # Safety
    ///
    /// TODO investigate
    ///
    unsafe fn end_event(&mut self);
}

//
//
// _________________________________________________________________________________________________
// Command Lists

pub trait ICommandList: IAny + IGetPlatformInterface + Send {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError>;

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError>;

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError>;
}

//
//
// _________________________________________________________________________________________________
// Descriptors

#[repr(transparent)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DescriptorSetHandle(NonNull<()>);

impl DescriptorSetHandle {
    /// Unsafe utility function for constructing a new [DescriptorSetHandle] from a raw pointer.
    ///
    /// # Warning
    ///
    /// This technically doesn't cause immediate UB, the implementation is safe, but doing this
    /// outside of an RHI implementation is almost certainly incorrect. This function is marked as
    /// unsafe to discourage using it. There should be zero need to call this unless you're
    /// constructing handles from internal RHI types.
    ///
    /// This function exists to avoid using *actual* unsafe via [core::mem::transmute] to allow
    /// RHI implementations to construct this otherwise opaque type safely.
    pub unsafe fn from_raw(v: NonNull<()>) -> Self {
        DescriptorSetHandle(v)
    }

    pub unsafe fn from_raw_int(v: u64) -> Option<Self> {
        NonNull::new(v as *mut ()).map(DescriptorSetHandle)
    }
}

impl From<DescriptorSetHandle> for NonNull<()> {
    fn from(val: DescriptorSetHandle) -> Self {
        val.0
    }
}

unsafe impl Send for DescriptorSetHandle {}

pub trait IDescriptorPool: IAny + IGetPlatformInterface + Send {
    /// Allocates a new individual descriptor set from the pool.
    ///
    /// May fail if the pool's backing memory has been exhausted.
    ///
    /// # Warning
    ///
    /// The descriptor sets returned by a pool will by default contain invalid descriptors. That is,
    /// assume they contain uninitialized memory. It is required to update the set with fresh
    /// descriptors before use.
    ///
    /// Vulkan requires this behavior for valid API usage. Other implementations may re-use
    /// previously freed descriptor sets without zeroing out their contents meaning you may reuse
    /// stale descriptors.
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError>;

    /// Allocates `num_sets` descriptors from the pool. Some implementations may be able to
    /// implement this more efficiently than naively calling [IDescriptorPool::allocate_set] in a
    /// loop.
    ///
    /// # Warning
    ///
    /// See [IDescriptorPool::allocate_set] for some pitfalls and warnings to check for.
    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set()?);
        }
        Ok(sets)
    }

    /// Will free the given descriptor sets, allowing them and their memory to be reused.
    ///
    /// # Safety
    ///
    /// [DescriptorSetHandle] is semantically a pointer. This function will take ownership of the
    /// set, so it is unsafe to call this function and then use the [DescriptorSetHandle] again.
    /// That would be an immediate use after free.
    ///
    /// This also means double-freeing is unsafe.
    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]);

    /// Will free all the descriptor sets allocated from the pool, resetting it to an empty state
    /// where it can allocate sets again. Even after an OOM error.
    ///
    /// # Safety
    ///
    /// The safety requirements are similar to [IDescriptorPool::free]. This will implicitly take
    /// ownership of all [DescriptorSetHandle]s allocated from the pool and free them. It is the
    /// responsibility of the caller to ensure that all handles are never re-used after they are
    /// freed.
    ///
    /// This function requires extra care as it will affect every set in the pool instead of only
    /// the individual sets requested like in 'free'.
    unsafe fn reset(&mut self);
}

pub trait IDescriptorSetLayout: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IDescriptorSetLayout);
}

//
//
// _________________________________________________________________________________________________
// Pipeline Objects

pub trait IPipelineLayout: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IPipelineLayout);
}

pub trait IGraphicsPipeline: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IGraphicsPipeline);
}

pub trait IComputePipeline: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IComputePipeline);
}

//
//
// _________________________________________________________________________________________________
// Shader

pub trait IShader: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IShader);

    fn shader_type(&self) -> ShaderType;
    fn entry_point(&self) -> &str;
}

//
// =================================================================================================
// DATA
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// General

#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Rect {
    /// Origin of the rectangle on the `x` axis
    pub x: u32,

    /// Origin of the rectangle on the `y` axis
    pub y: u32,

    /// Width of the rectangle
    pub w: u32,

    /// Height of the rectangle
    pub h: u32,
}

impl Rect {
    /// Returns the origin of the rectangle as `(x, y)`
    pub const fn origin(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Returns the dimensions of the rectangle as `(w, h)`
    pub const fn dimensions(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    /// Returns the maximum point of the rectangle as `(x, y)` (origin + dimensions)
    pub const fn maximum(&self) -> (u32, u32) {
        (self.x + self.w, self.y + self.h)
    }
}

/// A three-component vector of [i32], conventionally used for specifying offsets.
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Offset3D {
    /// Offset along the `x` axis
    pub x: i32,

    /// Offset along the `y` axis
    pub y: i32,

    /// Offset along the `z` axis
    pub z: i32,
}

impl Offset3D {
    /// Construct a new [Offset3D] from the 3 provided coordinates
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns an offset equal to the maximum point of a box with origin `self` and the provided
    /// extents.
    ///
    /// Produces a new [Offset3D] where each component is equal to the sum of the corresponding
    /// components in `self` and `extent`.
    pub const fn maximum_with_extent(&self, extent: &Extent3D) -> Self {
        Self {
            x: self.x + (extent.width as i32),
            y: self.y + (extent.height as i32),
            z: self.z + (extent.depth as i32),
        }
    }
}

/// An unsigned version of [Offset3D].
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct UOffset3D {
    /// Extent along the `x` axis
    pub x: u32,

    /// Extent along the `y` axis
    pub y: u32,

    /// Extent along the `z` axis
    pub z: u32,
}

impl UOffset3D {
    /// Construct a new [UOffset3D] from the 3 provided coordinates
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Returns an offset equal to the maximum point of a box with origin `self` and the provided
    /// extents.
    ///
    /// Produces a new [UOffset3D] where each component is equal to the sum of the corresponding
    /// components in `self` and `extent`.
    pub const fn maximum_with_extent(&self, extent: &Extent3D) -> Self {
        Self {
            x: self.x + extent.width,
            y: self.y + extent.height,
            z: self.z + extent.depth,
        }
    }
}

/// A two-component vector of [u32], canonically used for specifying extents.
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Extent2D {
    /// Extent along the `x` axis
    pub width: u32,

    /// Extent along the `y` axis
    pub height: u32,
}

impl Extent2D {
    /// Construct a new [Extent2D] from the 3 provided coordinates
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// A three-component vector of [u32], canonically used for specifying extents.
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Extent3D {
    /// Extent along the `x` axis
    pub width: u32,

    /// Extent along the `y` axis
    pub height: u32,

    /// Extent along the `z` axis
    pub depth: u32,
}

impl Extent3D {
    /// Construct a new [Extent3D] from the 3 provided coordinates
    pub const fn new(width: u32, height: u32, depth: u32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AttachmentLoadOp<ClearValue> {
    /// Specifies that the attachment will be loaded from the data in memory
    Load,

    /// Specifies that the attachment will be cleared with a specified colour
    Clear(ClearValue),

    /// Specifies that the contents of the attachment are not important and can be safely discarded.
    /// Any loads will read undefined data.
    DontCare,

    /// Specifies that the attachment is *not* loaded (unused). This is similar to
    /// [AttachmentLoadOp::DontCare], but will leave the attachment untouched rather than undefined.
    None,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AttachmentStoreOp {
    /// Specifies that the results of rendering operations will be written to the attachment's
    /// memory
    Store,

    /// Specifies that the results of rendering operations will be discarded and *not* written to
    /// memory. The contents of the attachment will become undefined.
    DontCare,

    /// Specifies that the attachment is *not* stored to (unused). This is similar to
    /// [AttachmentStoreOp::DontCare], but will leave the attachment untouched rather than
    /// undefined.
    None,
}

/// An `ARGB` color value packed into a single u64. Bit layout: 0xAARRGGBB
#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Color(pub u64);

impl Color {
    pub const RED: Self = Self(0xFFFF0000);
    pub const GREEN: Self = Self(0xFF00FF00);
    pub const BLUE: Self = Self(0xFF0000FF);
    pub const YELLOW: Self = Self(0xFFFFFF00);
    pub const MAGENTA: Self = Self(0xFFFF00FF);
    pub const CYAN: Self = Self(0xFF00FFFF);
    pub const WHITE: Self = Self(0xFFFFFFFF);
    pub const BLACK: Self = Self(0xFF000000);
}

impl From<u64> for Color {
    #[inline(always)]
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl From<Color> for u64 {
    #[inline(always)]
    fn from(v: Color) -> Self {
        v.0
    }
}

impl From<Color> for (f32, f32, f32, f32) {
    #[inline(always)]
    fn from(val: Color) -> Self {
        #[inline(always)]
        fn convert_channel(c: u64) -> f32 {
            ((c & 0xFF) as f32) / 255.0
        }
        let a = convert_channel(val.0 >> 48);
        let r = convert_channel(val.0 >> 32);
        let g = convert_channel(val.0 >> 16);
        let b = convert_channel(val.0);
        (a, r, g, b)
    }
}

impl From<Color> for [f32; 4] {
    fn from(value: Color) -> Self {
        type V = (f32, f32, f32, f32);
        let (a, r, g, b) = V::from(value);
        [r, g, b, a]
    }
}

//
//
// _________________________________________________________________________________________________
// Context

/// Enumeration of all available backends.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BackendAPI {
    Vulkan,
    D3D12,
}

impl Display for BackendAPI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendAPI::Vulkan => f.write_str("Vulkan"),
            BackendAPI::D3D12 => f.write_str("D3D12"),
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Adapter

/// The set of preferences that can be requested for the type of adapter to select.
#[derive(Copy, Clone, Debug)]
pub enum AdapterTypePreference {
    /// Instructs the context to prefer a hardware adapter if one is available. This option means
    /// that a hardware adapter will always be selected over a software adapter unconditionally.
    Hardware,

    /// Instructs the context to prefer a software adapter if one is available. This option means
    /// that a software adapter will always be selected over a hardware adapter unconditionally.
    Software,
}

impl Default for AdapterTypePreference {
    #[inline(always)]
    fn default() -> Self {
        Self::Hardware
    }
}

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

impl Default for AdapterPowerClass {
    #[inline(always)]
    fn default() -> Self {
        Self::LowPower
    }
}

#[derive(Clone)]
pub struct AdapterRequestOptions<'a> {
    /// A handle to an [ISurface] which the device adapter must be able to render and present to.
    ///
    /// Can be set to `None` to indicate we aren't going to present. Useful for compute-only
    /// workloads.
    pub surface: Option<&'a dyn ISurface>,

    /// Specifies the preferred power class of the adapter the context should return. See
    /// [AdapterPowerClass] for the meaning of each power class.
    ///
    /// This only specifies a preference. There is no guarantee that the returned adapter will be
    /// of any particular power class, only that the context will chose the best available match
    /// out of the set of compatible adapters.
    ///
    /// e.g. If a system only has a single dedicated GPU and the preferred power class is low-power
    /// then the context will still yield the dedicated GPU.
    pub power_class: AdapterPowerClass,

    /// What type of device is preferred when selecting an adapter.
    pub type_preference: AdapterTypePreference,

    /// Whether to allow the implementation to select a software adapter in any capacity. This
    /// option can be used to force the context to never select software adapters, unlike
    /// 'type_preference' which is a soft request to prefer one over the other.
    pub allow_software_adapters: bool,

    /// Whether to allow the implementation to select a hardware adapter in any capacity. This
    /// option can be used to force the context to never select hardware adapters, unlike
    /// 'type_preference' which is a soft request to prefer one over the other.
    pub deny_hardware_adapters: bool,
}

impl<'a> Default for AdapterRequestOptions<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            // We can't make a "default" surface so just default to no surface.
            surface: None,

            // 99.9999% users will ask for the HighPower adapter so we default to that.
            power_class: AdapterPowerClass::HighPower,

            // Again, 99.9999% of users will ask for a hardware adapter so we default to that.
            type_preference: AdapterTypePreference::Hardware,

            // Again, 99.9999% of users will want a hard fail with no hardware adapter
            allow_software_adapters: false,

            // Again, 99.9999% of users will want hardware adapters
            deny_hardware_adapters: false,
        }
    }
}

impl<'a> Debug for AdapterRequestOptions<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdapterRequestOptions")
            .field("surface", &self.surface.as_ref().map(|_| "<ptr>"))
            .field("power_class", &self.power_class)
            .finish()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AdapterVendor {
    Unknown,
    NVIDIA,
    AMD,
    Intel,
    Apple,
    ImaginationTechnology,
    ARM,
    Qualcomm,
}

impl Display for AdapterVendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterVendor::Unknown => f.write_str("Unknown"),
            AdapterVendor::NVIDIA => f.write_str("NVIDIA"),
            AdapterVendor::AMD => f.write_str("AMD"),
            AdapterVendor::Intel => f.write_str("Intel"),
            AdapterVendor::Apple => f.write_str("Apple"),
            AdapterVendor::ImaginationTechnology => f.write_str("ImaginationTechnology"),
            AdapterVendor::ARM => f.write_str("ARM"),
            AdapterVendor::Qualcomm => f.write_str("Qualcomm"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AdapterDescription<'a> {
    /// The name of the adapter
    pub name: &'a str,

    /// The adapter's vendor, if one could be identified
    pub vendor: AdapterVendor,
}

//
//
// _________________________________________________________________________________________________
// SwapChain

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PresentationMode {
    Immediate,
    Mailbox,
    Fifo,
}

impl Default for PresentationMode {
    fn default() -> Self {
        Self::Immediate
    }
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

#[derive(Clone)]
pub struct AcquireDesc<'a> {
    /// A semaphore that will be signalled once the acquire operation is completed. Only once the
    /// acquire operation signals is the acquired image safe to use on the GPU timeline.
    pub signal_semaphore: &'a dyn ISemaphore,
}

#[derive(Debug)]
pub enum AcquireResult {
    /// Specifies a successful image acquisition. The associated value contains the index of the
    /// acquired image.
    Ok(usize),

    /// Specifies a successful image acquisition, but also flags that the current swap chain
    /// configuration is out-of-date. The swap chain can still be used, but it is encouraged to
    /// perform a rebuild operation to return to an optimal state. The associated value contains
    /// the index of the acquired image.
    ///
    /// This may happen, for example, when a window is resized and the swap chain resolution and
    /// backing window size don't match. The platform compositor can compensate for this but this
    /// does introduce latency and overhead which is sub-optimal.
    SubOptimal(usize),

    /// Specifies the image acquisition has failed, providing a reason in the associated value.
    Err(ImageAcquireError),
}

impl From<ImageAcquireError> for AcquireResult {
    fn from(value: ImageAcquireError) -> Self {
        Self::Err(value)
    }
}

impl AcquireResult {
    /// Coerces the result to an Option, discarding the error and ignoring the 'sub-optimal' case.
    ///
    /// # Warning
    ///
    /// In general the 'sub-optimal' case should *not* be ignored.
    #[inline]
    pub fn ok(self) -> Option<usize> {
        match self {
            AcquireResult::Ok(v) => Some(v),
            AcquireResult::SubOptimal(v) => Some(v),
            AcquireResult::Err(_) => None,
        }
    }

    /// Coerces the result to an Option, ignoring the 'sub-optimal' case and treating it as Ok.
    ///
    /// # Warning
    ///
    /// In general the 'sub-optimal' case should *not* be ignored.
    #[inline]
    pub fn err(self) -> Result<usize, ImageAcquireError> {
        match self {
            AcquireResult::Ok(v) => Ok(v),
            AcquireResult::SubOptimal(v) => Ok(v),
            AcquireResult::Err(v) => Err(v),
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources

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

impl Default for CpuAccessMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ImageLayout {
    /// Specifies that the layout is unknown.
    Undefined,

    /// Supports all types of read device access. Writable access is not possible through this
    /// layout.
    /// TODO: The above might be wrong about write access
    Common,

    /// Must only be used for presenting a presentable image for display.
    PresentSrc,

    ///
    ColorAttachmentOptimal,

    ///
    DepthStencilAttachmentOptimal,

    ///
    DepthStencilReadOnlyOptimal,

    /// Specifies a layout allowing read-only access in a shader as a sampled image.
    ShaderReadOnlyOptimal,

    /// Must only be used as a source image of a copy command.
    CopySrc,

    /// Must only be used as a destination image of a copy command.
    CopyDst,

    /// Supports all types of access, potentially including unordered access.
    /// TODO: This might not be needed, D3D12_BARRIER_LAYOUT_COMMON might cover us like Vulkan as
    ///       this layout appears to only exist for backwards compatibility with old barriers.
    UnorderedAccess,

    /// TODO: Might not be needed like UnorderedAccess
    ResolveSource,

    /// TODO: Might not be needed like UnorderedAccess
    ResolveDest,
    // /// Must only be used as a fragment shading rate attachment or shading rate image.
    // ShadingRateAttachmentOptimal,
}

impl Default for ImageLayout {
    fn default() -> Self {
        ImageLayout::Undefined
    }
}

bitflags! {
    pub struct BarrierSync: u64 {
        ///
        /// ## Vulkan
        ///
        /// - `NONE`
        ///
        /// ## D3D12
        ///
        /// - `NONE`
        ///
        const NONE = 0x0;

        ///
        /// ## Vulkan
        ///
        /// - `ALL_COMMANDS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `ALL`
        ///
        const ALL  = 0x1;

        ///
        /// ## Vulkan
        ///
        /// - `ALL_GRAPHICS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DRAW`
        ///
        const DRAW  = 0x2;

        ///
        /// ## Vulkan
        ///
        /// - `INDEX_INPUT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDEX_INPUT`
        ///
        const INDEX_INPUT = 0x4;

        ///
        /// ## Vulkan
        ///
        /// - `PRE_RASTERIZATION_SHADERS_BIT | VERTEX_ATTRIBUTE_INPUT`
        ///
        /// ## D3D12
        ///
        /// - `VERTEX_SHADING`
        ///
        const VERTEX_SHADING = 0x8;

        ///
        /// ## Vulkan
        ///
        /// - `FRAGMENT_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `PIXEL_SHADING`
        ///
        const PIXEL_SHADING = 0x10;

        ///
        /// ## Vulkan
        ///
        /// - `EARLY_FRAGMENT_TESTS_BIT`
        /// - `LATE_FRAGMENT_TESTS_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL`
        ///
        const DEPTH_STENCIL = 0x20;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_OUTPUT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET = 0x40;

        ///
        /// ## Vulkan
        ///
        /// - `COMPUTE_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COMPUTE_SHADING`
        ///
        const COMPUTE_SHADING = 0x80;

        ///
        /// ## Vulkan
        ///
        /// - `RAY_TRACING_SHADER_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING`
        ///
        const RAYTRACING = 0x100;

        ///
        /// ## Vulkan
        ///
        /// - `COPY_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY`
        ///
        const COPY = 0x200;

        ///
        /// ## Vulkan
        ///
        /// - `RESOLVE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RESOLVE`
        ///
        const RESOLVE = 0x400;

        ///
        /// ## Vulkan
        ///
        /// - `DRAW_INDIRECT_BIT`
        ///
        /// ## D3D12
        ///
        /// - `EXECUTE_INDIRECT`
        /// - `PREDICATION`
        ///
        const EXECUTE_INDIRECT = 0x800;

        // const ALL_SHADING = 0x1000;

        ///
        /// ## Warning
        ///
        /// I don't know if this is needed, or can be mapped in a sane way. This will describe what
        /// I think this should map to.
        ///
        /// ## Vulkan
        ///
        /// - `CLEAR_BIT`
        ///
        /// ## D3D12
        ///
        /// - `CLEAR_UNORDERED_ACCESS_VIEW`
        ///
        #[deprecated]
        const CLEAR_UNORDERED_ACCESS_VIEW = 0x8000;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_BUILD_BIT`
        ///
        /// ## D3D12
        ///
        /// - `BUILD_RAYTRACING_ACCELERATION_STRUCTURE`
        /// - `EMIT_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO`
        ///
        const BUILD_RAYTRACING_ACCELERATION_STRUCTURE = 0x800000;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_COPY_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_RAYTRACING_ACCELERATION_STRUCTURE`
        ///
        const COPY_RAYTRACING_ACCELERATION_STRUCTURE = 0x1000000;
    }
}

bitflags! {
    pub struct BarrierAccess: u64 {

        ///
        /// ## Vulkan
        ///
        /// - `NONE`
        ///
        /// ## D3D12
        ///
        /// - `NO_ACCESS`
        ///
        const NONE = 0x0;

        ///
        /// ## Vulkan
        ///
        /// - `VERTEX_ATTRIBUTE_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `VERTEX_BUFFER`
        ///
        const VERTEX_BUFFER_READ = 0x1;

        ///
        /// ## Vulkan
        ///
        /// - `INDEX_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDEX_BUFFER`
        ///
        const INDEX_BUFFER_READ = 0x2;

        ///
        /// ## Vulkan
        ///
        /// - `UNIFORM_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `CONSTANT_BUFFER`
        ///
        const CONSTANT_BUFFER_READ = 0x4;

        ///
        /// ## Vulkan
        ///
        /// - `INDIRECT_COMMAND_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `INDIRECT_ARGUMENT`
        ///
        const INDIRECT_COMMAND_READ = 0x8;

        ///
        /// ## Vulkan
        ///
        /// - `SHADER_SAMPLED_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `SHADER_RESOURCE`
        ///
        const SHADER_SAMPLED_READ = 0x10;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET_READ = 0x20;

        ///
        /// ## Vulkan
        ///
        /// - `COLOR_ATTACHMENT_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RENDER_TARGET`
        ///
        const RENDER_TARGET_WRITE = 0x40;

        ///
        /// ## Vulkan
        ///
        /// - `DEPTH_STENCIL_ATTACHMENT_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL_READ`
        ///
        const DEPTH_STENCIL_READ = 0x80;

        ///
        /// ## Vulkan
        ///
        /// - `DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `DEPTH_STENCIL_WRITE`
        ///
        const DEPTH_STENCIL_WRITE = 0x100;

        ///
        /// ## Vulkan
        ///
        /// - `TRANSFER_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_SOURCE`
        ///
        const COPY_READ = 0x200;

        ///
        /// ## Vulkan
        ///
        /// - `TRANSFER_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `COPY_DEST`
        ///
        const COPY_WRITE = 0x400;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_READ_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING_ACCELERATION_STRUCTURE_READ`
        ///
        const RAYTRACING_ACCELERATION_STRUCTURE_READ = 0x800;

        ///
        /// ## Vulkan
        ///
        /// - `ACCELERATION_STRUCTURE_WRITE_BIT`
        ///
        /// ## D3D12
        ///
        /// - `RAYTRACING_ACCELERATION_STRUCTURE_WRITE`
        ///
        const RAYTRACING_ACCELERATION_STRUCTURE_WRITE = 0x1000;
    }
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

impl Default for Format {
    fn default() -> Self {
        Self::R8Unorm
    }
}

impl Format {
    /// Returns whether the format is a depth texture format
    pub const fn is_depth(&self) -> bool {
        matches!(self, Self::Depth32Float)
    }

    /// Returns whether the format is a stencil texture format
    pub const fn is_stencil(&self) -> bool {
        matches!(self, Self::Depth24Stencil8)
    }

    /// Returns whether the format is a depth/stencil texture format
    pub const fn is_depth_stencil(&self) -> bool {
        matches!(self, Self::Depth32Float | Self::Depth24Stencil8)
    }

    /// Returns whether the format is a float format
    pub const fn is_float(&self) -> bool {
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
    pub const fn is_sint(&self) -> bool {
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
    pub const fn is_uint(&self) -> bool {
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
    pub const fn is_snorm(&self) -> bool {
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
    pub const fn is_unorm(&self) -> bool {
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

    /// Returns the number of bytes the format consumes per individual element.
    ///
    /// For standard formats this will return the number of bytes per texel, for block formats this
    /// will return the number of bytes per block (block formats smallest 'element' is a single
    /// block).
    pub const fn bytes_per_element(&self) -> u32 {
        match self {
            Format::R8Unorm => 1,
            Format::R8Snorm => 1,
            Format::R8Uint => 1,
            Format::R8Sint => 1,
            Format::R16Uint => 2,
            Format::R16Sint => 2,
            Format::R16Unorm => 2,
            Format::R16Snorm => 2,
            Format::R16Float => 2,
            Format::Rg8Unorm => 2,
            Format::Rg8Snorm => 2,
            Format::Rg8Uint => 2,
            Format::Rg8Sint => 2,
            Format::R32Uint => 4,
            Format::R32Sint => 4,
            Format::R32Float => 4,
            Format::Rg16Uint => 4,
            Format::Rg16Sint => 4,
            Format::Rg16Unorm => 4,
            Format::Rg16Snorm => 4,
            Format::Rg16Float => 4,
            Format::Rgba8Unorm => 4,
            Format::Rgba8UnormSrgb => 4,
            Format::Rgba8Snorm => 4,
            Format::Rgba8Uint => 4,
            Format::Rgba8Sint => 4,
            Format::Bgra8Unorm => 4,
            Format::Bgra8UnormSrgb => 4,
            Format::Rgb10a2Unorm => 4,
            Format::Rg11b10Float => 4,
            Format::Rg32Uint => 8,
            Format::Rg32Sint => 8,
            Format::Rg32Float => 8,
            Format::Rgba16Uint => 8,
            Format::Rgba16Sint => 8,
            Format::Rgba16Unorm => 8,
            Format::Rgba16Snorm => 8,
            Format::Rgba16Float => 8,
            Format::Rgba32Uint => 16,
            Format::Rgba32Sint => 16,
            Format::Rgba32Float => 16,
            Format::Depth32Float => 4,
            Format::Depth24Stencil8 => 4,
        }
    }

    pub const fn has_aspect(&self, aspect: TextureCopyAspect) -> bool {
        self.aspect_mask().contains(aspect.as_flag())
    }

    pub const fn is_aspect_compatible(&self, aspect: TextureAspect) -> bool {
        self.aspect_mask().contains(aspect)
    }

    pub const fn aspect_mask(&self) -> TextureAspect {
        match self {
            Format::R8Unorm
            | Format::R8Snorm
            | Format::R8Uint
            | Format::R8Sint
            | Format::R16Uint
            | Format::R16Sint
            | Format::R16Unorm
            | Format::R16Snorm
            | Format::R16Float
            | Format::Rg8Unorm
            | Format::Rg8Snorm
            | Format::Rg8Uint
            | Format::Rg8Sint
            | Format::R32Uint
            | Format::R32Sint
            | Format::R32Float
            | Format::Rg16Uint
            | Format::Rg16Sint
            | Format::Rg16Unorm
            | Format::Rg16Snorm
            | Format::Rg16Float
            | Format::Rgba8Unorm
            | Format::Rgba8UnormSrgb
            | Format::Rgba8Snorm
            | Format::Rgba8Uint
            | Format::Rgba8Sint
            | Format::Bgra8Unorm
            | Format::Bgra8UnormSrgb
            | Format::Rgb10a2Unorm
            | Format::Rg11b10Float
            | Format::Rg32Uint
            | Format::Rg32Sint
            | Format::Rg32Float
            | Format::Rgba16Uint
            | Format::Rgba16Sint
            | Format::Rgba16Unorm
            | Format::Rgba16Snorm
            | Format::Rgba16Float
            | Format::Rgba32Uint
            | Format::Rgba32Sint
            | Format::Rgba32Float => TextureAspect::COLOR,
            Format::Depth32Float => TextureAspect::DEPTH,
            Format::Depth24Stencil8 => TextureAspect::DEPTH_STENCIL,
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::R8Unorm => f.write_str("Format::R8Unorm"),
            Format::R8Snorm => f.write_str("Format::R8Snorm"),
            Format::R8Uint => f.write_str("Format::R8Uint"),
            Format::R8Sint => f.write_str("Format::R8Sint"),
            Format::R16Uint => f.write_str("Format::R16Uint"),
            Format::R16Sint => f.write_str("Format::R16Sint"),
            Format::R16Unorm => f.write_str("Format::R16Unorm"),
            Format::R16Snorm => f.write_str("Format::R16Snorm"),
            Format::R16Float => f.write_str("Format::R16Float"),
            Format::Rg8Unorm => f.write_str("Format::Rg8Unorm"),
            Format::Rg8Snorm => f.write_str("Format::Rg8Snorm"),
            Format::Rg8Uint => f.write_str("Format::Rg8Uint"),
            Format::Rg8Sint => f.write_str("Format::Rg8Sint"),
            Format::R32Uint => f.write_str("Format::R32Uint"),
            Format::R32Sint => f.write_str("Format::R32Sint"),
            Format::R32Float => f.write_str("Format::R32Float"),
            Format::Rg16Uint => f.write_str("Format::Rg16Uint"),
            Format::Rg16Sint => f.write_str("Format::Rg16Sint"),
            Format::Rg16Unorm => f.write_str("Format::Rg16Unorm"),
            Format::Rg16Snorm => f.write_str("Format::Rg16Snorm"),
            Format::Rg16Float => f.write_str("Format::Rg16Float"),
            Format::Rgba8Unorm => f.write_str("Format::Rgba8Unorm"),
            Format::Rgba8UnormSrgb => f.write_str("Format::Rgba8UnormSrgb"),
            Format::Rgba8Snorm => f.write_str("Format::Rgba8Snorm"),
            Format::Rgba8Uint => f.write_str("Format::Rgba8Uint"),
            Format::Rgba8Sint => f.write_str("Format::Rgba8Sint"),
            Format::Bgra8Unorm => f.write_str("Format::Bgra8Unorm"),
            Format::Bgra8UnormSrgb => f.write_str("Format::Bgra8UnormSrgb"),
            Format::Rgb10a2Unorm => f.write_str("Format::Rgb10a2Unorm"),
            Format::Rg11b10Float => f.write_str("Format::Rg11b10Float"),
            Format::Rg32Uint => f.write_str("Format::Rg32Uint"),
            Format::Rg32Sint => f.write_str("Format::Rg32Sint"),
            Format::Rg32Float => f.write_str("Format::Rg32Float"),
            Format::Rgba16Uint => f.write_str("Format::Rgba16Uint"),
            Format::Rgba16Sint => f.write_str("Format::Rgba16Sint"),
            Format::Rgba16Unorm => f.write_str("Format::Rgba16Unorm"),
            Format::Rgba16Snorm => f.write_str("Format::Rgba16Snorm"),
            Format::Rgba16Float => f.write_str("Format::Rgba16Float"),
            Format::Rgba32Uint => f.write_str("Format::Rgba32Uint"),
            Format::Rgba32Sint => f.write_str("Format::Rgba32Sint"),
            Format::Rgba32Float => f.write_str("Format::Rgba32Float"),
            Format::Depth32Float => f.write_str("Format::Depth32Float"),
            Format::Depth24Stencil8 => f.write_str("Format::Depth24Stencil8"),
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Buffer

/// Description object used for creating a new buffer.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct BufferDesc<'a> {
    /// The size of the buffer in bytes
    pub size: u64,

    /// What kind of CPU access is allowed.
    /// - None -> device local
    /// - Read -> read back
    /// - Write -> upload
    pub cpu_access: CpuAccessMode,

    /// Enables the buffer to be used with unordered access (unordered access view, storage buffer)
    pub allow_unordered_access: bool,

    /// Enables the buffer to be used as a texel buffer
    pub allow_texel_buffer: bool,

    /// Enables the buffer to be used as a vertex buffer
    pub is_vertex_buffer: bool,

    /// Enables the buffer to be used as an index buffer
    pub is_index_buffer: bool,

    /// Enables the buffer to be used as a constant buffer
    pub is_constant_buffer: bool,

    /// Enables the buffer to be used as an argument buffer for indirect draw calls
    pub is_indirect_draw_args: bool,

    /// Enables the buffer to be used as input for ray tracing acceleration structure builds
    pub is_accel_struct_build_input: bool,

    /// Enables the buffer to store a constructed and ready to use rt acceleration structure
    pub is_accel_struct_storage: bool,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> BufferDesc<'a> {
    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub fn strip_name(self) -> BufferDesc<'static> {
        BufferDesc::<'static> {
            size: self.size,
            cpu_access: self.cpu_access,
            allow_unordered_access: self.allow_unordered_access,
            allow_texel_buffer: self.allow_texel_buffer,
            is_vertex_buffer: self.is_vertex_buffer,
            is_index_buffer: self.is_index_buffer,
            is_constant_buffer: self.is_constant_buffer,
            is_indirect_draw_args: self.is_indirect_draw_args,
            is_accel_struct_build_input: self.is_accel_struct_build_input,
            is_accel_struct_storage: self.is_accel_struct_storage,
            name: None,
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Texture

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

impl Default for TextureDimension {
    fn default() -> Self {
        Self::Texture1D
    }
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
                write!(f, "OptimalClearValue::ColorF32({r}, {g}, {b}, {a})")
            }
            OptimalClearValue::ColorInt(v) => {
                write!(f, "OptimalClearValue::ColorInt({:X})", *v)
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                write!(f, "OptimalClearValue::DepthStencil({depth}, {stencil})")
            }
        }
    }
}

/// Description object used for creating a new texture.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TextureDesc<'a> {
    /// The width of the texture
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: Format,

    /// The dimensionality of the texture.
    ///
    /// Declares whether the texture should be a 1D, 2D, 3D or cube texture.
    pub dimension: TextureDimension,

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

    /// Allows the texture to be used as a copy destination
    pub allow_copy_dest: bool,

    /// Allows the texture to be used as a copy source
    pub allow_copy_source: bool,

    /// Enables the texture to be used with unordered access (unordered access view, storage
    /// texture)
    pub allow_unordered_access: bool,

    /// Enables the texture to be used as a face for a cube map
    pub allow_cube_face: bool,

    /// Enables the texture to be used as a render target
    pub is_render_target: bool,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> TextureDesc<'a> {
    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub fn strip_name(self) -> TextureDesc<'static> {
        TextureDesc::<'static> {
            width: self.width,
            height: self.height,
            depth: self.depth,
            format: self.format,
            dimension: self.dimension,
            clear_value: self.clear_value,
            array_size: self.array_size,
            mip_levels: self.mip_levels,
            sample_count: self.sample_count,
            sample_quality: self.sample_quality,
            allow_copy_dest: self.allow_copy_dest,
            allow_copy_source: self.allow_copy_source,
            allow_unordered_access: self.allow_unordered_access,
            allow_cube_face: self.allow_cube_face,
            is_render_target: self.is_render_target,
            name: None,
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Sampler

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerAddressMode {
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}

impl Default for SamplerAddressMode {
    fn default() -> Self {
        Self::Wrap
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerFilter {
    Nearest,
    Linear,
}

impl Default for SamplerFilter {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerMipFilter {
    Nearest,
    Linear,
}

impl Default for SamplerMipFilter {
    fn default() -> Self {
        Self::Nearest
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum SamplerBorderColor {
    BlackTransparent,
    BlackOpaque,
    WhiteOpaque,
}

impl Default for SamplerBorderColor {
    fn default() -> Self {
        Self::BlackTransparent
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SamplerDesc<'a> {
    pub min_filter: SamplerFilter,
    pub mag_filter: SamplerFilter,
    pub mip_filter: SamplerMipFilter,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub lod_bias: f32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub enable_anisotropy: bool,
    pub max_anisotropy: u32,
    pub compare_op: Option<CompareOp>,
    pub border_color: SamplerBorderColor,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> Default for SamplerDesc<'a> {
    fn default() -> Self {
        Self {
            min_filter: SamplerFilter::Linear,
            mag_filter: SamplerFilter::Linear,
            mip_filter: SamplerMipFilter::Linear,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            lod_bias: 0.0,
            min_lod: 0.0,
            max_lod: 1000.0,
            enable_anisotropy: false,
            max_anisotropy: 0,
            compare_op: Default::default(),
            border_color: Default::default(),
            name: None,
        }
    }
}

impl<'a> SamplerDesc<'a> {
    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub fn strip_name(self) -> SamplerDesc<'static> {
        SamplerDesc::<'static> {
            min_filter: self.min_filter,
            mag_filter: self.mag_filter,
            mip_filter: self.mip_filter,
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            lod_bias: self.lod_bias,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            enable_anisotropy: self.enable_anisotropy,
            max_anisotropy: self.max_anisotropy,
            compare_op: self.compare_op,
            border_color: self.border_color,
            name: None,
        }
    }
}

//
//
// _________________________________________________________________________________________________
// Resources - Shader

/// An enumeration of the supported set of shader input types.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderBinary<'a> {
    /// This variant encloses a SPIR-V binary. Only supported by the `Vulkan` backend.
    Spirv(&'a [u8]),

    /// This variant encloses a DXIL binary. Only supported by the `D3D12` backend.
    Dxil(&'a [u8]),
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

impl Default for ShaderType {
    fn default() -> Self {
        Self::Compute
    }
}

/// Set of options for creating a new shader module
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ShaderOptions<'a> {
    /// Specifies the type of shader that this module will hold
    pub shader_type: ShaderType,

    /// The raw bytes of the shader module, discriminated as either SPIR-V or DXIL
    pub data: ShaderBinary<'a>,

    /// The name of the entry point function that will be married to the shader module
    pub entry_point: &'a str,

    /// The name of the object
    pub name: Option<&'a str>,
}

//
//
// _________________________________________________________________________________________________
// Descriptors

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DescriptorType {
    Sampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    StructuredBuffer,
    InputAttachment,
    // TODO: Can we do something with VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK?
    // TODO: VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR
}

impl Default for DescriptorType {
    fn default() -> Self {
        Self::Sampler
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DescriptorShaderVisibility {
    All,
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

impl Default for DescriptorShaderVisibility {
    fn default() -> Self {
        Self::All
    }
}

impl From<ShaderType> for DescriptorShaderVisibility {
    #[inline]
    fn from(v: ShaderType) -> Self {
        match v {
            ShaderType::Compute => DescriptorShaderVisibility::Compute,
            ShaderType::Vertex => DescriptorShaderVisibility::Vertex,
            ShaderType::Hull => DescriptorShaderVisibility::Hull,
            ShaderType::Domain => DescriptorShaderVisibility::Domain,
            ShaderType::Geometry => DescriptorShaderVisibility::Geometry,
            ShaderType::Fragment => DescriptorShaderVisibility::Fragment,
            ShaderType::Amplification => DescriptorShaderVisibility::Amplification,
            ShaderType::Mesh => DescriptorShaderVisibility::Mesh,
        }
    }
}

#[derive(Clone, Default)]
pub struct DescriptorSetLayoutBinding<'a> {
    /// The binding number of this entry and corresponds to a resource of the same binding number in
    /// the shader stages.
    pub binding_num: u32,

    /// Specifies which type of resource descriptors are used for this binding
    pub binding_type: DescriptorType,

    /// Specifies the number of descriptors contained in the binding. Should be 1 to declare a
    /// single binding, or >1 to declare an array of descriptors.
    pub binding_count: Option<NonZeroU32>,

    /// Declares whether the descriptor's underlying resource can be accessed with write access.
    pub allow_writes: bool,

    /// An optional list of `binding_count` samplers to specify static samplers for `Sampler`
    /// descriptors. If `binding_type` is `Sampler` but `static_samplers` is `None` then the
    /// samplers are dynamic.
    pub static_samplers: Option<&'a [&'a dyn ISampler]>,
}

#[derive(Clone, Default)]
pub struct DescriptorSetLayoutDesc<'a> {
    /// Specifies which shader stages can access a resource for this set
    pub visibility: DescriptorShaderVisibility,

    /// A list of all bindings that are a part of this descriptor set layout
    pub items: &'a [DescriptorSetLayoutBinding<'a>],

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Clone)]
pub struct DescriptorPoolDesc<'a> {
    /// The descriptor set layout that the descriptor pool will allocate descriptor sets for. A pool
    /// can only allocate descriptor sets with a single layout.
    pub layout: &'a dyn IDescriptorSetLayout,

    /// The number of sets the pool should have capacity for. A pool is only guaranteed to have
    /// enough space for `num_sets` descriptor sets.
    pub num_sets: u32,

    /// The name of the object
    pub name: Option<&'a str>,
}

/// A description of a descriptor write. Specifies the target descriptor set, binding index and
/// array element. Then specifies the type of, number of, and target of the descriptors to write.
#[derive(Clone)]
pub struct DescriptorWriteDesc<'a> {
    /// The descriptor set that will be the target of this write operation.
    pub set: DescriptorSetHandle,

    /// The descriptor binding index that will be the target of the write operation.
    pub binding: u32,

    /// The array element in the binding to write. Ignored for non-array bindings.
    pub array_element: u32,

    /// The type of descriptor writing. This must match the descriptor type described in the set
    /// layout, and determines the expected variant of [DescriptorWrites] in `writes`.
    pub descriptor_type: DescriptorType,

    /// The list of descriptor writes to perform. The variant to use depends on `descriptor_type`.
    pub writes: DescriptorWrites<'a>,
}

/// The set of descriptor write types.
///
/// Each descriptor type needs different pieces of information in order to construct or write the
/// descriptors into the device-visible set memory. Each variant of this enum covers some of the
/// types in [DescriptorType].
#[derive(Clone)]
pub enum DescriptorWrites<'a> {
    /// Variant expected for writing
    /// - [DescriptorType::Sampler]
    Sampler(&'a [SamplerDescriptorWrite<'a>]),

    /// Variant expected for writing
    /// - [DescriptorType::SampledImage]
    /// - [DescriptorType::StorageImage]
    Image(&'a [ImageDescriptorWrite]),

    /// Variant expected for writing
    /// - [DescriptorType::UniformBuffer]
    /// - [DescriptorType::StorageBuffer]
    Buffer(&'a [BufferDescriptorWrite<'a>]),

    /// Variant expected for writing
    /// - [DescriptorType::StructuredBuffer]
    StructuredBuffer(&'a [StructuredBufferDescriptorWrite<'a>]),

    /// Variant expected for writing
    /// - [DescriptorType::UniformTexelBuffer]
    /// - [DescriptorType::StorageTexelBuffer]
    TexelBuffer(&'a [TexelBufferDescriptorWrite<'a>]),

    /// Variant expected for writing
    /// - [DescriptorType::InputAttachment]
    InputAttachment(&'a [ImageDescriptorWrite]),
}

impl<'a> DescriptorWrites<'a> {
    /// Returns true if the array stored on the active variant of `self` is empty, that is: when
    /// `self.len() == 0`.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of array elements are contained in the array stored on the active variant
    /// of `self`.
    pub const fn len(&self) -> usize {
        match self {
            DescriptorWrites::Sampler(v) => v.len(),
            DescriptorWrites::Image(v) => v.len(),
            DescriptorWrites::Buffer(v) => v.len(),
            DescriptorWrites::StructuredBuffer(v) => v.len(),
            DescriptorWrites::TexelBuffer(v) => v.len(),
            DescriptorWrites::InputAttachment(v) => v.len(),
        }
    }
}

/// Describes the parameters of a descriptor to write when writing into a sampler binding.
#[derive(Clone)]
pub struct SamplerDescriptorWrite<'a> {
    /// The sampler target.
    pub sampler: &'a dyn ISampler,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ImageViewType {
    Tex1D,
    Tex2D,
    Tex3D,
    TexCube,
    TexArray1D,
    TexArray2D,
    TexCubeArray,
}

/// Describes the parameters of a descriptor to write when writing into a texture binding.
#[derive(Clone, Hash, Debug)]
pub struct ImageDescriptorWrite {
    /// The image target.
    pub image_view: ImageView,

    /// The layout of the image
    pub image_layout: ImageLayout,
}

/// Describes the parameters of a descriptor to write when writing into a simple buffer like
/// binding.
#[derive(Clone)]
pub struct BufferDescriptorWrite<'a> {
    /// The buffer target
    pub buffer: &'a dyn IBuffer,

    /// The offset in bytes from the start of buffer. Access to buffer memory via this descriptor
    /// uses addressing that is relative to this starting offset.
    pub offset: u64,

    /// The size in bytes that is used for this descriptor update, or VK_WHOLE_SIZE to use the range
    /// from offset to the end of the buffer.
    pub len: u32,

    /// Whether the buffer can be written to through this descriptor.
    pub writable: bool,
}

/// Describes the parameters of a descriptor to write when writing into a structured buffer like
/// binding.
#[derive(Clone)]
pub struct StructuredBufferDescriptorWrite<'a> {
    /// The buffer target
    pub buffer: &'a dyn IBuffer,

    /// The offset in bytes from the start of buffer. Access to buffer memory via this descriptor
    /// uses addressing that is relative to this starting offset.
    pub offset: u64,

    /// The size in bytes that is used for this descriptor update, or VK_WHOLE_SIZE to use the range
    /// from offset to the end of the buffer.
    pub len: u32,

    /// The stride/size of an individual structure in the structured buffer, in bytes
    pub structure_byte_stride: u32,

    /// Whether the buffer can be written to through this descriptor.
    pub writable: bool,
}

/// Describes the parameters of a descriptor to write when writing into a texel buffer binding.
#[derive(Clone)]
pub struct TexelBufferDescriptorWrite<'a> {
    /// The buffer target
    pub buffer: &'a dyn IBuffer,

    /// The texel format the buffer should be interpreted as.
    pub format: Format,

    /// The offset in bytes from the start of buffer. Access to buffer memory via this descriptor
    /// uses addressing that is relative to this starting offset.
    pub offset: u64,

    /// The size in bytes that is used for this descriptor update, or VK_WHOLE_SIZE to use the range
    /// from offset to the end of the buffer.
    pub len: u32,

    /// Whether the buffer can be written to through this descriptor.
    pub writable: bool,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ImageViewDesc {
    /// The format that the texture will be viewed as through this descriptor
    pub format: Format,

    /// The type of view of the given image to create.
    pub view_type: ImageViewType,

    /// The set of sub resources that will be accessed through this descriptor
    pub sub_resources: TextureSubResourceSet,

    /// Whether the image can be written to through this descriptor.
    pub writable: bool,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ImageView(NonNull<()>);

unsafe impl Send for ImageView {}
unsafe impl Sync for ImageView {}

//
//
// _________________________________________________________________________________________________
// Pipeline State Description

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct PushConstantBlock {
    /// Specifies the binding index that the push constant range will be attached to in the shader.
    ///
    /// # Warning
    ///
    /// This is ignored on Vulkan. Vulkan has a dedicated 'push constant' location specifier. D3D12
    /// maps its 'root constants' (D3D12's analogue of push constants) to a register index.
    ///
    /// There is no robust way to automatically choose a register index, so we leave the choice as
    /// an exercise for the user.
    pub binding: u32,

    /// Specifies which shader stages the push constant range will be
    pub visibility: DescriptorShaderVisibility,

    /// Specifies the size, in bytes, of the push constant range.
    pub size: u16,
}

#[derive(Clone, Default)]
pub struct PipelineLayoutDesc<'a> {
    /// Specifies the layouts of all descriptor sets that will be combined into this pipeline
    /// layout. The order of this array is meaningful: the `n`th element will define the layout for
    /// the `n`th descriptor set.
    pub set_layouts: &'a [&'a dyn IDescriptorSetLayout],

    /// Specifies the set of push constant ranges that the pipeline layout will hold.
    pub push_constant_blocks: &'a [PushConstantBlock],

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum VertexInputRate {
    /// Specifies that vertex attribute addressing is a function of the vertex index
    PerVertex,

    /// Specifies that vertex attribute addressing is a function of the instance index
    PerInstance,
}

impl Default for VertexInputRate {
    #[inline]
    fn default() -> Self {
        Self::PerVertex
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct VertexInputBindingDesc {
    /// The binding number that this structure describes
    pub binding: u32,

    /// The byte stride between consecutive elements within the buffer
    pub stride: u32,

    /// Value specifying the rate at which this input binding is fetched (per-vertex or
    /// per-instance)
    pub input_rate: VertexInputRate,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct VertexInputAttributeDesc {
    /// The shader input location number for this attribute
    pub location: u32,

    /// The binding number which this attribute takes its data from
    pub binding: u32,

    /// The format of the vertex attribute, describing size and layout
    pub format: Format,

    /// Byte offset of this attribute relative to the start of an element in the vertex input
    /// binding
    pub offset: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VertexInputStateDesc<'a> {
    /// The list of input bindings. An input binding describes the access pattern of a single buffer
    /// bound at a specific binding slot. Each binding specifies the stride of a binding element
    /// as well as the input rate (per-vertex/per-instance) the elements are used at.
    pub input_bindings: &'a [VertexInputBindingDesc],

    /// The list of input attachments. An input attachment describes an individual vertex attribute.
    /// Conceptually it marks up a single 'field' within the input binding it is read from. Multiple
    /// attributes can be fetched from the same binding.
    pub input_attributes: &'a [VertexInputAttributeDesc],
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveTopology {
    /// Specifies a series of separate point primitives
    PointList,

    /// specifies a series of separate line primitives
    LineList,

    /// Specifies a series of connected line primitives with consecutive lines sharing a vertex
    LineStrip,

    /// Specifies a series of separate triangle primitives
    TriangleList,

    /// Specifies a series of connected triangle primitives with consecutive triangles sharing an
    /// edge
    TriangleStrip,
}

impl Default for PrimitiveTopology {
    #[inline]
    fn default() -> Self {
        Self::PointList
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InputAssemblyStateDesc {
    pub primitive_topology: PrimitiveTopology,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PolygonMode {
    Fill,
    Line,
}

impl Default for PolygonMode {
    #[inline]
    fn default() -> Self {
        Self::Fill
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CullMode {
    /// Specifies that no triangles are discarded
    None,

    /// Specifies that back-facing triangles are discarded
    Back,

    /// Specifies that front-facing triangles are discarded
    Front,
}

impl Default for CullMode {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FrontFaceOrder {
    /// Specifies that a front-facing triangle is defined as one with a counter-clockwise winding
    /// order
    CounterClockwise,

    /// Specifies that a front-facing triangle is defined as one with a clockwise winding order
    Clockwise,
}

impl Default for FrontFaceOrder {
    #[inline]
    fn default() -> Self {
        Self::CounterClockwise
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RasterizerStateDesc {
    /// Specifies the triangle facing directions used for primitive culling
    pub cull_mode: CullMode,

    /// Specifies what winding order defines a 'front' facing triangle
    pub front_face: FrontFaceOrder,

    /// Specifies the triangle rendering mode
    pub polygon_mode: PolygonMode,

    /// Specifies the value to apply as a depth bias. If 0, depth bias is disabled
    pub depth_bias: i32,

    /// Specifies the depth bias clamp value, if depth bias is enabled
    pub depth_bias_clamp: f32,

    /// Specifies the depth bias slop factor value, if depth bias is enabled
    pub depth_bias_slope_factor: f32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturate,
    BlendFactor,
    OneMinusBlendFactor,
}

impl Default for BlendFactor {
    #[inline]
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl Default for BlendOp {
    #[inline]
    fn default() -> Self {
        Self::Add
    }
}

/// Enumeration of the available logical operations that can be applied as part of attachment blend
/// operations.
///
/// To describe the behavior of each operation we define the following:
///
/// * ¬ is bitwise invert
/// * ∧ is bitwise and
/// * ∨ is bitwise or
/// * ⊕ is bitwise exclusive or
/// * s is the fragment’s Rs0, Gs0, Bs0 or As0 component value for the fragment output corresponding
///   to the color attachment being updated
/// * d is the color attachment’s R, G, B or A component value
///
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum LogicOp {
    /// 0
    Clear,

    /// s ∧ d
    And,

    /// s ∧ ¬ d
    AndReverse,

    /// s
    Copy,

    /// ¬ s ∧ d
    AndInverted,

    /// d
    Noop,

    /// s ⊕ d
    Xor,

    /// s ∨ d
    Or,

    /// ¬ (s ∨ d)
    Nor,

    /// ¬ (s ⊕ d)
    Equivalent,

    /// ¬ d
    Invert,

    /// s ∨ ¬ d
    OrReverse,

    /// ¬ s
    CopyInverted,

    /// ¬ s ∨ d
    OrInverted,

    /// ¬ (s ∧ d)
    Nand,

    /// all 1s
    Set,
}

impl Default for LogicOp {
    fn default() -> Self {
        Self::Clear
    }
}

bitflags! {
    /// Bit flags used for identifying and/or masking the color components in operations regarding
    /// texels.
    pub struct ColorComponentFlags: u8 {
        /// Specifies the 'red' channel
        const R = 0b0001;

        /// Specifies the 'green' channel
        const G = 0b0010;

        /// Specifies the 'blue' channel
        const B = 0b0100;

        /// Specifies the 'alpha' channel
        const A = 0b1000;
    }
}

impl Default for ColorComponentFlags {
    #[inline]
    fn default() -> Self {
        ColorComponentFlags::empty()
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentBlendState {
    /// Enables color blending for matching attachment. All other fields will be ignored if this
    /// value is `false`.
    pub blend_enabled: bool,

    /// Selects which blend factor is used to determine the source factors (Sr,Sg,Sb)
    pub src_factor: BlendFactor,

    /// Selects which blend factor is used to determine the destination factors (Dr,Dg,Db)
    pub dst_factor: BlendFactor,

    /// Selects which blend operation is used to calculate the RGB values to write to the color
    /// attachment
    pub blend_op: BlendOp,

    /// Selects which blend factor is used to determine the source factor (Sa)
    pub alpha_src_factor: BlendFactor,

    /// Selects which blend factor is used to determine the destination factor (Da)
    pub alpha_dst_factor: BlendFactor,

    /// Selects which blend operation is use to calculate the alpha values to write to the color
    /// attachment
    pub alpha_blend_op: BlendOp,

    /// Is a bitmask of [ColorComponentFlags] specifying which of the R, G, B, and/or A components
    /// are enabled for writing
    pub color_write_mask: ColorComponentFlags,
}

/// Enumeration of available comparison operators. Comparison operators compare a 'reference' and a
/// 'test' value, and return a true (“passed”) or false (“failed”) value depending on the comparison
/// operator chosen.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CompareOp {
    /// Specifies that the comparison always evaluates false
    Never,

    /// Specifies that the comparison always evaluates true
    Always,

    /// Specifies that the comparison evaluates reference = test
    Equal,

    /// Specifies that the comparison evaluates reference ≠ test
    NotEqual,

    /// Specifies that the comparison evaluates reference < test
    Less,

    /// Specifies that the comparison evaluates reference ≤ test
    LessEqual,

    /// Specifies that the comparison evaluates reference > test
    Greater,

    /// Specifies that the comparison evaluates reference ≥ test
    GreaterOrEqual,
}

impl Default for CompareOp {
    fn default() -> Self {
        Self::Never
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum StencilOp {
    /// Keeps the current value
    Keep,

    /// Sets the value to 0
    Zero,

    /// Sets the value to reference
    Replace,

    /// Increments the current value and clamps to the maximum representable unsigned value
    IncrementClamp,

    /// Decrements the current value and clamps to 0
    DecrementClamp,

    /// Bitwise-inverts the current value
    Invert,

    /// Increments the current value and wraps to 0 when the maximum value would have been exceeded
    IncrementWrap,

    /// Decrements the current value and wraps to the maximum possible value when the value would go
    /// below 0
    DecrementWrap,
}

impl Default for StencilOp {
    #[inline]
    fn default() -> Self {
        Self::Keep
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StencilOpState {
    /// Value specifying the action performed on samples that fail the stencil test
    pub fail_op: StencilOp,

    /// Value specifying the action performed on samples that pass both the depth and stencil tests
    pub pass_op: StencilOp,

    /// Value specifying the action performed on samples that pass the stencil test and fail the
    /// depth test
    pub depth_fail_op: StencilOp,

    /// Value specifying the comparison operator used in the stencil test
    pub compare_op: CompareOp,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct DepthStencilStateDesc {
    /// Controls whether depth testing is enabled
    pub depth_test: bool,

    /// Controls whether depth writes are enabled when 'depth_test' is true. Depth writes are always
    /// disabled when 'depth_test' is false
    pub depth_write: bool,

    /// Specifies the comparison operator to use in the 'comparison' step of the depth test
    pub depth_compare_op: CompareOp,

    /// Controls whether stencil testing is enabled
    pub stencil_test: bool,

    /// Selects the bits of the unsigned integer stencil values participating in the stencil test
    pub stencil_read_mask: u8,

    /// Selects the bits of the unsigned integer stencil values updated by the stencil test in the
    /// stencil framebuffer attachment
    pub stencil_write_mask: u8,

    /// Control the 'front' parameters of the stencil test
    pub stencil_front: StencilOpState,

    /// Control the 'back' parameters of the stencil test
    pub stencil_back: StencilOpState,

    /// Enables the depth bounds check. Requires a device capability for support.
    pub depth_bounds_enable: bool,

    /// Specifies the min range of the depth bounds, if the depth bound test is enabled. Ignored if
    /// depth bounds is not enabled.
    pub min_depth_bounds: f32,

    /// Specifies the max range of the depth bounds, if the depth bound test is enabled. Ignored if
    /// depth bounds is not enabled.
    pub max_depth_bounds: f32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BlendStateDesc<'a> {
    /// An array of blend state descriptions that will be applied to each matching output attachment
    pub attachments: &'a [AttachmentBlendState],
}

#[derive(Clone)]
pub struct GraphicsPipelineDesc<'a> {
    /// The list of shader modules that the pipeline configuration will use. The shader stage for
    /// each module is specified on the [IShader] object.
    pub shader_stages: &'a [&'a dyn IShader],

    /// The description of binding locations used by both the pipeline and descriptor sets used with
    /// the pipeline
    pub pipeline_layout: &'a dyn IPipelineLayout,

    /// Structure that describes the vertex input piece of the graphics pipeline
    pub vertex_layout: &'a VertexInputStateDesc<'a>,

    /// Structure that describes the input assembly piece of the graphics pipeline
    pub input_assembly_state: &'a InputAssemblyStateDesc,

    /// Structure that describes the rasterizer piece of the graphics pipeline
    pub rasterizer_state: &'a RasterizerStateDesc,

    /// Structure that describes the depth/stencil test piece of the graphics pipeline
    pub depth_stencil_state: &'a DepthStencilStateDesc,

    /// Structure that describes the color blending piece of the graphics pipeline
    pub blend_state: &'a BlendStateDesc<'a>,

    /// Specifies the number of and format of render target attachments
    pub render_target_formats: &'a [Format],

    /// Specifies the format of the depth stencil attachment, if any.
    pub depth_stencil_format: Option<Format>,

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Clone)]
pub struct ComputePipelineDesc<'a> {
    /// The compute shader module that will be used by the compute pipeline being created.
    pub shader_module: &'a dyn IShader,

    /// The description of binding locations used by both the pipeline and descriptor sets used with
    /// the pipeline
    pub pipeline_layout: &'a dyn IPipelineLayout,

    /// The name of the object
    pub name: Option<&'a str>,
}

//
//
// _________________________________________________________________________________________________
// Queue

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
    fn default() -> Self {
        Self::General
    }
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

#[derive(Clone)]
pub struct QueueSubmitDesc<'a> {
    /// A list of the command lists that are to be submitted in this batch
    pub command_lists: &'a [Cell<Option<Box<dyn ICommandList>>>],

    /// A list of semaphores that will block the execution of the batch until all semaphores in the
    /// list are signaled.
    pub wait_semaphores: &'a [&'a dyn ISemaphore],

    /// A list of semaphores that will be signaled once all command lists in the batch have
    /// completed executing.
    pub signal_semaphores: &'a [&'a dyn ISemaphore],

    /// A fence that will be signaled once all command lists in the batch have completed executing.
    pub fence: Option<&'a dyn IFence>,
}

#[derive(Clone)]
pub struct QueuePresentDesc<'a> {
    /// The [ISwapChain] to queue a present operation for.
    pub swap_chain: &'a dyn ISwapChain,

    /// The index of the image to queue a present operation for.
    pub image_index: u32,

    /// A list of semaphores that will block the execution of the batch until all semaphores in the
    /// list are signaled.
    pub wait_semaphores: &'a [&'a dyn ISemaphore],
}

//
//
// _________________________________________________________________________________________________
// Fence

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FenceWaitResult {
    /// The wait condition was met and the call has returned successfully.
    Complete,

    /// The timeout time was reached before the condition was met.
    Timeout,
}

//
//
// _________________________________________________________________________________________________
// Command Lists

pub struct CommandListDesc<'a> {
    /// The type of queue this command list will be compatible with. This affects what kinds of
    /// commands can be recorded and what kind of queues the list can be submitted to.
    pub queue_type: QueueType,

    /// The name of the object
    pub name: Option<&'a str>,
}

//
//
// _________________________________________________________________________________________________
// Command Options

/// An enumeration of all possible input types to a color texture clear operation
#[derive(Clone, Debug, PartialEq)]
pub enum ColorClearValue {
    /// A full 4-channel f32 colour
    Float { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    Int(u32),
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
                write!(f, "ColorClearValue::Float({r}, {g}, {b}, {a})")
            }
            ColorClearValue::Int(v) => {
                write!(f, "ColorClearValue::Int({:X})", *v)
            }
        }
    }
}

/// An enumeration of all possible input types to a depth/stencil texture clear operation
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepthStencilClearValue {
    pub depth: f32,
    pub stencil: u8,
}

/// Enum flags for barrier commands for specifying queue ownership transition behavior.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct QueueTransition {
    /// The queue that the resource is being transferred _from_ to another queue
    pub before_queue: QueueType,

    /// The queue that the resource is being transferred _to_ from another queue
    pub after_queue: QueueType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextureSubResourceSet {
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub num_mip_levels: u32,
    pub base_array_slice: u32,
    pub num_array_slices: u32,
}

/// Describes a global memory barrier
#[derive(Clone, Debug)]
pub struct GlobalBarrier {
    pub before_sync: BarrierSync,
    pub after_sync: BarrierSync,

    pub before_access: BarrierAccess,
    pub after_access: BarrierAccess,
}

/// Describes a resource barrier that will apply to an [IBuffer] resource on a command queue
#[derive(Clone)]
pub struct BufferBarrier<'a> {
    /// The buffer that the barrier will describe a state transition for
    pub buffer: &'a dyn IBuffer,

    /// The offset from the start of the buffer, in bytes, the barrier applies to.
    pub offset: u64,

    /// The size of the affected region of the buffer, in bytes, or `u64::MAX` to indicate the whole
    /// buffer.
    pub size: u64,

    pub before_sync: BarrierSync,
    pub after_sync: BarrierSync,

    pub before_access: BarrierAccess,
    pub after_access: BarrierAccess,

    /// Enables describing a queue ownership transition. Ownership of resources must be explicitly
    /// passed from one queue to another to be used across multiple queues.
    pub queue_transition: Option<QueueTransition>,
}

impl<'a> Debug for BufferBarrier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferBarrier")
            .field("buffer", &"<ptr>")
            .field("before_sync", &self.before_sync)
            .field("after_sync", &self.after_sync)
            .field("before_access", &self.before_access)
            .field("after_access", &self.after_access)
            .field("queue_transition", &self.queue_transition)
            .finish()
    }
}

/// Describes a resource barrier that will apply to an [ITexture] resource on a command queue
#[derive(Clone)]
pub struct TextureBarrier<'a> {
    /// The texture that the barrier will describe a state transition for
    pub texture: &'a dyn ITexture,

    pub subresource_range: TextureSubResourceSet,

    pub before_sync: BarrierSync,
    pub after_sync: BarrierSync,

    pub before_access: BarrierAccess,
    pub after_access: BarrierAccess,

    pub before_layout: ImageLayout,
    pub after_layout: ImageLayout,

    /// Enables describing a queue ownership transition. Ownership of resources must be explicitly
    /// passed from one queue to another to be used across multiple queues.
    pub queue_transition: Option<QueueTransition>,
}

impl<'a> Debug for TextureBarrier<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureBarrier")
            .field("texture", &"<ptr>")
            .field("subresource_range", &self.subresource_range)
            .field("before_sync", &self.before_sync)
            .field("after_sync", &self.after_sync)
            .field("before_access", &self.before_access)
            .field("after_access", &self.after_access)
            .field("before_layout", &self.before_layout)
            .field("after_layout", &self.after_layout)
            .field("queue_transition", &self.queue_transition)
            .finish()
    }
}

#[derive(Clone)]
pub struct InputAssemblyBufferBinding<'a> {
    pub buffer: &'a dyn IBuffer,
    pub offset: u64,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum IndexType {
    U16,
    U32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[derive(Clone, Debug)]
pub struct RenderingColorAttachmentInfo {
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp<ColorClearValue>,
    pub store_op: AttachmentStoreOp,
}

#[derive(Clone, Debug)]
pub struct RenderingDepthStencilAttachmentInfo {
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub depth_load_op: AttachmentLoadOp<DepthStencilClearValue>,
    pub depth_store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp<DepthStencilClearValue>,
    pub stencil_store_op: AttachmentStoreOp,
}

#[derive(Clone, Debug)]
pub struct BeginRenderingInfo<'a> {
    pub layer_count: u32,
    pub extent: Extent2D,
    pub color_attachments: &'a [RenderingColorAttachmentInfo],
    pub depth_stencil_attachment: Option<&'a RenderingDepthStencilAttachmentInfo>,
}

/// A simple description of a buffer -> buffer copy
#[derive(Clone, Debug)]
pub struct BufferCopyRegion {
    /// Offset in bytes from the start of the source buffer to copy from
    pub src_offset: u64,

    /// Offset in bytes from the start of the destination buffer to start copying into
    pub dst_offset: u64,

    /// Number of bytes to copy from the source buffer into the destination buffer
    pub size: u64,
}

/// A description of an image's data inside buffer memory
#[derive(Clone, Debug)]
pub struct ImageDataLayout {
    /// Offset in bytes from the start of the buffer that the image data begins at
    pub offset: u64,

    /// The extents of the image data.
    ///
    /// Minimum stride is 256 bytes, so `<width> * <format bytes per texel>` must be a multiple of
    /// 256.
    pub extent: Extent3D,
}

/// A description of a region within a texture for a buffer -> texture copy operation
#[derive(Clone, Debug)]
pub struct TextureCopyInfo {
    /// The mip layer to copy into
    pub mip_level: u32,

    /// The array layer to copy into
    pub array_layer: u32,

    /// The image aspect to copy into
    pub aspect: TextureCopyAspect,

    /// The origin of the region to copy into
    pub origin: UOffset3D,

    /// The extent of the region to copy into
    pub extent: Extent3D,
}

/// An enumeration of all possible 'image aspects' for a texture copy
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureCopyAspect {
    // TODO: Pick a better name
    Color,
    Depth,
    Stencil,
}

impl TextureCopyAspect {
    /// Returns the [TextureAspect] flag for the aspect the variant `self` represents.
    pub const fn as_flag(self) -> TextureAspect {
        match self {
            TextureCopyAspect::Color => TextureAspect::COLOR,
            TextureCopyAspect::Depth => TextureAspect::DEPTH,
            TextureCopyAspect::Stencil => TextureAspect::STENCIL,
        }
    }
}

impl From<TextureCopyAspect> for TextureAspect {
    fn from(val: TextureCopyAspect) -> Self {
        val.as_flag()
    }
}

bitflags! {
    #[derive(Default)]
    pub struct TextureAspect: u32 {
        /// Bit that specifies the 'color' aspect of a texture
        const COLOR = 0b00000001;

        /// Bit that specifies the 'depth' aspect of a texture
        const DEPTH = 0b00000010;

        /// Bit that specifies the 'stencil' aspect of a texture
        const STENCIL = 0b00000100;

        /// A combination of the [TextureAspect::DEPTH] and [TextureAspect::STENCIL] flags
        const DEPTH_STENCIL = Self::DEPTH.bits | Self::STENCIL.bits;
    }
}

/// A description of a buffer to texture copy operation
#[derive(Clone, Debug)]
pub struct BufferToTextureCopyRegion {
    /// A description of the source image in the source buffer.
    ///
    /// This is included here, instead of in [ITransferEncoder::copy_buffer_to_texture], so that
    /// copies from multiple sources can be queued in a single command. Some backends (read: Vulkan)
    /// can emit copies containing a list of source and destination regions as a single command.
    pub src: ImageDataLayout,

    /// The destination region inside the destination texture to copy the source data into.
    pub dst: TextureCopyInfo,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PipelineBindPoint {
    Compute,
    Graphics,
}

//
// =================================================================================================
// ERROR TYPES
// =================================================================================================
//

//
//
// _________________________________________________________________________________________________
// Surface

/// Set of errors that can occur when creating an [ISurface]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SurfaceCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// SwapChain

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

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SwapChainRebuildError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ImageAcquireError {
    #[error("The swap chain is out of date and needs to be rebuilt")]
    OutOfDate,

    /// This 'error' is a soft failure case for [ISwapChain::acquire_next_image]. In some cases it
    /// is possible for the swapchain to be placed in a state where it does not fully match the
    /// underlying surface being rendered too. For example, when the window is resized but the
    /// surface isn't lost. This can happen on composited platforms where they stretch/squash the
    /// swap images into the real surface.
    ///
    /// This is not a hard error, and it is perfectly valid to continue using and presenting to a
    /// sub-optimal swapchain. It is, however, recommended that the swapchain be rebuilt to
    /// correctly match the underlying surface again. This error variant flags the sub-optimal case
    /// for the caller to handle.
    #[error("The swapchain is sub-optimal for the surface and should be rebuilt")]
    SubOptimal(u32),

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

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Device

/// Set of errors that can occur when creating an [IDevice]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Fence

#[derive(Error, Debug)]
pub enum FenceCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Semaphore

#[derive(Error, Debug)]
pub enum SemaphoreCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource

/// Set of errors that can occur when mapping an [IBuffer]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResourceMapError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),

    #[error("The backend got a null pointer when attempting to map the buffer memory")]
    MappedNullPointer,
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Buffer

#[derive(Error, Debug)]
pub enum BufferCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Texture

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

//
//
// _________________________________________________________________________________________________
// Resource Construction - Sampler

#[derive(Error, Debug)]
pub enum SamplerCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Resource Construction - Shader

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

    /// This error occurs when the entry point name string is invalid. The primary trigger for this
    /// will be getting dodgy null-terminated strings as '&str'.
    ///
    /// Do not 'pre-null-terminate' the entry point names.
    #[error("The string provided for the entry point name is invalid")]
    InvalidEntryPointName,

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

//
//
// _________________________________________________________________________________________________
// Descriptors

#[derive(Error, Debug)]
pub enum DescriptorSetLayoutCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DescriptorPoolCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),

    #[error("There is not enough descriptor memory to create a pool with the requested capacity")]
    OutOfMemory,
}

#[derive(Error, Debug)]
pub enum DescriptorPoolAllocateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),

    #[error("The descriptor pool's backing memory has been exhausted due to pool fragmentation")]
    FragmentedPool,

    #[error("The descriptor pool's backing memory has been exhausted")]
    OutOfPoolMemory,

    #[error("The host or device's memory has been exhausted")]
    OutOfMemory,
}

//
//
// _________________________________________________________________________________________________
// Pipelines

#[derive(Error, Debug)]
pub enum PipelineLayoutCreateError {
    #[error("A push constant block has an invalid size")]
    InvalidPushConstantBlockSize,

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GraphicsPipelineCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum ComputePipelineCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// CommandPool

#[derive(Error, Debug)]
pub enum CommandPoolCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Command List

#[derive(Error, Debug)]
pub enum CommandListCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum CommandListBeginError {
    #[error("The command list does not support encoding commands for a '{0}' queue")]
    InvalidEncoderType(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

//
//
// _________________________________________________________________________________________________
// Queue

#[derive(Error, Debug)]
pub enum QueueSubmitError {
    #[error("The queue does not support submitting '{0}' commands")]
    InvalidEncoderType(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum QueuePresentError {
    #[error("The queue '{0}' does not support presentation to the requested swap chain")]
    QueuePresentationNotSupported(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
