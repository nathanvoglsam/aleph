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

use raw::windows::win32::dxgi::DXGI_SWAP_CHAIN_FLAG;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct SwapChainFlags(pub u32);

impl SwapChainFlags {
    pub const NONE: Self = Self(0);
    pub const NON_PRE_ROTATED: Self = Self(1);
    pub const ALLOW_MODE_SWITCH: Self = Self(2);
    pub const GDI_COMPATIBLE: Self = Self(4);
    pub const RESTRICTED_CONTENT: Self = Self(8);
    pub const RESTRICT_SHARED_RESOURCE_DRIVER: Self = Self(16);
    pub const DISPLAY_ONLY: Self = Self(32);
    pub const FRAME_LATENCY_WAITABLE_OBJECT: Self = Self(64);
    pub const FOREGROUND_LAYER: Self = Self(128);
    pub const FULLSCREEN_VIDEO: Self = Self(256);
    pub const YUV_VIDEO: Self = Self(512);
    pub const HW_PROTECTED: Self = Self(1024);
    pub const ALLOW_TEARING: Self = Self(2048);
    pub const RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: Self = Self(4096);
}
