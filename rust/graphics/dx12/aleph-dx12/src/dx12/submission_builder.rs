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

use crate::ClosedGraphicsCommandList;
use raw::windows::win32::direct3d12::ID3D12CommandList;
use std::marker::PhantomData;

#[repr(transparent)]
struct ListEntry<'a>(
    ID3D12CommandList,
    PhantomData<&'a ClosedGraphicsCommandList>,
);

pub struct SubmissionBuilder<'a> {
    buffer: Vec<ListEntry<'a>>,
}

impl<'a> SubmissionBuilder<'a> {
    pub fn new() -> SubmissionBuilder<'a> {
        // Assert the layouts are the same
        use std::mem::align_of;
        use std::mem::size_of;
        assert_eq!(size_of::<ListEntry<'a>>(), size_of::<ID3D12CommandList>());
        assert_eq!(align_of::<ListEntry<'a>>(), align_of::<ID3D12CommandList>());

        Self { buffer: vec![] }
    }

    pub fn add(&mut self, list: &'a ClosedGraphicsCommandList) -> &mut Self {
        let entry = ListEntry(list.0.clone().into(), Default::default());
        self.buffer.push(entry);
        self
    }

    pub(crate) fn lists<'s>(&'s self) -> (u32, &'s ID3D12CommandList) {
        let len = self.buffer.len() as u32;
        let ptr: &'s ID3D12CommandList = unsafe { std::mem::transmute(&self.buffer[0]) };
        (len, ptr)
    }
}
