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

mod opcode_translate;
mod utils;

pub(crate) use opcode_translate::translate_opcode;
pub(crate) use utils::find_source_span;
pub(crate) use utils::get_basic_block_predecessor_list;
pub(crate) use utils::handle_ssa_phi_import;
pub(crate) use utils::handle_ssa_write;
pub(crate) use utils::handle_ssa_write_no_register;

use crate::basic_block_graph::BasicBlockGraph;
use eon_bytecode::function::{BasicBlock, Function, Register, RegisterMetadata, SSAValue};
use eon_bytecode::indexes::{InstructionIndex, RegisterIndex, TypeIndex, ValueIndex};
use eon_bytecode::module::Module;
use eon_bytecode::opcode::{OpCode, Phi, ReceiveException};
use eon_bytecode::type_::{Type, TypeFunction};
use std::collections::{HashMap, HashSet};

pub fn build_bb(
    new_fn: &mut Function,
    old_fn: &hashlink_bytecode::Function,
    module: &Module,
    bb_graph: BasicBlockGraph,
    mut spans: Vec<(InstructionIndex, InstructionIndex)>,
) -> Option<()> {
    // Get the actual function type value, checking to ensure it is of the correct type category
    // (Function or Method)
    let fn_ty = &module.types[new_fn.type_.0];
    let fn_ty = fn_ty.get_type_function()?;

    // We need to find the index of the bool type as we need it for part of the translation process
    // later
    let bool_type_index = module.types.iter().enumerate().find_map(|(i, v)| {
        if let Type::Bool = v {
            Some(TypeIndex(i))
        } else {
            None
        }
    })?;

    // As we go we'll be generating various bits of metadata about the transcoded instructions
    let registers = vec![Register::default(); old_fn.registers.len()];
    let register_map = HashMap::new();
    let basic_block_registers_read = Vec::new();
    let basic_block_registers_written = Vec::new();
    let mut reg_meta = RegisterMetadata {
        registers,
        register_map,
        basic_block_registers_read,
        basic_block_registers_written,
    };

    // Pre allocate the list of empty basic blocks
    for _ in 0..spans.len() {
        new_fn.basic_blocks.push(BasicBlock { ops: Vec::new() });
    }

    // This will check the type signature of the function against the registers the definition says
    // the arguments should be
    type_check_signature(new_fn, &mut reg_meta, old_fn, fn_ty)?;

    // The spans array is a list of ranges into the source hashlink bytecode. Each entry encodes the
    // span of instructions that should be encoded into a basic block.
    //
    // Instruction index 0 in the source bytecode is special as it **must** be the first instruction
    // of the first basic block (the function entry point). The algorithm that generates the spans
    // list does not guarantee that the first span corresponds to the first instruction in the
    // source.
    //
    // To fix this we sort the array so that the span for instruction 0 will be the first item in
    // the array, meaning we can transparently handle all of them and it will always be the first
    // basic block
    spans.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Now we need to build information about the registers read and written by each basic block so
    // we can use it to produce the final SSA form instruction stream
    build_register_usage_map(&mut reg_meta, &old_fn, &spans);

    // Now begins the fun part where we start translating the HashLink bytecode
    translate_basic_blocks(
        new_fn,
        &mut reg_meta,
        &old_fn,
        &bb_graph,
        fn_ty,
        &spans,
        bool_type_index,
    )?;

    // The next phase requires a second pass over the now partially translated instructions.
    // Currently all OpCodes are now in Eon form but the OpCodes are not all encoded in a valid
    // state. Most ValueIndex values in an instruction's parameters are invalid, and actually
    // contain the *register index* that is being read, not the value index.
    //
    // This is because the information needed to correctly translate these reads is only created
    // once the above first pass is completed. Now the information is available, we can use it to
    // remap the register indices to value indices
    remap_register_indices(new_fn, &mut reg_meta)?;

    new_fn.metadata.reg_data = Some(reg_meta);

    Some(())
}

pub fn type_check_signature(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    fn_ty: &TypeFunction,
) -> Option<()> {
    // Go over the function arguments and check that the types in the signature match the registers
    // in the actual function definition while inserting the SSA values for them at the same time
    for (i, arg_ty) in fn_ty.args.iter().enumerate() {
        // Get the type for the register that matches the function argument
        let reg_ty = old_fn.registers[i] as usize;

        // Error if the types don't match
        if arg_ty.0 != reg_ty {
            return None;
        }

        // Insert an SSA value for this argument that points to the first instruction in the first
        // basic block. The first instruction will always be a special no-op type instruction so
        // that bb: 0 and instr: 0 can be used as a marker for function arguments.
        new_fn.ssa_values.push(SSAValue {
            type_: TypeIndex(reg_ty),
        });

        // Insert the information to map the SSA value back to the register it refers to
        reg_meta
            .register_map
            .insert(ValueIndex(i), RegisterIndex(i));
    }

    Some(())
}

pub fn build_register_usage_map(
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
) {
    for (lower_bound, upper_bound) in spans {
        // Unwrap the bounds and get the sub slice that the span refers to
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;
        let ops = &old_fn.ops[lower_bound..=upper_bound];

        let mut reg_reads = HashSet::new();
        let mut reg_writes = HashMap::new();

        // Iterate over every opcode and record what registers it reads and writes
        for op in ops {
            // Build the set of reads
            if let Some(reads) = op.register_reads() {
                for read in reads {
                    reg_reads.insert(RegisterIndex(read as usize));
                }
            }

            // Build the set of writes
            if let Some(write) = op.register_write() {
                reg_writes.insert(RegisterIndex(write as usize), ValueIndex(0));
            }
        }

        // Add to the metadata
        reg_meta.basic_block_registers_read.push(reg_reads);
        reg_meta.basic_block_registers_written.push(reg_writes);
    }
}

pub fn translate_basic_blocks(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    bb_graph: &BasicBlockGraph,
    fn_ty: &TypeFunction,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
    bool_type_index: TypeIndex,
) -> Option<()> {
    for (bb_index, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        // Unwrap the lower and upper bounds from the reference and new-type
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;

        // We need to get some info based on the instructions that jump to this block
        let (
            has_multiple_predecessors,
            has_single_predecessor,
            has_no_predecessor,
            is_trap_handler,
            trap_register,
        ) = get_basic_block_info(&old_fn, bb_graph, lower_bound)?;

        // Get the set of predecessor basic block indexes that we will need for emitting phi
        // instructions
        let predecessors = get_basic_block_predecessor_list(bb_graph, spans, lower_bound)?;

        // If we have multiple predecessors we need to emit phi instructions that import the
        // state of each register from the predecessors
        if has_multiple_predecessors {
            for reg in 0..old_fn.registers.len() {
                // Dereference and new-type the register into our own type
                let reg = RegisterIndex(reg);

                // We allocate a new SSA value for the result of our phi instruction
                let assigns = handle_ssa_phi_import(new_fn, old_fn, reg_meta, bb_index, reg)?;

                // We produce the list of source blocks for the phi instruction with a
                // ValueIndex that actually holds the *register index* so we can remap it later
                // once all the information we need is available.
                let block_values = predecessors
                    .iter()
                    .map(|v| (ValueIndex(reg.0), *v))
                    .collect();

                // Build the phi layout
                let phi = Phi {
                    assigns,
                    block_values,
                };

                // And pack into the instruction enum
                let phi = OpCode::OpPhi(phi);

                // And insert the new instruction
                new_fn.basic_blocks[bb_index].ops.push(phi);
            }
        }

        // If this block is a trap handler then we need to emit an instruction to import the
        // exception value
        //
        // The `get_basic_block_info` function automatically checks for errors so
        // `has_multiple_predecessors` and `is_trap_handler` are implicitly mutually exclusive
        if is_trap_handler {
            // Emit a new SSA value that gets assigned the exception value
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(trap_register),
            )?;

            // Build the instruction layout
            let receive_exception = ReceiveException { assigns };

            // Package the instruction enum
            let receive_exception = OpCode::OpReceiveException(receive_exception);

            // Insert the new instruction
            new_fn.basic_blocks[bb_index].ops.push(receive_exception);
        }

        // Iterate over all the opcodes that we've deduced to be a part of this basic block
        for (i, old_op) in old_fn.ops[lower_bound..=upper_bound].iter().enumerate() {
            // Get the actual index in the opcode array rather than the index in the sub-slice
            // we've taken
            let op_index = lower_bound + i;
            translate_opcode(
                new_fn,
                reg_meta,
                old_fn,
                spans,
                bool_type_index,
                bb_index,
                op_index,
                old_op,
            )?;
        }
    }

    Some(())
}

/// This function uses the source HashLink function, the earlier computed BBGraph and the index of
/// the first instruction in the source HashLink of the relevant basic block to compute some info
/// about the basic block in question.
///
/// This function also does some error checking
pub fn get_basic_block_info(
    old_fn: &hashlink_bytecode::Function,
    bb_graph: &BasicBlockGraph,
    first_instruction_index: usize,
) -> Option<(bool, bool, bool, bool, usize)> {
    // Whether this basic block has more than one predecessor
    let has_multiple_predecessors;

    // Whether this basic block is only reached from a single other basic block. This is
    // useful so we can elide some phi instructions.
    let has_single_predecessor;

    // We also need to know if for w/e reason this block has no predecessors so we can
    // ensure that this is *ONLY* true for the entry block
    let has_no_predecessor;

    // Whether we've detected that this basic block is intended to be used as a trap handler
    // for when an exception is thrown. This requires us to enforce that the basic block is
    // only reached from a single source (the OpTrap) and we need to emit our own
    // instruction for handling reading the exception
    let is_trap_handler;

    // The register the HashLink bytecode was told to *STORE* the exception into. We need
    // this so we can remap the HashLink `OpTrap` into our own pair of `OpTrap` and
    // `OpReceiveException`.
    let trap_register;

    // We need to get the list of instruction indexes that contain a branch instruction
    // with this basic block as the target so we can deduce the above information
    //
    // We can use the info calculated earlier from the BBGraph
    let sources = bb_graph
        .destination_sources
        .get(&InstructionIndex(first_instruction_index));
    if let Some(sources) = sources {
        // These are pretty self explanatory
        has_multiple_predecessors = sources.len() > 1;
        has_single_predecessor = sources.len() == 1;
        has_no_predecessor = sources.len() == 0;

        // Here we filter out and create a vector of only the trap instructions. This way
        let mut trap_sources = sources.iter().filter_map(|v| {
            let source_op = &old_fn.ops[v.0];
            match source_op {
                hashlink_bytecode::OpCode::OpTrap(v) => Some(v),
                _ => None,
            }
        });

        // We only care about the first response, if there's any more than a single OpTrap that
        // targets this basic block that is an error we need to surface.
        if let Some(trap) = trap_sources.next() {
            is_trap_handler = true;
            trap_register = trap.param_1 as usize;

            // If a trap handler block has multiple predecessor blocks then that is an error
            if has_multiple_predecessors {
                return None;
            }

            // The iterator should only yield a single result. If it can yield more then we have
            // multiple traps leading to the same handler, which is an error.
            if trap_sources.count() > 0 {
                return None;
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

    Some((
        has_multiple_predecessors,
        has_single_predecessor,
        has_no_predecessor,
        is_trap_handler,
        trap_register,
    ))
}

pub fn remap_register_indices(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
) -> Option<()> {
    for (bb_index, bb) in new_fn.basic_blocks.iter().enumerate() {}

    Some(())
}
