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
use crate::opcode_translators::{
    translate_alloc_enum, translate_binop, translate_call, translate_call_closure,
    translate_call_field, translate_cast, translate_comp_branch, translate_cond_branch,
    translate_end_trap, translate_field_load, translate_field_store, translate_instance_closure,
    translate_load, translate_load_bool, translate_load_bytes, translate_load_enum_field,
    translate_load_float, translate_load_global, translate_load_int, translate_load_string,
    translate_load_type, translate_make_enum, translate_no_params, translate_read_memory,
    translate_ref_data, translate_ref_offset, translate_static_closure, translate_store,
    translate_store_enum_field, translate_store_global, translate_switch, translate_trap,
    translate_unconditional_branch, translate_unop, translate_value_index,
    translate_virtual_closure, translate_write_memory,
};
use crate::utils::offset_from;
use eon_bytecode::function::{BasicBlock, Function, Register, RegisterMetadata, SSAValue};
use eon_bytecode::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, InstructionIndex, IntegerIndex, RegisterIndex, StringIndex, TypeIndex, ValueIndex,
};
use eon_bytecode::module::Module;
use eon_bytecode::opcode::{OpCode, Phi, ReceiveException};
use eon_bytecode::type_::TypeFunction;
use std::collections::{HashMap, HashSet};

pub fn build_bb(
    out: &mut Function,
    module: &Module,
    f: &hashlink_bytecode::Function,
    bb_graph: BasicBlockGraph,
    mut spans: Vec<(InstructionIndex, InstructionIndex)>,
) -> Option<()> {
    // Get the actual function type value, checking to ensure it is of the correct type category
    // (Function or Method)
    let fn_ty = &module.types[out.type_.0];
    let fn_ty = fn_ty.get_type_function()?;

    // As we go we'll be generating various bits of metadata about the transcoded instructions
    let registers = vec![Register::default(); f.registers.len()];
    let register_map = Vec::new();
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
        out.basic_blocks.push(BasicBlock { ops: Vec::new() });
    }

    // This will check the type signature of the function against the registers the definition says
    // the arguments should be
    type_check_signature(out, f, fn_ty, &mut reg_meta)?;

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
    build_register_usage_map(&f, &spans, &mut reg_meta);

    // Now begins the fun part where we start translating the HashLink bytecode
    translate_basic_blocks(out, &f, &bb_graph, fn_ty, &spans, &mut reg_meta)?;

    out.metadata.reg_data = Some(reg_meta);

    Some(())
}

pub fn type_check_signature(
    out: &mut Function,
    f: &hashlink_bytecode::Function,
    fn_ty: &TypeFunction,
    reg_meta: &mut RegisterMetadata,
) -> Option<()> {
    // Go over the function arguments and check that the types in the signature match the registers
    // in the actual function definition while inserting the SSA values for them at the same time
    for (i, arg_ty) in fn_ty.args.iter().enumerate() {
        // Get the type for the register that matches the function argument
        let reg_ty = f.registers[i] as usize;

        // Error if the types don't match
        if arg_ty.0 != reg_ty {
            return None;
        }

        // Insert an SSA value for this argument that points to the first instruction in the first
        // basic block. The first instruction will always be a special no-op type instruction so
        // that bb: 0 and instr: 0 can be used as a marker for function arguments.
        out.ssa_values.push(SSAValue {
            type_: TypeIndex(reg_ty),
        });

        // Insert the information to map the SSA value back to the register it refers to
        reg_meta.register_map.push(RegisterIndex(i));
    }

    Some(())
}

pub fn build_register_usage_map(
    f: &hashlink_bytecode::Function,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
    reg_meta: &mut RegisterMetadata,
) {
    for (lower_bound, upper_bound) in spans {
        // Unwrap the bounds and get the sub slice that the span refers to
        let lower_bound = lower_bound.0;
        let upper_bound = upper_bound.0;
        let ops = &f.ops[lower_bound..=upper_bound];

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
    out: &mut Function,
    f: &hashlink_bytecode::Function,
    bb_graph: &BasicBlockGraph,
    fn_ty: &TypeFunction,
    spans: &Vec<(InstructionIndex, InstructionIndex)>,
    reg_meta: &mut RegisterMetadata,
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
        ) = get_basic_block_info(&f, bb_graph, lower_bound)?;

        // Get the set of predecessor basic block indexes that we will need for emitting phi
        // instructions
        let predecessors = get_basic_block_predecessor_list(bb_graph, spans, lower_bound)?;

        // If we have multiple predecessors we need to emit phi instructions that import the
        // state of each register from the predecessors
        if has_multiple_predecessors {
            for reg in 0..f.registers.len() {
                // Dereference and new-type the register into our own type
                let reg = RegisterIndex(reg);

                // We allocate a new SSA value for the result of our phi instruction
                let assigns = handle_ssa_phi_import(out, reg_meta, f, bb_index, reg)?;

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
                out.basic_blocks[bb_index].ops.push(phi);
            }
        }

        // If this block is a trap handler then we need to emit an instruction to import the
        // exception value
        //
        // The `get_basic_block_info` function automatically checks for errors so
        // `has_multiple_predecessors` and `is_trap_handler` are implicitly mutually exclusive
        if is_trap_handler {
            // Emit a new SSA value that gets assigned the exception value
            let assigns =
                handle_ssa_write(out, reg_meta, f, bb_index, RegisterIndex(trap_register))?;

            // Build the instruction layout
            let receive_exception = ReceiveException { assigns };

            // Package the instruction enum
            let receive_exception = OpCode::OpReceiveException(receive_exception);

            // Insert the new instruction
            out.basic_blocks[bb_index].ops.push(receive_exception);
        }

        // Iterate over all the opcodes that we've deduced to be a part of this basic block
        for (i, old_op) in f.ops[lower_bound..=upper_bound].iter().enumerate() {
            // Get the actual index in the opcode array rather than the index in the sub-slice
            // we've taken
            let op_index = lower_bound + i;

            let new_op = translate_opcode(out, f, spans, reg_meta, bb_index, op_index, old_op)?;

            out.basic_blocks[bb_index].ops.push(new_op);
        }
    }

    Some(())
}

/// This is one of the core function that performs the first stage of opcode translation.
///
/// This will translate the opcode variant of each source HashLink opcode into a corresponding
/// opcode in our own bytecode. This will also perform half of the SSA transformation.
///
/// When translating an opcode, any write to a register will be mapped into the creation and
/// assignment of a new SSA value in the function. Any opcode that assigns an SSA value will have
/// the value index for the assignment correctly filled and valid. No further translation will be
/// needed for those instruction parameters.
///
/// Unfortunately we can not translate the register reads into a read of the correct SSA value as
/// we don't even know how many SSA values exists yet. The information needed to remap the reads of
/// registers into reading SSA values is *PRODUCED* by this function. As such, be very aware of the
/// following:
///
/// # Warning
///
/// The instructions emitted by this function are *NOT* guaranteed to be valid, and most will only
/// be partially translated. Any instruction parameter that encodes a *read* of an SSA value will
/// actually contain the *register index* directly copied from the HashLink source. A later pass
/// over the translated instructions is needed to correctly map the register indexes to the value
/// indexes.
pub fn translate_opcode(
    out: &mut Function,
    f: &hashlink_bytecode::Function,
    spans: &[(InstructionIndex, InstructionIndex)],
    reg_meta: &mut RegisterMetadata,
    bb_index: usize,
    op_index: usize,
    old_op: &hashlink_bytecode::OpCode,
) -> Option<OpCode> {
    let new_op = match old_op {
        hashlink_bytecode::OpCode::OpMov(params) => {
            // Allocate new SSA value for the assignment param
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Temporarily store the register of the read in the second parameter to be
            // remapped later
            let source = ValueIndex(params.param_2 as usize);
            translate_load(old_op, assigns, source)?
        }
        hashlink_bytecode::OpCode::OpInt(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Unpack the index into the integer table
            let integer = IntegerIndex(params.param_2 as usize);

            translate_load_int(old_op, assigns, integer)?
        }
        hashlink_bytecode::OpCode::OpFloat(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Unpack the index into the float table
            let float = FloatIndex(params.param_2 as usize);

            translate_load_float(old_op, assigns, float)?
        }
        hashlink_bytecode::OpCode::OpBool(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // `OpBool` holds the value to assign directly in the opcode, not in a separate
            // table like the other opcodes. We convert this into a native boolean type and
            // then pack the value directly into the instruction too.
            let value = params.param_2 != 0;

            translate_load_bool(old_op, assigns, value)?
        }
        hashlink_bytecode::OpCode::OpBytes(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Unpack the index into the bytes table
            let bytes = BytesIndex(params.param_2 as usize);

            translate_load_bytes(old_op, assigns, bytes)?
        }
        hashlink_bytecode::OpCode::OpString(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Unpack the index into the string table
            let string = StringIndex(params.param_2 as usize);

            translate_load_string(old_op, assigns, string)?
        }

        hashlink_bytecode::OpCode::OpNull(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            translate_value_index(old_op, assigns)?
        }

        hashlink_bytecode::OpCode::OpAdd(params)
        | hashlink_bytecode::OpCode::OpSub(params)
        | hashlink_bytecode::OpCode::OpMul(params)
        | hashlink_bytecode::OpCode::OpSDiv(params)
        | hashlink_bytecode::OpCode::OpUDiv(params)
        | hashlink_bytecode::OpCode::OpSMod(params)
        | hashlink_bytecode::OpCode::OpUMod(params)
        | hashlink_bytecode::OpCode::OpShl(params)
        | hashlink_bytecode::OpCode::OpSShr(params)
        | hashlink_bytecode::OpCode::OpUShr(params)
        | hashlink_bytecode::OpCode::OpAnd(params)
        | hashlink_bytecode::OpCode::OpOr(params)
        | hashlink_bytecode::OpCode::OpXor(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Once again, these are *register indexes* for remapping later
            let lhs = ValueIndex(params.param_2 as usize);
            let rhs = ValueIndex(params.param_3 as usize);

            translate_binop(old_op, assigns, lhs, rhs)?
        }

        hashlink_bytecode::OpCode::OpNeg(params) | hashlink_bytecode::OpCode::OpNot(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // These are both unops where the HashLink source actually separately encodes
            // the assignment target and operand so we can just translate this directly
            let operand = ValueIndex(params.param_2 as usize);

            translate_unop(old_op, assigns, operand)?
        }

        hashlink_bytecode::OpCode::OpIncr(params) | hashlink_bytecode::OpCode::OpDecr(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // A unop in HashLink is similar to a `i++` statement where it both reads and
            // writes to the register it specifies. Such an operation requires a separate
            // source parameter when in SSA form as all values can only be assigned to once.
            //
            // Regardless, this contains the *register index* of the value so we can remap
            // to the actual SSA value later
            let operand = ValueIndex(params.param_1 as usize);

            translate_unop(old_op, assigns, operand)?
        }

        hashlink_bytecode::OpCode::OpCall0(_)
        | hashlink_bytecode::OpCode::OpCall1(_)
        | hashlink_bytecode::OpCode::OpCall2(_)
        | hashlink_bytecode::OpCode::OpCall3(_)
        | hashlink_bytecode::OpCode::OpCall4(_)
        | hashlink_bytecode::OpCode::OpCallN(_) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = old_op.register_write()? as usize;
            let assigns = handle_ssa_write(out, reg_meta, f, bb_index, RegisterIndex(assigns))?;

            // Unpack the function index
            let function = FunctionIndex(old_op.get_param_2()? as usize);

            // Convert the parameter list. It will produce a list of *register indexes*
            // disguised as value indexes which we need to remap in a later pass over the
            // translated instructions
            let fn_params = old_op.get_static_call_args()?;
            let fn_params = fn_params
                .into_iter()
                .map(|v| ValueIndex(v as usize))
                .collect();

            translate_call(old_op, assigns, function, fn_params)?
        }

        hashlink_bytecode::OpCode::OpCallMethod(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Convert the field index into the form we need
            let function = FieldIndex(params.param_2 as usize);

            // The object is stored implicitly as the first argument in HashLink's bytecode.
            // Ours explicitly makes it a parameter on the opcode rather than implicitly
            // hidden in the argument list.
            //
            // Regardless, we need to store the *register index* into the value index so we
            // can remap it later.
            let object = ValueIndex(params.extra[0] as usize);

            // All params in the list after the first are the actual function arguments so
            // we skip the first when converting the argument list.
            //
            // Again these actually contain *register indexes* for later remapping
            let fn_params = params.extra[1..]
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            translate_call_field(old_op, assigns, object, function, fn_params)?
        }
        hashlink_bytecode::OpCode::OpCallThis(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // `OpCallThis` implicitly refers to register 0 as the first argument when
            // calling the function. There is no sane way to represent this in an SSA form
            // language so we decay this down to an `OpCallMethod`.
            //
            // As such we implicitly create a *register index* of 0 which is used as the
            // source object. This will be remapped later
            let object = ValueIndex(0);

            // Convert the field index for the function into the form we need
            let function = FieldIndex(params.param_2 as usize);

            // Map the argument list into ValueIndexes that actually hold *register indexes*
            // so we can remap later
            let fn_params = params
                .extra
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            translate_call_field(old_op, assigns, object, function, fn_params)?
        }

        hashlink_bytecode::OpCode::OpCallClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // The closure which is read and used as the function to call
            let closure = ValueIndex(params.param_2 as usize);

            // Map the argument list into ValueIndexes that actually hold *register indexes*
            // so we can remap later
            let fn_params = params
                .extra
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            translate_call_closure(old_op, assigns, closure, fn_params)?
        }

        hashlink_bytecode::OpCode::OpStaticClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Get the function index to call
            let function = FunctionIndex(params.param_2 as usize);

            translate_static_closure(old_op, assigns, function)?
        }
        hashlink_bytecode::OpCode::OpInstanceClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Get the function index to call
            let function = FunctionIndex(params.param_2 as usize);

            // Register index for remapping later
            let object = ValueIndex(params.param_3 as usize);

            translate_instance_closure(old_op, assigns, function, object)?
        }
        hashlink_bytecode::OpCode::OpVirtualClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index for remapping later
            let object = ValueIndex(params.param_2 as usize);

            // Field index
            let field = FieldIndex(params.param_3 as usize);

            translate_virtual_closure(old_op, assigns, object, field)?
        }

        hashlink_bytecode::OpCode::OpGetGlobal(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Map into global index
            let source = GlobalIndex(params.param_2 as usize);

            translate_load_global(old_op, assigns, source)?
        }
        hashlink_bytecode::OpCode::OpSetGlobal(params) => {
            // The register index for remapping later. This reads the register into the
            // target global
            let source = ValueIndex(params.param_1 as usize);

            // The global to write into
            let target = GlobalIndex(params.param_2 as usize);

            translate_store_global(old_op, source, target)?
        }

        hashlink_bytecode::OpCode::OpGetThis(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let object = ValueIndex(0);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            translate_field_load(old_op, assigns, object, field)?
        }

        hashlink_bytecode::OpCode::OpField(params)
        | hashlink_bytecode::OpCode::OpDynGet(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let object = ValueIndex(params.param_2 as usize);

            // Field index
            let field = FieldIndex(params.param_3 as usize);

            translate_field_load(old_op, assigns, object, field)?
        }

        hashlink_bytecode::OpCode::OpSetThis(params) => {
            // Register index
            let object = ValueIndex(0);

            // Field index
            let field = FieldIndex(params.param_1 as usize);

            // Register index of source to read into object
            let source = ValueIndex(params.param_2 as usize);

            translate_field_store(old_op, object, field, source)?
        }

        hashlink_bytecode::OpCode::OpSetField(params)
        | hashlink_bytecode::OpCode::OpDynSet(params) => {
            // Register index
            let object = ValueIndex(params.param_1 as usize);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            // Register index of source to read into object
            let source = ValueIndex(params.param_3 as usize);

            translate_field_store(old_op, object, field, source)?
        }

        hashlink_bytecode::OpCode::OpJTrue(params)
        | hashlink_bytecode::OpCode::OpJFalse(params)
        | hashlink_bytecode::OpCode::OpJNull(params)
        | hashlink_bytecode::OpCode::OpJNotNull(params) => {
            // Get the value to read for the conditional branch. Stores a *register index*
            // for remapping later
            let check = ValueIndex(params.param_1 as usize);

            // HashLink uses offsets relative to the current instruction index for encoding
            // it's jump destinations. Our bytecode uses basic block indices. As such we
            // need to find the basic block we're jumping to by looking for what span the
            // calculated index is in.
            let success = offset_from(op_index, params.param_2)?; // Apply offset
            let success = find_source_span(spans, success)?; // Find block
            let success = BasicBlockIndex(success); // Wrap value

            // The HashLink bytecode encodes two branch edges, one with the parameter and
            // a second implicitly in the behaviour of the instruction. If the instruction's
            // check fails then the instruction should just continue to the next instruction
            // in the stream.
            //
            // This effectively encodes two edges, one to the instruction in the parameter
            // and a second as the instruction after the branch.
            //
            // We explicitly encode both edges, as we branch to basic blocks and not to
            // instruction indices. Here we find the basic block the next instruction can be
            // found in and use that as the branch target for the failure case
            let failure = find_source_span(spans, op_index + 1)?;
            let failure = BasicBlockIndex(failure);

            translate_cond_branch(old_op, check, success, failure)?
        }

        hashlink_bytecode::OpCode::OpJSLt(params)
        | hashlink_bytecode::OpCode::OpJSGte(params)
        | hashlink_bytecode::OpCode::OpJSGt(params)
        | hashlink_bytecode::OpCode::OpJSLte(params)
        | hashlink_bytecode::OpCode::OpJULt(params)
        | hashlink_bytecode::OpCode::OpJUGte(params)
        | hashlink_bytecode::OpCode::OpJNotLt(params)
        | hashlink_bytecode::OpCode::OpJNotGte(params)
        | hashlink_bytecode::OpCode::OpJEq(params)
        | hashlink_bytecode::OpCode::OpJNotEq(params) => {
            // Get the value to read for the conditional branch. Stores a *register index*
            // for remapping later
            let lhs = ValueIndex(params.param_1 as usize);
            let rhs = ValueIndex(params.param_2 as usize);

            // See above for explanation
            let success = offset_from(op_index, params.param_3)?; // Apply offset
            let success = find_source_span(spans, success)?; // Find block
            let success = BasicBlockIndex(success); // Wrap value

            let failure = find_source_span(spans, op_index + 1)?;
            let failure = BasicBlockIndex(failure);

            translate_comp_branch(old_op, lhs, rhs, success, failure)?
        }

        hashlink_bytecode::OpCode::OpJAlways(params) => {
            // See above for explanation
            let inner = offset_from(op_index, params.param_1)?; // Apply offset
            let inner = find_source_span(spans, inner)?; // Find block
            let inner = BasicBlockIndex(inner); // Wrap value

            translate_unconditional_branch(old_op, inner)?
        }

        hashlink_bytecode::OpCode::OpToDyn(params)
        | hashlink_bytecode::OpCode::OpToSFloat(params)
        | hashlink_bytecode::OpCode::OpToUFloat(params)
        | hashlink_bytecode::OpCode::OpToInt(params)
        | hashlink_bytecode::OpCode::OpSafeCast(params)
        | hashlink_bytecode::OpCode::OpUnsafeCast(params)
        | hashlink_bytecode::OpCode::OpToVirtual(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            translate_cast(old_op, assigns, source)?
        }

        hashlink_bytecode::OpCode::OpSwitch(params) => {
            // Register index
            let input = ValueIndex(params.param_1 as usize);

            // Translate the jump table, which deals with jump offsets, into the format we
            // need which jumps to basic block indices
            let mut jump_table = Vec::new();
            for destination in &params.extra {
                let destination = offset_from(op_index, *destination)?;
                let destination = find_source_span(spans, destination)?;
                let destination = BasicBlockIndex(destination);
                jump_table.push(destination);
            }

            // Translate and map the fallback branch offset into the basic block it is
            // supposed to jump to
            let fallback = params.param_3;
            let fallback = offset_from(op_index, fallback)?;
            let fallback = find_source_span(spans, fallback)?;
            let fallback = BasicBlockIndex(fallback);

            translate_switch(old_op, input, jump_table, fallback)?
        }
        hashlink_bytecode::OpCode::OpTrap(params) => {
            // See above for explanation
            let destination = offset_from(op_index, params.param_1)?; // Apply offset
            let destination = find_source_span(spans, destination)?; // Find block
            let destination = BasicBlockIndex(destination); // Wrap value
            translate_trap(old_op, destination)?
        }
        hashlink_bytecode::OpCode::OpEndTrap(params) => {
            // The `OpEndTrap` instruction has a boolean value. I currently have no idea
            // what information it encodes and it looks like the original HashLink JIT
            // compiler doesn't actually use it. I'm going to hold on to it until I am 100%
            // confident in the information being useless.
            //
            // The interpreter in the Haxe compiler doesn't read the parameter either so I
            // don't believe it encodes any useful semantics for execution. Maybe it gets
            // used by the optimizer?
            //
            // Either way, this translates the raw integer into a native boolean type.
            let inner = params.param_1 != 0;

            translate_end_trap(old_op, inner)?
        }

        hashlink_bytecode::OpCode::OpGetI8(params)
        | hashlink_bytecode::OpCode::OpGetI16(params)
        | hashlink_bytecode::OpCode::OpGetMem(params)
        | hashlink_bytecode::OpCode::OpGetArray(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            // Register index
            let offset = ValueIndex(params.param_3 as usize);

            translate_read_memory(old_op, assigns, source, offset)?
        }

        hashlink_bytecode::OpCode::OpSetI8(params)
        | hashlink_bytecode::OpCode::OpSetI16(params)
        | hashlink_bytecode::OpCode::OpSetMem(params)
        | hashlink_bytecode::OpCode::OpSetArray(params) => {
            // Register index
            let target = ValueIndex(params.param_1 as usize);

            // Register index
            let offset = ValueIndex(params.param_2 as usize);

            // Register index
            let source = ValueIndex(params.param_3 as usize);

            translate_write_memory(old_op, target, offset, source)?
        }

        hashlink_bytecode::OpCode::OpType(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Type index
            let source = TypeIndex(params.param_2 as usize);

            translate_load_type(old_op, assigns, source)?
        }

        hashlink_bytecode::OpCode::OpRet(params)
        | hashlink_bytecode::OpCode::OpThrow(params)
        | hashlink_bytecode::OpCode::OpRethrow(params)
        | hashlink_bytecode::OpCode::OpNullCheck(params) => {
            // Register index
            let value = ValueIndex(params.param_1 as usize);

            translate_value_index(old_op, value)?
        }

        hashlink_bytecode::OpCode::OpNew(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            translate_value_index(old_op, assigns)?
        }

        hashlink_bytecode::OpCode::OpArraySize(params)
        | hashlink_bytecode::OpCode::OpGetType(params)
        | hashlink_bytecode::OpCode::OpGetTID(params)
        | hashlink_bytecode::OpCode::OpRef(params)
        | hashlink_bytecode::OpCode::OpUnRef(params)
        | hashlink_bytecode::OpCode::OpEnumIndex(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            translate_load(old_op, assigns, source)?
        }

        hashlink_bytecode::OpCode::OpSetRef(params) => {
            // Register index
            let target = ValueIndex(params.param_1 as usize);

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            translate_store(old_op, target, source)?
        }
        hashlink_bytecode::OpCode::OpMakeEnum(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Constructor index
            let constructor = ConstructorIndex(params.param_2 as usize);

            // Build the list of *register indexes* that we remap to actual value indexes
            // later
            let args = params
                .extra
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            translate_make_enum(old_op, assigns, constructor, args)?
        }
        hashlink_bytecode::OpCode::OpEnumAlloc(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Constructor index
            let constructor = ConstructorIndex(params.param_2 as usize);

            translate_alloc_enum(old_op, assigns, constructor)?
        }
        hashlink_bytecode::OpCode::OpEnumField(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Constructor index
            let source = ValueIndex(params.param_2 as usize);

            // Constructor index
            let constructor = ConstructorIndex(params.param_3 as usize);

            // Constructor index
            let field_index = FieldIndex(params.param_4 as usize);

            translate_load_enum_field(old_op, assigns, source, constructor, field_index)?
        }
        hashlink_bytecode::OpCode::OpSetEnumField(params) => {
            // Register index
            let target = ValueIndex(params.param_1 as usize);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            // Register index
            let source = ValueIndex(params.param_3 as usize);

            translate_store_enum_field(old_op, target, field, source)?
        }
        hashlink_bytecode::OpCode::OpRefData(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            translate_ref_data(old_op, assigns, source)?
        }
        hashlink_bytecode::OpCode::OpRefOffset(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                out,
                reg_meta,
                f,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            )?;

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            // Register index
            let offset = ValueIndex(params.param_3 as usize);

            translate_ref_offset(old_op, assigns, source, offset)?
        }

        hashlink_bytecode::OpCode::OpAssert
        | hashlink_bytecode::OpCode::OpNop
        | hashlink_bytecode::OpCode::OpLabel => translate_no_params(old_op)?,
    };
    Some(new_op)
}

/// This function uses the source HashLink function, the earlier computed BBGraph and the index of
/// the first instruction in the source HashLink of the relevant basic block to compute some info
/// about the basic block in question.
///
/// This function also does some error checking
pub fn get_basic_block_info(
    f: &hashlink_bytecode::Function,
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
            let source_op = &f.ops[v.0];
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
    out: &mut Function,
    reg_meta: &mut RegisterMetadata,
    f: &hashlink_bytecode::Function,
    bb_index: usize,
    v: RegisterIndex,
) -> Option<ValueIndex> {
    // Assert that the two lengths match so that the vec will continue to work as a map
    debug_assert!(reg_meta.register_map.len() == out.ssa_values.len());

    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = f.registers[v.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(out.ssa_values.len());
    out.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // We ignore if this returns None deliberately as a phi instruction isn't a real write
    let _ = reg_meta.basic_block_registers_written[bb_index].insert(v, value);

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.push(v);

    Some(value)
}

/// Simple function that handles creating and adding SSA values for instructions
fn handle_ssa_write(
    out: &mut Function,
    reg_meta: &mut RegisterMetadata,
    f: &hashlink_bytecode::Function,
    bb_index: usize,
    v: RegisterIndex,
) -> Option<ValueIndex> {
    // Assert that the two lengths match so that the vec will continue to work as a map
    debug_assert!(reg_meta.register_map.len() == out.ssa_values.len());

    // Lookup the type from the source HashLink (we use the same indices)
    let type_ = f.registers[v.0] as usize;

    // Add the new SSA value to the function, yielding an index to it
    let value = ValueIndex(out.ssa_values.len());
    out.ssa_values.push(SSAValue {
        type_: TypeIndex(type_),
    });

    // Update the register's latest value state. This will also bubble an error up in the event that
    // the register was not already marked as being written in this basic block. We already found
    // the set of registers that a basic block writes to in an earlier pass, trying to say we write
    // any more at this stage is an error.
    reg_meta.basic_block_registers_written[bb_index].insert(v, value)?;

    // Add to the register map so we can map the ValueIndex back to the register it represents
    reg_meta.register_map.push(v);

    Some(value)
}
