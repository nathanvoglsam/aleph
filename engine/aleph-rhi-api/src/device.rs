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

use std::num::NonZeroU64;
use std::ptr::NonNull;

use aleph_any::{AnyArc, IAny};
use thiserror::Error;

use crate::*;

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

    fn create_parameter_block_layout(
        &self,
        desc: &ParameterBlockDesc,
    ) -> Result<AnyArc<dyn IParameterBlockLayout>, ParameterBlockLayoutCreateError>;

    fn create_binding_signature(
        &self,
        desc: &BindingSignatureDesc,
    ) -> Result<AnyArc<dyn IBindingSignature>, BindingSignatureCreateError>;

    fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineHandle, PipelineCreateError>;

    fn create_compute_pipeline(
        &self,
        desc: &ComputePipelineDesc,
    ) -> Result<ComputePipelineHandle, PipelineCreateError>;

    fn create_descriptor_pool(
        &self,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError>;

    fn create_descriptor_arena(
        &self,
        desc: &DescriptorArenaDesc,
    ) -> Result<Box<dyn IDescriptorArena>, DescriptorPoolCreateError>;

    fn create_buffer(&self, desc: &BufferDesc) -> Result<BufferHandle, BufferCreateError>;

    fn create_texture(&self, desc: &TextureDesc) -> Result<TextureHandle, TextureCreateError>;

    fn create_sampler(&self, desc: &SamplerDesc) -> Result<SamplerHandle, SamplerCreateError>;

    fn create_command_list(
        &self,
        desc: &CommandListDesc,
    ) -> Result<Box<dyn ICommandList>, CommandListCreateError>;

    fn get_queue(&self, queue_type: QueueType) -> Option<AnyArc<dyn IQueue>>;

    /// Perform the given list of parameter block updates.
    ///
    /// This function takes a flattened list of write parameters in 'writes'. Each write is a
    /// [`ParameterWrite`] struct that must be filled out with the appropriate information. The
    /// variant of each write must be correct w.r.t. the given parameter block layout. The given
    /// parameter block layout must be exactly the same layout (not the same object) as the
    /// parameter block was allocated from a pool with.
    ///
    /// The binding hierarchy is flattened for the purposes of writes. Specifically parameter arrays
    /// are flattened, and 'base' indexes into this flattened parameter space.
    ///
    /// We will illustrate using this example layout: [Texture2D, [Texture2D; 5], ConstantBuffer, Sampler]
    /// - There are 8 total parameters in this parameter block.
    /// - 'base' can not be 8 or larger.
    /// - 'base = 0' refers to layout[0].
    /// - 'base = 1' refers to layout[1][0].
    /// - 'base = 2' refers to layout[1][1].
    /// - 'base = 6' refers to layout[2].
    /// - 'base = 7' refers to layout[3].
    ///
    /// Notice how the array binding has been flattened.
    ///
    /// # Why?
    ///
    /// By flattening the parameter space we significantly reduce the amount of struct noise we need
    /// to encode the parameter updates. If we made this explicit we'd required multiple nested
    /// arrays of fields. Authoring code this way is very painful and error-prone.
    ///
    /// The nested field approach also makes it difficult for data driven rendering to generate
    /// parameter block updates as allocating the nested arrays requires arenas (complexity) or a
    /// duplicate 'owned' version of [`ParameterWrite`].
    ///
    /// The two most common ways we predict 'update_parameter_block' to be called are:
    /// - Infrequent updates to persistent parameter blocks, such as for materials. These will not
    ///   be super frequent, so the additional cost is small and the simpler interface may lead
    ///   to more efficient code elsewhere.
    /// - Once-per-frame updates to ephemeral parameters blocks for per-pass parameters. This will
    ///   be a very hot code-path. The vast majority of these calls will update the entire block in
    ///   a single call. We can special case these to reduce the overhead.
    ///
    /// # Safety
    ///
    /// Accesses to the parameter blocks referenced via [`ParameterBlockHandle`] are not
    /// synchronized. A descriptor write requires mutable (exclusive) access to the individual
    /// block. It is unsafe to call this function on the same [`ParameterBlockHandle`] from multiple
    /// threads without external synchronization.
    ///
    /// It is unsafe to try and write to a [`ParameterBlockHandle`] after it has been freed.
    ///
    /// It is the caller's responsibility to ensure that each [`ParameterWrite`] variant provided
    /// in 'writes' is the correct variant for every parameter slot it is associated to. Failing to
    /// do so will cause undefined behavior. For example: A caller must ensure for a parameter block
    /// layout with the layout '[Texture2D, ConstantBuffer, Sampler]' that the 'writes' array
    /// contains the following variants '[Texture, Buffer, Sampler]'.
    ///
    /// It is the caller's responsibility to ensure all texture views provided in the 'writes'
    /// array are valid for the usage the parameter block layout expects.
    ///
    /// It is the caller's responsibility to ensure all buffers provided in the 'writes' array are
    /// valid for the usages the parameter block layout expects.
    ///
    /// It is the caller's responsibility to ensure the given [`IParameterBlockLayout`] is the same
    /// layout (not necessarily the same object, but the layouts must exactly match) that the
    /// parameter block was allocated from a pool with.
    ///
    /// # Warning
    ///
    /// Some implementations may re-use handles, where allocating a new block may return a
    /// previously freed block using the same handle. The implication is that use-after free will
    /// not cause immediate UB or validation errors on the platform API in some cases due to
    /// implementation detail. Instead, you will observe 'spooky action at a distance' where two
    /// systems think they own the block, when instead they're sharing one, and they clobber each
    /// other's descriptors or have synchronization issues if they're being shared across threads.
    ///
    /// The take-away here is to be very careful with parameter blocks, buggy usage will be very
    /// hard to debug. Test with as many implementations as you can, especially Vulkan. Most of our
    /// descriptor API is based on Vulkan as it's the 'lowest common denominator', and can be
    /// implemented as thin wrappers to Vulkan. This is useful, being a thin wrapper to Vulkan means
    /// Vulkan's validation layers will also validate our own API if we mirror their semantics as
    /// close as we can.
    ///
    /// D3D12 will be very permissive to errors as D3D12's descriptor model is much less
    /// restrictive.
    unsafe fn update_parameter_block(
        &self,
        layout: &dyn IParameterBlockLayout,
        block: ParameterBlockHandle,
        base: u32,
        writes: &[ParameterWrite],
    );

    /// Constructs a new fence in the requested state.
    fn create_fence(&self, signalled: bool) -> Result<FenceHandle, FenceCreateError>;

    /// Constructs a new semaphore in the default (reset) state.
    fn create_semaphore(&self) -> Result<SemaphoreHandle, SemaphoreCreateError>;

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
    fn wait_fences(&self, fences: &[&FenceHandle], wait_all: bool, timeout: u32)
    -> FenceWaitResult;

    /// Polls, and returns, whether the fence has been signalled by the device.
    fn poll_fence(&self, fence: &FenceHandle) -> bool;

    /// Resets all the given fences to the default state, ready to be used again on a queue.
    fn reset_fences(&self, fences: &[&FenceHandle]);

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;

    // ================
    // BUFFER
    // ================

    /// Returns a globally unique ID that is guaranteed to not be shared by any other buffer object
    /// allocated from the same [`IDevice`] instance.
    fn get_buffer_id(&self, buffer: &BufferHandle) -> NonZeroU64;

    /// Returns a [BufferDesc] that describes this buffer object
    fn get_buffer_desc<'b>(&self, buffer: &'b BufferHandle) -> &'b BufferDesc<'b>;

    /// Returns a host virtual address pointer to a region of a mappable buffer.
    ///
    /// [IDevice::map_buffer] will map the entire buffer.
    ///
    /// Writes to buffer memory through a mapped pointer won't become available to the device until
    /// after a submission to an [IQueue], or when signalling an event/fence to the GPU. The writes
    /// will only be made available to the device commands when submitted, or when waiting for an
    /// event/fence to be triggered from the CPU.
    fn map_buffer(&self, buffer: &BufferHandle) -> Result<NonNull<u8>, ResourceMapError>;

    /// Unmaps the buffers memory, releasing the associated address space range to be reused.
    fn unmap_buffer(&self, buffer: &BufferHandle) -> Result<(), ResourceUnmapError>;

    /// Flushes any writes to mapped buffer memory for non `HOST_COHERENT` memory.
    ///
    /// Writes to non `HOST_COHERENT` memory will no be made available to the device until the
    /// written range is flushed with this function.
    ///
    /// This should be combined with an event/fence for writes from the host to become available
    ///
    /// Mapped memory that is considered `HOST_COHERENT` does not need to be flushed.
    fn flush_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64);

    /// Invalidate the requested region inside the mapped buffer memory for non `HOST_COHERENT`
    /// memory.
    ///
    /// Device writes to non `HOST_COHERENT` mapped memory will not be available to the host until
    /// this function is called for the region to be read.
    ///
    /// Mapped memory that is considered `HOST_COHERENT` does not need to be invalidated.
    fn invalidate_buffer_range(&self, buffer: &BufferHandle, offset: u64, len: u64);

    // ================
    // TEXTURE
    // ================

    /// Returns a globally unique ID that is guaranteed to not be shared by any other texture
    /// allocated from the same [`IDevice`] instance.
    fn get_texture_id(&self, texture: &TextureHandle) -> NonZeroU64;

    /// Returns a [TextureDesc] that describes this texture
    fn get_texture_desc<'b>(&self, texture: &'b TextureHandle) -> &'b TextureDesc<'b>;

    fn get_texture_view(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()>;

    fn get_texture_rtv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()>;

    fn get_texture_dsv(
        &self,
        texture: &TextureHandle,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()>;

    // ================
    // SAMPLER
    // ================

    /// Returns a globally unique ID that is guaranteed to not be shared by any other sampler
    /// allocated from the same [`IDevice`] instance.
    fn get_sampler_id(&self, sampler: &SamplerHandle) -> NonZeroU64;

    /// Returns a [SamplerDesc] that describes this sampler
    fn get_sampler_desc<'b>(&self, sampler: &'b SamplerHandle) -> &'b SamplerDesc<'b>;

    // ================
    // PIPELINE
    // ================

    /// Returns a globally unique ID that is guaranteed to not be shared by any other pipeline
    /// allocated from the same [`IDevice`] instance.
    fn get_graphics_pipeline_id(&self, pipeline: &GraphicsPipelineHandle) -> NonZeroU64;

    /// Returns a globally unique ID that is guaranteed to not be shared by any other pipeline
    /// allocated from the same [`IDevice`] instance.
    fn get_compute_pipeline_id(&self, pipeline: &ComputePipelineHandle) -> NonZeroU64;
}

#[derive(Error, Debug)]
pub enum FenceCreateError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(FenceCreateError);

#[derive(Error, Debug)]
pub enum SemaphoreCreateError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(SemaphoreCreateError);

/// Set of errors that can occur when mapping a buffer
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResourceMapError {
    #[error("The backend got a null pointer when attempting to map the buffer memory")]
    MappedNullPointer,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(ResourceMapError);

/// Set of errors that can occur when unmapping a buffer
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResourceUnmapError {
    #[error("The buffer was not mapped yet was asked to be unmapped.")]
    NotMapped,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(ResourceUnmapError);

#[derive(Error, Debug)]
pub enum BufferCreateError {
    #[error("There was not enough memory available to serve the requested buffer.")]
    OutOfMemory,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(BufferCreateError);

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

    #[error("There was not enough memory available to serve the requested texture.")]
    OutOfMemory,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(TextureCreateError);

#[derive(Error, Debug)]
pub enum SamplerCreateError {
    #[error("The device has run out of space to allocate additional samplers.")]
    OutOfSamplers,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(SamplerCreateError);

#[derive(Error, Debug)]
pub enum ParameterBlockLayoutCreateError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(ParameterBlockLayoutCreateError);

#[derive(Error, Debug)]
pub enum DescriptorPoolCreateError {
    #[error("There is not enough descriptor memory to create a pool with the requested capacity")]
    OutOfMemory,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(DescriptorPoolCreateError);

#[derive(Error, Debug)]
pub enum BindingSignatureCreateError {
    #[error("A push constant block has an invalid size")]
    InvalidPushConstantBlockSize,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(BindingSignatureCreateError);

#[derive(Error, Debug)]
pub enum PipelineCreateError {
    /// The shader code for the given stage was found to be invalid.
    #[error("The shader code for stage '{0}' is invalid")]
    InvalidShaderCode(ShaderType),

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
    #[error("The shader [{0}] binary size '{1}' is invalid")]
    InvalidInputSize(usize, usize),

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(PipelineCreateError);

#[derive(Error, Debug)]
pub enum CommandPoolCreateError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(CommandPoolCreateError);

#[derive(Error, Debug)]
pub enum CommandListCreateError {
    #[error("The device does not have a queue of type '{0}' available.")]
    NoSuchQueue(QueueType),

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
error_enum_from_unit_type!(CommandListCreateError);
