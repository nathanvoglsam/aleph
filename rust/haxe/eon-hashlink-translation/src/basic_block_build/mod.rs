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

pub use opcode_translate::translate_opcode;
pub use remap_reads::remap_reads;
pub use utils::build_basic_block_predecessor_sets;
pub use utils::find_source_span;
pub use utils::build_basic_block_infos;
pub use utils::handle_ssa_phi_import;
pub use utils::handle_ssa_write;
pub use utils::handle_ssa_write_no_register;
pub use utils::find_type_index_for;
pub use utils::BBInfo;

use crate::basic_block_graph::BasicBlockGraph;
use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
use eon_bytecode::function::{BasicBlock, Function, Register, SSAValue};
use eon_bytecode::indexes::{
    BasicBlockIndex, InstructionIndex, RegisterIndex, TypeIndex, ValueIndex,
};
use eon_bytecode::module::Module;
use eon_bytecode::opcode::{OpCode, Phi, ReceiveException};
use eon_bytecode::type_::{Type, TypeFunction};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub struct RegisterData {
    /// List of registers for the function's bytecode. This maps almost directly to the register
    /// system in hashlink bytecode but with some additional information.
    ///
    /// We hold on to this so we can simplify tracking what actual values the SSA items refer to so
    /// analyzing the bytecode for optimization opportunities is easier.
    pub registers: Vec<Register>,

    /// Maps an SSA value to a register in the register list
    pub register_map: HashMap<ValueIndex, RegisterIndex>,

    /// This list associates with each basic block the set of registers that it writes to, and the
    /// SSA value index that corresponds to the last write (final state) of the register within that
    /// basic block
    pub block_live_registers: Vec<HashMap<RegisterIndex, ValueIndex>>,
}

pub struct BuildContext<'a> {
    pub new_fn: RefCell<Function>,
    pub old_fn: &'a hashlink_bytecode::Function,
    pub module: &'a Module,
    pub bb_graph: RefCell<BasicBlockGraph>,
    pub spans: RefCell<Vec<(InstructionIndex, InstructionIndex)>>,
    pub fn_ty: &'a TypeFunction,
    pub bool_type_index: TypeIndex,
    pub void_type_index: TypeIndex,
    pub non_reg_values: RefCell<HashSet<ValueIndex>>,
    pub reg_meta: RefCell<RegisterData>,
    pub predecessors: RefCell<Vec<HashSet<BasicBlockIndex>>>,
    pub bb_infos: RefCell<Vec<BBInfo>>,
}

pub fn build_bb(
    mut new_fn: Function,
    spans: Vec<(InstructionIndex, InstructionIndex)>,
    old_fn: &hashlink_bytecode::Function,
    module: &Module,
    bb_graph: BasicBlockGraph,
) -> TranspileResult<Function> {
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
    let reg_meta = RegisterData {
        registers,
        register_map,
        block_live_registers,
    };

    // Pre allocate the list of empty basic blocks
    for _ in 0..spans.len() {
        new_fn.basic_blocks.push(BasicBlock { ops: Vec::new() });
    }

    // Calculate the set of predecessor blocks for all basic blocks
    let predecessors = build_basic_block_predecessor_sets(&bb_graph, &spans);

    // Precompute some information about all basic blocks
    let bb_infos = build_basic_block_infos(old_fn, &bb_graph, &spans)?;

    let mut ctx = BuildContext {
        new_fn: RefCell::new(new_fn),
        old_fn,
        module,
        bb_graph: RefCell::new(bb_graph),
        spans: RefCell::new(spans),
        fn_ty,
        bool_type_index,
        void_type_index,
        non_reg_values: RefCell::new(HashSet::new()),
        reg_meta: RefCell::new(reg_meta),
        predecessors: RefCell::new(predecessors),
        bb_infos: RefCell::new(bb_infos),
    };

    // This will check the type signature of the function against the registers the definition says
    // the arguments should be
    type_check_signature(&mut ctx)?;

    // Now we need to build information about the registers read and written by each basic block so
    // we can use it to produce the final SSA form instruction stream
    build_register_live_sets(&mut ctx);

    // Now we need to propagate the live sets through the entire basic block graph, which requires
    // a tree walk
    propagate_predecessor_live_sets(&mut ctx);

    // Now begins the fun part where we start translating the HashLink bytecode
    translate_basic_blocks(&mut ctx)?;

    // The next phase requires a second pass over the now partially translated instructions.
    // Currently all OpCodes are now in Eon form but the OpCodes are not all encoded in a valid
    // state. Most ValueIndex values in an instruction's parameters are invalid, and actually
    // contain the *register index* that is being read, not the value index.
    //
    // This is because the information needed to correctly translate these reads is only created
    // once the above first pass is completed. Now the information is available, we can use it to
    // remap the register indices to value indices
    remap_register_indices(&mut ctx)?;

    Ok(ctx.new_fn.into_inner())
}

pub fn type_check_signature(ctx: &mut BuildContext) -> TranspileResult<()> {
    // Unpack arguments from Cells
    let mut new_fn = ctx.new_fn.borrow_mut();
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let old_fn = ctx.old_fn;
    let fn_ty = ctx.fn_ty;

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

pub fn build_register_live_sets(ctx: &mut BuildContext) {
    // Take from cells
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();
    let old_fn = ctx.old_fn;
    let fn_ty = ctx.fn_ty;

    for (i, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        // Unwrap the bounds and get the sub slice that the span refers to
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;
        let ops = &old_fn.ops[lower_bound..=upper_bound];

        let mut live_regs = HashMap::new();

        // We special case the first basic block as that will be importing the latest states from
        // the function arguments
        if i == 0 {
            for (arg_index, _) in fn_ty.args.iter().enumerate() {
                live_regs.insert(RegisterIndex(arg_index), ValueIndex(arg_index));
            }
        }

        // Iterate over every opcode and record what registers it reads and writes
        for op in ops {
            // Build the set of writes
            if let Some(write) = op.register_write() {
                live_regs.insert(RegisterIndex(write as usize), ValueIndex(0));
            }
        }

        // Add to the metadata
        reg_meta.block_live_registers.push(live_regs);
    }
}

pub fn propagate_predecessor_live_sets(ctx: &mut BuildContext) {
    //let reg_meta = ctx.reg_meta.borrow();
    //let spans = ctx.spans.borrow();
    //let bb_graph = ctx.bb_graph.borrow();
    //let predecessors = ctx.predecessors.borrow();
    //let old_fn = ctx.old_fn;
    //let fn_ty = ctx.fn_ty;
    //
    //let mut handled = vec![false; spans.len()];
    //
    //for i in 0..handled.len() {
    //    if !handled[i] {}
    //}
    //for (bb_index, (lower_bound, _)) in spans.iter().enumerate() {
    //    let predecessors = &predecessors[bb_index];
    //}
}

pub fn translate_basic_blocks(ctx: &mut BuildContext) -> TranspileResult<()> {
    let mut new_fn = ctx.new_fn.borrow_mut();
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let mut non_reg_values = ctx.non_reg_values.borrow_mut();
    let spans = ctx.spans.borrow();
    let predecessors = ctx.predecessors.borrow();
    let bb_infos = ctx.bb_infos.borrow();
    let old_fn = ctx.old_fn;
    let bool_type_index = ctx.bool_type_index;
    let void_type_index = ctx.void_type_index;

    for (bb_index, (lower_bound, upper_bound)) in spans.iter().enumerate() {
        // We need to get some info based on the instructions that jump to this block
        let bb_info = &bb_infos[bb_index];

        // Get the set of predecessor basic block indexes that we will need for emitting phi
        // instructions
        let predecessors = &predecessors[bb_index];

        // We need to produce a set of register indices that holds the intersection of the live sets
        // from all the predecessors.
        //
        // It is only sane behaviour to import SSA values that are live in all predecessor blocks so
        // we need to produce this subset so we can correctly inject phi instructions.
        //
        // The algorithm to do this will be explained below

        // This is a "flat map" that maps a register index to the number of predecessors it is live
        // in. We use this to decide which registers are live in all predecessor blocks.
        let mut live_count = vec![0usize; old_fn.registers.len()];

        // Now we iterate over all the predecessors and accumulate into the `live_count` flat_map
        //
        // Each slot in the map corresponds to a count for each register. Each slot is used to
        // accumulate the number of predecessors the register is live in. If the count is lower than
        // the number of predecessors then the register is not live in all of them and we must not
        // emit phi instructions for that register
        for pred in predecessors.iter() {
            let pred = &reg_meta.block_live_registers[pred.0];
            for reg in pred.keys() {
                live_count[reg.0] += 1;
            }
        }

        // If we have multiple predecessors we need to emit phi instructions that import the
        // state of each register from the predecessors
        if bb_info.has_multiple_predecessors {
            // Filter out all registers that are not live in all predecessor blocks
            let live_regs = live_count
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == predecessors.len());
            for (reg, _) in live_regs {
                // Dereference and new-type the register into our own type
                let reg = RegisterIndex(reg);

                // We allocate a new SSA value for the result of our phi instruction
                let assigns =
                    handle_ssa_phi_import(&mut new_fn, old_fn, &mut reg_meta, bb_index, reg);

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
                &mut new_fn,
                old_fn,
                &mut reg_meta,
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
        for (i, old_op) in old_fn.ops[lower_bound.0..=upper_bound.0].iter().enumerate() {
            // Get the actual index in the opcode array rather than the index in the sub-slice
            // we've taken
            let op_index = lower_bound.0 + i;
            translate_opcode(
                &mut new_fn,
                &mut reg_meta,
                &mut non_reg_values,
                old_fn,
                &spans,
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
    for bb_index in 0..new_fn.basic_blocks.len() {
        let predecessors = &predecessors[bb_index];

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

pub fn remap_register_indices(ctx: &mut BuildContext) -> TranspileResult<()> {
    let mut new_fn = ctx.new_fn.borrow_mut();
    let reg_meta = ctx.reg_meta.borrow();
    let non_reg_values = ctx.non_reg_values.borrow();
    let predecessors = ctx.predecessors.borrow();
    let fn_ty = ctx.fn_ty;

    // We allocate reuse this between iterations to save allocating every iteration
    let mut latest_states = HashMap::new();

    // Iterate over all basic blocks
    for (bb_index, bb) in new_fn.basic_blocks.iter_mut().enumerate() {
        // Get the set of predecessor basic block indexes that we will need for importing values
        // in the special case of basic blocks with only a single predecessor
        let predecessors = &predecessors[bb_index];

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
        for op in bb.ops.iter_mut() {
            remap_reads(op, &reg_meta, &non_reg_values, &latest_states);

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
