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

use crate::GraphicsCommandList;
use raw::windows::win32::direct3d12::{ID3D12CommandList, ID3D12GraphicsCommandList};
use std::ops::Deref;

pub struct SubmissionBuilder<'a> {
    buffer: Vec<ID3D12CommandList>,
    locks: Vec<std::sync::RwLockReadGuard<'a, ID3D12GraphicsCommandList>>,
}

impl<'a> SubmissionBuilder<'a> {
    pub fn new() -> SubmissionBuilder<'a> {
        Self {
            buffer: vec![],
            locks: vec![],
        }
    }

    pub fn add(&mut self, list: &'a GraphicsCommandList) -> &mut Self {
        let entry = list.get_shared();
        self.buffer.push(entry.deref().clone().into());
        self.locks.push(entry);
        self
    }

    pub(crate) unsafe fn lists(&self) -> (u32, ID3D12CommandList) {
        let len = self.buffer.len() as u32;
        let ptr: ID3D12CommandList = unsafe { std::mem::transmute(self.buffer.as_ptr()) };
        (len, ptr)
    }
}
