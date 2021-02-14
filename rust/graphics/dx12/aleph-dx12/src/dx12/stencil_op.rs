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

use raw::windows::win32::direct3d12::D3D12_STENCIL_OP;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    IncrementSaturate,
    DecrementSaturate,
    Invert,
    Increment,
    Decrement,
}

impl Into<D3D12_STENCIL_OP> for StencilOp {
    fn into(self) -> D3D12_STENCIL_OP {
        match self {
            StencilOp::Keep => D3D12_STENCIL_OP::D3D12_STENCIL_OP_KEEP,
            StencilOp::Zero => D3D12_STENCIL_OP::D3D12_STENCIL_OP_ZERO,
            StencilOp::Replace => D3D12_STENCIL_OP::D3D12_STENCIL_OP_REPLACE,
            StencilOp::IncrementSaturate => D3D12_STENCIL_OP::D3D12_STENCIL_OP_INCR_SAT,
            StencilOp::DecrementSaturate => D3D12_STENCIL_OP::D3D12_STENCIL_OP_DECR_SAT,
            StencilOp::Invert => D3D12_STENCIL_OP::D3D12_STENCIL_OP_INVERT,
            StencilOp::Increment => D3D12_STENCIL_OP::D3D12_STENCIL_OP_INCR,
            StencilOp::Decrement => D3D12_STENCIL_OP::D3D12_STENCIL_OP_DECR,
        }
    }
}
