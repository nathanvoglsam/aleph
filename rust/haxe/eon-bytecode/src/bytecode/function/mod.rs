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

use crate::bytecode::opcode::OpCode;

#[derive(Clone, Debug)]
pub struct Register {
    /// Index into the type table for the type of value this register holds
    pub type_: u32,

    /// Does the allocated value outlive the function. Used for optimizing allocations
    pub outlives_function: bool,
}

#[derive(Clone, Debug)]
pub struct SSAValue {
    /// Index into the function's Register table that states what original value this SSA value is
    /// considered a version of
    pub register: u32,

    /// The index of the basic block that assigns this SSA value
    pub basic_block: u32,

    /// The index into the basic block for the instruction that assigns this SSA value
    pub instruction: u32,
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    /// This vector can be more considered a "map", which maps an index into the function's
    /// register table to a (maybe none) index into the SSA values table. If the map does yield an
    /// index then this index refers to the last write performed for the given register.
    ///
    /// This is used to identify the SSA value which holds the final state of a register at the end
    /// of a basic block so we can use this to build phi nodes when lowering to LLVM IR
    pub register_final_writes: Vec<Option<u32>>,

    /// This is just a flat, sequential list of opcodes
    pub ops: Vec<OpCode>,
}

#[derive(Clone, Debug)]
pub struct Function {
    /// Index into the type table for the type signature of this function
    pub type_: u32,

    /// ?
    pub f_index: u32,

    /// List of registers for the function's bytecode. This maps almost directly to the register
    /// system in hashlink bytecode but with some additional information.
    pub registers: Vec<Register>,

    /// This is the list of SSA values that get referred to by the
    pub ssa_values: Vec<SSAValue>,

    /// The list of basic blocks within the function
    pub basic_blocks: Vec<BasicBlock>,
}
