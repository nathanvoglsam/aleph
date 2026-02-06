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

use aleph_nstr::NStr;

use crate::*;

#[repr(transparent)]
pub struct CommandEncoder<'a>(Box<dyn ICommandEncoderAbi + 'a>);

impl<'a> CommandEncoder<'a> {
    /// Internal function that must be made available for backends to construct instances of this
    /// type.
    ///
    /// Do not use.
    #[doc(hidden)]
    pub unsafe fn from_abi(v: Box<dyn ICommandEncoderAbi + 'a>) -> Self {
        Self(v)
    }

    /// Internal function that must be made available for backends.
    ///
    /// Do not use.
    #[doc(hidden)]
    pub unsafe fn into_abi(self) -> Box<dyn ICommandEncoderAbi + 'a> {
        // Safety: the transmute is safe because repr(transparent).
        unsafe {
            // Copy and forget the box out of 'self' so we don't run the drop impl.
            let v: Box<dyn ICommandEncoderAbi + 'a> = std::mem::transmute_copy(&self.0);
            std::mem::forget(self);
            v
        }
    }

    #[inline]
    pub unsafe fn begin_rendering(
        &mut self,
        info: &BeginRenderingInfo,
        debug_name: &NStr,
    ) -> RenderEncoder<'_> {
        if cfg!(feature = "markers") {
            unsafe {
                self.0.__begin_event(Color::GREEN, debug_name);
            }
        }
        unsafe {
            self.0.__begin_rendering(info);
        }

        RenderEncoder(self.0.as_mut())
    }

    #[inline]
    pub unsafe fn begin_compute(&mut self, debug_name: &NStr) -> ComputeEncoder<'_> {
        if cfg!(feature = "markers") {
            unsafe {
                self.0.__begin_event(Color::BLUE, debug_name);
            }
        }

        ComputeEncoder(self.0.as_mut())
    }

    #[inline]
    pub unsafe fn begin_transfer(&mut self, debug_name: &NStr) -> TransferEncoder<'_> {
        if cfg!(feature = "markers") {
            unsafe {
                self.0.__begin_event(Color::MAGENTA, debug_name);
            }
        }

        TransferEncoder(self.0.as_mut())
    }

    #[inline(always)]
    pub unsafe fn resource_barrier(
        &mut self,
        memory_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        unsafe {
            self.0
                .__resource_barrier(memory_barriers, buffer_barriers, texture_barriers)
        }
    }

    #[inline(always)]
    pub unsafe fn close(self) -> Result<(), CommandListCloseError> {
        unsafe {
            let mut inner = Self::into_abi(self);
            inner.__close()
        }
    }

    #[inline]
    pub fn debug_zone(&mut self, color: Color, message: &NStr, f: impl FnOnce(&mut Self)) {
        unsafe {
            if cfg!(feature = "markers") {
                self.0.__begin_event(color, message);
            }

            f(self);

            if cfg!(feature = "markers") {
                self.0.__end_event();
            }
        }
    }
}

impl<'a> Drop for CommandEncoder<'a> {
    fn drop(&mut self) {
        // Safety: lmao. realistically we're only as safe as the commands we recorded so the safety
        //         contract is on the commands.
        unsafe {
            self.0.__close().unwrap();
        }
    }
}

#[repr(transparent)]
pub struct RenderEncoder<'a>(&'a mut dyn ICommandEncoderAbi);

impl<'a> RenderEncoder<'a> {
    #[inline(always)]
    pub unsafe fn bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        unsafe { self.0.__bind_graphics_pipeline(pipeline) }
    }

    #[inline(always)]
    pub unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        unsafe { self.0.__bind_vertex_buffers(first_binding, bindings) }
    }

    #[inline(always)]
    pub unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        unsafe { self.0.__bind_index_buffer(index_type, binding) }
    }

    #[inline(always)]
    pub unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        unsafe { self.0.__set_viewports(viewports) }
    }

    #[inline(always)]
    pub unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        unsafe { self.0.__set_scissor_rects(rects) }
    }

    #[inline(always)]
    pub unsafe fn bind_parameter_blocks(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        first_block: u32,
        blocks: &[ParameterBlockHandle],
    ) {
        unsafe {
            self.0.__bind_parameter_blocks(
                binding_signature,
                PipelineBindPoint::Graphics,
                first_block,
                blocks,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn push_parameters(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        block: u32,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        unsafe {
            self.0.__push_parameters(
                binding_signature,
                PipelineBindPoint::Graphics,
                block,
                base,
                writes,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn set_push_constant_block(&mut self, data: &[u8]) {
        unsafe { self.0.__set_push_constant_block(data) }
    }

    #[inline(always)]
    pub unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.0
                .__draw(vertex_count, instance_count, first_vertex, first_instance)
        }
    }

    #[inline(always)]
    pub unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        unsafe {
            self.0.__draw_indexed(
                index_count,
                instance_count,
                first_index,
                first_instance,
                vertex_offset,
            )
        }
    }
}

impl<'a> Drop for RenderEncoder<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.0.__end_rendering();
            if cfg!(feature = "markers") {
                self.0.__end_event();
            }
        }
    }
}

#[repr(transparent)]
pub struct ComputeEncoder<'a>(&'a mut dyn ICommandEncoderAbi);

impl<'a> ComputeEncoder<'a> {
    #[inline(always)]
    pub unsafe fn bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        unsafe { self.0.__bind_compute_pipeline(pipeline) }
    }

    #[inline(always)]
    pub unsafe fn bind_parameter_blocks(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        first_block: u32,
        blocks: &[ParameterBlockHandle],
    ) {
        unsafe {
            self.0.__bind_parameter_blocks(
                binding_signature,
                PipelineBindPoint::Compute,
                first_block,
                blocks,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn push_parameters(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        block: u32,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        unsafe {
            self.0.__push_parameters(
                binding_signature,
                PipelineBindPoint::Compute,
                block,
                base,
                writes,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn set_push_constant_block(&mut self, data: &[u8]) {
        unsafe { self.0.__set_push_constant_block(data) }
    }

    #[inline(always)]
    pub unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            self.0
                .__dispatch(group_count_x, group_count_y, group_count_z)
        }
    }

    #[inline(always)]
    pub unsafe fn resource_barrier(
        &mut self,
        memory_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        unsafe {
            self.0
                .__resource_barrier(memory_barriers, buffer_barriers, texture_barriers)
        }
    }
}

impl<'a> Drop for ComputeEncoder<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        if cfg!(feature = "markers") {
            unsafe {
                self.0.__end_event();
            }
        }
    }
}

#[repr(transparent)]
pub struct TransferEncoder<'a>(&'a mut dyn ICommandEncoderAbi);

impl<'a> TransferEncoder<'a> {
    #[inline(always)]
    pub unsafe fn copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    ) {
        unsafe { self.0.__copy_buffer_regions(src, dst, regions) }
    }

    #[inline(always)]
    pub unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        unsafe { self.0.__copy_buffer_to_texture(src, dst, regions) }
    }

    #[inline(always)]
    pub unsafe fn copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        unsafe { self.0.__copy_texture_regions(src, dst, regions) }
    }

    #[inline(always)]
    pub unsafe fn resource_barrier(
        &mut self,
        memory_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        unsafe {
            self.0
                .__resource_barrier(memory_barriers, buffer_barriers, texture_barriers)
        }
    }
}

impl<'a> Drop for TransferEncoder<'a> {
    #[inline(always)]
    fn drop(&mut self) {
        if cfg!(feature = "markers") {
            unsafe {
                self.0.__end_event();
            }
        }
    }
}

/// The internal ABI/API that a backend should implement for a command encoder.
///
/// This is not expected to be used directly. Use [`CommandEncoder`] along with [`RenderEncoder`],
/// [`ComputeEncoder`], and [`TransferEncoder`]. Those expose the intended public API.
pub trait ICommandEncoderAbi: Send {
    unsafe fn __bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle);

    unsafe fn __bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    );

    unsafe fn __bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    );

    unsafe fn __set_viewports(&mut self, viewports: &[Viewport]);

    unsafe fn __set_scissor_rects(&mut self, rects: &[Rect]);

    unsafe fn __set_push_constant_block(&mut self, data: &[u8]);

    unsafe fn __begin_rendering(&mut self, info: &BeginRenderingInfo);

    unsafe fn __end_rendering(&mut self);

    unsafe fn __draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );

    unsafe fn __draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    );

    unsafe fn __bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle);

    unsafe fn __bind_parameter_blocks(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        first_block: u32,
        blocks: &[ParameterBlockHandle],
    );

    unsafe fn __push_parameters(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        block: u32,
        base: u32,
        writes: &[ParameterWrite],
    );

    unsafe fn __dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);

    unsafe fn __resource_barrier(
        &mut self,
        memory_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    );

    unsafe fn __copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    );

    unsafe fn __copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    );

    unsafe fn __copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    );

    unsafe fn __close(&mut self) -> Result<(), CommandListCloseError>;

    unsafe fn __set_marker(&mut self, color: Color, message: &NStr);

    unsafe fn __begin_event(&mut self, color: Color, message: &NStr);

    unsafe fn __end_event(&mut self);
}

/// Enum flags for barrier commands for specifying queue ownership transition behavior.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct QueueTransition {
    /// The queue that the resource is being transferred _from_ to another queue
    pub before_queue: QueueType,

    /// The queue that the resource is being transferred _to_ from another queue
    pub after_queue: QueueType,
}

/// Describes a global memory barrier
#[derive(Clone, Debug, Default)]
pub struct GlobalBarrier {
    pub before_sync: BarrierSync,
    pub after_sync: BarrierSync,

    pub before_access: BarrierAccess,
    pub after_access: BarrierAccess,
}

/// Describes a resource barrier that will apply to a buffer resource on a command queue
#[derive(Clone)]
pub struct BufferBarrier<'a> {
    /// The buffer that the barrier will describe a state transition for.
    ///
    /// This field is _required_ for the barrier to be valid to issue. It may be useful to construct
    /// barrier structs without a buffer stored in them _yet_ so the field is marked as optional.
    pub buffer: Option<&'a BufferHandle>,

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

impl<'a> std::fmt::Debug for BufferBarrier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Describes a resource barrier that will apply to a texture resource on a command queue
#[derive(Clone)]
pub struct TextureBarrier<'a> {
    /// The texture that the barrier will describe a state transition for
    ///
    /// This field is _required_ for the barrier to be valid to issue. It may be useful to construct
    /// barrier structs without a texture stored in them _yet_ so the field is marked as optional.
    pub texture: Option<&'a TextureHandle>,

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

impl<'a> std::fmt::Debug for TextureBarrier<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PipelineBindPoint {
    Compute,
    Graphics,
}

impl std::fmt::Display for PipelineBindPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineBindPoint::Compute => f.write_str("Compute"),
            PipelineBindPoint::Graphics => f.write_str("Graphics"),
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AttachmentLoadOp<ClearValue> {
    /// Specifies that the contents of the attachment are not important and can be safely ignored.
    /// The result of a read from a "don't care" attachment is undefined. The implementation is free
    /// to not even access the attachment.
    ///
    /// This is still logically a read as 'DontCare' is allowed to read the texture even if the
    /// results are supposed to be undefined. This is just a hint that we don't use the attachment's
    /// existing contents and the driver can do whatever is fastest or possible on the device.
    /// However, you are still required to synchronize as-if this was a read even if you never
    /// explicitly access the attachment. Drivers are allowed to issue reads if they aren't able
    /// to skip them!
    DontCare,

    /// Specifies that the attachment will be loaded from the data in memory
    Load,

    /// Specifies that the attachment will be cleared with a specified colour
    Clear(ClearValue),
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AttachmentStoreOp {
    /// Specifies that the results of rendering operations will be discarded and *may* not be
    /// written to memory. The contents of the attachment will become undefined.
    ///
    /// This operation is not a guarantee that the attachment will not be written to. The driver is
    /// still allowed to write to the attachment if it is not able/not efficient to skip the writes.
    ///
    /// You must still synchronize as if this operation writes to the attachment.
    DontCare,

    /// Specifies that the results of rendering operations will be written to the attachment's
    /// memory
    Store,
}

#[derive(Clone, Debug)]
pub struct RenderingColorAttachmentInfo {
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp<ColorClearValue>,
    pub store_op: AttachmentStoreOp,
}

impl RenderingColorAttachmentInfo {
    pub const fn new(image_view: ImageView) -> Self {
        Self {
            image_view,
            image_layout: ImageLayout::ColorAttachment,
            load_op: AttachmentLoadOp::DontCare,
            store_op: AttachmentStoreOp::DontCare,
        }
    }

    pub const fn with_layout(mut self, image_layout: ImageLayout) -> Self {
        self.image_layout = image_layout;
        self
    }

    pub const fn with_load_op(mut self, load_op: AttachmentLoadOp<ColorClearValue>) -> Self {
        self.load_op = load_op;
        self
    }

    pub const fn with_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        self.store_op = store_op;
        self
    }

    pub const fn load(self) -> Self {
        self.with_load_op(AttachmentLoadOp::Load)
    }

    pub const fn clear(self, value: ColorClearValue) -> Self {
        self.with_load_op(AttachmentLoadOp::Clear(value))
    }

    pub const fn load_dont_care(self) -> Self {
        self.with_load_op(AttachmentLoadOp::DontCare)
    }

    pub const fn store(self) -> Self {
        self.with_store_op(AttachmentStoreOp::Store)
    }

    pub const fn store_dont_care(self) -> Self {
        self.with_store_op(AttachmentStoreOp::DontCare)
    }
}

#[derive(Clone, Debug)]
pub struct RenderingDepthStencilAttachmentInfo {
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub depth: Option<AttachmentOps<f32>>,
    pub stencil: Option<AttachmentOps<u8>>,
}

impl RenderingDepthStencilAttachmentInfo {
    pub const fn new(image_view: ImageView) -> Self {
        Self {
            image_view,
            image_layout: ImageLayout::DepthStencilAttachment,
            depth: None,
            stencil: None,
        }
    }

    pub const fn with_layout(mut self, image_layout: ImageLayout) -> Self {
        self.image_layout = image_layout;
        self
    }

    pub const fn with_depth_load_op(mut self, load_op: AttachmentLoadOp<f32>) -> Self {
        match &mut self.depth {
            None => {
                self.depth = Some(AttachmentOps {
                    load_op,
                    store_op: AttachmentStoreOp::DontCare,
                });
            }
            Some(v) => {
                v.load_op = load_op;
            }
        }
        self
    }

    pub const fn with_depth_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        match &mut self.depth {
            None => {
                self.depth = Some(AttachmentOps {
                    load_op: AttachmentLoadOp::DontCare,
                    store_op,
                });
            }
            Some(v) => {
                v.store_op = store_op;
            }
        }
        self
    }

    pub const fn with_stencil_load_op(mut self, load_op: AttachmentLoadOp<u8>) -> Self {
        match &mut self.stencil {
            None => {
                self.stencil = Some(AttachmentOps {
                    load_op,
                    store_op: AttachmentStoreOp::DontCare,
                });
            }
            Some(v) => {
                v.load_op = load_op;
            }
        }
        self
    }

    pub const fn with_stencil_store_op(mut self, store_op: AttachmentStoreOp) -> Self {
        match &mut self.stencil {
            None => {
                self.stencil = Some(AttachmentOps {
                    load_op: AttachmentLoadOp::DontCare,
                    store_op,
                });
            }
            Some(v) => {
                v.store_op = store_op;
            }
        }
        self
    }

    pub const fn depth_load(self) -> Self {
        self.with_depth_load_op(AttachmentLoadOp::Load)
    }

    pub const fn depth_clear(self, value: f32) -> Self {
        self.with_depth_load_op(AttachmentLoadOp::Clear(value))
    }

    pub const fn depth_load_dont_care(self) -> Self {
        self.with_depth_load_op(AttachmentLoadOp::DontCare)
    }

    pub const fn depth_store(self) -> Self {
        self.with_depth_store_op(AttachmentStoreOp::Store)
    }

    pub const fn depth_store_dont_care(self) -> Self {
        self.with_depth_store_op(AttachmentStoreOp::DontCare)
    }

    pub const fn stencil_load(self) -> Self {
        self.with_stencil_load_op(AttachmentLoadOp::Load)
    }

    pub const fn stencil_clear(self, value: u8) -> Self {
        self.with_stencil_load_op(AttachmentLoadOp::Clear(value))
    }

    pub const fn stencil_load_dont_care(self) -> Self {
        self.with_stencil_load_op(AttachmentLoadOp::DontCare)
    }

    pub const fn stencil_store(self) -> Self {
        self.with_stencil_store_op(AttachmentStoreOp::Store)
    }

    pub const fn stencil_store_dont_care(self) -> Self {
        self.with_stencil_store_op(AttachmentStoreOp::DontCare)
    }
}

#[derive(Clone, Debug)]
pub struct AttachmentOps<ClearValue> {
    pub load_op: AttachmentLoadOp<ClearValue>,
    pub store_op: AttachmentStoreOp,
}

impl<ClearValue> AttachmentOps<ClearValue> {
    pub const fn new() -> Self {
        Self {
            load_op: AttachmentLoadOp::DontCare,
            store_op: AttachmentStoreOp::DontCare,
        }
    }
}

impl<ClearValue> Default for AttachmentOps<ClearValue> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct BeginRenderingInfo<'a> {
    pub layer_count: u32,
    pub extent: Extent2D,
    pub color_attachments: &'a [RenderingColorAttachmentInfo],
    pub depth_stencil_attachment: Option<&'a RenderingDepthStencilAttachmentInfo>,
    pub allow_uav_writes: bool,
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

#[derive(Clone, Debug)]
pub struct TextureSubresourceCopyInfo {
    /// The mip layer to copy to/from
    pub mip_level: u32,

    /// The array layer to copy to/from
    pub array_layer: u32,

    /// The image aspect to copy to/from
    pub aspect: TextureCopyAspect,

    /// The origin of the region to copy to/from
    pub offset: UOffset3D,
}

/// A description of a region within a texture for a buffer -> texture copy operation
#[derive(Clone, Debug)]
pub struct TextureToTextureCopyInfo {
    /// Description of the copy source
    pub src: TextureSubresourceCopyInfo,

    /// Description of the copy dest
    pub dst: TextureSubresourceCopyInfo,

    /// The extent of the region to copy
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

impl std::fmt::Display for TextureCopyAspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureCopyAspect::Color => f.write_str("Color"),
            TextureCopyAspect::Depth => f.write_str("Depth"),
            TextureCopyAspect::Stencil => f.write_str("Stencil"),
        }
    }
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

/// A description of an image's data inside buffer memory
#[derive(Clone, Debug)]
pub struct ImageDataLayout {
    /// Offset in bytes from the start of the buffer that the image data begins at.
    ///
    /// # Requirements
    ///
    /// For buffer to image copies this must be aligned to 512 bytes within the source buffer. This
    /// limit is imposed primarily by D3D12 but must be observed everywhere.
    pub offset: u64,

    /// The row pitch in texels.
    ///
    /// This describes the in-memory width of a row of texels in memory, which may need to be wider
    /// than the actual width of the texture. This should always be _at least_ equal to
    /// `extent.width`.
    pub row_pitch: u32,
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

#[derive(Clone)]
pub struct InputAssemblyBufferBinding<'a> {
    pub buffer: &'a BufferHandle,
    pub offset: u64,
}

impl<'a> InputAssemblyBufferBinding<'a> {
    pub const fn new(buffer: &'a BufferHandle) -> Self {
        Self { buffer, offset: 0 }
    }

    pub const fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }
}
