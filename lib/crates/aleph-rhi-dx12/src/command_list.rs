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

use std::any::TypeId;

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use bumpalo::Bump;
use windows::Win32::Graphics::Direct3D12::*;

use crate::device::Device;
use crate::encoder::Encoder;

pub struct CommandList {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) allocator: ID3D12CommandAllocator,
    pub(crate) list: ID3D12GraphicsCommandList7,
    pub(crate) descriptor_heaps: [Option<ID3D12DescriptorHeap>; 2],
    pub(crate) list_type: QueueType,
}

declare_interfaces!(CommandList, [ICommandList]);

impl IGetPlatformInterface for CommandList {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        if try_clone_value_into_slot(&self.list, out, target).is_some() {
            return Some(());
        };
        if try_clone_value_into_slot(&self.allocator, out, target).is_some() {
            return Some(());
        };
        None
    }
}

unsafe impl Send for CommandList {}

impl ICommandList for CommandList {
    fn begin_general<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IGeneralEncoder + 'a>, CommandListBeginError> {
        if matches!(self.list_type, QueueType::General) {
            // Open the command list for recording with no bound pipeline so we can attach it to
            // the command allocator
            unsafe {
                self.list
                    .Reset(&self.allocator, None)
                    .map_err(|v| log::error!("Platform Error: {:#?}", v))?;

                self.list.SetDescriptorHeaps(&self.descriptor_heaps);
            }

            let encoder = Encoder::<'a> {
                _list: self.list.clone(),
                _queue_type: self.list_type,
                bound_graphics_pipeline: None,
                bound_compute_pipeline: None,
                input_binding_strides: [0; 16],
                arena: Bump::with_capacity(1024 * 16),
                phantom_data: Default::default(),
                bound_graphics_sets: vec![None; 16].into(),
                bound_compute_sets: vec![None; 16].into(),
            };
            Ok(Box::new(encoder))
        } else {
            Err(CommandListBeginError::InvalidEncoderType(
                QueueType::General,
            ))
        }
    }

    fn begin_compute<'a>(
        &'a mut self,
    ) -> Result<Box<dyn IComputeEncoder + 'a>, CommandListBeginError> {
        Err(CommandListBeginError::InvalidEncoderType(
            QueueType::Compute,
        ))
    }

    fn begin_transfer<'a>(
        &'a mut self,
    ) -> Result<Box<dyn ITransferEncoder + 'a>, CommandListBeginError> {
        Err(CommandListBeginError::InvalidEncoderType(
            QueueType::Transfer,
        ))
    }
}
