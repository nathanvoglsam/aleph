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

use crate::basic_block_build::RegisterData;
use crate::basic_block_compute::BasicBlockSpans;
use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
use eon_bytecode::function::{Function, SSAValue};
use eon_bytecode::indexes::{BasicBlockIndex, RegisterIndex, TypeIndex, ValueIndex};
use eon_bytecode::type_::Type;
use std::collections::HashSet;

// Get the set of predecessor basic blocks for the basic block that starts with instruction
// `first_instruction_index`
pub fn build_basic_block_predecessor_sets(
    spans: &BasicBlockSpans,
) -> Vec<HashSet<BasicBlockIndex>> {
    // Build the predecessor set for every basic block
    spans
        .spans
        .iter()
        .map(|span| {
            let lower_bound = &span.begin;

            let mut mapped_predecessors = HashSet::new();

            // Get the list of predecessors and map the instruction back to the basic block (span) that it
            // came from
            if let Some(predecessors) = spans.destination_sources.get(lower_bound) {
                for predecessor in predecessors {
                    // Find the source span
                    //
                    // Hard fail here as failing to find the source span for the predecessor is a bug in the
                    // algorithm. Bugs are not errors and should be very violently surfaced so they can be
                    // found and fixed
                    let block = find_source_span(spans, predecessor.0).unwrap();

                    // Insert our mapped index into our new list
                    mapped_predecessors.insert(BasicBlockIndex(block));
                }
            }

            mapped_predecessors
        })
        .collect()
}

/// Find the span, in the given list, that holds the given instruction index
pub fn find_source_span(spans: &BasicBlockSpans, i: usize) -> Option<usize> {
    spans
        .spans
        .iter()
        .enumerate()
        .find(|(_, v)| v.begin.0 <= i && v.end.0 >= i)
        .map(|(i, _)| i)
}

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_phi_import(
    new_fn: &mut Function,
    old_fn: &hashlink::Function,
    reg_meta: &mut RegisterData,
    bb_index: usize,
    v: RegisterIndex,
) -> ValueIndex {
    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = old_fn.registers[v.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(new_fn.ssa_values.len());
    new_fn.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // While this isn't a real write, the basic block should already have this marked as live from
    // an earlier part of the algorithm. If it doesn't then it is likely a bug
    reg_meta.block_live_registers[bb_index]
        .insert(v, value)
        .unwrap();

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.insert(value, v);

    value
}

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_write(
    new_fn: &mut Function,
    old_fn: &hashlink::Function,
    reg_meta: &mut RegisterData,
    bb_index: usize,
    v: RegisterIndex,
) -> ValueIndex {
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
    //
    // Once again, an error here is a bug and so should be surfaced as a panic
    reg_meta.block_live_registers[bb_index]
        .insert(v, value)
        .unwrap();

    // Add to the register map so we can map the ValueIndex back to the register it represents
    //
    // This should never overwrite another value so we panic if it does
    assert!(reg_meta.register_map.insert(value, v).is_none());

    value
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

/// A struct that represents the set of information computed from the `get_basic_block_info`
/// function
pub struct BBInfo {
    /// Whether this basic block has more than one predecessor
    pub has_multiple_predecessors: bool,

    /// Whether this basic block is only reached from a single other basic block. This is
    /// useful so we can elide some phi instructions.
    pub has_single_predecessor: bool,

    /// We also need to know if for w/e reason this block has no predecessors so we can
    /// ensure that this is *ONLY* true for the entry block
    pub has_no_predecessor: bool,

    /// Whether we've detected that this basic block is intended to be used as a trap handler
    /// for when an exception is thrown. This requires us to enforce that the basic block is
    /// only reached from a single source (the OpTrap) and we need to emit our own
    /// instruction for handling reading the exception
    pub is_trap_handler: bool,

    /// The register the HashLink bytecode was told to *STORE* the exception into. We need
    /// this so we can remap the HashLink `OpTrap` into our own pair of `OpTrap` and
    /// `OpReceiveException`.
    pub trap_register: usize,
}

/// This function uses the source HashLink function, the earlier computed BBGraph and the index of
/// the first instruction in the source HashLink of the relevant basic block to compute some info
/// about the basic block in question.
///
/// This function also does some error checking
pub fn build_basic_block_infos(
    old_fn: &hashlink::Function,
    spans: &BasicBlockSpans,
) -> TranspileResult<Vec<BBInfo>> {
    let mut out = Vec::new();

    for span in spans.spans.iter() {
        let lower_bound = &span.begin;

        let has_multiple_predecessors;
        let has_single_predecessor;
        let has_no_predecessor;
        let is_trap_handler;
        let trap_register;

        // We need to get the list of instruction indexes that contain a branch instruction
        // with this basic block as the target so we can deduce the above information
        //
        // We can use the info calculated earlier from the BBGraph
        let sources = spans.destination_sources.get(lower_bound);
        if let Some(sources) = sources {
            // These are pretty self explanatory
            has_multiple_predecessors = sources.len() > 1;
            has_single_predecessor = sources.len() == 1;
            has_no_predecessor = sources.len() == 0;

            // Here we filter out and create a vector of only the trap instructions. This way
            let mut trap_sources = sources.iter().filter_map(|v| {
                let source_op = &old_fn.ops[v.0];
                match source_op {
                    hashlink::OpCode::OpTrap(v) => Some(v),
                    _ => None,
                }
            });

            // We only care about the first response, if there's any more than a single OpTrap that
            // targets this basic block that is an error we need to surface.
            if let Some(trap) = trap_sources.next() {
                is_trap_handler = true;
                trap_register = trap.param_1 as usize;

                // The iterator should only yield a single result. If it can yield more then we have
                // multiple traps leading to the same handler, which is an error.
                if trap_sources.count() > 0 {
                    let reason = InvalidFunctionReason::TrapHandlerHasMultipleTrapPredecessors {
                        func: old_fn.clone(),
                    };
                    let err = TranspileError::InvalidFunction(reason);
                    return Err(err);
                }

                // If a trap handler block has multiple predecessor blocks then that is an error
                if has_multiple_predecessors {
                    let reason = InvalidFunctionReason::TrapHandlerHasMultiplePredecessors {
                        func: old_fn.clone(),
                    };
                    let err = TranspileError::InvalidFunction(reason);
                    return Err(err);
                }
            } else {
                is_trap_handler = false;
                trap_register = 0;
            }
        } else {
            has_multiple_predecessors = false;
            has_single_predecessor = false;
            has_no_predecessor = true;
            is_trap_handler = false;
            trap_register = 0;
        }

        out.push(BBInfo {
            has_multiple_predecessors,
            has_single_predecessor,
            has_no_predecessor,
            is_trap_handler,
            trap_register,
        });
    }
    Ok(out)
}

/// Simple function that wraps an iterator to find the matching type definition in the type list and
/// returning its TypeIndex.
pub fn find_type_index_for(types: &[Type], val: &Type) -> Option<TypeIndex> {
    types
        .iter()
        .enumerate()
        .find_map(|(i, v)| if v == val { Some(TypeIndex(i)) } else { None })
}
