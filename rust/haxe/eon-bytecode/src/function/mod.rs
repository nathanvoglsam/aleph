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

use crate::indexes::{BasicBlockIndex, TypeIndex, ValueIndex};
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SSAValue {
    /// The type this ssa value holds
    pub type_: TypeIndex,
}

#[derive(Clone, Default, Debug, Hash, Serialize, Deserialize)]
pub struct BasicBlock {
    /// This is just a flat, sequential list of opcodes
    pub ops: Vec<OpCode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    /// Index into the type table for the type signature of this function
    pub type_: TypeIndex,

    /// ?
    pub f_index: i32,

    /// This is the list of SSA values that get referred to by the
    pub ssa_values: Vec<SSAValue>,

    /// The list of basic blocks within the function
    pub basic_blocks: Vec<BasicBlock>,
}

impl Index<ValueIndex> for Function {
    type Output = SSAValue;

    fn index(&self, index: ValueIndex) -> &Self::Output {
        self.ssa_values.index(index.0)
    }
}

impl Index<BasicBlockIndex> for Function {
    type Output = BasicBlock;

    fn index(&self, index: BasicBlockIndex) -> &Self::Output {
        self.basic_blocks.index(index.0)
    }
}

impl IndexMut<ValueIndex> for Function {
    fn index_mut(&mut self, index: ValueIndex) -> &mut Self::Output {
        self.ssa_values.index_mut(index.0)
    }
}

impl IndexMut<BasicBlockIndex> for Function {
    fn index_mut(&mut self, index: BasicBlockIndex) -> &mut Self::Output {
        self.basic_blocks.index_mut(index.0)
    }
}
