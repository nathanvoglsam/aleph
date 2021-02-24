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

use raw::windows::win32::direct3d12::D3D12_RESOURCE_BARRIER_FLAGS;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct ResourceBarrierFlags(pub i32);

impl ResourceBarrierFlags {
    pub const NONE: Self = Self(0i32);
    pub const BEGIN_ONLY: Self = Self(1i32);
    pub const END_ONLY: Self = Self(2i32);
}

crate::flags_bitwise_impl!(ResourceBarrierFlags);

impl Into<D3D12_RESOURCE_BARRIER_FLAGS> for ResourceBarrierFlags {
    fn into(self) -> D3D12_RESOURCE_BARRIER_FLAGS {
        D3D12_RESOURCE_BARRIER_FLAGS(self.0)
    }
}
