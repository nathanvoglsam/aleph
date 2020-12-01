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

use crate::OpCode;

/// Struct used for iterating over every opcode in the hashlink bytecode definition
pub struct TestOpCodeIter {
    current: Option<OpCode>,
}

impl TestOpCodeIter {
    pub fn new() -> Self {
        Self {
            current: Some(OpCode::OpMov(Default::default())),
        }
    }
}

impl Iterator for TestOpCodeIter {
    type Item = OpCode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.as_ref() {
            let new_val = match current {
                OpCode::OpMov(_) => Some(OpCode::OpInt(Default::default())),
                OpCode::OpInt(_) => Some(OpCode::OpFloat(Default::default())),
                OpCode::OpFloat(_) => Some(OpCode::OpBool(Default::default())),
                OpCode::OpBool(_) => Some(OpCode::OpBytes(Default::default())),
                OpCode::OpBytes(_) => Some(OpCode::OpString(Default::default())),
                OpCode::OpString(_) => Some(OpCode::OpNull(Default::default())),
                OpCode::OpNull(_) => Some(OpCode::OpAdd(Default::default())),
                OpCode::OpAdd(_) => Some(OpCode::OpSub(Default::default())),
                OpCode::OpSub(_) => Some(OpCode::OpMul(Default::default())),
                OpCode::OpMul(_) => Some(OpCode::OpSDiv(Default::default())),
                OpCode::OpSDiv(_) => Some(OpCode::OpUDiv(Default::default())),
                OpCode::OpUDiv(_) => Some(OpCode::OpSMod(Default::default())),
                OpCode::OpSMod(_) => Some(OpCode::OpUMod(Default::default())),
                OpCode::OpUMod(_) => Some(OpCode::OpShl(Default::default())),
                OpCode::OpShl(_) => Some(OpCode::OpSShr(Default::default())),
                OpCode::OpSShr(_) => Some(OpCode::OpUShr(Default::default())),
                OpCode::OpUShr(_) => Some(OpCode::OpAnd(Default::default())),
                OpCode::OpAnd(_) => Some(OpCode::OpOr(Default::default())),
                OpCode::OpOr(_) => Some(OpCode::OpXor(Default::default())),
                OpCode::OpXor(_) => Some(OpCode::OpNeg(Default::default())),
                OpCode::OpNeg(_) => Some(OpCode::OpNot(Default::default())),
                OpCode::OpNot(_) => Some(OpCode::OpIncr(Default::default())),
                OpCode::OpIncr(_) => Some(OpCode::OpDecr(Default::default())),
                OpCode::OpDecr(_) => Some(OpCode::OpCall0(Default::default())),
                OpCode::OpCall0(_) => Some(OpCode::OpCall1(Default::default())),
                OpCode::OpCall1(_) => Some(OpCode::OpCall2(Default::default())),
                OpCode::OpCall2(_) => Some(OpCode::OpCall3(Default::default())),
                OpCode::OpCall3(_) => Some(OpCode::OpCall4(Default::default())),
                OpCode::OpCall4(_) => Some(OpCode::OpCallN(Default::default())),
                OpCode::OpCallN(_) => Some(OpCode::OpCallMethod(Default::default())),
                OpCode::OpCallMethod(_) => Some(OpCode::OpCallThis(Default::default())),
                OpCode::OpCallThis(_) => Some(OpCode::OpCallClosure(Default::default())),
                OpCode::OpCallClosure(_) => Some(OpCode::OpStaticClosure(Default::default())),
                OpCode::OpStaticClosure(_) => Some(OpCode::OpInstanceClosure(Default::default())),
                OpCode::OpInstanceClosure(_) => Some(OpCode::OpVirtualClosure(Default::default())),
                OpCode::OpVirtualClosure(_) => Some(OpCode::OpGetGlobal(Default::default())),
                OpCode::OpGetGlobal(_) => Some(OpCode::OpSetGlobal(Default::default())),
                OpCode::OpSetGlobal(_) => Some(OpCode::OpField(Default::default())),
                OpCode::OpField(_) => Some(OpCode::OpSetField(Default::default())),
                OpCode::OpSetField(_) => Some(OpCode::OpGetThis(Default::default())),
                OpCode::OpGetThis(_) => Some(OpCode::OpSetThis(Default::default())),
                OpCode::OpSetThis(_) => Some(OpCode::OpDynGet(Default::default())),
                OpCode::OpDynGet(_) => Some(OpCode::OpDynSet(Default::default())),
                OpCode::OpDynSet(_) => Some(OpCode::OpJTrue(Default::default())),
                OpCode::OpJTrue(_) => Some(OpCode::OpJFalse(Default::default())),
                OpCode::OpJFalse(_) => Some(OpCode::OpJNull(Default::default())),
                OpCode::OpJNull(_) => Some(OpCode::OpJNotNull(Default::default())),
                OpCode::OpJNotNull(_) => Some(OpCode::OpJSLt(Default::default())),
                OpCode::OpJSLt(_) => Some(OpCode::OpJSGte(Default::default())),
                OpCode::OpJSGte(_) => Some(OpCode::OpJSGt(Default::default())),
                OpCode::OpJSGt(_) => Some(OpCode::OpJSLte(Default::default())),
                OpCode::OpJSLte(_) => Some(OpCode::OpJULt(Default::default())),
                OpCode::OpJULt(_) => Some(OpCode::OpJUGte(Default::default())),
                OpCode::OpJUGte(_) => Some(OpCode::OpJNotLt(Default::default())),
                OpCode::OpJNotLt(_) => Some(OpCode::OpJNotGte(Default::default())),
                OpCode::OpJNotGte(_) => Some(OpCode::OpJEq(Default::default())),
                OpCode::OpJEq(_) => Some(OpCode::OpJNotEq(Default::default())),
                OpCode::OpJNotEq(_) => Some(OpCode::OpJAlways(Default::default())),
                OpCode::OpJAlways(_) => Some(OpCode::OpToDyn(Default::default())),
                OpCode::OpToDyn(_) => Some(OpCode::OpToSFloat(Default::default())),
                OpCode::OpToSFloat(_) => Some(OpCode::OpToUFloat(Default::default())),
                OpCode::OpToUFloat(_) => Some(OpCode::OpToInt(Default::default())),
                OpCode::OpToInt(_) => Some(OpCode::OpSafeCast(Default::default())),
                OpCode::OpSafeCast(_) => Some(OpCode::OpUnsafeCast(Default::default())),
                OpCode::OpUnsafeCast(_) => Some(OpCode::OpToVirtual(Default::default())),
                OpCode::OpToVirtual(_) => Some(OpCode::OpLabel),
                OpCode::OpLabel => Some(OpCode::OpRet(Default::default())),
                OpCode::OpRet(_) => Some(OpCode::OpThrow(Default::default())),
                OpCode::OpThrow(_) => Some(OpCode::OpRethrow(Default::default())),
                OpCode::OpRethrow(_) => Some(OpCode::OpSwitch(Default::default())),
                OpCode::OpSwitch(_) => Some(OpCode::OpNullCheck(Default::default())),
                OpCode::OpNullCheck(_) => Some(OpCode::OpTrap(Default::default())),
                OpCode::OpTrap(_) => Some(OpCode::OpEndTrap(Default::default())),
                OpCode::OpEndTrap(_) => Some(OpCode::OpGetI8(Default::default())),
                OpCode::OpGetI8(_) => Some(OpCode::OpGetI16(Default::default())),
                OpCode::OpGetI16(_) => Some(OpCode::OpGetMem(Default::default())),
                OpCode::OpGetMem(_) => Some(OpCode::OpGetArray(Default::default())),
                OpCode::OpGetArray(_) => Some(OpCode::OpSetI8(Default::default())),
                OpCode::OpSetI8(_) => Some(OpCode::OpSetI16(Default::default())),
                OpCode::OpSetI16(_) => Some(OpCode::OpSetMem(Default::default())),
                OpCode::OpSetMem(_) => Some(OpCode::OpSetArray(Default::default())),
                OpCode::OpSetArray(_) => Some(OpCode::OpNew(Default::default())),
                OpCode::OpNew(_) => Some(OpCode::OpArraySize(Default::default())),
                OpCode::OpArraySize(_) => Some(OpCode::OpType(Default::default())),
                OpCode::OpType(_) => Some(OpCode::OpGetType(Default::default())),
                OpCode::OpGetType(_) => Some(OpCode::OpGetTID(Default::default())),
                OpCode::OpGetTID(_) => Some(OpCode::OpRef(Default::default())),
                OpCode::OpRef(_) => Some(OpCode::OpUnRef(Default::default())),
                OpCode::OpUnRef(_) => Some(OpCode::OpSetRef(Default::default())),
                OpCode::OpSetRef(_) => Some(OpCode::OpMakeEnum(Default::default())),
                OpCode::OpMakeEnum(_) => Some(OpCode::OpEnumAlloc(Default::default())),
                OpCode::OpEnumAlloc(_) => Some(OpCode::OpEnumIndex(Default::default())),
                OpCode::OpEnumIndex(_) => Some(OpCode::OpEnumField(Default::default())),
                OpCode::OpEnumField(_) => Some(OpCode::OpSetEnumField(Default::default())),
                OpCode::OpSetEnumField(_) => Some(OpCode::OpAssert),
                OpCode::OpAssert => Some(OpCode::OpRefData(Default::default())),
                OpCode::OpRefData(_) => Some(OpCode::OpRefOffset(Default::default())),
                OpCode::OpRefOffset(_) => Some(OpCode::OpNop),
                OpCode::OpNop => None,
            };
            self.current = new_val;
            self.current.clone()
        } else {
            None
        }
    }
}
