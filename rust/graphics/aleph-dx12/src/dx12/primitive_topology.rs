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

use windows_raw::Win32::Direct3D11::D3D_PRIMITIVE_TOPOLOGY;

#[repr(i32)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum PrimitiveTopology {
    Undefined = 0,
    PointList = 1,
    LineList = 2,
    LineStrip = 3,
    TriangleList = 4,
    TriangleStrip = 5,
    LineListAdj = 10,
    LineStripAdj = 11,
    TriangleListAdj = 12,
    TriangleStripAdj = 13,
    ControlPointPatchList1 = 33,
    ControlPointPatchList2 = 34,
    ControlPointPatchList3 = 35,
    ControlPointPatchList4 = 36,
    ControlPointPatchList5 = 37,
    ControlPointPatchList6 = 38,
    ControlPointPatchList7 = 39,
    ControlPointPatchList8 = 40,
    ControlPointPatchList9 = 41,
    ControlPointPatchList10 = 42,
    ControlPointPatchList11 = 43,
    ControlPointPatchList12 = 44,
    ControlPointPatchList13 = 45,
    ControlPointPatchList14 = 46,
    ControlPointPatchList15 = 47,
    ControlPointPatchList16 = 48,
    ControlPointPatchList17 = 49,
    ControlPointPatchList18 = 50,
    ControlPointPatchList19 = 51,
    ControlPointPatchList20 = 52,
    ControlPointPatchList21 = 53,
    ControlPointPatchList22 = 54,
    ControlPointPatchList23 = 55,
    ControlPointPatchList24 = 56,
    ControlPointPatchList25 = 57,
    ControlPointPatchList26 = 58,
    ControlPointPatchList27 = 59,
    ControlPointPatchList28 = 60,
    ControlPointPatchList29 = 61,
    ControlPointPatchList30 = 62,
    ControlPointPatchList31 = 63,
    ControlPointPatchList32 = 64,
}

impl Default for PrimitiveTopology {
    #[inline]
    fn default() -> Self {
        Self::Undefined
    }
}

impl Into<D3D_PRIMITIVE_TOPOLOGY> for PrimitiveTopology {
    #[inline]
    fn into(self) -> D3D_PRIMITIVE_TOPOLOGY {
        D3D_PRIMITIVE_TOPOLOGY(self as i32)
    }
}
