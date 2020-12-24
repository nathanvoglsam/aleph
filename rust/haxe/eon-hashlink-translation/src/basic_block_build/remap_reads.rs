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
use eon_bytecode::indexes::{RegisterIndex, ValueIndex};
use eon_bytecode::opcode::OpCode;
use std::collections::{HashMap, HashSet};

pub fn remap_reads(
    op: &mut OpCode,
    reg_meta: &RegisterData,
    non_reg_values: &HashSet<ValueIndex>,
    latest_states: &HashMap<RegisterIndex, ValueIndex>,
) {
    match op {
        OpCode::OpMov(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpAdd(v)
        | OpCode::OpSub(v)
        | OpCode::OpMul(v)
        | OpCode::OpSDiv(v)
        | OpCode::OpUDiv(v)
        | OpCode::OpSMod(v)
        | OpCode::OpUMod(v)
        | OpCode::OpShl(v)
        | OpCode::OpSShr(v)
        | OpCode::OpUShr(v)
        | OpCode::OpAnd(v)
        | OpCode::OpOr(v)
        | OpCode::OpXor(v) => {
            v.lhs = handle_value_remap(non_reg_values, latest_states, v.lhs);
            v.rhs = handle_value_remap(non_reg_values, latest_states, v.rhs);
        }
        OpCode::OpNeg(v) | OpCode::OpNot(v) | OpCode::OpIncr(v) | OpCode::OpDecr(v) => {
            v.operand = handle_value_remap(non_reg_values, latest_states, v.operand);
        }
        OpCode::OpCall(v) => {
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpCallMethod(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpCallClosure(v) => {
            v.closure = handle_value_remap(non_reg_values, latest_states, v.closure);
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpInvoke(v) => {
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpInvokeMethod(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpInvokeClosure(v) => {
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpInstanceClosure(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
        }
        OpCode::OpVirtualClosure(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
        }
        OpCode::OpSetGlobal(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpGetField(v) | OpCode::OpDynGet(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
        }
        OpCode::OpSetField(v) | OpCode::OpDynSet(v) => {
            v.object = handle_value_remap(non_reg_values, latest_states, v.object);
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpJTrue(v) | OpCode::OpJFalse(v) | OpCode::OpJNull(v) | OpCode::OpJNotNull(v) => {
            v.check = handle_value_remap(non_reg_values, latest_states, v.check);
        }
        OpCode::OpCmp(v) => {
            v.lhs = handle_value_remap(non_reg_values, latest_states, v.lhs);
            v.rhs = handle_value_remap(non_reg_values, latest_states, v.rhs);
        }
        OpCode::OpRet(v) => {
            *v = handle_value_remap(non_reg_values, latest_states, *v);
        }
        OpCode::OpSwitch(v) => {
            v.input = handle_value_remap(non_reg_values, latest_states, v.input);
        }
        OpCode::OpPhi(v) => {
            for (v, bb) in v.block_values.iter_mut() {
                let latest_in_block = *reg_meta.block_live_set[bb.0]
                    .get(&RegisterIndex(v.0))
                    .unwrap();
                *v = latest_in_block;
            }
        }
        OpCode::OpToDyn(v)
        | OpCode::OpToSFloat(v)
        | OpCode::OpToUFloat(v)
        | OpCode::OpToInt(v)
        | OpCode::OpSafeCast(v)
        | OpCode::OpUnsafeCast(v)
        | OpCode::OpToVirtual(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpThrow(v) | OpCode::OpRethrow(v) | OpCode::OpNullCheck(v) => {
            *v = handle_value_remap(non_reg_values, latest_states, *v);
        }
        OpCode::OpGetI8(v) | OpCode::OpGetI16(v) | OpCode::OpGetMem(v) | OpCode::OpGetArray(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
            v.offset = handle_value_remap(non_reg_values, latest_states, v.offset);
        }
        OpCode::OpSetI8(v) | OpCode::OpSetI16(v) | OpCode::OpSetMem(v) | OpCode::OpSetArray(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
            v.offset = handle_value_remap(non_reg_values, latest_states, v.offset);
            v.target = handle_value_remap(non_reg_values, latest_states, v.target);
        }
        OpCode::OpArraySize(v)
        | OpCode::OpGetType(v)
        | OpCode::OpGetTID(v)
        | OpCode::OpRef(v)
        | OpCode::OpUnRef(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpSetRef(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
            v.target = handle_value_remap(non_reg_values, latest_states, v.target);
        }
        OpCode::OpMakeEnum(v) => {
            for v in v.args.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpEnumIndex(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpEnumField(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpSetEnumField(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
            v.target = handle_value_remap(non_reg_values, latest_states, v.target);
        }
        OpCode::OpRefData(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
        }
        OpCode::OpRefOffset(v) => {
            v.source = handle_value_remap(non_reg_values, latest_states, v.source);
            v.offset = handle_value_remap(non_reg_values, latest_states, v.offset);
        }
        OpCode::OpInt(_)
        | OpCode::OpFloat(_)
        | OpCode::OpBool(_)
        | OpCode::OpBytes(_)
        | OpCode::OpString(_)
        | OpCode::OpNull(_)
        | OpCode::OpStaticClosure(_)
        | OpCode::OpGetGlobal(_)
        | OpCode::OpJAlways(_)
        | OpCode::OpRetVoid
        | OpCode::OpTrap(_)
        | OpCode::OpEndTrap(_)
        | OpCode::OpReceiveException(_)
        | OpCode::OpNew(_)
        | OpCode::OpType(_)
        | OpCode::OpEnumAlloc(_)
        | OpCode::OpAssert
        | OpCode::OpNop => {}
        OpCode::OpThrowInvoke(v) => {
            v.exception = handle_value_remap(non_reg_values, latest_states, v.exception);
        }
        OpCode::OpRethrowInvoke(v) => {
            v.exception = handle_value_remap(non_reg_values, latest_states, v.exception);
        }
        OpCode::OpCallIntrinsic(v) => {
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
        OpCode::OpInvokeIntrinsic(v) => {
            for v in v.fn_params.iter_mut() {
                *v = handle_value_remap(non_reg_values, latest_states, *v);
            }
        }
    }
}

fn handle_value_remap(
    non_reg_values: &HashSet<ValueIndex>,
    latest_states: &HashMap<RegisterIndex, ValueIndex>,
    v: ValueIndex,
) -> ValueIndex {
    if non_reg_values.contains(&v) {
        v
    } else {
        *latest_states.get(&RegisterIndex(v.0)).unwrap()
    }
}
