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

use crate::error::TranspileResult;
use crate::utils::offset_to_index;
use eon_bytecode::indexes::InstructionIndex;
use std::cmp::Ordering;

pub struct OpCodeBranchTargetIter<'a> {
    func: &'a hashlink::Function,
    base: usize,
    branch_index: usize,
}

impl<'a> OpCodeBranchTargetIter<'a> {
    /// Given the fu
    pub fn new(func: &'a hashlink::Function, base: usize) -> Option<Self> {
        // this iterator only works on a subset of all opcodes so we need to return `None` as the
        // error case if the iterator is created with an unsupported opcode
        match &func.ops[base] {
            hashlink::OpCode::OpSwitch(_)
            | hashlink::OpCode::OpJAlways(_)
            | hashlink::OpCode::OpJTrue(_)
            | hashlink::OpCode::OpJFalse(_)
            | hashlink::OpCode::OpJNull(_)
            | hashlink::OpCode::OpJNotNull(_)
            | hashlink::OpCode::OpJSLt(_)
            | hashlink::OpCode::OpJSGte(_)
            | hashlink::OpCode::OpJSGt(_)
            | hashlink::OpCode::OpJSLte(_)
            | hashlink::OpCode::OpJULt(_)
            | hashlink::OpCode::OpJUGte(_)
            | hashlink::OpCode::OpJNotLt(_)
            | hashlink::OpCode::OpJNotGte(_)
            | hashlink::OpCode::OpJEq(_)
            | hashlink::OpCode::OpJNotEq(_) => {
                let out = Self {
                    func,
                    base,
                    branch_index: 0,
                };
                Some(out)
            }
            _ => None,
        }
    }
}

impl<'a> Iterator for OpCodeBranchTargetIter<'a> {
    type Item = TranspileResult<InstructionIndex>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = match &self.func.ops[self.base] {
            hashlink::OpCode::OpSwitch(op) => {
                let offset = match self.branch_index.cmp(&op.extra.len()) {
                    Ordering::Less => op.extra[self.branch_index],
                    Ordering::Equal => 0,
                    Ordering::Greater => {
                        return None;
                    }
                };
                Some(offset_to_index(self.base, offset, self.func))
            }
            hashlink::OpCode::OpJAlways(op) => {
                let offset = match self.branch_index {
                    0 => op.param_1,
                    _ => return None,
                };
                Some(offset_to_index(self.base, offset, self.func))
            }
            hashlink::OpCode::OpJTrue(v)
            | hashlink::OpCode::OpJFalse(v)
            | hashlink::OpCode::OpJNull(v)
            | hashlink::OpCode::OpJNotNull(v) => {
                // Get the offset basic on `branch_index`
                let offset = match self.branch_index {
                    0 => v.param_2,
                    1 => 0,
                    _ => {
                        return None;
                    }
                };
                Some(offset_to_index(self.base, offset, self.func))
            }
            hashlink::OpCode::OpJSLt(v)
            | hashlink::OpCode::OpJSGte(v)
            | hashlink::OpCode::OpJSGt(v)
            | hashlink::OpCode::OpJSLte(v)
            | hashlink::OpCode::OpJULt(v)
            | hashlink::OpCode::OpJUGte(v)
            | hashlink::OpCode::OpJNotLt(v)
            | hashlink::OpCode::OpJNotGte(v)
            | hashlink::OpCode::OpJEq(v)
            | hashlink::OpCode::OpJNotEq(v) => {
                // Get the offset basic on `branch_index`
                let offset = match self.branch_index {
                    0 => v.param_3,
                    1 => 0,
                    _ => {
                        return None;
                    }
                };
                Some(offset_to_index(self.base, offset, self.func))
            }
            _ => panic!(
                "{:#?} is not a branch instruction supported by OpCodeBranchTargetIter",
                &self.func.ops[self.branch_index]
            ),
        };
        self.branch_index += 1;
        val
    }
}
