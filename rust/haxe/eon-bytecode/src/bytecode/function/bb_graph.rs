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

pub struct BBGraph {
    /// Maps an instruction index to the list of instructions
    pub destination_sources: HashMap<usize, Vec<usize>>,

    /// A list of all of the branch instruction indexes in the function
    pub branches: Vec<usize>,
}

/// Produces SSA graph nodes and edges
pub fn compute_bb_graph(f: &hashlink_bytecode::Function) -> Option<BBGraph> {
    // Holds the list of instruction indexes that have instructions that branch to the
    // instruction given by the key
    let mut destination_sources: HashMap<usize, Vec<usize>> = HashMap::new();

    // A flat list of all instructions which are branching instructions
    let mut branches: Vec<usize> = Vec::new();

    for (index, op) in f.ops.iter().enumerate() {
        compute_bb_graph_loop_inner(f, index, op, &mut destination_sources, &mut branches)?;
    }

    Some(BBGraph {
        destination_sources,
        branches,
    })
}

fn compute_bb_graph_loop_inner(
    f: &hashlink_bytecode::Function,
    index: usize,
    op: &hashlink_bytecode::OpCode,
    destination_sources: &mut HashMap<usize, Vec<usize>>,
    branches: &mut Vec<usize>,
) -> Option<()> {
    // We need to handle switch specially as it holds an array of branch targets rather than
    // a single target
    if let hashlink_bytecode::OpCode::OpSwitch(op) = op {
        compute_bb_graph_loop_inner_switch(f, index, op, destination_sources, branches)?;
    } else if let hashlink_bytecode::OpCode::OpJAlways(op) = op {
        compute_bb_graph_loop_inner_unconditional(f, index, op, destination_sources, branches)?;
    } else {
        compute_bb_graph_loop_inner_conditional(f, index, op, destination_sources, branches)?;
    }
    Some(())
}

fn compute_bb_graph_loop_inner_switch(
    f: &hashlink_bytecode::Function,
    index: usize,
    op: &hashlink_bytecode::OpSwitchParam,
    destination_sources: &mut HashMap<usize, Vec<usize>>,
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

        let block_source = destination_sources.entry(target).or_default();
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
    let block_source = destination_sources.entry(target).or_default();
    block_source.push(index);

    // Add this instruction to the list of branch instruction indexes
    branches.push(index);

    Some(())
}

fn compute_bb_graph_loop_inner_unconditional(
    f: &hashlink_bytecode::Function,
    index: usize,
    op: &hashlink_bytecode::OpOneParam,
    destination_sources: &mut HashMap<usize, Vec<usize>>,
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
    let block_source = destination_sources.entry(target).or_default();
    block_source.push(index);

    // Add this instruction to the list of branch instruction indexes
    branches.push(index);

    Some(())
}

fn compute_bb_graph_loop_inner_conditional(
    f: &hashlink_bytecode::Function,
    index: usize,
    op: &hashlink_bytecode::OpCode,
    destination_sources: &mut HashMap<usize, Vec<usize>>,
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
    let block_source = destination_sources.entry(target).or_default();
    block_source.push(index);

    // Now we add this instruction to the list of branch sources for the fail target,
    // which is just the instruction after the branch
    let block_source = destination_sources.entry(target_fail).or_default();
    block_source.push(index);

    // Add this instruction to the list of branch instruction indexes
    branches.push(index);

    Some(())
}

// We need to apply the offset to the current instruction index. We do it in this
// convoluted way so that we don't discard the full bit width of a usize in order
// to apply the offset. If we cast the index to isize and applied using a simple add
// then we could only represent offsets up to `isize::max`.
//
// Because we're going to this effort I may as well make it panic on overflow
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
