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

use eon_bytecode::function::RegisterMetadata;
use eon_bytecode::indexes::{RegisterIndex, ValueIndex};
use eon_bytecode::opcode::OpCode;
use std::collections::HashMap;

pub fn remap_reads(
    op: &mut OpCode,
    reg_meta: &RegisterMetadata,
    latest_states: &HashMap<RegisterIndex, ValueIndex>,
) -> Option<()> {
    match op {
        OpCode::OpMov(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
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
            v.lhs = *latest_states.get(&RegisterIndex(v.lhs.0))?;
            v.rhs = *latest_states.get(&RegisterIndex(v.rhs.0))?;
        }
        OpCode::OpNeg(v) | OpCode::OpNot(v) | OpCode::OpIncr(v) | OpCode::OpDecr(v) => {
            v.operand = *latest_states.get(&RegisterIndex(v.operand.0))?;
        }
        OpCode::OpCall(v) => {
            for v in v.fn_params.iter_mut() {
                *v = *latest_states.get(&RegisterIndex(v.0))?;
            }
        }
        OpCode::OpCallMethod(v) => {
            v.object = *latest_states.get(&RegisterIndex(v.object.0))?;
            for v in v.fn_params.iter_mut() {
                *v = *latest_states.get(&RegisterIndex(v.0))?;
            }
        }
        OpCode::OpCallClosure(v) => {
            v.closure = *latest_states.get(&RegisterIndex(v.closure.0))?;
            for v in v.fn_params.iter_mut() {
                *v = *latest_states.get(&RegisterIndex(v.0))?;
            }
        }
        OpCode::OpInstanceClosure(v) => {
            v.object = *latest_states.get(&RegisterIndex(v.object.0))?;
        }
        OpCode::OpVirtualClosure(v) => {
            v.object = *latest_states.get(&RegisterIndex(v.object.0))?;
        }
        OpCode::OpSetGlobal(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpGetField(v) | OpCode::OpDynGet(v) => {
            v.object = *latest_states.get(&RegisterIndex(v.object.0))?;
        }
        OpCode::OpSetField(v) | OpCode::OpDynSet(v) => {
            v.object = *latest_states.get(&RegisterIndex(v.object.0))?;
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpJTrue(v) | OpCode::OpJFalse(v) | OpCode::OpJNull(v) | OpCode::OpJNotNull(v) => {
            v.check = *latest_states.get(&RegisterIndex(v.check.0))?;
        }
        OpCode::OpCmp(v) => {
            v.lhs = *latest_states.get(&RegisterIndex(v.lhs.0))?;
            v.rhs = *latest_states.get(&RegisterIndex(v.rhs.0))?;
        }
        OpCode::OpRet(v) => {
            *v = *latest_states.get(&RegisterIndex(v.0))?;
        }
        OpCode::OpSwitch(v) => {
            v.input = *latest_states.get(&RegisterIndex(v.input.0))?;
        }
        OpCode::OpPhi(v) => {
            for (v, bb) in v.block_values.iter_mut() {
                let latest_in_block =
                    *reg_meta.basic_block_registers_written[bb.0].get(&RegisterIndex(v.0))?;
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
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpThrow(v) | OpCode::OpRethrow(v) | OpCode::OpNullCheck(v) => {
            *v = *latest_states.get(&RegisterIndex(v.0))?;
        }
        OpCode::OpGetI8(v) | OpCode::OpGetI16(v) | OpCode::OpGetMem(v) | OpCode::OpGetArray(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
            v.offset = *latest_states.get(&RegisterIndex(v.offset.0))?;
        }
        OpCode::OpSetI8(v) | OpCode::OpSetI16(v) | OpCode::OpSetMem(v) | OpCode::OpSetArray(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
            v.offset = *latest_states.get(&RegisterIndex(v.offset.0))?;
            v.target = *latest_states.get(&RegisterIndex(v.target.0))?;
        }
        OpCode::OpArraySize(v)
        | OpCode::OpGetType(v)
        | OpCode::OpGetTID(v)
        | OpCode::OpRef(v)
        | OpCode::OpUnRef(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpSetRef(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
            v.target = *latest_states.get(&RegisterIndex(v.target.0))?;
        }
        OpCode::OpMakeEnum(v) => {
            for v in v.args.iter_mut() {
                *v = *latest_states.get(&RegisterIndex(v.0))?;
            }
        }
        OpCode::OpEnumIndex(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpEnumField(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpSetEnumField(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
            v.target = *latest_states.get(&RegisterIndex(v.target.0))?;
        }
        OpCode::OpRefData(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
        }
        OpCode::OpRefOffset(v) => {
            v.source = *latest_states.get(&RegisterIndex(v.source.0))?;
            v.offset = *latest_states.get(&RegisterIndex(v.offset.0))?;
        }
        _ => {}
    }
    Some(())
}
