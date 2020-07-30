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

use crate::graph::PassIndex;
use aleph_vulkan_core::erupt::vk1_0::{AccessFlags, ImageLayout, PipelineStageFlags};

///
/// Holds all usage information for a single render graph resource
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ImageUsage {
    /// The pass that writes this value
    pub writen_by: PassIndex,

    /// Holds the number of ways that the image is subsequently read in
    pub read_types: Vec<ImageRead>,

    /// Which passes read from the resource and how. (pass index, read_type index)
    pub read_by: Vec<(PassIndex, usize)>,

    /// The layout this value will be written in
    pub layout: ImageLayout,

    /// The pipeline stages this value will be written in
    pub stages: PipelineStageFlags,

    /// The set of access types this image will be written in
    pub access_types: AccessFlags,
}

///
/// Represents the information needed for an image read
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct ImageRead {
    /// The layout this value will be read in
    pub layout: ImageLayout,

    /// The pipeline stages this value will be read in
    pub stages: PipelineStageFlags,

    /// The set of access types this image will be read in
    pub access_types: AccessFlags,
}

///
/// Holds all usage information for a single render graph resource
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct BufferUsage {
    /// The pass that writes this value
    pub writen_by: PassIndex,

    /// Holds the number of ways that the image is subsequently read in
    pub read_type: BufferRead,

    /// Which passes read from the resource and how. (pass index, read_type index). ignore read_type
    pub read_by: Vec<(PassIndex, usize)>,

    /// The pipeline stages this value will be written in
    pub stages: PipelineStageFlags,

    /// The set of access types this buffer will be written in
    pub access_types: AccessFlags,
}

///
/// Represents the information needed for buffer read
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub struct BufferRead {
    /// The pipeline stages this value will be read by
    pub stages: PipelineStageFlags,

    /// The ways this image will be written
    pub access_types: AccessFlags,
}

///
/// Represents a usage of one of the supported types of resources.
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ResourceUsage {
    Image(ImageUsage),
    Buffer(BufferUsage),
}

impl ResourceUsage {
    ///
    /// Returns the `PassIndex` of the pass that writes this resource
    ///
    pub fn writen_by(&self) -> PassIndex {
        match self {
            ResourceUsage::Image(v) => v.writen_by,
            ResourceUsage::Buffer(v) => v.writen_by,
        }
    }

    ///
    /// Returns an iterator of `PassIndex`s over all the readers of this resource
    ///
    pub fn read_by(&self) -> &[(PassIndex, usize)] {
        match self {
            ResourceUsage::Image(v) => &v.read_by,
            ResourceUsage::Buffer(v) => &v.read_by,
        }
    }
}
