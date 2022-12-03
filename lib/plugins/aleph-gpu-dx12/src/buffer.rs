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
use crate::internal::set_name::set_name;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{BufferDesc, IBuffer, INamedObject, ResourceMapError};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::utils::GPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;

pub struct Buffer {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) base_address: GPUDescriptorHandle,
    pub(crate) desc: BufferDesc,
    pub(crate) debug_mapped_tracker: AtomicBool,
}

declare_interfaces!(Buffer, [IBuffer, IBufferExt]);

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

    fn desc(&self) -> &BufferDesc {
        &self.desc
    }

    fn map(&self) -> Result<NonNull<u8>, ResourceMapError> {
        // Debug check for tracking that the resource is unmapped when trying to map it
        debug_assert!(self
            .debug_mapped_tracker
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());

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

        // Debug check for tracking that the resource is mapped when trying to unmap it
        debug_assert!(self
            .debug_mapped_tracker
            .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok());
    }

    fn flush_range(&self, _offset: u64, _len: u64) {
        // intentional no-op
    }

    fn invalidate_range(&self, _offset: u64, _len: u64) {
        // intentional no-op
    }
}

pub trait IBufferExt: IBuffer {
    fn get_raw_handle(&self) -> ID3D12Resource;
}

impl IBufferExt for Buffer {
    fn get_raw_handle(&self) -> ID3D12Resource {
        self.resource.clone()
    }
}

impl INamedObject for Buffer {
    fn set_name(&self, name: &str) {
        set_name(&self.resource, name).unwrap();
    }
}
