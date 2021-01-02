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
pub use utils::build_basic_block_infos;
pub use utils::find_type_index_for;
pub use utils::handle_ssa_phi_import;
pub use utils::handle_ssa_write;
pub use utils::handle_ssa_write_no_register;
pub use utils::BBInfo;

use crate::basic_block_compute::BasicBlockSpans;
use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
use crate::utils::intersect_hash_sets;
use eon_bytecode::function::{Function, SSAValue};
use eon_bytecode::indexes::{BasicBlockIndex, RegisterIndex, TypeIndex, ValueIndex};
use eon_bytecode::intrinsic::Intrinsic;
use eon_bytecode::module::Module;
use eon_bytecode::opcode::{CallIntrinsic, OpCode, Phi};
use eon_bytecode::type_::{Type, TypeFunction};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct RegisterData {
    /// Maps a register index (one that likely doesn't exist in the source HashLink) to the value
    /// index that represents it. A virtual register by definition only has a single assignment as
    /// they are generated during the instruction translation and map directly to SSA values
    pub virtual_registers: HashMap<RegisterIndex, ValueIndex>,

    /// Maps an SSA value to a register in the register list
    pub register_map: HashMap<ValueIndex, RegisterIndex>,

    /// A flat map. Holds the set of registers that each basic block writes to
    pub block_writes: Vec<HashSet<RegisterIndex>>,

    /// A flat map. Holds the set of registers that each basic block imports from its
    /// predecessors with phi instructions
    pub block_imports: Vec<HashSet<RegisterIndex>>,

    /// A flat map. Holds the set of all registers that are live at the end of a block, either
    /// because they were written to in that block or were imported with a phi instruction. The set
    /// also maps the register to the value index for the final SSA value form of the register in
    /// the block.
    pub block_live_set: Vec<HashMap<RegisterIndex, ValueIndex>>,
}

impl RegisterData {
    pub fn new(block_count: usize) -> Self {
        let virtual_registers = HashMap::new();
        let register_map = HashMap::new();
        let block_writes = vec![Default::default(); block_count];
        let block_imports = vec![Default::default(); block_count];
        let block_live_set = vec![Default::default(); block_count];
        RegisterData {
            virtual_registers,
            register_map,
            block_writes,
            block_imports,
            block_live_set,
        }
    }
}

#[derive(Debug)]
pub struct BuildContext<'a> {
    pub new_fn: RefCell<Function>,
    pub old_fn: &'a hashlink::Function,
    pub module: &'a Module,
    pub spans: RefCell<BasicBlockSpans>,
    pub fn_ty: &'a TypeFunction,
    pub bool_type_index: TypeIndex,
    pub void_type_index: TypeIndex,
    pub reg_meta: RefCell<RegisterData>,
    pub block_reachability: RefCell<Vec<bool>>,
    pub bb_infos: RefCell<Vec<BBInfo>>,
}

pub fn build_bb(
    spans: BasicBlockSpans,
    old_fn: &hashlink::Function,
    module: &Module,
) -> TranspileResult<Function> {
    let mut ctx = {
        let mut new_fn = Function {
            type_: TypeIndex(old_fn.type_ as usize),
            f_index: old_fn.f_index,
            ssa_values: vec![],
            basic_blocks: vec![],
        };

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
        let reg_meta = RegisterData::new(spans.spans.len());

        // Pre allocate the list of empty basic blocks
        new_fn
            .basic_blocks
            .resize(spans.spans.len(), Default::default());

        // Calculate the set of predecessor blocks for all basic blocks

        let block_reachability = vec![false; spans.spans.len()];

        // Precompute some information about all basic blocks
        let bb_infos = build_basic_block_infos(old_fn, &spans)?;

        BuildContext {
            new_fn: RefCell::new(new_fn),
            old_fn,
            module,
            spans: RefCell::new(spans),
            fn_ty,
            bool_type_index,
            void_type_index,
            reg_meta: RefCell::new(reg_meta),
            block_reachability: RefCell::new(block_reachability),
            bb_infos: RefCell::new(bb_infos),
        }
    };

    // This will check the type signature of the function against the registers the definition says
    // the arguments should be
    type_check_signature(&mut ctx)?;

    // Initializes the live set information of the first basic block
    build_first_block_imports_and_live_set(&mut ctx);

    // We need to go over each basic block and calculate whether it can actually be reached.
    //
    // This is needed for building the live sets as when propagating live sets we should skip
    // importing from unreachable blocks.
    calculate_block_reachability(&mut ctx)?;

    // Now we need to build information about the registers read and written by each basic block so
    // we can use it to produce the final SSA form instruction stream
    build_block_write_sets(&mut ctx);

    // Now we need to propagate the live sets through the entire basic block graph, which requires
    // a tree walk
    propagate_imports(&mut ctx);

    // Merges the register imports and writes for each basic block into the "live set"
    resolve_live_sets(&mut ctx);

    // Now begins the fun part where we start translating the HashLink bytecode
    translate_basic_blocks(&mut ctx)?;

    propagate_latest_states(&mut ctx);

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

pub fn build_first_block_imports_and_live_set(ctx: &mut BuildContext) {
    // Take from cells
    let mut reg_meta = ctx.reg_meta.borrow_mut();

    let fn_ty = ctx.fn_ty;

    for (arg_index, _) in fn_ty.args.iter().enumerate() {
        // The function arguments are "imported" from the caller, which we make clear here.
        // The first basic block is already a special case all over the code so making the
        // imports of the first basic block special doesn't really make anything more
        // complex than it otherwise would've been
        reg_meta.block_imports[0].insert(arg_index.into());
        reg_meta.block_live_set[0].insert(arg_index.into(), arg_index.into());
    }
}

pub fn calculate_block_reachability(ctx: &mut BuildContext) -> TranspileResult<()> {
    let mut block_reachability = ctx.block_reachability.borrow_mut();
    let spans = ctx.spans.borrow();
    let immediate_predecessors = &spans.predecessors;

    let entry_block = BasicBlockIndex(0);
    block_reachability[entry_block.0] = true;

    let mut handled = HashSet::with_capacity(32);
    let mut next = Vec::with_capacity(32);

    let real_blocks = spans
        .spans
        .iter()
        .enumerate()
        .skip(1)
        .map(|v| BasicBlockIndex(v.0));
    for block_being_checked in real_blocks {
        // Clear these, ready for the next iteration
        handled.clear();
        next.clear();

        // Push the block we want to check the reachability of as the first item on the stack
        next.push(block_being_checked);

        // Keep popping items off the queue until it is empty
        while let Some(current) = next.pop() {
            // Get the predecessors for the current item
            let predecessors = &immediate_predecessors[current.0];

            // We check if a block has already been confirmed to be reachable from an earlier check
            //
            // This also checks for the entry block as the entry block is automatically marked as
            // reachable
            //
            // If we can reach an already guaranteed reachable block then we know the source block
            // we started from must also be reachable. As such we can use this as an early exit
            if block_reachability[current.0] {
                // If so then this block is confirmed to be reachable. Mark it as such and break
                // from the loop
                block_reachability[block_being_checked.0] = true;
                break;
            }

            // Mark the current block as handled
            handled.insert(current);

            // We only want to insert items that have not been handled into the queue, otherwise we
            // may loop infinitely.
            //
            // So we filter out anything that has already been handled
            let unhandled_predecessors = predecessors
                .iter()
                .cloned()
                .filter(|v| !handled.contains(v));

            // Queue all unhandled predecessors
            next.extend(unhandled_predecessors);
        }
    }

    Ok(())
}

pub fn build_block_write_sets(ctx: &mut BuildContext) {
    // Take from cells
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();
    let bb_infos = ctx.bb_infos.borrow();
    let old_fn = ctx.old_fn;

    for (i, span) in spans.spans.iter().enumerate().skip(1) {
        let info = &bb_infos[i];

        // Unwrap the bounds and get the sub slice that the span refers to
        let lower_bound = span.begin.0;
        let upper_bound = span.end.0;
        let ops = &old_fn.ops[lower_bound..=upper_bound];

        // Trap handlers are a somewhat special type of basic block that start with a unique
        // instruction, OpReceiveException. OpReceiveException is similar to a phi instruction, but
        // actually does perform a real write to the register and is not an import so we need to
        // correctly flag this as such.
        //
        // The source HashLink does not have an `OpReceiveException`, our translation generates it.
        // The only way for the `block_writes` set to be generated correctly is to inject the write
        // manually with this special case.
        if info.is_trap_handler {
            reg_meta.block_writes[i].insert(info.trap_register);
        }

        // Iterate over every opcode and record what registers it writes
        for op in ops {
            // Build the set of writes

            // We have to ignore OpTrap specifically as although it does perform a write of sorts
            // under the semantics of HashLink, it is not considered a write for the purposes of our
            // translation.
            //
            // As such we special case it out
            if !op.is_trap() {
                // When the op isn't an OpTrap we add the write to the set of written registers
                if let Some(write) = op.register_write() {
                    reg_meta.block_writes[i].insert(RegisterIndex(write as usize));
                }
            }
        }
    }
}

pub fn propagate_imports(ctx: &mut BuildContext) {
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();
    let immediate_predecessors = &spans.predecessors;
    let immediate_successors = &spans.successors;
    let block_reachability = ctx.block_reachability.borrow();

    // This stack holds the queue of instruction indices to be handled. This is the core part of
    // our recursive tree walk of the source instruction stream
    let mut next_stack = Vec::new();

    // We hold a set of all basic blocks that have been handled at least once
    let mut handled_once_blocks = HashSet::new();

    // The entry basic block is guaranteed to be valid by this point so we just flag it as valid
    // by default
    handled_once_blocks.insert(BasicBlockIndex(0));

    // We start at the first basic block after the entry block
    next_stack.push(BasicBlockIndex(1));

    // Continuously loop until the queue is empty
    while let Some(current) = next_stack.pop() {
        // Get an iterator that yields the live set of all handled predecessors
        let predecessors = &immediate_predecessors[current.0];
        let handled_predecessor_live_sets = predecessors
            .intersection(&handled_once_blocks)
            .cloned()
            .filter(|v| block_reachability[v.0])
            .map(|v| {
                reg_meta.block_writes[v.0]
                    .union(&reg_meta.block_imports[v.0])
                    .cloned()
            });

        // Now we need to produce the intersection of all predecessor block's live sets so we have
        // a final set that is a subset of all handled predecessor's live set.
        let new_imports = intersect_hash_sets(handled_predecessor_live_sets);

        let mut arg_indices = 0..ctx.fn_ty.args.len();
        let arg_missing_from_import_set =
            arg_indices.any(|v| !new_imports.contains(&RegisterIndex(v)));
        if arg_missing_from_import_set {
            panic!("An argument has failed to be propagated");
        }

        // If this isn't the first time a block is being handled then we check if there was no
        // change between the new and old set
        //
        // If there is no change between the new and old import set then we should terminate at the
        // current block. If the old and new sets are identical then there is no change to propagate
        // to successor blocks so there's no point continuing
        let imports_changed = &new_imports != &reg_meta.block_imports[current.0];
        let not_yet_handled = !handled_once_blocks.contains(&current);
        if imports_changed || not_yet_handled {
            next_stack.extend(&immediate_successors[current.0]);
        }

        // Mark as handled
        handled_once_blocks.insert(current);

        // Store the new import set
        reg_meta.block_imports[current.0] = new_imports;
    }
}

pub fn resolve_live_sets(ctx: &mut BuildContext) {
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();

    // Merge the imports and writes for each block, skipping the entry block as the entry block is
    // already resolved correctly
    for (i, _) in spans.spans.iter().enumerate().skip(1) {
        // The live set is the union of the imports and block writes
        let live_set = reg_meta.block_writes[i]
            .union(&reg_meta.block_imports[i])
            .cloned();

        // Build the hashmap with a dummy ValueIndex which will be patched up later
        let mut live_set = live_set.map(|v| (v, ValueIndex(0))).collect();

        // Insert the new map into the list
        std::mem::swap(&mut reg_meta.block_live_set[i], &mut live_set);

        // Debug check to make sure this function isn't overriding data (the existing set/map should
        // be empty)
        debug_assert!(live_set.is_empty());
    }
}

pub fn translate_basic_blocks(ctx: &mut BuildContext) -> TranspileResult<()> {
    let mut new_fn = ctx.new_fn.borrow_mut();
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();
    let predecessors = &spans.predecessors;
    let block_reachability = ctx.block_reachability.borrow();
    let bb_infos = ctx.bb_infos.borrow();
    let old_fn = ctx.old_fn;
    let bool_type_index = ctx.bool_type_index;
    let void_type_index = ctx.void_type_index;

    for (bb_index, span) in spans.real_block_iter().enumerate() {
        let lower_bound = &span.begin;
        let upper_bound = &span.end;

        // We need to get some info based on the instructions that jump to this block
        let bb_info = &bb_infos[bb_index];

        // Get the set of predecessor basic block indexes that we will need for emitting phi
        // instructions
        let predecessors = &predecessors[bb_index];

        // If we have multiple predecessors we need to emit phi instructions that import the
        // state of each register from the predecessors
        if bb_info.has_multiple_predecessors {
            let block_imports = std::mem::take(&mut reg_meta.block_imports[bb_index]);

            for reg in block_imports.iter().cloned() {
                // We allocate a new SSA value for the result of our phi instruction
                let assigns =
                    handle_ssa_phi_import(&mut new_fn, old_fn, &mut reg_meta, bb_index, reg);

                // We produce the list of source blocks for the phi instruction with a
                // ValueIndex that actually holds the *register index* so we can remap it later
                // once all the information we need is available.
                //
                // We only import from reachable blocks
                let block_values = predecessors
                    .iter()
                    .cloned()
                    .filter(|v| block_reachability[v.0])
                    .map(|v| (ValueIndex(reg.0), v))
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

            reg_meta.block_imports[bb_index] = block_imports;
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
                bb_info.trap_register,
            );

            let receive_exception = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::ReceiveException,
                fn_params: Vec::new(),
            };
            let receive_exception = OpCode::OpCallIntrinsic(receive_exception);

            // Insert the new instruction
            new_fn.basic_blocks[bb_index].ops.push(receive_exception);
        }

        // Iterate over all the opcodes that we've deduced to be a part of this basic block
        for (op_index, old_op) in old_fn.ops[lower_bound.0..=upper_bound.0]
            .iter()
            .enumerate()
            .map(|(i, v)| (i + lower_bound.0, v))
        {
            translate_opcode(
                &mut new_fn,
                &mut reg_meta,
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

    // Fill last block with OpUnreachable
    new_fn
        .basic_blocks
        .last_mut()
        .unwrap()
        .ops
        .push(OpCode::OpUnreachable);

    Ok(())
}

pub fn propagate_latest_states(ctx: &mut BuildContext) {
    let mut reg_meta = ctx.reg_meta.borrow_mut();
    let spans = ctx.spans.borrow();
    let immediate_predecessors = &spans.predecessors;
    let immediate_successors = &spans.successors;

    // This stack holds the queue of instruction indices to be handled. This is the core part of
    // our recursive tree walk of the source instruction stream
    let mut next_stack = Vec::new();

    // We hold a set of all basic blocks that have been handled at least once
    let mut handled_once_blocks = HashSet::new();

    next_stack.push(BasicBlockIndex(0));

    // Continuously loop until the queue is empty
    while let Some(current) = next_stack.pop() {
        let mut current_live_set = std::mem::take(&mut reg_meta.block_live_set[current.0]);
        let current_writes = &reg_meta.block_writes[current.0];
        let current_imports = &reg_meta.block_imports[current.0];

        // We only need fix blocks with singular predecessors as multi predecessor blocks will have
        // the latest states handled when emitting phi instructions
        let predecessors = &immediate_predecessors[current.0];
        if predecessors.len() == 1 {
            // Unpack the predecessor and get the predecessor's live set
            let predecessor = predecessors.iter().cloned().next().unwrap();
            let predecessor_live_set = &reg_meta.block_live_set[predecessor.0];

            // We need to update the latest state for only the registers that were imported but not
            // written to within the current block
            for import in current_imports.difference(current_writes).cloned() {
                // Get the state from the predecessor
                let state = predecessor_live_set.get(&import).cloned().unwrap();

                // Update the live set, panic if we're inserting a new value
                current_live_set.insert(import, state).unwrap();
            }
        }

        let unhandled_successors = immediate_successors[current.0]
            .iter()
            .cloned()
            .filter(|v| !handled_once_blocks.contains(v));
        next_stack.extend(unhandled_successors);

        // Mark as handled
        handled_once_blocks.insert(current);

        reg_meta.block_live_set[current.0] = current_live_set;
    }
}

pub fn remap_register_indices(ctx: &mut BuildContext) -> TranspileResult<()> {
    let mut new_fn = ctx.new_fn.borrow_mut();
    let spans = ctx.spans.borrow();
    let reg_meta = ctx.reg_meta.borrow();
    let predecessors = &spans.predecessors;

    // We allocate reuse this between iterations to save allocating every iteration
    let mut latest_states = HashMap::new();

    // Iterate over all basic blocks
    for (bb_index, bb) in new_fn.basic_blocks.iter_mut().enumerate() {
        // Get the set of predecessor basic block indexes that we will need for importing values
        // in the special case of basic blocks with only a single predecessor
        let predecessors = &predecessors[bb_index];

        // If there's only a single predecessor we don't emit phi instructions but import the values
        // directly from the predecessor blocks. Phi instructions are only needed to merge values
        // that converge from distinct branches/execution paths.
        //
        // As such we fill the latest states map directly with the contents of the predecessor's
        // final states.
        if predecessors.len() == 1 {
            let predecessor = predecessors.iter().next().unwrap();
            for (r, v) in reg_meta.block_live_set[predecessor.0].iter() {
                latest_states.insert(*r, *v);
            }
        }

        // Iterate over every opcode, remapping reads and updating the rolling "latest state" for
        // each register
        for op in bb.ops.iter_mut() {
            remap_reads(op, &reg_meta, &latest_states);

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

fn type_index_error(old_fn: &hashlink::Function) -> TranspileError {
    let reason = InvalidFunctionReason::TypeIndexNotFunction {
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}
