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

use crate::utils::debug_check_buffer_read_access_type;
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, AccessFlags, PipelineStageFlagBits, PipelineStageFlags,
};

///
/// Describes a write to a buffer resource with the given `identifier`. There are further functions
/// that must be called on the BufferWriteDescription in order to specify the pipeline stages the
/// buffer will be used in as well as the access flags to state how it will be used.
///
pub struct BufferWriteDescription {
    /// The identifier for the buffer we're writing
    pub(crate) identifier: String,

    /// The pipeline stages we'll be writing to the buffer in
    pub(crate) stages: PipelineStageFlags,

    /// The types of access we will be writing to the buffer in
    pub(crate) access_types: AccessFlags,
}

impl BufferWriteDescription {
    ///
    /// Construct a new BufferWriteDescription for a buffer with the given `identifier`
    ///
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
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
    /// # Panics
    ///
    /// `debug_assert!()` is used to check if nonsensical access flags are provided
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        self.access_types |= access.bitmask();
        self
    }
}

///
/// Describes a read to a buffer resource with the given `identifier`. There are further functions
/// that must be called on the BufferReadDescription in order to specify the pipeline stages the
/// buffer will be used in as well as the access flags to state how it will be used.
///
pub struct BufferReadDescription {
    /// The identifier for the buffer we're reading
    pub(crate) identifier: String,

    /// The pipeline stages we'll be reading the buffer in
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this buffer will be read as
    pub(crate) access_types: AccessFlags,
}

impl BufferReadDescription {
    ///
    /// Construct a new BufferReadDescription for a buffer with the given `identifier`
    ///
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
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
    /// # Panics
    ///
    /// `debug_assert!()` is used to check if nonsensical access flags are provided
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_buffer_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}
