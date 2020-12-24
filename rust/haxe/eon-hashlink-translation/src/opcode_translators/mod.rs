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

use eon_bytecode::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, IntegerIndex, StringIndex, TypeIndex, ValueIndex,
};
use eon_bytecode::intrinsic::Intrinsic;
use eon_bytecode::opcode::{
    AllocEnum, Binop, Call, CallClosure, CallIntrinsic, CallMethod, Cast, Comparison, ComparisonFn,
    CondBranch, FieldLoad, FieldStore, InstanceClosure, Invoke, InvokeIntrinsic, Load, LoadBool,
    LoadBytes, LoadEnumField, LoadFloat, LoadGlobal, LoadInt, LoadString, LoadType, MakeEnum,
    OpCode, ReadMemory, RefData, RefOffset, StaticClosure, Store, StoreEnumField, StoreGlobal,
    Switch, Unop, VirtualClosure, WriteMemory,
};

/// Translate a load type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Load { assigns, source };
    match op {
        hashlink::OpCode::OpMov(_) => Some(OpCode::OpMov(inner)),
        hashlink::OpCode::OpArraySize(_) => Some(OpCode::OpArraySize(inner)),
        hashlink::OpCode::OpGetType(_) => Some(OpCode::OpGetType(inner)),
        hashlink::OpCode::OpGetTID(_) => Some(OpCode::OpGetTID(inner)),
        hashlink::OpCode::OpRef(_) => Some(OpCode::OpRef(inner)),
        hashlink::OpCode::OpUnRef(_) => Some(OpCode::OpUnRef(inner)),
        hashlink::OpCode::OpEnumIndex(_) => Some(OpCode::OpEnumIndex(inner)),
        _ => None,
    }
}

/// Translate a load int type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_int(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    integer: IntegerIndex,
) -> Option<OpCode> {
    let inner = LoadInt { assigns, integer };
    match op {
        hashlink::OpCode::OpInt(_) => Some(OpCode::OpInt(inner)),
        _ => None,
    }
}

/// Translate a load float type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_float(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    float: FloatIndex,
) -> Option<OpCode> {
    let inner = LoadFloat { assigns, float };
    match op {
        hashlink::OpCode::OpFloat(_) => Some(OpCode::OpFloat(inner)),
        _ => None,
    }
}

/// Translate a load bool type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_bool(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    value: bool,
) -> Option<OpCode> {
    let inner = LoadBool { assigns, value };
    match op {
        hashlink::OpCode::OpBool(_) => Some(OpCode::OpBool(inner)),
        _ => None,
    }
}

/// Translate a load bytes type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_bytes(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    bytes: BytesIndex,
) -> Option<OpCode> {
    let inner = LoadBytes { assigns, bytes };
    match op {
        hashlink::OpCode::OpBytes(_) => Some(OpCode::OpBytes(inner)),
        _ => None,
    }
}

/// Translate a load string type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_string(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    string: StringIndex,
) -> Option<OpCode> {
    let inner = LoadString { assigns, string };
    match op {
        hashlink::OpCode::OpString(_) => Some(OpCode::OpString(inner)),
        _ => None,
    }
}

/// Translate a load global type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_global(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: GlobalIndex,
) -> Option<OpCode> {
    let inner = LoadGlobal { assigns, source };
    match op {
        hashlink::OpCode::OpGetGlobal(_) => Some(OpCode::OpGetGlobal(inner)),
        _ => None,
    }
}

/// Translate a load type type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_type(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: TypeIndex,
) -> Option<OpCode> {
    let inner = LoadType { assigns, source };
    match op {
        hashlink::OpCode::OpType(_) => Some(OpCode::OpType(inner)),
        _ => None,
    }
}

/// Translate a load enum fieldtype HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_load_enum_field(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
    constructor: ConstructorIndex,
    field_index: FieldIndex,
) -> Option<OpCode> {
    let inner = LoadEnumField {
        assigns,
        source,
        constructor,
        field_index,
    };
    match op {
        hashlink::OpCode::OpEnumField(_) => Some(OpCode::OpEnumField(inner)),
        _ => None,
    }
}

/// Translate a store enum field type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_store_enum_field(
    op: &hashlink::OpCode,
    target: ValueIndex,
    field: FieldIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = StoreEnumField {
        target,
        field,
        source,
    };
    match op {
        hashlink::OpCode::OpSetEnumField(_) => Some(OpCode::OpSetEnumField(inner)),
        _ => None,
    }
}

/// Translate a store global HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_store_global(
    op: &hashlink::OpCode,
    target: GlobalIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = StoreGlobal { target, source };
    match op {
        hashlink::OpCode::OpSetGlobal(_) => Some(OpCode::OpSetGlobal(inner)),
        _ => None,
    }
}

/// Translate a store type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_store(
    op: &hashlink::OpCode,
    target: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Store { target, source };
    match op {
        hashlink::OpCode::OpSetRef(_) => Some(OpCode::OpSetRef(inner)),
        _ => None,
    }
}

/// Translate a binop type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_binop(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    lhs: ValueIndex,
    rhs: ValueIndex,
) -> Option<OpCode> {
    let inner = Binop { assigns, lhs, rhs };
    match op {
        hashlink::OpCode::OpAdd(_) => Some(OpCode::OpAdd(inner)),
        hashlink::OpCode::OpSub(_) => Some(OpCode::OpSub(inner)),
        hashlink::OpCode::OpMul(_) => Some(OpCode::OpMul(inner)),
        hashlink::OpCode::OpSDiv(_) => Some(OpCode::OpSDiv(inner)),
        hashlink::OpCode::OpUDiv(_) => Some(OpCode::OpUDiv(inner)),
        hashlink::OpCode::OpSMod(_) => Some(OpCode::OpSMod(inner)),
        hashlink::OpCode::OpUMod(_) => Some(OpCode::OpUMod(inner)),
        hashlink::OpCode::OpShl(_) => Some(OpCode::OpShl(inner)),
        hashlink::OpCode::OpSShr(_) => Some(OpCode::OpSShr(inner)),
        hashlink::OpCode::OpUShr(_) => Some(OpCode::OpUShr(inner)),
        hashlink::OpCode::OpAnd(_) => Some(OpCode::OpAnd(inner)),
        hashlink::OpCode::OpOr(_) => Some(OpCode::OpOr(inner)),
        hashlink::OpCode::OpXor(_) => Some(OpCode::OpXor(inner)),
        _ => None,
    }
}

/// Translate a unop type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_unop(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    operand: ValueIndex,
) -> Option<OpCode> {
    let inner = Unop { assigns, operand };
    match op {
        hashlink::OpCode::OpNeg(_) => Some(OpCode::OpNeg(inner)),
        hashlink::OpCode::OpNot(_) => Some(OpCode::OpNot(inner)),
        hashlink::OpCode::OpIncr(_) => Some(OpCode::OpIncr(inner)),
        hashlink::OpCode::OpDecr(_) => Some(OpCode::OpDecr(inner)),
        _ => None,
    }
}

/// Translate a call type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_call(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    function: FunctionIndex,
    fn_params: Vec<ValueIndex>,
) -> Option<OpCode> {
    let inner = Call {
        assigns,
        function,
        fn_params,
    };
    match op {
        hashlink::OpCode::OpCall0(_) => Some(OpCode::OpCall(inner)),
        hashlink::OpCode::OpCall1(_) => Some(OpCode::OpCall(inner)),
        hashlink::OpCode::OpCall2(_) => Some(OpCode::OpCall(inner)),
        hashlink::OpCode::OpCall3(_) => Some(OpCode::OpCall(inner)),
        hashlink::OpCode::OpCall4(_) => Some(OpCode::OpCall(inner)),
        hashlink::OpCode::OpCallN(_) => Some(OpCode::OpCall(inner)),
        _ => None,
    }
}

/// Translate a call type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_invoke(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    function: FunctionIndex,
    fn_params: Vec<ValueIndex>,
    continuation: BasicBlockIndex,
    exception_target: BasicBlockIndex,
) -> Option<OpCode> {
    let inner = Invoke {
        assigns,
        function,
        fn_params,
        continuation,
        exception_target,
    };
    match op {
        hashlink::OpCode::OpCall0(_) => Some(OpCode::OpInvoke(inner)),
        hashlink::OpCode::OpCall1(_) => Some(OpCode::OpInvoke(inner)),
        hashlink::OpCode::OpCall2(_) => Some(OpCode::OpInvoke(inner)),
        hashlink::OpCode::OpCall3(_) => Some(OpCode::OpInvoke(inner)),
        hashlink::OpCode::OpCall4(_) => Some(OpCode::OpInvoke(inner)),
        hashlink::OpCode::OpCallN(_) => Some(OpCode::OpInvoke(inner)),
        _ => None,
    }
}

/// Translate a call field type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_call_field(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    object: ValueIndex,
    function: FieldIndex,
    fn_params: Vec<ValueIndex>,
) -> Option<OpCode> {
    let inner = CallMethod {
        assigns,
        object,
        function,
        fn_params,
    };
    match op {
        hashlink::OpCode::OpCallMethod(_) => Some(OpCode::OpCallMethod(inner)),
        hashlink::OpCode::OpCallThis(_) => Some(OpCode::OpCallMethod(inner)),
        _ => None,
    }
}

/// Translate a call closure type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_call_closure(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    closure: ValueIndex,
    fn_params: Vec<ValueIndex>,
) -> Option<OpCode> {
    let inner = CallClosure {
        assigns,
        closure,
        fn_params,
    };
    match op {
        hashlink::OpCode::OpCallClosure(_) => Some(OpCode::OpCallClosure(inner)),
        _ => None,
    }
}

/// Translate a switch type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_switch(
    op: &hashlink::OpCode,
    input: ValueIndex,
    jump_table: Vec<BasicBlockIndex>,
    fallback: BasicBlockIndex,
) -> Option<OpCode> {
    let inner = Switch {
        input,
        jump_table,
        fallback,
    };
    match op {
        hashlink::OpCode::OpSwitch(_) => Some(OpCode::OpSwitch(inner)),
        _ => None,
    }
}

/// Translate a field load type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_field_load(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    object: ValueIndex,
    field: FieldIndex,
) -> Option<OpCode> {
    let inner = FieldLoad {
        assigns,
        object,
        field,
    };
    match op {
        hashlink::OpCode::OpField(_) => Some(OpCode::OpGetField(inner)),
        hashlink::OpCode::OpGetThis(_) => Some(OpCode::OpGetField(inner)),
        hashlink::OpCode::OpDynGet(_) => Some(OpCode::OpDynGet(inner)),
        _ => None,
    }
}

/// Translate a field store type HashLink opcode into the corresponding form in Eon, built from the
/// provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_field_store(
    op: &hashlink::OpCode,
    object: ValueIndex,
    field: FieldIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = FieldStore {
        object,
        field,
        source,
    };
    match op {
        hashlink::OpCode::OpSetField(_) => Some(OpCode::OpSetField(inner)),
        hashlink::OpCode::OpSetThis(_) => Some(OpCode::OpSetField(inner)),
        hashlink::OpCode::OpDynSet(_) => Some(OpCode::OpDynSet(inner)),
        _ => None,
    }
}

/// Translate a conditional branch type HashLink opcode into the corresponding form in Eon, built
/// from the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_cond_branch(
    op: &hashlink::OpCode,
    check: ValueIndex,
    success: BasicBlockIndex,
    failure: BasicBlockIndex,
) -> Option<OpCode> {
    let inner = CondBranch {
        check,
        success,
        failure,
    };
    match op {
        hashlink::OpCode::OpJTrue(_) => Some(OpCode::OpJTrue(inner)),
        hashlink::OpCode::OpJFalse(_) => Some(OpCode::OpJFalse(inner)),
        hashlink::OpCode::OpJNull(_) => Some(OpCode::OpJNull(inner)),
        hashlink::OpCode::OpJNotNull(_) => Some(OpCode::OpJNotNull(inner)),
        _ => None,
    }
}

/// Translate a comparison branch type HashLink opcode into the corresponding form in Eon, built
/// from the provided values
///
/// # Warning
///
/// This function can *NOT* be used in isolation to translate a comparison based branch. Eon
/// bytecode is sane, and separates the comparison from the branching instruction so to correctly
/// translate this opcode requires emitting multiple instructions. This function will translate the
/// branch in to a *COMPARISON*. The actual branch, which should be based on the value assigned by
/// the comparison, will need to be emitted into the instruction stream separately
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_comp_branch(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    lhs: ValueIndex,
    rhs: ValueIndex,
) -> Option<OpCode> {
    let comp_fn = match op {
        hashlink::OpCode::OpJSLt(_) => Some(ComparisonFn::SignedLT),
        hashlink::OpCode::OpJSGte(_) => Some(ComparisonFn::SignedGTE),
        hashlink::OpCode::OpJSGt(_) => Some(ComparisonFn::SignedGT),
        hashlink::OpCode::OpJSLte(_) => Some(ComparisonFn::SignedLTE),
        hashlink::OpCode::OpJULt(_) => Some(ComparisonFn::UnsignedLT),
        hashlink::OpCode::OpJUGte(_) => Some(ComparisonFn::UnsignedGTE),
        hashlink::OpCode::OpJNotLt(_) => Some(ComparisonFn::SignedGTE),
        hashlink::OpCode::OpJNotGte(_) => Some(ComparisonFn::SignedLT),
        hashlink::OpCode::OpJEq(_) => Some(ComparisonFn::Eq),
        hashlink::OpCode::OpJNotEq(_) => Some(ComparisonFn::Ne),
        _ => None,
    }?;
    let inner = Comparison {
        comp_fn,
        assigns,
        lhs,
        rhs,
    };
    Some(OpCode::OpCmp(inner))
}

/// Translate a static closure type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_static_closure(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    function: FunctionIndex,
) -> Option<OpCode> {
    let inner = StaticClosure { assigns, function };
    match op {
        hashlink::OpCode::OpStaticClosure(_) => Some(OpCode::OpStaticClosure(inner)),
        _ => None,
    }
}

/// Translate an instance closure type HashLink opcode into the corresponding form in Eon, built
/// from the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_instance_closure(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    function: FunctionIndex,
    object: ValueIndex,
) -> Option<OpCode> {
    let inner = InstanceClosure {
        assigns,
        function,
        object,
    };
    match op {
        hashlink::OpCode::OpInstanceClosure(_) => Some(OpCode::OpInstanceClosure(inner)),
        _ => None,
    }
}

/// Translate a virtual closure type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_virtual_closure(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    object: ValueIndex,
    field: FieldIndex,
) -> Option<OpCode> {
    let inner = VirtualClosure {
        assigns,
        object,
        field,
    };
    match op {
        hashlink::OpCode::OpVirtualClosure(_) => Some(OpCode::OpVirtualClosure(inner)),
        _ => None,
    }
}

/// Translate a cast type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_cast(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Cast { assigns, source };
    match op {
        hashlink::OpCode::OpToDyn(_) => Some(OpCode::OpToDyn(inner)),
        hashlink::OpCode::OpToSFloat(_) => Some(OpCode::OpToSFloat(inner)),
        hashlink::OpCode::OpToUFloat(_) => Some(OpCode::OpToUFloat(inner)),
        hashlink::OpCode::OpToInt(_) => Some(OpCode::OpToInt(inner)),
        hashlink::OpCode::OpSafeCast(_) => {
            let inner = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::SafeCast,
                fn_params: vec![source],
            };
            Some(OpCode::OpCallIntrinsic(inner))
        }
        hashlink::OpCode::OpUnsafeCast(_) => {
            let inner = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::UnsafeCast,
                fn_params: vec![source],
            };
            Some(OpCode::OpCallIntrinsic(inner))
        }
        hashlink::OpCode::OpToVirtual(_) => Some(OpCode::OpToVirtual(inner)),
        _ => None,
    }
}

/// Translate a read memory type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_read_memory(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
    offset: ValueIndex,
) -> Option<OpCode> {
    let inner = ReadMemory {
        assigns,
        source,
        offset,
    };
    match op {
        hashlink::OpCode::OpGetI8(_) => Some(OpCode::OpGetI8(inner)),
        hashlink::OpCode::OpGetI16(_) => Some(OpCode::OpGetI16(inner)),
        hashlink::OpCode::OpGetMem(_) => Some(OpCode::OpGetMem(inner)),
        hashlink::OpCode::OpGetArray(_) => Some(OpCode::OpGetArray(inner)),
        _ => None,
    }
}

/// Translate a write memory type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_write_memory(
    op: &hashlink::OpCode,
    target: ValueIndex,
    offset: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = WriteMemory {
        target,
        source,
        offset,
    };
    match op {
        hashlink::OpCode::OpSetI8(_) => Some(OpCode::OpSetI8(inner)),
        hashlink::OpCode::OpSetI16(_) => Some(OpCode::OpSetI16(inner)),
        hashlink::OpCode::OpSetMem(_) => Some(OpCode::OpSetMem(inner)),
        hashlink::OpCode::OpSetArray(_) => Some(OpCode::OpSetArray(inner)),
        _ => None,
    }
}

/// Translate a make enum type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_make_enum(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    constructor: ConstructorIndex,
    args: Vec<ValueIndex>,
) -> Option<OpCode> {
    let inner = MakeEnum {
        assigns,
        constructor,
        args,
    };
    match op {
        hashlink::OpCode::OpMakeEnum(_) => Some(OpCode::OpMakeEnum(inner)),
        _ => None,
    }
}

/// Translate an alloc enum type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_alloc_enum(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    constructor: ConstructorIndex,
) -> Option<OpCode> {
    let inner = AllocEnum {
        assigns,
        constructor,
    };
    match op {
        hashlink::OpCode::OpEnumAlloc(_) => Some(OpCode::OpEnumAlloc(inner)),
        _ => None,
    }
}

/// Translate a value index type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_value_index(op: &hashlink::OpCode, value: ValueIndex) -> Option<OpCode> {
    match op {
        hashlink::OpCode::OpNull(_) => Some(OpCode::OpNull(value)),
        hashlink::OpCode::OpRet(_) => Some(OpCode::OpRet(value)),
        hashlink::OpCode::OpNew(_) => Some(OpCode::OpNew(value)),
        _ => None,
    }
}

pub fn translate_intrinsic(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    args: impl IntoIterator<Item = ValueIndex>,
) -> Option<OpCode> {
    let args = args.into_iter();
    match op {
        hashlink::OpCode::OpThrow(_) => {
            let inner = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::Throw,
                fn_params: args.collect(),
            };
            Some(OpCode::OpCallIntrinsic(inner))
        }
        hashlink::OpCode::OpRethrow(_) => {
            let inner = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::Rethrow,
                fn_params: args.collect(),
            };
            Some(OpCode::OpCallIntrinsic(inner))
        }
        hashlink::OpCode::OpNullCheck(_) => {
            let inner = CallIntrinsic {
                assigns,
                intrinsic: Intrinsic::NullCheck,
                fn_params: args.collect(),
            };
            Some(OpCode::OpCallIntrinsic(inner))
        }
        _ => None,
    }
}

/// Translate a trap type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_intrinsic_invoke(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    args: impl IntoIterator<Item = ValueIndex>,
    continuation: BasicBlockIndex,
    exception_target: BasicBlockIndex,
) -> Option<OpCode> {
    let args = args.into_iter();
    let intrinsic = match op {
        hashlink::OpCode::OpTrap(_) => Some(Intrinsic::BeginTrap),
        hashlink::OpCode::OpEndTrap(_) => Some(Intrinsic::EndTrap),
        hashlink::OpCode::OpThrow(_) => Some(Intrinsic::Throw),
        hashlink::OpCode::OpRethrow(_) => Some(Intrinsic::Rethrow),
        hashlink::OpCode::OpNullCheck(_) => Some(Intrinsic::NullCheck),
        hashlink::OpCode::OpSafeCast(_) => Some(Intrinsic::SafeCast),
        _ => None,
    };
    let inner = InvokeIntrinsic {
        assigns,
        intrinsic: intrinsic?,
        fn_params: args.collect(),
        continuation,
        exception_target,
    };
    Some(OpCode::OpInvokeIntrinsic(inner))
}

/// Translate an unconditional branch type HashLink opcode into the corresponding form in Eon, built
/// from the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_unconditional_branch(
    op: &hashlink::OpCode,
    inner: BasicBlockIndex,
) -> Option<OpCode> {
    match op {
        hashlink::OpCode::OpJAlways(_) => Some(OpCode::OpJAlways(inner)),
        _ => None,
    }
}

/// Translate a ref data type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_ref_data(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = RefData { assigns, source };
    match op {
        hashlink::OpCode::OpRefData(_) => Some(OpCode::OpRefData(inner)),
        _ => None,
    }
}

/// Translate a ref offset type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_ref_offset(
    op: &hashlink::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
    offset: ValueIndex,
) -> Option<OpCode> {
    let inner = RefOffset {
        assigns,
        source,
        offset,
    };
    match op {
        hashlink::OpCode::OpRefOffset(_) => Some(OpCode::OpRefOffset(inner)),
        _ => None,
    }
}

/// Translate a no params type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_no_params(op: &hashlink::OpCode) -> Option<OpCode> {
    match op {
        hashlink::OpCode::OpAssert => Some(OpCode::OpAssert),
        hashlink::OpCode::OpNop => Some(OpCode::OpNop),
        hashlink::OpCode::OpLabel => Some(OpCode::OpNop),
        _ => None,
    }
}
