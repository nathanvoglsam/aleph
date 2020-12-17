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

use serde::{Deserialize, Serialize};

mod opcode_number;
mod opcode_params;
mod opcode_type;

pub use opcode_number::OpCodeNumber;
pub use opcode_params::{
    OpCallNParam, OpFiveParam, OpFourParam, OpOneParam, OpSixParam, OpSwitchParam, OpThreeParam,
    OpTwoParam,
};
pub use opcode_type::OpCodeType;

#[derive(Clone, Serialize, Debug, Deserialize)]
pub enum OpCode {
    OpMov(OpTwoParam),
    OpInt(OpTwoParam),
    OpFloat(OpTwoParam),
    OpBool(OpTwoParam),
    OpBytes(OpTwoParam),
    OpString(OpTwoParam),
    OpNull(OpOneParam),
    OpAdd(OpThreeParam),
    OpSub(OpThreeParam),
    OpMul(OpThreeParam),
    OpSDiv(OpThreeParam),
    OpUDiv(OpThreeParam),
    OpSMod(OpThreeParam),
    OpUMod(OpThreeParam),
    OpShl(OpThreeParam),
    OpSShr(OpThreeParam),
    OpUShr(OpThreeParam),
    OpAnd(OpThreeParam),
    OpOr(OpThreeParam),
    OpXor(OpThreeParam),
    OpNeg(OpTwoParam),
    OpNot(OpTwoParam),
    OpIncr(OpOneParam),
    OpDecr(OpOneParam),
    OpCall0(OpTwoParam),
    OpCall1(OpThreeParam),
    OpCall2(OpFourParam),
    OpCall3(OpFiveParam),
    OpCall4(OpSixParam),
    OpCallN(OpCallNParam),
    OpCallMethod(OpCallNParam),
    OpCallThis(OpCallNParam),
    OpCallClosure(OpCallNParam),
    OpStaticClosure(OpTwoParam),
    OpInstanceClosure(OpThreeParam),
    OpVirtualClosure(OpThreeParam),
    OpGetGlobal(OpTwoParam),
    OpSetGlobal(OpTwoParam),
    OpField(OpThreeParam),
    OpSetField(OpThreeParam),
    OpGetThis(OpTwoParam),
    OpSetThis(OpTwoParam),
    OpDynGet(OpThreeParam),
    OpDynSet(OpThreeParam),
    OpJTrue(OpTwoParam),
    OpJFalse(OpTwoParam),
    OpJNull(OpTwoParam),
    OpJNotNull(OpTwoParam),
    OpJSLt(OpThreeParam),
    OpJSGte(OpThreeParam),
    OpJSGt(OpThreeParam),
    OpJSLte(OpThreeParam),
    OpJULt(OpThreeParam),
    OpJUGte(OpThreeParam),
    OpJNotLt(OpThreeParam),
    OpJNotGte(OpThreeParam),
    OpJEq(OpThreeParam),
    OpJNotEq(OpThreeParam),
    OpJAlways(OpOneParam),
    OpToDyn(OpTwoParam),
    OpToSFloat(OpTwoParam),
    OpToUFloat(OpTwoParam),
    OpToInt(OpTwoParam),
    OpSafeCast(OpTwoParam),
    OpUnsafeCast(OpTwoParam),
    OpToVirtual(OpTwoParam),
    OpLabel,
    OpRet(OpOneParam),
    OpThrow(OpOneParam),
    OpRethrow(OpOneParam),
    OpSwitch(OpSwitchParam),
    OpNullCheck(OpOneParam),
    OpTrap(OpTwoParam),
    OpEndTrap(OpOneParam),
    OpGetI8(OpThreeParam),
    OpGetI16(OpThreeParam),
    OpGetMem(OpThreeParam),
    OpGetArray(OpThreeParam),
    OpSetI8(OpThreeParam),
    OpSetI16(OpThreeParam),
    OpSetMem(OpThreeParam),
    OpSetArray(OpThreeParam),
    OpNew(OpOneParam),
    OpArraySize(OpTwoParam),
    OpType(OpTwoParam),
    OpGetType(OpTwoParam),
    OpGetTID(OpTwoParam),
    OpRef(OpTwoParam),
    OpUnRef(OpTwoParam),
    OpSetRef(OpTwoParam),
    OpMakeEnum(OpCallNParam),
    OpEnumAlloc(OpTwoParam),
    OpEnumIndex(OpTwoParam),
    OpEnumField(OpFourParam),
    OpSetEnumField(OpThreeParam),
    OpAssert,
    OpRefData(OpTwoParam),
    OpRefOffset(OpThreeParam),
    OpNop,
}

impl OpCode {
    pub fn from_no_param(num: OpCodeNumber) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpLabel => Some(OpCode::OpLabel),
            OpCodeNumber::OpAssert => Some(OpCode::OpAssert),
            OpCodeNumber::OpNop => Some(OpCode::OpNop),
            _ => None,
        }
    }

    pub fn from_one_param(num: OpCodeNumber, params: OpOneParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpNull => Some(OpCode::OpNull(params)),
            OpCodeNumber::OpIncr => Some(OpCode::OpIncr(params)),
            OpCodeNumber::OpDecr => Some(OpCode::OpDecr(params)),
            OpCodeNumber::OpJAlways => Some(OpCode::OpJAlways(params)),
            OpCodeNumber::OpRet => Some(OpCode::OpRet(params)),
            OpCodeNumber::OpThrow => Some(OpCode::OpThrow(params)),
            OpCodeNumber::OpRethrow => Some(OpCode::OpRethrow(params)),
            OpCodeNumber::OpNullCheck => Some(OpCode::OpNullCheck(params)),
            OpCodeNumber::OpEndTrap => Some(OpCode::OpEndTrap(params)),
            OpCodeNumber::OpNew => Some(OpCode::OpNew(params)),
            _ => None,
        }
    }

    pub fn from_two_param(num: OpCodeNumber, params: OpTwoParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpMov => Some(OpCode::OpMov(params)),
            OpCodeNumber::OpInt => Some(OpCode::OpInt(params)),
            OpCodeNumber::OpFloat => Some(OpCode::OpFloat(params)),
            OpCodeNumber::OpBool => Some(OpCode::OpBool(params)),
            OpCodeNumber::OpBytes => Some(OpCode::OpBytes(params)),
            OpCodeNumber::OpString => Some(OpCode::OpString(params)),
            OpCodeNumber::OpNeg => Some(OpCode::OpNeg(params)),
            OpCodeNumber::OpNot => Some(OpCode::OpNot(params)),
            OpCodeNumber::OpCall0 => Some(OpCode::OpCall0(params)),
            OpCodeNumber::OpStaticClosure => Some(OpCode::OpStaticClosure(params)),
            OpCodeNumber::OpGetGlobal => Some(OpCode::OpGetGlobal(params)),
            OpCodeNumber::OpSetGlobal => Some(OpCode::OpSetGlobal(params)),
            OpCodeNumber::OpGetThis => Some(OpCode::OpGetThis(params)),
            OpCodeNumber::OpSetThis => Some(OpCode::OpSetThis(params)),
            OpCodeNumber::OpJTrue => Some(OpCode::OpJTrue(params)),
            OpCodeNumber::OpJFalse => Some(OpCode::OpJFalse(params)),
            OpCodeNumber::OpJNull => Some(OpCode::OpJNull(params)),
            OpCodeNumber::OpJNotNull => Some(OpCode::OpJNotNull(params)),
            OpCodeNumber::OpToDyn => Some(OpCode::OpToDyn(params)),
            OpCodeNumber::OpToSFloat => Some(OpCode::OpToSFloat(params)),
            OpCodeNumber::OpToUFloat => Some(OpCode::OpToUFloat(params)),
            OpCodeNumber::OpToInt => Some(OpCode::OpToInt(params)),
            OpCodeNumber::OpSafeCast => Some(OpCode::OpSafeCast(params)),
            OpCodeNumber::OpUnsafeCast => Some(OpCode::OpUnsafeCast(params)),
            OpCodeNumber::OpToVirtual => Some(OpCode::OpToVirtual(params)),
            OpCodeNumber::OpTrap => Some(OpCode::OpTrap(params)),
            OpCodeNumber::OpArraySize => Some(OpCode::OpArraySize(params)),
            OpCodeNumber::OpType => Some(OpCode::OpType(params)),
            OpCodeNumber::OpGetType => Some(OpCode::OpGetType(params)),
            OpCodeNumber::OpGetTID => Some(OpCode::OpGetTID(params)),
            OpCodeNumber::OpRef => Some(OpCode::OpRef(params)),
            OpCodeNumber::OpUnRef => Some(OpCode::OpUnRef(params)),
            OpCodeNumber::OpSetRef => Some(OpCode::OpSetRef(params)),
            OpCodeNumber::OpEnumAlloc => Some(OpCode::OpEnumAlloc(params)),
            OpCodeNumber::OpEnumIndex => Some(OpCode::OpEnumIndex(params)),
            OpCodeNumber::OpRefData => Some(OpCode::OpRefData(params)),
            _ => None,
        }
    }

    pub fn from_three_param(num: OpCodeNumber, params: OpThreeParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpAdd => Some(OpCode::OpAdd(params)),
            OpCodeNumber::OpSub => Some(OpCode::OpSub(params)),
            OpCodeNumber::OpMul => Some(OpCode::OpMul(params)),
            OpCodeNumber::OpSDiv => Some(OpCode::OpSDiv(params)),
            OpCodeNumber::OpUDiv => Some(OpCode::OpUDiv(params)),
            OpCodeNumber::OpSMod => Some(OpCode::OpSMod(params)),
            OpCodeNumber::OpUMod => Some(OpCode::OpUMod(params)),
            OpCodeNumber::OpShl => Some(OpCode::OpShl(params)),
            OpCodeNumber::OpSShr => Some(OpCode::OpSShr(params)),
            OpCodeNumber::OpUShr => Some(OpCode::OpUShr(params)),
            OpCodeNumber::OpAnd => Some(OpCode::OpAnd(params)),
            OpCodeNumber::OpOr => Some(OpCode::OpOr(params)),
            OpCodeNumber::OpXor => Some(OpCode::OpXor(params)),
            OpCodeNumber::OpCall1 => Some(OpCode::OpCall1(params)),
            OpCodeNumber::OpInstanceClosure => Some(OpCode::OpInstanceClosure(params)),
            OpCodeNumber::OpVirtualClosure => Some(OpCode::OpVirtualClosure(params)),
            OpCodeNumber::OpField => Some(OpCode::OpField(params)),
            OpCodeNumber::OpSetField => Some(OpCode::OpSetField(params)),
            OpCodeNumber::OpDynGet => Some(OpCode::OpDynGet(params)),
            OpCodeNumber::OpDynSet => Some(OpCode::OpDynSet(params)),
            OpCodeNumber::OpJSLt => Some(OpCode::OpJSLt(params)),
            OpCodeNumber::OpJSGte => Some(OpCode::OpJSGte(params)),
            OpCodeNumber::OpJSGt => Some(OpCode::OpJSGt(params)),
            OpCodeNumber::OpJSLte => Some(OpCode::OpJSLte(params)),
            OpCodeNumber::OpJULt => Some(OpCode::OpJULt(params)),
            OpCodeNumber::OpJUGte => Some(OpCode::OpJUGte(params)),
            OpCodeNumber::OpJNotLt => Some(OpCode::OpJNotLt(params)),
            OpCodeNumber::OpJNotGte => Some(OpCode::OpJNotGte(params)),
            OpCodeNumber::OpJEq => Some(OpCode::OpJEq(params)),
            OpCodeNumber::OpJNotEq => Some(OpCode::OpJNotEq(params)),
            OpCodeNumber::OpGetI8 => Some(OpCode::OpGetI8(params)),
            OpCodeNumber::OpGetI16 => Some(OpCode::OpGetI16(params)),
            OpCodeNumber::OpGetMem => Some(OpCode::OpGetMem(params)),
            OpCodeNumber::OpGetArray => Some(OpCode::OpGetArray(params)),
            OpCodeNumber::OpSetI8 => Some(OpCode::OpSetI8(params)),
            OpCodeNumber::OpSetI16 => Some(OpCode::OpSetI16(params)),
            OpCodeNumber::OpSetMem => Some(OpCode::OpSetMem(params)),
            OpCodeNumber::OpSetArray => Some(OpCode::OpSetArray(params)),
            OpCodeNumber::OpSetEnumField => Some(OpCode::OpSetEnumField(params)),
            OpCodeNumber::OpRefOffset => Some(OpCode::OpRefOffset(params)),
            _ => None,
        }
    }

    pub fn from_four_param(num: OpCodeNumber, params: OpFourParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpEnumField => Some(OpCode::OpEnumField(params)),
            OpCodeNumber::OpCall2 => Some(OpCode::OpCall2(params)),
            _ => None,
        }
    }

    pub fn from_five_param(num: OpCodeNumber, params: OpFiveParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpCall3 => Some(OpCode::OpCall3(params)),
            _ => None,
        }
    }

    pub fn from_six_param(num: OpCodeNumber, params: OpSixParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpCall4 => Some(OpCode::OpCall4(params)),
            _ => None,
        }
    }

    pub fn from_call_n_param(num: OpCodeNumber, params: OpCallNParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpCallN => Some(OpCode::OpCallN(params)),
            OpCodeNumber::OpCallMethod => Some(OpCode::OpCallMethod(params)),
            OpCodeNumber::OpCallThis => Some(OpCode::OpCallThis(params)),
            OpCodeNumber::OpCallClosure => Some(OpCode::OpCallClosure(params)),
            OpCodeNumber::OpMakeEnum => Some(OpCode::OpMakeEnum(params)),
            _ => None,
        }
    }

    pub fn from_switch_param(num: OpCodeNumber, params: OpSwitchParam) -> Option<OpCode> {
        match num {
            OpCodeNumber::OpSwitch => Some(OpCode::OpSwitch(params)),
            _ => None,
        }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            OpCode::OpJTrue(_)
            | OpCode::OpJFalse(_)
            | OpCode::OpJNull(_)
            | OpCode::OpJNotNull(_)
            | OpCode::OpJSLt(_)
            | OpCode::OpJSGte(_)
            | OpCode::OpJSGt(_)
            | OpCode::OpJSLte(_)
            | OpCode::OpJULt(_)
            | OpCode::OpJUGte(_)
            | OpCode::OpJNotLt(_)
            | OpCode::OpJNotGte(_)
            | OpCode::OpJEq(_)
            | OpCode::OpJNotEq(_)
            | OpCode::OpJAlways(_)
            | OpCode::OpSwitch(_) => true,
            _ => false,
        }
    }

    pub fn is_ret(&self) -> bool {
        match self {
            OpCode::OpRet(_) => true,
            _ => false,
        }
    }

    pub fn is_call(&self) -> bool {
        match self {
            OpCode::OpCall0(_)
            | OpCode::OpCall1(_)
            | OpCode::OpCall2(_)
            | OpCode::OpCall3(_)
            | OpCode::OpCall4(_)
            | OpCode::OpCallN(_)
            | OpCode::OpCallMethod(_)
            | OpCode::OpCallThis(_)
            | OpCode::OpCallClosure(_) => true,
            _ => false,
        }
    }

    pub fn is_throw(&self) -> bool {
        match self {
            OpCode::OpThrow(_) => true,
            OpCode::OpRethrow(_) => true,
            _ => false,
        }
    }

    pub fn is_trap(&self) -> bool {
        match self {
            OpCode::OpTrap(_) => true,
            _ => false,
        }
    }

    pub fn is_end_trap(&self) -> bool {
        match self {
            OpCode::OpEndTrap(_) => true,
            _ => false,
        }
    }

    /// Returns the index for which register the instruction performs a write to, if it makes a
    /// write
    pub fn register_write(&self) -> Option<i32> {
        match self {
            OpCode::OpNull(v) | OpCode::OpIncr(v) | OpCode::OpDecr(v) | OpCode::OpNew(v) => {
                Some(v.param_1)
            }

            OpCode::OpMov(v)
            | OpCode::OpInt(v)
            | OpCode::OpFloat(v)
            | OpCode::OpBool(v)
            | OpCode::OpBytes(v)
            | OpCode::OpCall0(v)
            | OpCode::OpString(v)
            | OpCode::OpNeg(v)
            | OpCode::OpNot(v)
            | OpCode::OpStaticClosure(v)
            | OpCode::OpGetGlobal(v)
            | OpCode::OpSetGlobal(v)
            | OpCode::OpToDyn(v)
            | OpCode::OpToSFloat(v)
            | OpCode::OpToUFloat(v)
            | OpCode::OpToInt(v)
            | OpCode::OpSafeCast(v)
            | OpCode::OpUnsafeCast(v)
            | OpCode::OpToVirtual(v)
            | OpCode::OpGetThis(v)
            | OpCode::OpArraySize(v)
            | OpCode::OpType(v)
            | OpCode::OpGetType(v)
            | OpCode::OpGetTID(v)
            | OpCode::OpRef(v)
            | OpCode::OpUnRef(v)
            | OpCode::OpEnumAlloc(v)
            | OpCode::OpEnumIndex(v)
            | OpCode::OpRefData(v)
            | OpCode::OpTrap(v) => Some(v.param_1),

            // Arithmetic opcodes
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
            | OpCode::OpXor(v)
            | OpCode::OpCall1(v)
            | OpCode::OpInstanceClosure(v)
            | OpCode::OpVirtualClosure(v)
            | OpCode::OpGetI8(v)
            | OpCode::OpGetI16(v)
            | OpCode::OpGetMem(v)
            | OpCode::OpGetArray(v)
            | OpCode::OpField(v)
            | OpCode::OpDynGet(v)
            | OpCode::OpSetEnumField(v)
            | OpCode::OpRefOffset(v) => Some(v.param_1),

            OpCode::OpCall2(v) | OpCode::OpEnumField(v) => Some(v.param_1),

            OpCode::OpCall3(v) => Some(v.param_1),
            OpCode::OpCall4(v) => Some(v.param_1),

            OpCode::OpCallN(v)
            | OpCode::OpCallMethod(v)
            | OpCode::OpCallThis(v)
            | OpCode::OpCallClosure(v)
            | OpCode::OpMakeEnum(v) => Some(v.param_1),

            _ => None,
        }
    }

    /// This returns the list of registers that are read by this instruction.
    ///
    /// # Information
    ///
    /// Some of the information provided may be a little unintuitive. For many of the object access
    /// opcodes the `obj` parameter is marked as a read, even if the operation is to write to the
    /// object. For example: `OpSetField`.
    ///
    /// `OpSetField` takes 3 arguments, the register that holds the object, the index of the field
    /// to perform the write to and the register to use as the source of the write. The access to
    /// `obj` in this opcode is considered a *read*, not a *write*.
    ///
    /// This is where i'm making stuff up as I go because there's about three fifths of fuck all
    /// documentation for HashLink's actual bytecode semantics. If I understand Haxe and the general
    /// semantics of an Java/C# like GC'd OO language correctly, then I would expect that a register
    /// that holds a non primitive type (i.e holds an object) would have the semantics of a
    /// *pointer* to the object somewhere in memory, rather than the object register being a value
    /// type in of itself.
    ///
    /// What this means is that the `OpSetField` encodes a *read* of the pointer so it can be
    /// de-referenced to the memory the object is stored in and the actual write can be performed.
    ///
    /// This code here was designed to be used for transpiling HashLink bytecode to LLVM-IR which
    /// encodes the instructions as SSA form. Memory ops escape the SSA form (as they should) and so
    /// a write to memory does not imply a write to the pointer.
    ///
    /// The `register_reads` and `register_writes` functions are intended to be used for building an
    /// SSA graph from the flat register slot system HashLink uses and so these functions are
    /// tailored for that purpose so some of the return values may seem a little odd compared to
    /// what would normally be expected
    pub fn register_reads(&self) -> Option<Vec<i32>> {
        match self {
            // Init opcodes
            OpCode::OpMov(v) => Some(vec![v.param_2]),

            // Arithmetic opcodes
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
            | OpCode::OpXor(v) => Some(vec![v.param_2, v.param_3]),

            OpCode::OpNeg(v) | OpCode::OpNot(v) => Some(vec![v.param_2]),

            OpCode::OpIncr(v) | OpCode::OpDecr(v) => Some(vec![v.param_1]),

            // Call opcodes
            OpCode::OpCall1(v) => Some(vec![v.param_3]),
            OpCode::OpCall2(v) => Some(vec![v.param_3, v.param_4]),
            OpCode::OpCall3(v) => Some(vec![v.param_3, v.param_4, v.param_5]),
            OpCode::OpCall4(v) => Some(vec![v.param_3, v.param_4, v.param_5, v.param_6]),

            // Call N opcodes
            OpCode::OpCallN(v) => Some(v.extra.clone()),
            OpCode::OpCallMethod(v) | OpCode::OpCallClosure(v) => {
                let mut list = Vec::new();
                list.push(v.param_2);
                list.extend_from_slice(&v.extra);
                Some(list)
            }
            OpCode::OpCallThis(v) => {
                let mut list = Vec::new();
                list.push(0);
                list.extend_from_slice(&v.extra);
                Some(list)
            }

            // Closure creation
            OpCode::OpInstanceClosure(v) => Some(vec![v.param_3]),
            OpCode::OpVirtualClosure(v) => Some(vec![v.param_2]),

            OpCode::OpSetGlobal(v) => Some(vec![v.param_2]),

            OpCode::OpField(v) => Some(vec![v.param_2]),
            OpCode::OpSetField(v) => Some(vec![v.param_1, v.param_3]),
            OpCode::OpGetThis(_) => Some(vec![0]),
            OpCode::OpSetThis(v) => Some(vec![0, v.param_2]),
            OpCode::OpDynGet(v) => Some(vec![v.param_2]),
            OpCode::OpDynSet(v) => Some(vec![v.param_1, v.param_3]),
            OpCode::OpNullCheck(v) => Some(vec![v.param_1]),

            // Jump on implicit condition
            OpCode::OpJTrue(v)
            | OpCode::OpJFalse(v)
            | OpCode::OpJNull(v)
            | OpCode::OpJNotNull(v) => Some(vec![v.param_1]),

            // Comparison based jumps
            OpCode::OpJSLt(v)
            | OpCode::OpJSGte(v)
            | OpCode::OpJSGt(v)
            | OpCode::OpJSLte(v)
            | OpCode::OpJULt(v)
            | OpCode::OpJUGte(v)
            | OpCode::OpJNotLt(v)
            | OpCode::OpJNotGte(v)
            | OpCode::OpJEq(v)
            | OpCode::OpJNotEq(v) => Some(vec![v.param_1, v.param_2]),

            // Conversion
            OpCode::OpToDyn(v)
            | OpCode::OpToSFloat(v)
            | OpCode::OpToUFloat(v)
            | OpCode::OpToInt(v) => Some(vec![v.param_2]),

            OpCode::OpSafeCast(v) | OpCode::OpUnsafeCast(v) | OpCode::OpToVirtual(v) => {
                Some(vec![v.param_2])
            }

            OpCode::OpRet(v) => Some(vec![v.param_1]),
            OpCode::OpSwitch(v) => Some(vec![v.param_1 as i32]),

            OpCode::OpThrow(v) | OpCode::OpRethrow(v) => Some(vec![v.param_1]),

            OpCode::OpTrap(v) => Some(vec![v.param_1]),

            OpCode::OpGetI8(v) | OpCode::OpGetI16(v) | OpCode::OpGetMem(v) => {
                Some(vec![v.param_2, v.param_3])
            }

            OpCode::OpSetI8(v) | OpCode::OpSetI16(v) | OpCode::OpSetMem(v) => {
                Some(vec![v.param_1, v.param_2, v.param_3])
            }

            OpCode::OpArraySize(v) => Some(vec![v.param_2]),
            OpCode::OpGetArray(v) => Some(vec![v.param_2, v.param_3]),
            OpCode::OpSetArray(v) => Some(vec![v.param_1, v.param_2, v.param_3]),

            OpCode::OpGetType(v) | OpCode::OpGetTID(v) => Some(vec![v.param_2]),

            OpCode::OpRef(v) | OpCode::OpUnRef(v) | OpCode::OpSetRef(v) => Some(vec![v.param_2]),

            OpCode::OpMakeEnum(v) => Some(v.extra.clone()),
            OpCode::OpEnumIndex(v) => Some(vec![v.param_2]),
            OpCode::OpEnumField(v) => Some(vec![v.param_2]),
            OpCode::OpSetEnumField(v) => Some(vec![v.param_3]),

            OpCode::OpRefData(v) => Some(vec![v.param_2]),
            OpCode::OpRefOffset(v) => Some(vec![v.param_2, v.param_3]),

            _ => None,
        }
    }

    /// Returns the number of parameters taken by this instruction
    pub fn opcode_param_count(&self) -> Option<usize> {
        self.opcode_number().opcode_type().arg_num()
    }

    /// Returns a vector of the arguments for static calls. This is useful for being able to handle
    /// all the static call variants (OpCall0, OpCall1, etc) without having to match them all
    /// separately.
    ///
    /// Returns none for all opcodes other than `OpCall0`, `OpCall1`, `OpCall2`, `OpCall3`,
    /// `OpCall4` and `OpCall5`.
    ///
    /// For `OpCall0` will return an empty array as this variant implies calling a function that
    /// takes no arguments
    pub fn get_static_call_args(&self) -> Option<Vec<i32>> {
        match self {
            OpCode::OpCall0(_) => Some(Vec::new()),
            OpCode::OpCall1(v) => Some(vec![v.param_3]),
            OpCode::OpCall2(v) => Some(vec![v.param_3, v.param_4]),
            OpCode::OpCall3(v) => Some(vec![v.param_3, v.param_4, v.param_5]),
            OpCode::OpCall4(v) => Some(vec![v.param_3, v.param_4, v.param_5, v.param_6]),
            OpCode::OpCallN(v) => Some(v.extra.clone()),
            _ => None,
        }
    }

    /// Returns param_2 for this variant. Useful for getting individual params when matching over a
    /// group of variants
    pub fn get_param_2(&self) -> Option<i32> {
        match self {
            OpCode::OpMov(v)
            | OpCode::OpInt(v)
            | OpCode::OpFloat(v)
            | OpCode::OpBool(v)
            | OpCode::OpBytes(v)
            | OpCode::OpCall0(v)
            | OpCode::OpString(v)
            | OpCode::OpNeg(v)
            | OpCode::OpNot(v)
            | OpCode::OpStaticClosure(v)
            | OpCode::OpGetGlobal(v)
            | OpCode::OpSetGlobal(v)
            | OpCode::OpToDyn(v)
            | OpCode::OpToSFloat(v)
            | OpCode::OpToUFloat(v)
            | OpCode::OpToInt(v)
            | OpCode::OpSafeCast(v)
            | OpCode::OpUnsafeCast(v)
            | OpCode::OpToVirtual(v)
            | OpCode::OpGetThis(v)
            | OpCode::OpArraySize(v)
            | OpCode::OpType(v)
            | OpCode::OpGetType(v)
            | OpCode::OpGetTID(v)
            | OpCode::OpRef(v)
            | OpCode::OpUnRef(v)
            | OpCode::OpEnumAlloc(v)
            | OpCode::OpEnumIndex(v)
            | OpCode::OpRefData(v)
            | OpCode::OpTrap(v)
            | OpCode::OpSetThis(v)
            | OpCode::OpJTrue(v)
            | OpCode::OpJFalse(v)
            | OpCode::OpJNull(v)
            | OpCode::OpJNotNull(v)
            | OpCode::OpSetRef(v) => Some(v.param_2),

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
            | OpCode::OpXor(v)
            | OpCode::OpCall1(v)
            | OpCode::OpInstanceClosure(v)
            | OpCode::OpVirtualClosure(v)
            | OpCode::OpGetI8(v)
            | OpCode::OpGetI16(v)
            | OpCode::OpGetMem(v)
            | OpCode::OpGetArray(v)
            | OpCode::OpField(v)
            | OpCode::OpDynGet(v)
            | OpCode::OpSetEnumField(v)
            | OpCode::OpRefOffset(v)
            | OpCode::OpSetField(v)
            | OpCode::OpDynSet(v)
            | OpCode::OpJSLt(v)
            | OpCode::OpJSGte(v)
            | OpCode::OpJSGt(v)
            | OpCode::OpJSLte(v)
            | OpCode::OpJULt(v)
            | OpCode::OpJUGte(v)
            | OpCode::OpJNotLt(v)
            | OpCode::OpJNotGte(v)
            | OpCode::OpJEq(v)
            | OpCode::OpJNotEq(v)
            | OpCode::OpSetI8(v)
            | OpCode::OpSetI16(v)
            | OpCode::OpSetMem(v)
            | OpCode::OpSetArray(v) => Some(v.param_2),

            OpCode::OpCall2(v) | OpCode::OpEnumField(v) => Some(v.param_2),

            OpCode::OpCall3(v) => Some(v.param_2),
            OpCode::OpCall4(v) => Some(v.param_2),

            OpCode::OpCallN(v)
            | OpCode::OpCallMethod(v)
            | OpCode::OpCallThis(v)
            | OpCode::OpCallClosure(v)
            | OpCode::OpMakeEnum(v) => Some(v.param_2),

            OpCode::OpSwitch(v) => Some(v.param_2),

            _ => None,
        }
    }

    pub fn opcode_type(&self) -> OpCodeType {
        self.opcode_number().opcode_type()
    }

    pub fn opcode_number(&self) -> OpCodeNumber {
        match self {
            OpCode::OpMov(_) => OpCodeNumber::OpMov,
            OpCode::OpInt(_) => OpCodeNumber::OpInt,
            OpCode::OpFloat(_) => OpCodeNumber::OpFloat,
            OpCode::OpBool(_) => OpCodeNumber::OpBool,
            OpCode::OpBytes(_) => OpCodeNumber::OpBytes,
            OpCode::OpString(_) => OpCodeNumber::OpString,
            OpCode::OpNull(_) => OpCodeNumber::OpNull,
            OpCode::OpAdd(_) => OpCodeNumber::OpAdd,
            OpCode::OpSub(_) => OpCodeNumber::OpSub,
            OpCode::OpMul(_) => OpCodeNumber::OpMul,
            OpCode::OpSDiv(_) => OpCodeNumber::OpSDiv,
            OpCode::OpUDiv(_) => OpCodeNumber::OpUDiv,
            OpCode::OpSMod(_) => OpCodeNumber::OpSMod,
            OpCode::OpUMod(_) => OpCodeNumber::OpUMod,
            OpCode::OpShl(_) => OpCodeNumber::OpShl,
            OpCode::OpSShr(_) => OpCodeNumber::OpSShr,
            OpCode::OpUShr(_) => OpCodeNumber::OpUShr,
            OpCode::OpAnd(_) => OpCodeNumber::OpAnd,
            OpCode::OpOr(_) => OpCodeNumber::OpOr,
            OpCode::OpXor(_) => OpCodeNumber::OpXor,
            OpCode::OpNeg(_) => OpCodeNumber::OpNeg,
            OpCode::OpNot(_) => OpCodeNumber::OpNot,
            OpCode::OpIncr(_) => OpCodeNumber::OpIncr,
            OpCode::OpDecr(_) => OpCodeNumber::OpDecr,
            OpCode::OpCall0(_) => OpCodeNumber::OpCall0,
            OpCode::OpCall1(_) => OpCodeNumber::OpCall1,
            OpCode::OpCall2(_) => OpCodeNumber::OpCall2,
            OpCode::OpCall3(_) => OpCodeNumber::OpCall3,
            OpCode::OpCall4(_) => OpCodeNumber::OpCall4,
            OpCode::OpCallN(_) => OpCodeNumber::OpCallN,
            OpCode::OpCallMethod(_) => OpCodeNumber::OpCallMethod,
            OpCode::OpCallThis(_) => OpCodeNumber::OpCallThis,
            OpCode::OpCallClosure(_) => OpCodeNumber::OpCallClosure,
            OpCode::OpStaticClosure(_) => OpCodeNumber::OpStaticClosure,
            OpCode::OpInstanceClosure(_) => OpCodeNumber::OpInstanceClosure,
            OpCode::OpVirtualClosure(_) => OpCodeNumber::OpVirtualClosure,
            OpCode::OpGetGlobal(_) => OpCodeNumber::OpGetGlobal,
            OpCode::OpSetGlobal(_) => OpCodeNumber::OpSetGlobal,
            OpCode::OpField(_) => OpCodeNumber::OpField,
            OpCode::OpSetField(_) => OpCodeNumber::OpSetField,
            OpCode::OpGetThis(_) => OpCodeNumber::OpGetThis,
            OpCode::OpSetThis(_) => OpCodeNumber::OpSetThis,
            OpCode::OpDynGet(_) => OpCodeNumber::OpDynGet,
            OpCode::OpDynSet(_) => OpCodeNumber::OpDynSet,
            OpCode::OpJTrue(_) => OpCodeNumber::OpJTrue,
            OpCode::OpJFalse(_) => OpCodeNumber::OpJFalse,
            OpCode::OpJNull(_) => OpCodeNumber::OpJNull,
            OpCode::OpJNotNull(_) => OpCodeNumber::OpJNotNull,
            OpCode::OpJSLt(_) => OpCodeNumber::OpJSLt,
            OpCode::OpJSGte(_) => OpCodeNumber::OpJSGte,
            OpCode::OpJSGt(_) => OpCodeNumber::OpJSGt,
            OpCode::OpJSLte(_) => OpCodeNumber::OpJSLte,
            OpCode::OpJULt(_) => OpCodeNumber::OpJULt,
            OpCode::OpJUGte(_) => OpCodeNumber::OpJUGte,
            OpCode::OpJNotLt(_) => OpCodeNumber::OpJNotLt,
            OpCode::OpJNotGte(_) => OpCodeNumber::OpJNotGte,
            OpCode::OpJEq(_) => OpCodeNumber::OpJEq,
            OpCode::OpJNotEq(_) => OpCodeNumber::OpJNotEq,
            OpCode::OpJAlways(_) => OpCodeNumber::OpJAlways,
            OpCode::OpToDyn(_) => OpCodeNumber::OpToDyn,
            OpCode::OpToSFloat(_) => OpCodeNumber::OpToSFloat,
            OpCode::OpToUFloat(_) => OpCodeNumber::OpToUFloat,
            OpCode::OpToInt(_) => OpCodeNumber::OpToInt,
            OpCode::OpSafeCast(_) => OpCodeNumber::OpSafeCast,
            OpCode::OpUnsafeCast(_) => OpCodeNumber::OpUnsafeCast,
            OpCode::OpToVirtual(_) => OpCodeNumber::OpToVirtual,
            OpCode::OpLabel => OpCodeNumber::OpLabel,
            OpCode::OpRet(_) => OpCodeNumber::OpRet,
            OpCode::OpThrow(_) => OpCodeNumber::OpThrow,
            OpCode::OpRethrow(_) => OpCodeNumber::OpRethrow,
            OpCode::OpSwitch(_) => OpCodeNumber::OpSwitch,
            OpCode::OpNullCheck(_) => OpCodeNumber::OpNullCheck,
            OpCode::OpTrap(_) => OpCodeNumber::OpTrap,
            OpCode::OpEndTrap(_) => OpCodeNumber::OpEndTrap,
            OpCode::OpGetI8(_) => OpCodeNumber::OpGetI8,
            OpCode::OpGetI16(_) => OpCodeNumber::OpGetI16,
            OpCode::OpGetMem(_) => OpCodeNumber::OpGetMem,
            OpCode::OpGetArray(_) => OpCodeNumber::OpGetArray,
            OpCode::OpSetI8(_) => OpCodeNumber::OpSetI8,
            OpCode::OpSetI16(_) => OpCodeNumber::OpSetI16,
            OpCode::OpSetMem(_) => OpCodeNumber::OpSetMem,
            OpCode::OpSetArray(_) => OpCodeNumber::OpSetArray,
            OpCode::OpNew(_) => OpCodeNumber::OpNew,
            OpCode::OpArraySize(_) => OpCodeNumber::OpArraySize,
            OpCode::OpType(_) => OpCodeNumber::OpType,
            OpCode::OpGetType(_) => OpCodeNumber::OpGetType,
            OpCode::OpGetTID(_) => OpCodeNumber::OpGetTID,
            OpCode::OpRef(_) => OpCodeNumber::OpRef,
            OpCode::OpUnRef(_) => OpCodeNumber::OpUnRef,
            OpCode::OpSetRef(_) => OpCodeNumber::OpSetRef,
            OpCode::OpMakeEnum(_) => OpCodeNumber::OpMakeEnum,
            OpCode::OpEnumAlloc(_) => OpCodeNumber::OpEnumAlloc,
            OpCode::OpEnumIndex(_) => OpCodeNumber::OpEnumIndex,
            OpCode::OpEnumField(_) => OpCodeNumber::OpEnumField,
            OpCode::OpSetEnumField(_) => OpCodeNumber::OpSetEnumField,
            OpCode::OpAssert => OpCodeNumber::OpAssert,
            OpCode::OpRefData(_) => OpCodeNumber::OpRefData,
            OpCode::OpRefOffset(_) => OpCodeNumber::OpRefOffset,
            OpCode::OpNop => OpCodeNumber::OpNop,
        }
    }
}
