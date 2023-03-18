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

use crate::device::Device;
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use std::any::TypeId;
use std::ptr::NonNull;
use windows::utils::GPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;

pub struct Buffer {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) base_address: GPUDescriptorHandle,
    pub(crate) desc: BufferDesc<'static>,
    pub(crate) name: Option<String>,
}

declare_interfaces!(Buffer, [IBuffer]);

impl IGetPlatformInterface for Buffer {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.resource, out, target)
    }
}

impl IBuffer for Buffer {
    fn upgrade(&self) -> AnyArc<dyn IBuffer> {
        AnyArc::map::<dyn IBuffer, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> BufferDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_deref();
        desc
    }

    fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        // TODO: should we expose 'read_range'?
        unsafe {
            let mut ptr = std::ptr::null_mut();
            self.resource
                .Map(0, std::ptr::null(), &mut ptr)
                .map_err(|v| anyhow!(v))?;
            NonNull::new(ptr as *mut u8).ok_or(ResourceMapError::MappedNullPointer)
        }
    }

    fn unmap(&self) {
        // TODO: should we expose 'written_range'
        unsafe {
            self.resource.Unmap(0, std::ptr::null());
        }
    }

    fn flush_range(&self, _offset: u64, _len: u64) {
        // intentional no-op
    }

    fn invalidate_range(&self, _offset: u64, _len: u64) {
        // intentional no-op
    }
}
