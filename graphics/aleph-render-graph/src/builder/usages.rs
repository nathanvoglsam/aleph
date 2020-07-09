//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::graph::PassIndex;
use aleph_vulkan_core::erupt::vk1_0::{AccessFlags, ImageLayout, PipelineStageFlags};

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
    /// Returns the `PassIndex` of the pass that writes this resource
    ///
    pub fn writen_by(&self) -> PassIndex {
        match self {
            ResourceUsage::Image(v) => v.writen_by,
        }
    }

    ///
    /// Returns an iterator of `PassIndex`s over all the readers of this resource
    ///
    pub fn read_by<'a>(&'a self) -> impl Iterator<Item = PassIndex> + 'a {
        match self {
            ResourceUsage::Image(v) => {
                let iter = v.read_by.iter();
                iter.map(|(index, _)| *index)
            }
        }
    }
}
