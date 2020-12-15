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

use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
use crate::utils::offset_from;
use eon_bytecode::indexes::InstructionIndex;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct BasicBlockGraph {
    /// Maps an instruction index to the set of instructions that branch to it
    pub destination_sources: HashMap<InstructionIndex, HashSet<InstructionIndex>>,

    /// A list of all of the branch instruction indexes in the function
    pub branches: HashSet<InstructionIndex>,
}

/// Produces SSA graph nodes and edges
pub fn compute_bb_graph(old_fn: &hashlink_bytecode::Function) -> TranspileResult<BasicBlockGraph> {
    // Holds the list of instruction indexes that have instructions that branch to the
    // instruction given by the key
    let mut destination_sources: HashMap<InstructionIndex, HashSet<InstructionIndex>> =
        HashMap::new();

    // A flat list of all instructions which are branching instructions
    let mut branches: HashSet<InstructionIndex> = HashSet::new();

    for (instruction_index, op) in old_fn.ops.iter().enumerate() {
        compute_bb_graph_loop_inner(
            old_fn,
            instruction_index,
            op,
            &mut destination_sources,
            &mut branches,
        )?;
    }

    Ok(BasicBlockGraph {
        destination_sources,
        branches,
    })
}

fn compute_bb_graph_loop_inner(
    old_fn: &hashlink_bytecode::Function,
    instruction_index: usize,
    op: &hashlink_bytecode::OpCode,
    destination_sources: &mut HashMap<InstructionIndex, HashSet<InstructionIndex>>,
    branches: &mut HashSet<InstructionIndex>,
) -> TranspileResult<()> {
    // We need to handle switch specially as it holds an array of branch targets rather than
    // a single target
    if let hashlink_bytecode::OpCode::OpSwitch(op) = op {
        compute_bb_graph_loop_inner_switch(
            destination_sources,
            branches,
            old_fn,
            instruction_index,
            op,
        )?;
    } else if let hashlink_bytecode::OpCode::OpJAlways(op) = op {
        compute_bb_graph_loop_inner_unconditional(
            destination_sources,
            branches,
            old_fn,
            instruction_index,
            op,
        )?;
    } else {
        compute_bb_graph_loop_inner_conditional(
            destination_sources,
            branches,
            old_fn,
            instruction_index,
            op,
        )?;
    }
    Ok(())
}

fn compute_bb_graph_loop_inner_switch(
    destination_sources: &mut HashMap<InstructionIndex, HashSet<InstructionIndex>>,
    branches: &mut HashSet<InstructionIndex>,
    old_fn: &hashlink_bytecode::Function,
    instruction_index: usize,
    op: &hashlink_bytecode::OpSwitchParam,
) -> TranspileResult<()> {
    // Handle all the distinct branch targets from the switch's jump table
    for offset in op.extra.iter() {
        // The inputs will never be bigger than `i32::max` because of how they're stored
        // so this shouldn't truncate anything. Even if it does the overflow and bounds
        // checks mean that it shouldn't cause issues anyway
        let offset = *offset as i32;

        // Calculate the actual offset
        let target = offset_from(instruction_index, offset)
            .ok_or(offset_error(instruction_index, old_fn))?;

        // Perform a bounds check
        bounds_check_offset(target, instruction_index, old_fn)?;

        // We don't need to check if the target branch is `OpLabel` as switches can't
        // encode a negative offset for w/e reason so we can just go straight to adding
        // it

        let block_source = destination_sources
            .entry(InstructionIndex(target))
            .or_default();
        block_source.insert(InstructionIndex(instruction_index));
    }

    // Lastly we handle the "fallback" branch. The fallback branch occurs when the switch index is
    // out of bounds of the jump table. When the index is out of bounds we use the third parameter
    // as an offset to jump to.
    let target = offset_from(instruction_index, op.param_3)
        .ok_or(offset_error(instruction_index, old_fn))?;

    // Perform a bounds check
    bounds_check_offset(target, instruction_index, old_fn)?;

    // Add the final branch target
    let block_source = destination_sources
        .entry(InstructionIndex(target))
        .or_default();
    block_source.insert(InstructionIndex(instruction_index));

    // Add this instruction to the list of branch instruction indexes
    branches.insert(InstructionIndex(instruction_index));

    Ok(())
}

fn compute_bb_graph_loop_inner_unconditional(
    destination_sources: &mut HashMap<InstructionIndex, HashSet<InstructionIndex>>,
    branches: &mut HashSet<InstructionIndex>,
    old_fn: &hashlink_bytecode::Function,
    instruction_index: usize,
    op: &hashlink_bytecode::OpOneParam,
) -> TranspileResult<()> {
    let target = offset_from(instruction_index, op.param_1)
        .ok_or(offset_error(instruction_index, old_fn))?;

    // Check if the computed index is in bounds
    bounds_check_offset(target, instruction_index, old_fn)?;

    // Check if a negative index offset branch does not branch to a label opcode
    if op.param_1 < 0 {
        match &old_fn.ops[target] {
            hashlink_bytecode::OpCode::OpLabel => {}
            _ => {
                //negative offset not targeting label
                return Err(negative_offset_error(instruction_index, old_fn));
            }
        }
    }

    // We only have a single branch target, as this is an unconditional branch
    let block_source = destination_sources
        .entry(InstructionIndex(target))
        .or_default();
    block_source.insert(InstructionIndex(instruction_index));

    // Add this instruction to the list of branch instruction indexes
    branches.insert(InstructionIndex(instruction_index));

    Ok(())
}

fn compute_bb_graph_loop_inner_conditional(
    destination_sources: &mut HashMap<InstructionIndex, HashSet<InstructionIndex>>,
    branches: &mut HashSet<InstructionIndex>,
    old_fn: &hashlink_bytecode::Function,
    instruction_index: usize,
    op: &hashlink_bytecode::OpCode,
) -> TranspileResult<()> {
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
        _ => return Ok(()),
    };

    // Apply the offset
    let target =
        offset_from(instruction_index, offset).ok_or(offset_error(instruction_index, old_fn))?;

    // These branch to either the next instruction or the target depending on the result
    // of some comparison
    let target_fail =
        offset_from(instruction_index, 0).ok_or(offset_error(instruction_index, old_fn))?;

    // Check if the computed index is in bounds
    bounds_check_offset(target, instruction_index, old_fn)?;
    bounds_check_offset(target_fail, instruction_index, old_fn)?;

    // Check if a negative index offset branch does not branch to a label opcode
    if offset < 0 {
        match &old_fn.ops[target] {
            hashlink_bytecode::OpCode::OpLabel => {}
            _ => {
                //negative offset not targeting label
                return Err(negative_offset_error(instruction_index, old_fn));
            }
        }
    }

    // First we add this instruction to the list of branch sources for the success
    // target
    let block_source = destination_sources
        .entry(InstructionIndex(target))
        .or_default();
    block_source.insert(InstructionIndex(instruction_index));

    // Now we add this instruction to the list of branch sources for the fail target,
    // which is just the instruction after the branch
    let block_source = destination_sources
        .entry(InstructionIndex(target_fail))
        .or_default();
    block_source.insert(InstructionIndex(instruction_index));

    // Add this instruction to the list of branch instruction indexes
    branches.insert(InstructionIndex(instruction_index));

    Ok(())
}

fn bounds_check_offset(
    target: usize,
    i_index: usize,
    old_fn: &hashlink_bytecode::Function,
) -> TranspileResult<()> {
    if target >= old_fn.ops.len() {
        let reason = InvalidFunctionReason::JumpOffsetOutOfBounds {
            i_index,
            func: old_fn.clone(),
        };
        let err = TranspileError::InvalidFunction(reason);
        return Err(err); // Out of bounds
    }
    Ok(())
}

fn negative_offset_error(i_index: usize, old_fn: &hashlink_bytecode::Function) -> TranspileError {
    let reason = InvalidFunctionReason::JumpNegativeOffsetNotTargetingLabel {
        i_index,
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}

fn offset_error(i_index: usize, old_fn: &hashlink_bytecode::Function) -> TranspileError {
    let reason = InvalidFunctionReason::JumpInvalidOffset {
        i_index,
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}
