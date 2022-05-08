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

use windows::Win32::Graphics::Direct3D12::D3D12_MESSAGE_CATEGORY;

#[repr(i32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum MessageCategory {
    ApplicationDefined = 0,
    Miscellaneous = 1,
    Initialization = 2,
    Cleanup = 3,
    Compilation = 4,
    StateCreation = 5,
    StateSetting = 6,
    StateGetting = 7,
    ResourceManipulation = 8,
    Execution = 9,
    Shader = 10,
}

impl From<D3D12_MESSAGE_CATEGORY> for MessageCategory {
    #[inline]
    fn from(v: D3D12_MESSAGE_CATEGORY) -> Self {
        match v.0 {
            0 => Self::ApplicationDefined,
            1 => Self::Miscellaneous,
            2 => Self::Initialization,
            3 => Self::Cleanup,
            4 => Self::Compilation,
            5 => Self::StateCreation,
            6 => Self::StateSetting,
            7 => Self::StateGetting,
            8 => Self::ResourceManipulation,
            9 => Self::Execution,
            10 => Self::Shader,
            _ => panic!("Unknown message category"),
        }
    }
}

impl Into<D3D12_MESSAGE_CATEGORY> for MessageCategory {
    #[inline]
    fn into(self) -> D3D12_MESSAGE_CATEGORY {
        D3D12_MESSAGE_CATEGORY(self as i32)
    }
}
