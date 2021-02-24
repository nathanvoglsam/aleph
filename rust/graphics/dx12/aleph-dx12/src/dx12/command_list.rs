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

use crate::raw::windows::win32::direct3d12::ID3D12GraphicsCommandList;
use crate::CommandListType;

#[repr(transparent)]
pub struct OpenGraphicsCommandList(pub(crate) ID3D12GraphicsCommandList);

impl OpenGraphicsCommandList {
    #[cfg(feature = "pix")]
    pub fn scoped_event(
        &mut self,
        colour: crate::pix::Colour,
        text: &str,
    ) -> crate::pix::ScopedEvent {
        unsafe { crate::pix::ScopedEvent::for_list(self, colour, text) }
    }

    #[cfg(feature = "pix")]
    pub fn scoped_event_cstr(
        &mut self,
        colour: crate::pix::Colour,
        text: &std::ffi::CStr,
    ) -> crate::pix::ScopedEvent {
        unsafe { crate::pix::ScopedEvent::for_list_cstr(self, colour, text) }
    }

    pub fn get_type(&self) -> CommandListType {
        unsafe { CommandListType::from_raw(self.0.GetType()).unwrap() }
    }
}

#[repr(transparent)]
pub struct ClosedGraphicsCommandList(pub(crate) ID3D12GraphicsCommandList);

impl ClosedGraphicsCommandList {
    pub unsafe fn get_type(&self) -> CommandListType {
        CommandListType::from_raw(self.0.GetType()).unwrap()
    }
}

crate::object_impl!(ClosedGraphicsCommandList);
crate::device_child_impl!(ClosedGraphicsCommandList);

crate::object_impl!(OpenGraphicsCommandList);
crate::device_child_impl!(OpenGraphicsCommandList);
