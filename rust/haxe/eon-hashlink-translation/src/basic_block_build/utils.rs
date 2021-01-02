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
use crate::utils::is_begin_trap;
use eon_bytecode::function::{Function, SSAValue};
use eon_bytecode::indexes::{RegisterIndex, TypeIndex, ValueIndex};
use eon_bytecode::type_::Type;

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_phi_import(
    new_fn: &mut Function,
    old_fn: &hashlink::Function,
    reg_meta: &mut RegisterData,
    bb_index: usize,
    reg: RegisterIndex,
) -> ValueIndex {
    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = old_fn.registers[reg.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(new_fn.ssa_values.len());
    new_fn.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // While this isn't a real write, the basic block should already have this marked as live from
    // an earlier part of the algorithm. If it doesn't then it is likely a bug
    reg_meta.block_live_set[bb_index].insert(reg, value).unwrap();

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.insert(value, reg);

    value
}

/// Simple function that handles creating and adding SSA values for instructions
pub fn handle_ssa_write(
    new_fn: &mut Function,
    old_fn: &hashlink::Function,
    reg_meta: &mut RegisterData,
    bb_index: usize,
    reg: RegisterIndex,
) -> ValueIndex {
    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = old_fn.registers[reg.0] as usize;

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
    reg_meta.block_live_set[bb_index].insert(reg, value).unwrap();

    // Add to the register map so we can map the ValueIndex back to the register it represents
    //
    // This should never overwrite another value so we panic if it does
    assert!(reg_meta.register_map.insert(value, reg).is_none());

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
#[derive(Clone, Debug, Default)]
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
    pub trap_register: RegisterIndex,
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
    let mut out: Vec<BBInfo> = vec![Default::default(); spans.spans.len()];

    for (span_index, span) in spans.spans.iter().enumerate() {
        // We need to get the list of instruction indexes that contain a branch instruction
        // with this basic block as the target so we can deduce the above information
        //
        // We can use the info calculated earlier from the BBGraph
        let predecessors = &spans.predecessors[span_index];
        let has_multiple_predecessors = predecessors.len() > 1;
        let has_single_predecessor = predecessors.len() == 1;
        let has_no_predecessor = predecessors.len() == 0;

        // We check if the current block ends with an `OpTrap`, if so we need to mark the
        // exceptional target as a trap handler
        if let Some(op_trap) = is_begin_trap(span.end.0, &old_fn.ops[span.end.0], old_fn) {
            // Propagate the error
            let (register, handler) = op_trap?;

            // Find the basic block for the exception handler, this must exist so panic if it
            // doesn't. (algorithm error)
            let handler = spans.find_source_span_starting_with(handler).unwrap();

            // If the handler has already been marked as a trap handler, we must ensure that the
            // same register is used as the exception target from both
            if out[handler.0].is_trap_handler {
                // If the registers don't match then we need to surface an error
                if out[handler.0].trap_register != register {
                    let reason = InvalidFunctionReason::TrapHandlerHasMultipleTrapPredecessors {
                        func: old_fn.clone(),
                    };
                    let err = TranspileError::InvalidFunction(reason);
                    return Err(err);
                }
            } else {
                out[handler.0].is_trap_handler = true;
                out[handler.0].trap_register = register;
            }
        }

        out[span_index].has_multiple_predecessors = has_multiple_predecessors;
        out[span_index].has_single_predecessor = has_single_predecessor;
        out[span_index].has_no_predecessor = has_no_predecessor;
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
