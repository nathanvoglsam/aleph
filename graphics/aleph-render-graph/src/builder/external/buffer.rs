//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::utils::debug_check_buffer_read_access_type;
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, AccessFlags, Buffer, BufferUsageFlags, PipelineStageFlagBits,
    PipelineStageFlags,
};

///
///
///
pub struct BufferImport {
    /// The buffer for this buffer resource.
    pub(crate) buffer: Buffer,

    /// The supported usages for this buffer
    pub(crate) usage_flags: BufferUsageFlags,

    /// The size of the buffer
    pub(crate) size: usize,

    /// The pipeline stages that the buffer was used in prior to being imported into the graph
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this buffer was used as prior to being imported into the graph
    pub(crate) access_types: AccessFlags,
}

impl BufferImport {
    ///
    /// Construct a new BufferReadDescription for buffer with the given `identifier` where the
    /// buffer was created with the provided usage flags and is of the given size.
    ///
    /// `usage_flags` refers to the `BufferUsageFlags` the buffer was created with and so represents
    /// the set of operation the buffer can be used in. `size` is the size in bytes of the buffer.
    /// These values are used for validation if a pass so wishes.
    ///
    pub fn new(buffer: Buffer, usage_flags: BufferUsageFlags, size: usize) -> Self {
        Self {
            buffer,
            usage_flags,
            size,
            stages: Default::default(),
            access_types: Default::default(),
        }
    }

    ///
    /// Add the given stage to the set of stages the buffer was used in prior to being imported into
    /// the graph. This maps to `srcStageMask` as used in `vkCmdPipelineBarrier` and
    /// `vkCmdWaitEvents` in what ever barrier is injected by the render graph to synchronize with
    /// the external usage of the buffer.
    ///
    pub fn in_stage(mut self, stage: PipelineStageFlagBits) -> Self {
        self.stages |= stage.bitmask();
        self
    }

    ///
    /// Add the given access type to the set of accesses that this buffer was used in prior to being
    /// imported into the graph. This maps to `srcAccessMask` in `VkBufferMemoryBarrier` in whatever
    /// barrier is injected by the render graph to synchronize with the external usage of the
    /// buffer.
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        self.access_types |= access.bitmask();
        self
    }
}

///
///
///
pub struct BufferExport {
    /// The pipeline stages we'll be using the buffer in
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this buffer will be used as
    pub(crate) access_types: AccessFlags,
}

impl BufferExport {
    ///
    /// Construct a new BufferReadDescription for a buffer with the given `identifier`
    ///
    pub fn new() -> Self {
        Self {
            stages: Default::default(),
            access_types: Default::default(),
        }
    }

    ///
    /// Add the given stage to the set of stages the buffer will be used in
    ///
    pub fn in_stage(mut self, stage: PipelineStageFlagBits) -> Self {
        self.stages |= stage.bitmask();
        self
    }

    ///
    /// Add the given access type to the set of access types this buffer will be used
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_buffer_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}
