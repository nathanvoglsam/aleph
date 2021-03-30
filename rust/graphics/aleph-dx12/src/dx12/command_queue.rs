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

use crate::{Fence, GraphicsCommandList};
use std::ffi::c_void;
use windows_raw::win32::direct3d12::ID3D12CommandQueue;

#[derive(Clone)]
#[repr(transparent)]
pub struct CommandQueue(pub(crate) ID3D12CommandQueue);

impl CommandQueue {
    pub unsafe fn signal(&mut self, fence: &Fence, value: u64) -> crate::Result<()> {
        self.0.Signal(&fence.0, value).ok()
    }

    pub unsafe fn execute_command_lists<const NUM: usize>(
        &mut self,
        command_lists: &[&GraphicsCommandList; NUM],
    ) {
        // We need an array to put the ID3D12GraphicsCommandList pointers into
        let mut lists: [*mut c_void; NUM] = [std::ptr::null_mut(); NUM];

        // Iterate over the given set of lists, accessing and storing the locks and putting the
        // ID3D12GraphicsCommandList pointer into their respective arrays
        for (i, list) in command_lists.iter().enumerate() {
            let list = list.as_raw();

            // First we store the pointer in the list we pass to `ExecuteCommandLists`
            lists[i] = std::mem::transmute_copy(list);
        }

        // The actual call to ExecuteCommandLists
        self.0
            .ExecuteCommandLists(NUM as u32, lists.as_mut_ptr() as *mut _);
    }

    pub unsafe fn execute_command_lists_dynamic(&mut self, command_lists: &[&GraphicsCommandList]) {
        // We need an array to put the ID3D12GraphicsCommandList pointers into
        let mut lists: Vec<*mut c_void> = Vec::new();

        // Iterate over the given set of lists, accessing and storing the locks and putting the
        // ID3D12GraphicsCommandList pointer into their respective arrays
        for list in command_lists.iter() {
            let list = list.as_raw();

            // First we store the pointer in the list we pass to `ExecuteCommandLists`
            lists.push(std::mem::transmute_copy(list));
        }

        // The actual call to ExecuteCommandLists
        self.0
            .ExecuteCommandLists(command_lists.len() as u32, lists.as_mut_ptr() as *mut _);
    }
}

crate::object_impl!(CommandQueue);
crate::device_child_impl!(CommandQueue);
windows_raw::deref_impl!(CommandQueue, ID3D12CommandQueue);
