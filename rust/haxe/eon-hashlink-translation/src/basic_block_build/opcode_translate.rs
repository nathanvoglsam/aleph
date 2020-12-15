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

use crate::basic_block_build::{find_source_span, handle_ssa_write, handle_ssa_write_no_register};
use crate::opcode_translators::*;
use crate::utils::offset_from;
use eon_bytecode::function::{Function, RegisterMetadata};
use eon_bytecode::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, InstructionIndex, IntegerIndex, RegisterIndex, StringIndex, TypeIndex, ValueIndex,
};
use eon_bytecode::opcode::{CondBranch, OpCode};

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
    new_fn: &mut Function,
    reg_meta: &mut RegisterMetadata,
    old_fn: &hashlink_bytecode::Function,
    spans: &[(InstructionIndex, InstructionIndex)],
    bool_type_index: TypeIndex,
    bb_index: usize,
    op_index: usize,
    old_op: &hashlink_bytecode::OpCode,
) {
    match old_op {
        hashlink_bytecode::OpCode::OpMov(params) => {
            // Allocate new SSA value for the assignment param
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Temporarily store the register of the read in the second parameter to be
            // remapped later
            let source = ValueIndex(params.param_2 as usize);
            let new_op = translate_load(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpInt(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Unpack the index into the integer table
            let integer = IntegerIndex(params.param_2 as usize);

            let new_op = translate_load_int(old_op, assigns, integer).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpFloat(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Unpack the index into the float table
            let float = FloatIndex(params.param_2 as usize);

            let new_op = translate_load_float(old_op, assigns, float).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpBool(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // `OpBool` holds the value to assign directly in the opcode, not in a separate
            // table like the other opcodes. We convert this into a native boolean type and
            // then pack the value directly into the instruction too.
            let value = params.param_2 != 0;

            let new_op = translate_load_bool(old_op, assigns, value).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpBytes(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Unpack the index into the bytes table
            let bytes = BytesIndex(params.param_2 as usize);

            let new_op = translate_load_bytes(old_op, assigns, bytes).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpString(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Unpack the index into the string table
            let string = StringIndex(params.param_2 as usize);

            let new_op = translate_load_string(old_op, assigns, string).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpNull(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            let new_op = translate_value_index(old_op, assigns).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Once again, these are *register indexes* for remapping later
            let lhs = ValueIndex(params.param_2 as usize);
            let rhs = ValueIndex(params.param_3 as usize);

            let new_op = translate_binop(old_op, assigns, lhs, rhs).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpNeg(params) | hashlink_bytecode::OpCode::OpNot(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // These are both unops where the HashLink source actually separately encodes
            // the assignment target and operand so we can just translate this directly
            let operand = ValueIndex(params.param_2 as usize);

            let new_op = translate_unop(old_op, assigns, operand).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpIncr(params) | hashlink_bytecode::OpCode::OpDecr(params) => {
            // Assign a new SSA value for the result of the operation
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // A unop in HashLink is similar to a `i++` statement where it both reads and
            // writes to the register it specifies. Such an operation requires a separate
            // source parameter when in SSA form as all values can only be assigned to once.
            //
            // Regardless, this contains the *register index* of the value so we can remap
            // to the actual SSA value later
            let operand = ValueIndex(params.param_1 as usize);

            let new_op = translate_unop(old_op, assigns, operand).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpCall0(_)
        | hashlink_bytecode::OpCode::OpCall1(_)
        | hashlink_bytecode::OpCode::OpCall2(_)
        | hashlink_bytecode::OpCode::OpCall3(_)
        | hashlink_bytecode::OpCode::OpCall4(_)
        | hashlink_bytecode::OpCode::OpCallN(_) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = old_op.register_write().unwrap() as usize;
            let assigns =
                handle_ssa_write(new_fn, old_fn, reg_meta, bb_index, RegisterIndex(assigns));

            // Unpack the function index
            let function = FunctionIndex(old_op.get_param_2().unwrap() as usize);

            // Convert the parameter list. It will produce a list of *register indexes*
            // disguised as value indexes which we need to remap in a later pass over the
            // translated instructions
            let fn_params = old_op.get_static_call_args().unwrap();
            let fn_params = fn_params
                .into_iter()
                .map(|v| ValueIndex(v as usize))
                .collect();

            let new_op = translate_call(old_op, assigns, function, fn_params).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpCallMethod(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

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

            let new_op =
                translate_call_field(old_op, assigns, object, function, fn_params).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpCallThis(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

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

            let new_op =
                translate_call_field(old_op, assigns, object, function, fn_params).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpCallClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // The closure which is read and used as the function to call
            let closure = ValueIndex(params.param_2 as usize);

            // Map the argument list into ValueIndexes that actually hold *register indexes*
            // so we can remap later
            let fn_params = params
                .extra
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            let new_op = translate_call_closure(old_op, assigns, closure, fn_params).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpStaticClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Get the function index to call
            let function = FunctionIndex(params.param_2 as usize);

            let new_op = translate_static_closure(old_op, assigns, function).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpInstanceClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Get the function index to call
            let function = FunctionIndex(params.param_2 as usize);

            // Register index for remapping later
            let object = ValueIndex(params.param_3 as usize);

            let new_op = translate_instance_closure(old_op, assigns, function, object).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpVirtualClosure(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index for remapping later
            let object = ValueIndex(params.param_2 as usize);

            // Field index
            let field = FieldIndex(params.param_3 as usize);

            let new_op = translate_virtual_closure(old_op, assigns, object, field).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpGetGlobal(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Map into global index
            let source = GlobalIndex(params.param_2 as usize);

            let new_op = translate_load_global(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpSetGlobal(params) => {
            // The register index for remapping later. This reads the register into the
            // target global
            let source = ValueIndex(params.param_1 as usize);

            // The global to write into
            let target = GlobalIndex(params.param_2 as usize);

            let new_op = translate_store_global(old_op, source, target).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpGetThis(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let object = ValueIndex(0);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            let new_op = translate_field_load(old_op, assigns, object, field).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpField(params)
        | hashlink_bytecode::OpCode::OpDynGet(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let object = ValueIndex(params.param_2 as usize);

            // Field index
            let field = FieldIndex(params.param_3 as usize);

            let new_op = translate_field_load(old_op, assigns, object, field).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpSetThis(params) => {
            // Register index
            let object = ValueIndex(0);

            // Field index
            let field = FieldIndex(params.param_1 as usize);

            // Register index of source to read into object
            let source = ValueIndex(params.param_2 as usize);

            let new_op = translate_field_store(old_op, object, field, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpSetField(params)
        | hashlink_bytecode::OpCode::OpDynSet(params) => {
            // Register index
            let object = ValueIndex(params.param_1 as usize);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            // Register index of source to read into object
            let source = ValueIndex(params.param_3 as usize);

            let new_op = translate_field_store(old_op, object, field, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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
            let success = offset_from(op_index, params.param_2).unwrap(); // Apply offset
            let success = find_source_span(spans, success).unwrap(); // Find block
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
            let failure = find_source_span(spans, op_index + 1).unwrap();
            let failure = BasicBlockIndex(failure);

            let new_op = translate_cond_branch(old_op, check, success, failure).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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
            // These opcodes get translated to two eon instructions, a OpCmp and OpJTrue/OpJFalse.
            // The OpCmp assigns an SSA value which has no corresponding register in the source code
            // so we need to avoid emitting register information for this value
            let assigns = handle_ssa_write_no_register(new_fn, bool_type_index);

            // Get the values to read for the comparison. Stores a *register index*
            // for remapping later
            let lhs = ValueIndex(params.param_1 as usize);
            let rhs = ValueIndex(params.param_2 as usize);

            // See OpJTrue/OpJFalse/etc for explanation
            let success = offset_from(op_index, params.param_3).unwrap(); // Apply offset
            let success = find_source_span(spans, success).unwrap(); // Find block
            let success = BasicBlockIndex(success); // Wrap value

            let failure = find_source_span(spans, op_index + 1).unwrap();
            let failure = BasicBlockIndex(failure);

            let comparison = translate_comp_branch(old_op, assigns, lhs, rhs).unwrap();

            let inner = CondBranch {
                check: assigns,
                success,
                failure,
            };
            let branch = OpCode::OpJTrue(inner);

            new_fn.basic_blocks[bb_index].ops.push(comparison);
            new_fn.basic_blocks[bb_index].ops.push(branch);
        }

        hashlink_bytecode::OpCode::OpJAlways(params) => {
            // See above for explanation
            let inner = offset_from(op_index, params.param_1).unwrap(); // Apply offset
            let inner = find_source_span(spans, inner).unwrap(); // Find block
            let inner = BasicBlockIndex(inner); // Wrap value

            let new_op = translate_unconditional_branch(old_op, inner).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            let new_op = translate_cast(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpSwitch(params) => {
            // Register index
            let input = ValueIndex(params.param_1 as usize);

            // Translate the jump table, which deals with jump offsets, into the format we
            // need which jumps to basic block indices
            let mut jump_table = Vec::new();
            for destination in &params.extra {
                let destination = offset_from(op_index, *destination).unwrap();
                let destination = find_source_span(spans, destination).unwrap();
                let destination = BasicBlockIndex(destination);
                jump_table.push(destination);
            }

            // Translate and map the fallback branch offset into the basic block it is
            // supposed to jump to
            let fallback = params.param_3;
            let fallback = offset_from(op_index, fallback).unwrap();
            let fallback = find_source_span(spans, fallback).unwrap();
            let fallback = BasicBlockIndex(fallback);

            let new_op = translate_switch(old_op, input, jump_table, fallback).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpTrap(params) => {
            // See above for explanation
            let destination = offset_from(op_index, params.param_1).unwrap(); // Apply offset
            let destination = find_source_span(spans, destination).unwrap(); // Find block
            let destination = BasicBlockIndex(destination); // Wrap value
            let new_op = translate_trap(old_op, destination).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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

            let new_op = translate_end_trap(old_op, inner).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpGetI8(params)
        | hashlink_bytecode::OpCode::OpGetI16(params)
        | hashlink_bytecode::OpCode::OpGetMem(params)
        | hashlink_bytecode::OpCode::OpGetArray(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            // Register index
            let offset = ValueIndex(params.param_3 as usize);

            let new_op = translate_read_memory(old_op, assigns, source, offset).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
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

            let new_op = translate_write_memory(old_op, target, offset, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpType(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Type index
            let source = TypeIndex(params.param_2 as usize);

            let new_op = translate_load_type(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpRet(params)
        | hashlink_bytecode::OpCode::OpThrow(params)
        | hashlink_bytecode::OpCode::OpRethrow(params)
        | hashlink_bytecode::OpCode::OpNullCheck(params) => {
            // Register index
            let value = ValueIndex(params.param_1 as usize);

            let new_op = translate_value_index(old_op, value).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpNew(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            let new_op = translate_value_index(old_op, assigns).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpArraySize(params)
        | hashlink_bytecode::OpCode::OpGetType(params)
        | hashlink_bytecode::OpCode::OpGetTID(params)
        | hashlink_bytecode::OpCode::OpRef(params)
        | hashlink_bytecode::OpCode::OpUnRef(params)
        | hashlink_bytecode::OpCode::OpEnumIndex(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            let new_op = translate_load(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpSetRef(params) => {
            // Register index
            let target = ValueIndex(params.param_1 as usize);

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            let new_op = translate_store(old_op, target, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpMakeEnum(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Constructor index
            let constructor = ConstructorIndex(params.param_2 as usize);

            // Build the list of *register indexes* that we remap to actual value indexes
            // later
            let args = params
                .extra
                .iter()
                .map(|v| ValueIndex(*v as usize))
                .collect();

            let new_op = translate_make_enum(old_op, assigns, constructor, args).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpEnumAlloc(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Constructor index
            let constructor = ConstructorIndex(params.param_2 as usize);

            let new_op = translate_alloc_enum(old_op, assigns, constructor).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpEnumField(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Constructor index
            let source = ValueIndex(params.param_2 as usize);

            // Constructor index
            let constructor = ConstructorIndex(params.param_3 as usize);

            // Constructor index
            let field_index = FieldIndex(params.param_4 as usize);

            let new_op =
                translate_load_enum_field(old_op, assigns, source, constructor, field_index)
                    .unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpSetEnumField(params) => {
            // Register index
            let target = ValueIndex(params.param_1 as usize);

            // Field index
            let field = FieldIndex(params.param_2 as usize);

            // Register index
            let source = ValueIndex(params.param_3 as usize);

            let new_op = translate_store_enum_field(old_op, target, field, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpRefData(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            let new_op = translate_ref_data(old_op, assigns, source).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
        hashlink_bytecode::OpCode::OpRefOffset(params) => {
            // Allocate a new SSA value for the call to assign into
            let assigns = handle_ssa_write(
                new_fn,
                old_fn,
                reg_meta,
                bb_index,
                RegisterIndex(params.param_1 as usize),
            );

            // Register index
            let source = ValueIndex(params.param_2 as usize);

            // Register index
            let offset = ValueIndex(params.param_3 as usize);

            let new_op = translate_ref_offset(old_op, assigns, source, offset).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }

        hashlink_bytecode::OpCode::OpAssert
        | hashlink_bytecode::OpCode::OpNop
        | hashlink_bytecode::OpCode::OpLabel => {
            let new_op = translate_no_params(old_op).unwrap();
            new_fn.basic_blocks[bb_index].ops.push(new_op);
        }
    };
}
