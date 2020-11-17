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

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum OpCodeType {
    OpNoParam,
    OpOneParam,
    OpTwoParam,
    OpThreeParam,
    OpFourParam,
    OpFiveParam,
    OpSixParam,
    OpCallNParam,
    OpSwitchParam,
}

impl OpCodeType {
    /// Returns the static arg count for the given opcode.
    ///
    /// Will return `None` if the opcode has a dynamic arg count
    pub fn arg_num(&self) -> Option<usize> {
        match self {
            OpCodeType::OpNoParam => Some(0),
            OpCodeType::OpOneParam => Some(1),
            OpCodeType::OpTwoParam => Some(2),
            OpCodeType::OpThreeParam => Some(3),
            OpCodeType::OpFourParam => Some(4),
            OpCodeType::OpFiveParam => Some(5),
            OpCodeType::OpSixParam => Some(6),
            OpCodeType::OpCallNParam => None,
            OpCodeType::OpSwitchParam => None,
        }
    }
}
