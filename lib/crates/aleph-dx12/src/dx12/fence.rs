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

use crate::Event;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D12::ID3D12Fence;

#[repr(transparent)]
pub struct Fence(pub(crate) ID3D12Fence);

impl Fence {
    #[inline]
    pub fn signal(&self, value: u64) -> windows::core::Result<()> {
        unsafe { self.0.Signal(value) }
    }

    #[inline]
    pub fn set_event_on_completion<'a, 'b>(
        &'a self,
        value: u64,
        event: impl Into<Option<&'b Event>>,
    ) -> windows::core::Result<()> {
        let handle = if let Some(event) = event.into() {
            HANDLE(event.0.get())
        } else {
            HANDLE(0)
        };
        unsafe { self.0.SetEventOnCompletion(value, handle) }
    }

    #[inline]
    pub fn get_completed_value(&self) -> u64 {
        unsafe { self.0.GetCompletedValue() }
    }
}

crate::object_impl!(Fence);
crate::device_child_impl!(Fence);
crate::shared_object!(Fence);
