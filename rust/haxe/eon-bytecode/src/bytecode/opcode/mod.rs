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

#[derive(Clone, Debug)]
pub struct CallParam {
    pub op_params: [i32; 2],
    pub fn_params: Vec<i32>,
}

#[derive(Clone, Debug)]
pub struct SwitchParam {
    pub input_reg: u32,
    pub jump_table: Vec<u32>,
    pub fallback: u32,
}

#[derive(Clone, Debug)]
pub enum OpCode {
    // Type and value initialization op codes
    OpMov([i32; 2]),
    OpInt([i32; 2]),
    OpFloat([i32; 2]),
    OpBool([i32; 2]),
    OpBytes([i32; 2]),
    OpString([i32; 2]),
    OpNull(i32),

    // Arithmetic opcodes
    OpAdd([i32; 3]),
    OpSub([i32; 3]),
    OpMul([i32; 3]),
    OpSDiv([i32; 3]),
    OpUDiv([i32; 3]),
    OpSMod([i32; 3]),
    OpUMod([i32; 3]),
    OpShl([i32; 3]),
    OpSShr([i32; 3]),
    OpUShr([i32; 3]),
    OpAnd([i32; 3]),
    OpOr([i32; 3]),
    OpXor([i32; 3]),
    OpNeg([i32; 2]),
    OpNot([i32; 2]),
    OpIncr(i32),
    OpDecr(i32),

    // Function calling opcodes
    OpCall(CallParam),
    OpCallMethod(CallParam),
    OpCallThis(CallParam),
    OpCallClosure(CallParam),

    // No idea what the specifics of these are, but I'm guessing allocate closures
    OpStaticClosure([i32; 2]),
    OpInstanceClosure([i32; 3]),
    OpVirtualClosure([i32; 3]),

    // Global getting and setting opcodes
    OpGetGlobal([i32; 2]),
    OpSetGlobal([i32; 2]),

    // Object field access
    OpField([i32; 3]),    // Gets a field on the given object
    OpSetField([i32; 3]), // Sets a field on the given object
    OpGetThis([i32; 2]),  // Gets a field on `this` (just OpField with object implicitly reg 0)
    OpSetThis([i32; 2]),  // Sets a field on `this` (just OpSetField with object implicitly reg 0)
    OpDynGet([i32; 3]),   // Gets a field on a dyn object
    OpDynSet([i32; 3]),   // Sets a field on a dyn object

    // Branching opcodes
    OpJTrue([i32; 2]),
    OpJFalse([i32; 2]),
    OpJNull([i32; 2]),
    OpJNotNull([i32; 2]),
    OpJSLt([i32; 3]),
    OpJSGte([i32; 3]),
    OpJSGt([i32; 3]),
    OpJSLte([i32; 3]),
    OpJULt([i32; 3]),
    OpJUGte([i32; 3]),
    OpJNotLt([i32; 3]),
    OpJNotGte([i32; 3]),
    OpJEq([i32; 3]),
    OpJNotEq([i32; 3]),
    OpJAlways(i32),
    OpLabel,
    OpRet(i32),
    OpSwitch(SwitchParam),

    // Casting opcodes
    OpToDyn([i32; 2]),
    OpToSFloat([i32; 2]),
    OpToUFloat([i32; 2]),
    OpToInt([i32; 2]),

    // Coercions opcodes
    OpSafeCast([i32; 2]),
    OpUnsafeCast([i32; 2]),
    OpToVirtual([i32; 2]),

    // Exception opcodes
    OpThrow(i32),
    OpRethrow(i32),
    OpTrap([i32; 2]),
    OpEndTrap(i32),
    OpNullCheck(i32),

    // Bytes section reading opcodes
    OpGetI8([i32; 3]),
    OpGetI16([i32; 3]),
    OpGetMem([i32; 3]),
    OpGetArray([i32; 3]),
    OpSetI8([i32; 3]),
    OpSetI16([i32; 3]),
    OpSetMem([i32; 3]),
    OpSetArray([i32; 3]),
    OpNew(i32),
    OpArraySize([i32; 2]),
    OpType([i32; 2]),
    OpGetType([i32; 2]),
    OpGetTID([i32; 2]),

    // Reference opcodes
    OpRef([i32; 2]),
    OpUnref([i32; 2]),
    OpSetRef([i32; 2]),

    // Enum opcodes
    OpMakeEnum(CallParam),
    OpEnumAlloc([i32; 2]),
    OpEnumIndex([i32; 2]),
    OpEnumField([i32; 4]),
    OpSetEnumField([i32; 3]),

    // Not really sure at the moment
    OpAssert,
    OpRefData([i32; 2]),
    OpRefOffset([i32; 3]),

    // Noop
    OpNop,
}

static EMPTY: [i32; 0] = [];

impl OpCode {
    pub fn op_params(&self) -> &[i32] {
        match self {
            OpCode::OpMov(v) => v,
            OpCode::OpInt(v) => v,
            OpCode::OpFloat(v) => v,
            OpCode::OpBool(v) => v,
            OpCode::OpBytes(v) => v,
            OpCode::OpString(v) => v,
            OpCode::OpNull(v) => std::slice::from_ref(v),
            OpCode::OpAdd(v) => v,
            OpCode::OpSub(v) => v,
            OpCode::OpMul(v) => v,
            OpCode::OpSDiv(v) => v,
            OpCode::OpUDiv(v) => v,
            OpCode::OpSMod(v) => v,
            OpCode::OpUMod(v) => v,
            OpCode::OpShl(v) => v,
            OpCode::OpSShr(v) => v,
            OpCode::OpUShr(v) => v,
            OpCode::OpAnd(v) => v,
            OpCode::OpOr(v) => v,
            OpCode::OpXor(v) => v,
            OpCode::OpNeg(v) => v,
            OpCode::OpNot(v) => v,
            OpCode::OpIncr(v) => std::slice::from_ref(v),
            OpCode::OpDecr(v) => std::slice::from_ref(v),
            OpCode::OpCall(v) => &v.op_params,
            OpCode::OpCallMethod(v) => &v.op_params,
            OpCode::OpCallThis(v) => &v.op_params,
            OpCode::OpCallClosure(v) => &v.op_params,
            OpCode::OpStaticClosure(v) => v,
            OpCode::OpInstanceClosure(v) => v,
            OpCode::OpVirtualClosure(v) => v,
            OpCode::OpGetGlobal(v) => v,
            OpCode::OpSetGlobal(v) => v,
            OpCode::OpField(v) => v,
            OpCode::OpSetField(v) => v,
            OpCode::OpGetThis(v) => v,
            OpCode::OpSetThis(v) => v,
            OpCode::OpDynGet(v) => v,
            OpCode::OpDynSet(v) => v,
            OpCode::OpJTrue(v) => v,
            OpCode::OpJFalse(v) => v,
            OpCode::OpJNull(v) => v,
            OpCode::OpJNotNull(v) => v,
            OpCode::OpJSLt(v) => v,
            OpCode::OpJSGte(v) => v,
            OpCode::OpJSGt(v) => v,
            OpCode::OpJSLte(v) => v,
            OpCode::OpJULt(v) => v,
            OpCode::OpJUGte(v) => v,
            OpCode::OpJNotLt(v) => v,
            OpCode::OpJNotGte(v) => v,
            OpCode::OpJEq(v) => v,
            OpCode::OpJNotEq(v) => v,
            OpCode::OpJAlways(v) => std::slice::from_ref(v),
            OpCode::OpToDyn(v) => v,
            OpCode::OpToSFloat(v) => v,
            OpCode::OpToUFloat(v) => v,
            OpCode::OpToInt(v) => v,
            OpCode::OpSafeCast(v) => v,
            OpCode::OpUnsafeCast(v) => v,
            OpCode::OpToVirtual(v) => v,
            OpCode::OpLabel => &EMPTY,
            OpCode::OpRet(v) => std::slice::from_ref(v),
            OpCode::OpThrow(v) => std::slice::from_ref(v),
            OpCode::OpRethrow(v) => std::slice::from_ref(v),
            OpCode::OpSwitch(v) => &std::slice::from_ref(&v.input_reg),
            OpCode::OpNullCheck(v) => std::slice::from_ref(v),
            OpCode::OpTrap(v) => v,
            OpCode::OpEndTrap(v) => std::slice::from_ref(v),
            OpCode::OpGetI8(v) => v,
            OpCode::OpGetI16(v) => v,
            OpCode::OpGetMem(v) => v,
            OpCode::OpGetArray(v) => v,
            OpCode::OpSetI8(v) => v,
            OpCode::OpSetI16(v) => v,
            OpCode::OpSetMem(v) => v,
            OpCode::OpSetArray(v) => v,
            OpCode::OpNew(v) => std::slice::from_ref(v),
            OpCode::OpArraySize(v) => v,
            OpCode::OpType(v) => v,
            OpCode::OpGetType(v) => v,
            OpCode::OpGetTID(v) => v,
            OpCode::OpRef(v) => v,
            OpCode::OpUnref(v) => v,
            OpCode::OpSetRef(v) => v,
            OpCode::OpMakeEnum(v) => &v.op_params,
            OpCode::OpEnumAlloc(v) => v,
            OpCode::OpEnumIndex(v) => v,
            OpCode::OpEnumField(v) => v,
            OpCode::OpSetEnumField(v) => v,
            OpCode::OpAssert => &EMPTY,
            OpCode::OpRefData(v) => v,
            OpCode::OpRefOffset(v) => v,
            OpCode::OpNop => &EMPTY,
        }
    }
}
