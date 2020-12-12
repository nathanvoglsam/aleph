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

use crate::basic_block_graph::BasicBlockGraph;
use eon_bytecode::function::{Function, RegisterMetadata, SSAValue};
use eon_bytecode::indexes::{
    BasicBlockIndex, InstructionIndex, RegisterIndex, TypeIndex, ValueIndex,
};
use std::collections::HashSet;

// Get the set of predecessor basic blocks for the basic block that starts with instruction
// `first_instruction_index`
pub fn get_basic_block_predecessor_list(
    bb_graph: &BasicBlockGraph,
    spans: &[(InstructionIndex, InstructionIndex)],
    first_instruction_index: usize,
) -> Option<HashSet<BasicBlockIndex>> {
    let mut mapped_predecessors = HashSet::new();

    // Get the list of predecessors and map the instruction back to the basic block (span) that it
    // came from
    if let Some(predecessors) = bb_graph
        .destination_sources
        .get(&InstructionIndex(first_instruction_index))
    {
        for predecessor in predecessors {
            // Find the source span
            let block = find_source_span(spans, predecessor.0)?;

            // Insert our mapped index into our new list
            mapped_predecessors.insert(BasicBlockIndex(block));
        }
    }

    Some(mapped_predecessors)
}

/// Find the span, in the given list, that holds the given instruction index
pub fn find_source_span(spans: &[(InstructionIndex, InstructionIndex)], i: usize) -> Option<usize> {
    spans
        .iter()
        .enumerate()
        .find(|(_, (l, u))| l.0 <= i && u.0 >= i)
        .map(|(i, _)| i)
}

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_phi_import(
    new_fn: &mut Function,
    old_fn: &hashlink_bytecode::Function,
    reg_meta: &mut RegisterMetadata,
    bb_index: usize,
    v: RegisterIndex,
) -> Option<ValueIndex> {
    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = old_fn.registers[v.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(new_fn.ssa_values.len());
    new_fn.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // We ignore if this returns None deliberately as a phi instruction isn't a real write
    let _ = reg_meta.basic_block_registers_written[bb_index].insert(v, value);

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.insert(value, v);

    Some(value)
}

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_write(
    new_fn: &mut Function,
    old_fn: &hashlink_bytecode::Function,
    reg_meta: &mut RegisterMetadata,
    bb_index: usize,
    v: RegisterIndex,
) -> Option<ValueIndex> {
    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = old_fn.registers[v.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(new_fn.ssa_values.len());
    new_fn.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // Update the register's latest value state. This will also bubble an error up in the event that
    // the register was not already marked as being written in this basic block. We already found
    // the set of registers that a basic block writes to in an earlier pass, trying to say we write
    // any more at this stage is an error.
    reg_meta.basic_block_registers_written[bb_index].insert(v, value)?;

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.insert(value, v);

    Some(value)
}

/// Simple function that handles creating and adding SSA values for instructions. This is a special
/// case where some instruction translations emit multiple eon instructions. These translations can
/// emit SSA values that don't have any mapping to registers and in those cases we should use this
/// function for creating SSA values with no register mapping
pub fn handle_ssa_write_no_register(new_fn: &mut Function, type_: TypeIndex) -> ValueIndex {
    let value = ValueIndex(new_fn.ssa_values.len());
    new_fn.ssa_values.push(SSAValue { type_ });
    value
}
