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

use std::convert::TryFrom;
use windows_raw::Win32::Dxgi::DXGI_SCALING;

#[repr(i32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum Scaling {
    Stretch = 0,
    None = 1,
    AspectRatioStretch = 2,
}

impl TryFrom<DXGI_SCALING> for Scaling {
    type Error = ();

    #[inline]
    fn try_from(value: DXGI_SCALING) -> Result<Self, Self::Error> {
        match value {
            DXGI_SCALING::DXGI_SCALING_STRETCH => Ok(Scaling::Stretch),
            DXGI_SCALING::DXGI_SCALING_NONE => Ok(Scaling::None),
            DXGI_SCALING::DXGI_SCALING_ASPECT_RATIO_STRETCH => Ok(Scaling::AspectRatioStretch),
            _ => Err(()),
        }
    }
}

impl Into<DXGI_SCALING> for Scaling {
    #[inline]
    fn into(self) -> DXGI_SCALING {
        DXGI_SCALING(self as i32)
    }
}
