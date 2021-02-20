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

use crate::raw::windows::win32::direct3d12::{ID3D12CommandQueue, ID3D12DeviceChild, ID3D12Object};
use crate::{dxgi, Fence, SubmissionBuilder, SwapChainBuilder};
use raw_window_handle::HasRawWindowHandle;

#[repr(transparent)]
pub struct CommandQueue(pub(crate) ID3D12CommandQueue);

impl CommandQueue {
    pub unsafe fn signal(&self, fence: &Fence, value: u64) -> raw::windows::Result<()> {
        self.0.Signal(&fence.0, value).ok()
    }

    pub fn create_swapchain_builder<'a, 'b>(
        &'a mut self,
        factory: &'b dxgi::Factory,
        window_handle: &impl HasRawWindowHandle,
    ) -> SwapChainBuilder<'a, 'b> {
        SwapChainBuilder {
            queue: self,
            factory,
            window_handle: window_handle.raw_window_handle(),
            width: 0,
            height: 0,
            buffer_count: 2,
            allow_tearing: false,
        }
    }

    #[cfg(feature = "pix")]
    pub unsafe fn scoped_event(
        &mut self,
        colour: crate::pix::Colour,
        text: &str,
    ) -> crate::pix::ScopedEvent {
        crate::pix::ScopedEvent::for_queue(self, colour, text)
    }

    #[cfg(feature = "pix")]
    pub unsafe fn scoped_event_cstr(
        &mut self,
        colour: crate::pix::Colour,
        text: &std::ffi::CStr,
    ) -> crate::pix::ScopedEvent {
        crate::pix::ScopedEvent::for_queue_cstr(self, colour, text)
    }

    pub unsafe fn execute_command_lists(&mut self, command_lists: &SubmissionBuilder) {
        let lists = command_lists.lists();
        self.0.ExecuteCommandLists(lists.0, lists.1)
    }
}

impl Into<ID3D12Object> for CommandQueue {
    fn into(self) -> ID3D12Object {
        self.0.into()
    }
}

impl Into<ID3D12DeviceChild> for CommandQueue {
    fn into(self) -> ID3D12DeviceChild {
        self.0.into()
    }
}
