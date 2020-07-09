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

pub use buffer::BufferExport;
pub use buffer::BufferImport;
pub use image::ImageExport;
pub use image::ImageImport;

///
/// Represents the set of supported resource types that can be imported
///
pub enum ResourceImport {
    Image(ImageImport),
    Buffer(BufferImport),
}

///
/// Represents the set of supported resource types that can be exported
///
pub enum ResourceExport {
    Image(ImageExport),
    Buffer(BufferExport),
}
