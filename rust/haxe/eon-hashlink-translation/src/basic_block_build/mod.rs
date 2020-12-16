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
mod remap_reads;
mod utils;

pub(crate) use opcode_translate::translate_opcode;
pub(crate) use remap_reads::remap_reads;
pub(crate) use utils::find_source_span;
pub(crate) use utils::get_basic_block_info;
pub(crate) use utils::get_basic_block_predecessor_list;
pub(crate) use utils::handle_ssa_phi_import;
pub(crate) use utils::handle_ssa_write;
pub(crate) use utils::handle_ssa_write_no_register;
pub(crate) use utils::BBInfo;

use crate::basic_block_graph::BasicBlockGraph;
use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
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
) -> TranspileResult<()> {
    // Get the actual function type value, checking to ensure it is of the correct type category
    // (Function or Method)
    let fn_ty = &module.types[new_fn.type_.0];
    let fn_ty = fn_ty.get_type_function().ok_or(type_index_error(old_fn))?;

    // We need to find the index of the bool type as we need it for part of the translation process
    // later
    //
    // This is a hard error as realistically this should be declared in every module, so we panic.
    let bool_type_index = find_type_index_for(&module.types, &Type::Bool).unwrap();
    let void_type_index = find_type_index_for(&module.types, &Type::Void).unwrap();

    // As we go we'll be generating various bits of metadata about the transcoded instructions
    let registers = vec![Register::default(); old_fn.registers.len()];
    let register_map = HashMap::new();
    let block_live_registers = Vec::new();
    let mut reg_meta = RegisterMetadata {
        registers,
        register_map,
        block_live_registers,
    };

    // The set of values that do not have a matching register in the HashLink source.
    let mut non_reg_values = HashSet::new();

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
    build_register_live_sets(&mut reg_meta, &old_fn, fn_ty, &spans);

    // Now begins the fun part where we start translating the HashLink bytecode
    translate_basic_blocks(
        new_fn,
        &mut reg_meta,
        &mut non_reg_values,
        &old_fn,
        &bb_graph,
        fn_ty,
        &spans,
        bool_type_index,
        void_type_index,
    )?;

    // The next phase requires a second pass over the now partially translated instructions.
    // Currently all OpCodes are now in Eon form but the OpCodes are not all encoded in a valid
    // state. Most ValueIndex values in an instruction's parameters are invalid, and actually
    // contain the *register index* that is being read, not the value index.
    //
    // This is because the information needed to correctly translate these reads is only created
    // once the above first pass is completed. Now the information is available, we can use it to
    // remap the register indices to value indices
    remap_register_indices(
        new_fn,
        &mut reg_meta,
        &non_reg_values,
        &bb_graph,
        fn_ty,
        &spans,
    )?;

    new_fn.metadata.reg_data = Some(reg_meta);

    Ok(())
}

pub fn type_check_signature(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    fn_ty: &TypeFunction,
) -> TranspileResult<()> {
    // Go over the function arguments and check that the types in the signature match the registers
    // in the actual function definition while inserting the SSA values for them at the same time
    for (i, arg_ty) in fn_ty.args.iter().enumerate() {
        // Get the type for the register that matches the function argument
        let reg_ty = old_fn.registers[i] as usize;

        // Error if the types don't match
        if arg_ty.0 != reg_ty {
            let reason = InvalidFunctionReason::FunctionSignatureArgNotMatchRegister {
                a_index: i,
                func: old_fn.clone(),
            };
            let err = TranspileError::InvalidFunction(reason);
            return Err(err);
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

    Ok(())
}

pub fn build_register_live_sets(
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    fn_ty: &TypeFunction,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
) {
    for (i, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        // Unwrap the bounds and get the sub slice that the span refers to
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;
        let ops = &old_fn.ops[lower_bound..=upper_bound];

        let mut reg_writes = HashMap::new();

        // We special case the first basic block as that will be importing the latest states from
        // the function arguments
        if i == 0 {
            for (arg_index, _) in fn_ty.args.iter().enumerate() {
                reg_writes.insert(RegisterIndex(arg_index), ValueIndex(arg_index));
            }
        }

        // Iterate over every opcode and record what registers it reads and writes
        for op in ops {
            // Build the set of writes
            if let Some(write) = op.register_write() {
                reg_writes.insert(RegisterIndex(write as usize), ValueIndex(0));
            }
        }

        // Add to the metadata
        reg_meta.block_live_registers.push(reg_writes);
    }
}

pub fn translate_basic_blocks(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    non_reg_values: &mut HashSet<ValueIndex>,
    old_fn: &hashlink_bytecode::Function,
    bb_graph: &BasicBlockGraph,
    fn_ty: &TypeFunction,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
    bool_type_index: TypeIndex,
    void_type_index: TypeIndex,
) -> TranspileResult<()> {
    for (bb_index, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        // Unwrap the lower and upper bounds from the reference and new-type
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;

        // We need to get some info based on the instructions that jump to this block
        let bb_info = get_basic_block_info(&old_fn, bb_graph, lower_bound)?;

        // Get the set of predecessor basic block indexes that we will need for emitting phi
        // instructions
        let predecessors = get_basic_block_predecessor_list(bb_graph, spans, lower_bound);

        // If we have multiple predecessors we need to emit phi instructions that import the
        // state of each register from the predecessors
        if bb_info.has_multiple_predecessors {
            for reg in 0..old_fn.registers.len() {
                // Dereference and new-type the register into our own type
                let reg = RegisterIndex(reg);

                // We allocate a new SSA value for the result of our phi instruction
                let assigns = handle_ssa_phi_import(new_fn, old_fn, reg_meta, bb_index, reg);

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
        if bb_info.is_trap_handler {
            // Emit a new SSA value that gets assigned the exception value
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(bb_info.trap_register),
            );

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
                non_reg_values,
                old_fn,
                spans,
                bool_type_index,
                void_type_index,
                bb_index,
                op_index,
                old_op,
            );
        }
    }

    // Now we need to pass over the basic blocks again and patch up some of the metadata with
    // some missing information for the next phase of the algorithm.
    //
    // Specifically we require that the "latest state" map holds information for the entire set of
    // registers for every basic block
    for (bb_index, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        let predecessors = get_basic_block_predecessor_list(bb_graph, spans, lower_bound.0);

        if predecessors.len() == 1 {
            let predecessor = predecessors.iter().next().unwrap();

            // Swap the actual map out of the array as we need to mutate another member in the array
            let mut pred_info = HashMap::new();
            std::mem::swap(
                &mut pred_info,
                &mut reg_meta.block_live_registers[predecessor.0],
            );

            let iterator = pred_info.iter().map(|(k, v)| (*k, *v));
            for (k, v) in iterator {
                if !reg_meta.block_live_registers[bb_index].contains_key(&k) {
                    reg_meta.block_live_registers[bb_index].insert(k, v);
                }
            }

            // Move the original map back in to the list. Dropping the old one is find and an empty
            // map doesn't allocate so this should have almost no effect on performance
            reg_meta.block_live_registers[predecessor.0] = pred_info;
        }
    }

    Ok(())
}

pub fn remap_register_indices(
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    non_reg_values: &HashSet<ValueIndex>,
    bb_graph: &BasicBlockGraph,
    fn_ty: &TypeFunction,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
) -> TranspileResult<()> {
    // We allocate reuse this between iterations to save allocating every iteration
    let mut latest_states = HashMap::new();

    // Iterate over all basic blocks
    for (bb_index, bb) in new_fn.basic_blocks.iter_mut().enumerate() {
        let (lower_bound, _) = spans[bb_index];

        // Get the set of predecessor basic block indexes that we will need for importing values
        // in the special case of basic blocks with only a single predecessor
        let predecessors = get_basic_block_predecessor_list(bb_graph, spans, lower_bound.0);

        // The entry basic block is a special case where the latest state of the registers is
        // imported directly from the function arguments
        if bb_index == 0 {
            for arg_index in 0..fn_ty.args.len() {
                latest_states.insert(RegisterIndex(arg_index), ValueIndex(arg_index));
            }
        }

        // If there's only a single predecessor we don't emit phi instructions but import the values
        // directly from the predecessor blocks. Phi instructions are only needed to merge values
        // that converge from distinct branches/execution paths.
        //
        // As such we fill the latest states map directly with the contents of the predecessor's
        // final states.
        if predecessors.len() == 1 {
            let predecessor = predecessors.iter().next().unwrap();
            for (r, v) in reg_meta.block_live_registers[predecessor.0].iter() {
                latest_states.insert(*r, *v);
            }
        }

        // Iterate over every opcode, remapping reads and updating the rolling "latest state" for
        // each register
        for (op_index, op) in bb.ops.iter_mut().enumerate() {
            remap_reads(op, reg_meta, non_reg_values, &latest_states);

            if let Some(write) = op.get_assigned_value() {
                if let Some(register) = reg_meta.register_map.get(&write) {
                    latest_states.insert(*register, write);
                }
            }
        }

        // Clear the state map for use in the next iteration, but keep the allocated memory around
        latest_states.clear();
    }

    Ok(())
}

fn type_index_error(old_fn: &hashlink_bytecode::Function) -> TranspileError {
    let reason = InvalidFunctionReason::TypeIndexNotFunction {
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}

fn find_type_index_for(types: &[Type], val: &Type) -> Option<TypeIndex> {
    types
        .iter()
        .enumerate()
        .find_map(|(i, v)| if v == val { Some(TypeIndex(i)) } else { None })
}
