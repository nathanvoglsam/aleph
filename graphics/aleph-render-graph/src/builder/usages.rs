//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{AccessFlags, ImageLayout, PipelineStageFlags};

#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ImageUsage {
    /// The pass that writes this value
    pub writen_by: usize,

    /// Holds the number of ways that the image is subsequently read in
    pub read_types: Vec<ImageRead>,

    /// Which passes read from the resource and how. (pass index, read_type index)
    pub read_by: Vec<(usize, usize)>,

    /// The layout this value will be written in
    pub layout: ImageLayout,

    /// The pipeline stages this value will be written by
    pub stages: PipelineStageFlags,

    /// The ways this image will be written
    pub access_types: AccessFlags,
}

///
/// Represents the information needed for an image read
///
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ImageRead {
    /// The layout this value will be read in
    pub layout: ImageLayout,

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
}

impl ResourceUsage {
    ///
    /// If this is a write usage, return the index of the pass that wrote it. Returns `None` if this
    /// is a read access
    ///
    pub fn writen_by(&self) -> usize {
        match self {
            ResourceUsage::Image(v) => v.writen_by,
        }
    }

    ///
    /// If this is a read usage, return a list of pass indexes that refer to the passes that read
    /// from this resource. Returns `None` if this is a write access.
    ///
    pub fn read_by<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        match self {
            ResourceUsage::Image(v) => {
                let iter = v.read_by.iter();
                iter.map(|(index, _)| *index)
            }
        }
    }
}
