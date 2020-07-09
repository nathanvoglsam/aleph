//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod buffer;
mod image;

pub use buffer::BufferReadDescription;
pub use buffer::BufferWriteDescription;
pub use image::ImageReadDescription;
pub use image::ImageWriteDescription;

///
/// Struct passed into `register_access` for describing the resources accessed by the pass
///
pub struct ResourceAccess {
    pub(crate) image_reads: Vec<ImageReadDescription>,
    pub(crate) image_writes: Vec<ImageWriteDescription>,
    pub(crate) buffer_reads: Vec<BufferReadDescription>,
    pub(crate) buffer_writes: Vec<BufferWriteDescription>,
}

impl ResourceAccess {
    ///
    ///
    /// Register that the given image resource will be read in this pass
    ///
    pub fn read_image(&mut self, read: ImageReadDescription) {
        self.image_reads.push(read);
    }

    ///
    ///
    /// Register that the given image resource will be written in this pass
    ///
    pub fn write_image(&mut self, write: ImageWriteDescription) {
        self.image_writes.push(write);
    }

    ///
    ///
    /// Register that the given buffer resource will be read in this pass
    ///
    pub fn read_buffer(&mut self, read: BufferReadDescription) {
        self.buffer_reads.push(read);
    }

    ///
    ///
    /// Register that the given buffer resource will be written in this pass
    ///
    pub fn write_buffer(&mut self, write: BufferWriteDescription) {
        self.buffer_writes.push(write);
    }
}
