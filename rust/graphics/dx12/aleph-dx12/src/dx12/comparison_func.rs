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

use raw::windows::win32::direct3d12::D3D12_COMPARISON_FUNC;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub enum ComparisonFunc {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl Into<D3D12_COMPARISON_FUNC> for ComparisonFunc {
    fn into(self) -> D3D12_COMPARISON_FUNC {
        match self {
            ComparisonFunc::Never => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_NEVER,
            ComparisonFunc::Less => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_LESS,
            ComparisonFunc::Equal => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_EQUAL,
            ComparisonFunc::LessEqual => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_LESS_EQUAL,
            ComparisonFunc::Greater => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_GREATER,
            ComparisonFunc::NotEqual => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_NOT_EQUAL,
            ComparisonFunc::GreaterEqual => {
                D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_GREATER_EQUAL
            }
            ComparisonFunc::Always => D3D12_COMPARISON_FUNC::D3D12_COMPARISON_FUNC_ALWAYS,
        }
    }
}
