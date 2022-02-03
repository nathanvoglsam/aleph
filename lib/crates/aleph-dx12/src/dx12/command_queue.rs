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

use crate::utils::WeakRef;
use crate::{Fence, GraphicsCommandList};
use windows::Win32::Graphics::Direct3D12::ID3D12CommandQueue;

#[repr(transparent)]
pub struct CommandQueue(pub(crate) ID3D12CommandQueue);

impl CommandQueue {
    #[inline]
    pub unsafe fn signal(&mut self, fence: &Fence, value: u64) -> windows::core::Result<()> {
        self.0.Signal(&fence.0, value)
    }

    #[inline]
    pub unsafe fn execute_command_lists(&mut self, command_lists: &[WeakRef<GraphicsCommandList>]) {
        // The actual call to ExecuteCommandLists
        self.0.ExecuteCommandLists(
            command_lists.len() as u32,
            command_lists.as_ptr() as *const _,
        );
    }
}

crate::as_weak_ref_impl!(CommandQueue);
crate::object_impl!(CommandQueue);
crate::device_child_impl!(CommandQueue);
crate::shared_object!(CommandQueue);
windows::deref_impl!(CommandQueue, ID3D12CommandQueue);
