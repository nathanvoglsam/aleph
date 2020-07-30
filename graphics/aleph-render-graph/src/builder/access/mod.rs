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
