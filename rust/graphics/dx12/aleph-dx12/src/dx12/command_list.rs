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

use crate::raw::windows::win32::direct3d12::{
    ID3D12DeviceChild, ID3D12GraphicsCommandList, ID3D12Object, D3D12_COMMAND_LIST_TYPE,
};

/// Wrapper for `D3D12_COMMAND_LIST_TYPE`
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum CommandListType {
    Direct,
    Bundle,
    Compute,
    Copy,
    VideoDecode,
    VideoProcess,
    VideoEncode,
}

impl CommandListType {
    fn from_raw(v: D3D12_COMMAND_LIST_TYPE) -> Option<CommandListType> {
        match v {
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT => {
                Some(CommandListType::Direct)
            }
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_BUNDLE => {
                Some(CommandListType::Bundle)
            }
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_COMPUTE => {
                Some(CommandListType::Compute)
            }
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_COPY => Some(CommandListType::Copy),
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE => {
                Some(CommandListType::VideoDecode)
            }
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS => {
                Some(CommandListType::VideoProcess)
            }
            D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE => {
                Some(CommandListType::VideoEncode)
            }
            _ => None,
        }
    }
}

impl Into<D3D12_COMMAND_LIST_TYPE> for CommandListType {
    fn into(self) -> D3D12_COMMAND_LIST_TYPE {
        match self {
            CommandListType::Direct => D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT,
            CommandListType::Bundle => D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_BUNDLE,
            CommandListType::Compute => D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_COMPUTE,
            CommandListType::Copy => D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_COPY,
            CommandListType::VideoDecode => {
                D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE
            }
            CommandListType::VideoProcess => {
                D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS
            }
            CommandListType::VideoEncode => {
                D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE
            }
        }
    }
}

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

impl Into<ID3D12Object> for ClosedGraphicsCommandList {
    fn into(self) -> ID3D12Object {
        self.0.into()
    }
}

impl Into<ID3D12DeviceChild> for ClosedGraphicsCommandList {
    fn into(self) -> ID3D12DeviceChild {
        self.0.into()
    }
}

impl Into<ID3D12Object> for OpenGraphicsCommandList {
    fn into(self) -> ID3D12Object {
        self.0.into()
    }
}

impl Into<ID3D12DeviceChild> for OpenGraphicsCommandList {
    fn into(self) -> ID3D12DeviceChild {
        self.0.into()
    }
}
