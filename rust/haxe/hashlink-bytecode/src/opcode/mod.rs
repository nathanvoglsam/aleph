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

#[derive(Clone, Serialize, Deserialize)]
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
    OpUnref(OpTwoParam),
    OpSetref(OpTwoParam),
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
            OpCodeNumber::OpUnref => Some(OpCode::OpUnref(params)),
            OpCodeNumber::OpSetref => Some(OpCode::OpSetref(params)),
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

    pub fn arg_count(&self) -> Option<usize> {
        self.opcode_number().opcode_type().arg_num()
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
            OpCode::OpUnref(_) => OpCodeNumber::OpUnref,
            OpCode::OpSetref(_) => OpCodeNumber::OpSetref,
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
