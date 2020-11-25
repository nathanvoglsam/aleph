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

mod bb_graph;
mod bb_spans;

use crate::bytecode::module::Module;
use crate::bytecode::opcode::OpCode;
use std::collections::HashMap;

/// This struct maps very directly to a "register" in terms of the raw HashLink bytecode. We hold
/// on to the information the "registers" provide because it makes some analysis passes easier as we
/// don't need to reconstruct this information from the SSA graph every time we need it
#[derive(Clone, Debug)]
pub struct Register {
    /// Does the allocated value outlive the function. Used for optimizing allocations.
    ///
    /// This only really has meaning for allocated types. Value types like plain integers and floats
    /// will never outlive a function as they don't have the concept of a lifetime. Value types are
    /// always copied when they assign to something else so they will only ever live as long as the
    /// scope they are defined in.
    ///
    /// An allocated type (something created with `new`) can have the lifetime extended beyond the
    /// scope it was created in by passing the pointer around. The pointer itself is a value type
    /// but what it points to will always be alive as long as a pointer to it exists.
    ///
    /// We can do some analysis to decide if the allocated object will outlive the function it was
    /// created in so we leave a spot here to fill the information in later.
    pub outlives_function: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct SSAValue {
    /// Index into the function's Register table that states what original value this SSA value is
    /// considered a version of
    pub register: usize,

    /// The index of the basic block that assigns this SSA value
    pub basic_block: usize,

    /// The index into the basic block for the instruction that assigns this SSA value
    pub instruction: usize,
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    /// This vector can be more considered a "map", which maps an index into the function's
    /// register table to a (maybe none) index into the SSA values table. If the map does yield an
    /// index then this index refers to the last write performed for the given register.
    ///
    /// This is used to identify the SSA value which holds the final state of a register at the end
    /// of a basic block so we can use this to build phi nodes when lowering to LLVM IR
    pub register_final_writes: Vec<Option<usize>>,

    /// This is just a flat, sequential list of opcodes
    pub ops: Vec<OpCode>,
}

#[derive(Clone, Debug)]
pub struct Function {
    /// Index into the type table for the type signature of this function
    pub type_: usize,

    /// ?
    pub f_index: u32,

    /// This is the list of SSA values that get referred to by the
    pub ssa_values: Vec<SSAValue>,

    /// The list of basic blocks within the function
    pub basic_blocks: Vec<BasicBlock>,

    /// This holds all metadata information for the struct and is used *ONLY* in the analysis and
    /// optimization passes. Nothing in this change the semantics of the code. It only stores extra
    /// information needed by different parts of the transpiler.
    ///
    /// There is no guarantee that any of this information will be valid or up to date at any given
    /// point. It is imperative that information is kept up to date as code transformations are
    /// applied and that data is filled as it is generated.
    ///
    /// This is done to simplify the types involved and try to keep everything as plain old data.
    /// The consequences of this mean you have to be careful to run certain things in the right
    /// order to make sure that information being used has actually been generated.
    pub metadata: Metadata,
}

impl Function {
    pub fn transpile_hashlink(module: &Module, f: hashlink_bytecode::Function) -> Option<Self> {
        let out = Self {
            type_: f.type_ as usize,
            f_index: f.f_index,
            ssa_values: vec![],
            basic_blocks: vec![],
            metadata: Metadata { reg_data: None },
        };

        // First we need to find all branch instructions and where they branch to
        let bb_graph = bb_graph::compute_bb_graph(&f)?;

        // Now we need to compute a list of spans for all the basic blocks in the bytecode
        let spans = bb_spans::compute_bb_spans(&f, &bb_graph)?;

        Some(out)
    }
}

/// Holds all function metadata that is used in the various optimization stages
///
/// Every field is optional as we can't generate it all at once and much of it requires multiple
/// passes to fully and correctly generate. Some metadata depends on other metadata existing before
/// it can itself be generated.
#[derive(Clone, Debug)]
pub struct Metadata {
    pub reg_data: Option<RegisterMetadata>,
}

#[derive(Clone, Debug)]
pub struct RegisterMetadata {
    /// List of registers for the function's bytecode. This maps almost directly to the register
    /// system in hashlink bytecode but with some additional information.
    ///
    /// We hold on to this so we can simplify tracking what actual values the SSA items refer to so
    /// analyzing the bytecode for optimization opportunities is easier.
    pub registers: Vec<Register>,

    /// Maps an SSA value to a register in the register list
    pub register_map: Vec<(usize, usize)>,
}
