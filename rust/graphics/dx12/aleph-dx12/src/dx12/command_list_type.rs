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

use crate::raw::windows::win32::direct3d12::D3D12_COMMAND_LIST_TYPE;

/// Wrapper for `D3D12_COMMAND_LIST_TYPE`
#[repr(i32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum CommandListType {
    Direct = 0,
    Bundle = 1,
    Compute = 2,
    Copy = 3,
    VideoDecode = 4,
    VideoProcess = 5,
    VideoEncode = 6,
}

impl CommandListType {
    pub(crate) fn from_raw(v: D3D12_COMMAND_LIST_TYPE) -> Option<CommandListType> {
        type T = D3D12_COMMAND_LIST_TYPE;
        match v {
            T::D3D12_COMMAND_LIST_TYPE_DIRECT => Some(CommandListType::Direct),
            T::D3D12_COMMAND_LIST_TYPE_BUNDLE => Some(CommandListType::Bundle),
            T::D3D12_COMMAND_LIST_TYPE_COMPUTE => Some(CommandListType::Compute),
            T::D3D12_COMMAND_LIST_TYPE_COPY => Some(CommandListType::Copy),
            T::D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE => Some(CommandListType::VideoDecode),
            T::D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS => Some(CommandListType::VideoProcess),
            T::D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE => Some(CommandListType::VideoEncode),
            _ => None,
        }
    }
}

impl Into<D3D12_COMMAND_LIST_TYPE> for CommandListType {
    fn into(self) -> D3D12_COMMAND_LIST_TYPE {
        D3D12_COMMAND_LIST_TYPE(self as i32)
    }
}
