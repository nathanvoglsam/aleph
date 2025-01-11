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

use aleph_rhi_api::*;

use crate::BufferObjectDesc;

pub struct BufferObject {
    /// The buffer object itself
    buffer: Option<BufferHandle>,

    /// The description the object was created with.
    desc: BufferObjectDesc,
}

impl BufferObject {
    pub fn new_for_desc(
        device: &dyn IDevice,
        desc: BufferObjectDesc,
    ) -> Result<Self, BufferCreateError> {
        let size = desc.size.get();
        let usage = desc.usage;
        let api_desc = BufferDesc {
            size,
            cpu_access: CpuAccessMode::None, // TODO: do we allow something else?
            usage: ResourceUsageFlags::COPY_DEST | usage, // Add copy dest for uploads
            name: None,
        };
        let buffer = device.create_buffer(&api_desc)?;

        Ok(Self {
            buffer: Some(buffer),
            desc,
        })
    }

    pub fn update(&mut self, device: &dyn IDevice, buffer: BufferHandle) -> Option<BufferHandle> {
        if let Some(old_buffer) = &self.buffer {
            let new_desc = device.get_buffer_desc(&buffer);
            let old_desc = device.get_buffer_desc(old_buffer);

            // It is illegal for any major property of the new buffer to change from the old
            // buffer.
            debug_assert_eq!(new_desc.usage, old_desc.usage);
        }

        // Swap the old buffer for the new, taking the old buffer to send it out to the caller
        let mut buffer = Some(buffer);
        std::mem::swap(&mut buffer, &mut self.buffer);

        // And give the old buffer back out to the caller
        buffer
    }

    pub const fn get(&self) -> Option<&BufferHandle> {
        self.buffer.as_ref()
    }

    #[inline]
    pub fn get_owned(&self) -> Option<BufferHandle> {
        self.buffer.clone()
    }

    pub const fn desc(&self) -> &BufferObjectDesc {
        &self.desc
    }
}
