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

use raw::windows::win32::direct3d12::D3D12_LOGIC_OP;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum LogicOp {
    Clear,
    Set,
    Copy,
    CopyInverted,
    Noop,
    Invert,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Equiv,
    AndReverse,
    AndInverted,
    OrReverse,
    OrInverted,
}

impl Into<D3D12_LOGIC_OP> for LogicOp {
    fn into(self) -> D3D12_LOGIC_OP {
        match self {
            LogicOp::Clear => D3D12_LOGIC_OP::D3D12_LOGIC_OP_CLEAR,
            LogicOp::Set => D3D12_LOGIC_OP::D3D12_LOGIC_OP_SET,
            LogicOp::Copy => D3D12_LOGIC_OP::D3D12_LOGIC_OP_COPY,
            LogicOp::CopyInverted => D3D12_LOGIC_OP::D3D12_LOGIC_OP_COPY_INVERTED,
            LogicOp::Noop => D3D12_LOGIC_OP::D3D12_LOGIC_OP_NOOP,
            LogicOp::Invert => D3D12_LOGIC_OP::D3D12_LOGIC_OP_INVERT,
            LogicOp::And => D3D12_LOGIC_OP::D3D12_LOGIC_OP_AND,
            LogicOp::Nand => D3D12_LOGIC_OP::D3D12_LOGIC_OP_NAND,
            LogicOp::Or => D3D12_LOGIC_OP::D3D12_LOGIC_OP_OR,
            LogicOp::Nor => D3D12_LOGIC_OP::D3D12_LOGIC_OP_NOR,
            LogicOp::Xor => D3D12_LOGIC_OP::D3D12_LOGIC_OP_XOR,
            LogicOp::Equiv => D3D12_LOGIC_OP::D3D12_LOGIC_OP_EQUIV,
            LogicOp::AndReverse => D3D12_LOGIC_OP::D3D12_LOGIC_OP_AND_REVERSE,
            LogicOp::AndInverted => D3D12_LOGIC_OP::D3D12_LOGIC_OP_AND_INVERTED,
            LogicOp::OrReverse => D3D12_LOGIC_OP::D3D12_LOGIC_OP_OR_REVERSE,
            LogicOp::OrInverted => D3D12_LOGIC_OP::D3D12_LOGIC_OP_OR_INVERTED,
        }
    }
}
