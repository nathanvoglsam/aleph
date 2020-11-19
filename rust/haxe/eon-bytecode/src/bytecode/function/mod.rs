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

        let (block_sources, branches) = Self::get_ssa_links(&f)?;

        Some(out)
    }

    fn get_ssa_links(
        f: &hashlink_bytecode::Function,
    ) -> Option<(HashMap<usize, Vec<usize>>, Vec<usize>)> {
        // Holds the list of instruction indexes that have instructions that branch to the
        // instruction given by the key
        let mut block_sources: HashMap<usize, Vec<usize>> = HashMap::new();

        // A flat list of all instructions which are branching instructions
        let mut branches: Vec<usize> = Vec::new();

        for (index, op) in f.ops.iter().enumerate() {
            Self::get_ssa_links_loop_inner(f, index, op, &mut block_sources, &mut branches)?;
        }

        Some((block_sources, branches))
    }

    fn get_ssa_links_loop_inner(
        f: &hashlink_bytecode::Function,
        index: usize,
        op: &hashlink_bytecode::OpCode,
        block_sources: &mut HashMap<usize, Vec<usize>>,
        branches: &mut Vec<usize>,
    ) -> Option<()> {
        // We need to handle switch specially as it holds an array of branch targets rather than
        // a single target
        if let hashlink_bytecode::OpCode::OpSwitch(op) = op {
            Self::get_ssa_links_loop_inner_switch(f, index, op, block_sources, branches)?;
        } else if let hashlink_bytecode::OpCode::OpJAlways(op) = op {
            Self::get_ssa_links_loop_inner_unconditional(f, index, op, block_sources, branches)?;
        } else {
            Self::get_ssa_links_loop_inner_conditional(f, index, op, block_sources, branches)?;
        }
        Some(())
    }

    fn get_ssa_links_loop_inner_switch(
        f: &hashlink_bytecode::Function,
        index: usize,
        op: &hashlink_bytecode::OpSwitchParam,
        block_sources: &mut HashMap<usize, Vec<usize>>,
        branches: &mut Vec<usize>,
    ) -> Option<()> {
        // A switch contains a list of targets in an array so first we need to deduplicate
        // the jump table by constructing an iterator that will do it for us.
        //
        // This isn't particularly efficient but the input sizes will be typically small
        // enough where the high constant cost of a better algorithm (big O better) will
        // hide whatever benefit it would bring with the very rare inputs large enough for
        // it to be faster. Our good friend latency vs throughput at it again.
        let table = op
            .extra
            .iter()
            .map(|v| *v)
            .filter(|v| !op.extra.contains(v));

        // Now we can handle all the distinct branch targets from the switch's jump table
        for offset in table {
            // The inputs will never be bigger than `i32::max` because of how they're stored
            // so this shouldn't truncate anything. Even if it does the overflow and bounds
            // checks mean that it shouldn't cause issues anyway
            let offset = offset as i32;

            // Calculate the actual offset
            let target = offset_from(index, offset)?;

            // Perform a bounds check
            if target >= f.ops.len() {
                return None; // Out of bounds
            }

            // We don't need to check if the target branch is `OpLabel` as switches can't
            // encode a negative offset for w/e reason so we can just go straight to adding
            // it

            let block_source = block_sources.entry(target).or_default();
            block_source.push(index);
        }

        // Lastly we handle the "fallback" branch

        // Same as in the jump table, this is fine as all errors this cast could cause are
        // correctly checked
        let offset = op.param_3 as i32;

        // Once again, perform the offset
        let target = offset_from(index, offset)?;

        // Perform a bounds check
        if target >= f.ops.len() {
            return None; // Out of bounds
        }

        // Add the final branch target
        let block_source = block_sources.entry(target).or_default();
        block_source.push(index);

        // Add this instruction to the list of branch instruction indexes
        branches.push(index);

        Some(())
    }

    fn get_ssa_links_loop_inner_unconditional(
        f: &hashlink_bytecode::Function,
        index: usize,
        op: &hashlink_bytecode::OpOneParam,
        block_sources: &mut HashMap<usize, Vec<usize>>,
        branches: &mut Vec<usize>,
    ) -> Option<()> {
        let target = offset_from(index, op.param_1)?;

        // Check if the computed index is in bounds
        if target >= f.ops.len() {
            return None; // Out of bounds
        }

        // Check if a negative index offset branch does not branch to a label opcode
        if op.param_1 < 0 {
            match &f.ops[target] {
                hashlink_bytecode::OpCode::OpLabel => {}
                _ => return None, //negative offset not targeting label
            }
        }

        // We only have a single branch target, as this is an unconditional branch
        let block_source = block_sources.entry(target).or_default();
        block_source.push(index);

        // Add this instruction to the list of branch instruction indexes
        branches.push(index);

        Some(())
    }

    fn get_ssa_links_loop_inner_conditional(
        f: &hashlink_bytecode::Function,
        index: usize,
        op: &hashlink_bytecode::OpCode,
        block_sources: &mut HashMap<usize, Vec<usize>>,
        branches: &mut Vec<usize>,
    ) -> Option<()> {
        // Get the branch target offset if it exists, otherwise skip to the next instruction
        let offset = match op {
            hashlink_bytecode::OpCode::OpJTrue(v) => v.param_2,
            hashlink_bytecode::OpCode::OpJFalse(v) => v.param_2,
            hashlink_bytecode::OpCode::OpJNull(v) => v.param_2,
            hashlink_bytecode::OpCode::OpJNotNull(v) => v.param_2,
            hashlink_bytecode::OpCode::OpJSLt(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJSGte(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJSGt(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJSLte(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJULt(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJUGte(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJNotLt(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJNotGte(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJEq(v) => v.param_3,
            hashlink_bytecode::OpCode::OpJNotEq(v) => v.param_3,
            _ => return Some(()),
        };

        // Apply the offset
        let target = offset_from(index, offset)?;

        // These branch to either the next instruction or the target depending on the result
        // of some comparison
        let target_fail = offset_from(index, 1)?;

        // Check if the computed index is in bounds
        if target >= f.ops.len() || target_fail >= f.ops.len() {
            return None; // Out of bounds
        }

        // Check if a negative index offset branch does not branch to a label opcode
        if offset < 0 {
            match &f.ops[target] {
                hashlink_bytecode::OpCode::OpLabel => {}
                _ => return None, //negative offset not targeting label
            }
        }

        // First we add this instruction to the list of branch sources for the success
        // target
        let block_source = block_sources.entry(target).or_default();
        block_source.push(index);

        // Now we add this instruction to the list of branch sources for the fail target,
        // which is just the instruction after the branch
        let block_source = block_sources.entry(target_fail).or_default();
        block_source.push(index);

        // Add this instruction to the list of branch instruction indexes
        branches.push(index);

        Some(())
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

// We need to apply the offset to the current instruction index. We do it in this
// convoluted way so that we don't discard the full bit width of a usize in order
// to apply the offset. If we cast the index to isize and applied using a simple add
// then we could only represent offsets up to `isize::max`.
//
// Because we're going to this effort I may as well make it panic on overflow which
// would be very bad
fn offset_from(base: usize, offset: i32) -> Option<usize> {
    if offset.is_negative() {
        // Convert negative to positive so it will fit into a usize
        let offset = -offset;
        let offset = offset as usize;

        // Subtract the inverted negative offset. This is mathematically identical to just
        // adding a signed offset but does not discard the precision of the base value
        let out = base.checked_sub(offset)?;
        let out = out.checked_add(1)?;

        Some(out)
    } else {
        // If the offset is positive we can just cast it straight to usize and add
        let out = base.checked_add(offset as usize)?;
        let out = out.checked_add(1)?;

        Some(out)
    }
}
