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
use eon_bytecode::opcode::{
    AllocEnum, Binop, Call, CallClosure, CallMethod, Cast, Comparison, ComparisonFn, CondBranch,
    FieldLoad, FieldStore, InstanceClosure, Load, LoadBool, LoadBytes, LoadEnumField, LoadFloat,
    LoadGlobal, LoadInt, LoadString, LoadType, MakeEnum, OpCode, ReadMemory, RefData, RefOffset,
    StaticClosure, Store, StoreEnumField, StoreGlobal, Switch, Trap, Unop, VirtualClosure,
    WriteMemory,
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Load { assigns, source };
    match op {
        hashlink_bytecode::OpCode::OpMov(_) => Some(OpCode::OpMov(inner)),
        hashlink_bytecode::OpCode::OpArraySize(_) => Some(OpCode::OpArraySize(inner)),
        hashlink_bytecode::OpCode::OpGetType(_) => Some(OpCode::OpGetType(inner)),
        hashlink_bytecode::OpCode::OpGetTID(_) => Some(OpCode::OpGetTID(inner)),
        hashlink_bytecode::OpCode::OpRef(_) => Some(OpCode::OpRef(inner)),
        hashlink_bytecode::OpCode::OpUnRef(_) => Some(OpCode::OpUnRef(inner)),
        hashlink_bytecode::OpCode::OpEnumIndex(_) => Some(OpCode::OpEnumIndex(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    integer: IntegerIndex,
) -> Option<OpCode> {
    let inner = LoadInt { assigns, integer };
    match op {
        hashlink_bytecode::OpCode::OpInt(_) => Some(OpCode::OpInt(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    float: FloatIndex,
) -> Option<OpCode> {
    let inner = LoadFloat { assigns, float };
    match op {
        hashlink_bytecode::OpCode::OpFloat(_) => Some(OpCode::OpFloat(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    value: bool,
) -> Option<OpCode> {
    let inner = LoadBool { assigns, value };
    match op {
        hashlink_bytecode::OpCode::OpBool(_) => Some(OpCode::OpBool(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    bytes: BytesIndex,
) -> Option<OpCode> {
    let inner = LoadBytes { assigns, bytes };
    match op {
        hashlink_bytecode::OpCode::OpBytes(_) => Some(OpCode::OpBytes(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    string: StringIndex,
) -> Option<OpCode> {
    let inner = LoadString { assigns, string };
    match op {
        hashlink_bytecode::OpCode::OpString(_) => Some(OpCode::OpString(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    source: GlobalIndex,
) -> Option<OpCode> {
    let inner = LoadGlobal { assigns, source };
    match op {
        hashlink_bytecode::OpCode::OpGetGlobal(_) => Some(OpCode::OpGetGlobal(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    source: TypeIndex,
) -> Option<OpCode> {
    let inner = LoadType { assigns, source };
    match op {
        hashlink_bytecode::OpCode::OpType(_) => Some(OpCode::OpType(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpEnumField(_) => Some(OpCode::OpEnumField(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpSetEnumField(_) => Some(OpCode::OpSetEnumField(inner)),
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
    op: &hashlink_bytecode::OpCode,
    source: ValueIndex,
    target: GlobalIndex,
) -> Option<OpCode> {
    let inner = StoreGlobal { source, target };
    match op {
        hashlink_bytecode::OpCode::OpSetGlobal(_) => Some(OpCode::OpSetGlobal(inner)),
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
    op: &hashlink_bytecode::OpCode,
    target: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Store { target, source };
    match op {
        hashlink_bytecode::OpCode::OpSetRef(_) => Some(OpCode::OpSetRef(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    lhs: ValueIndex,
    rhs: ValueIndex,
) -> Option<OpCode> {
    let inner = Binop { assigns, lhs, rhs };
    match op {
        hashlink_bytecode::OpCode::OpAdd(_) => Some(OpCode::OpAdd(inner)),
        hashlink_bytecode::OpCode::OpSub(_) => Some(OpCode::OpSub(inner)),
        hashlink_bytecode::OpCode::OpMul(_) => Some(OpCode::OpMul(inner)),
        hashlink_bytecode::OpCode::OpSDiv(_) => Some(OpCode::OpSDiv(inner)),
        hashlink_bytecode::OpCode::OpUDiv(_) => Some(OpCode::OpUDiv(inner)),
        hashlink_bytecode::OpCode::OpSMod(_) => Some(OpCode::OpSMod(inner)),
        hashlink_bytecode::OpCode::OpUMod(_) => Some(OpCode::OpUMod(inner)),
        hashlink_bytecode::OpCode::OpShl(_) => Some(OpCode::OpShl(inner)),
        hashlink_bytecode::OpCode::OpSShr(_) => Some(OpCode::OpSShr(inner)),
        hashlink_bytecode::OpCode::OpUShr(_) => Some(OpCode::OpUShr(inner)),
        hashlink_bytecode::OpCode::OpAnd(_) => Some(OpCode::OpAnd(inner)),
        hashlink_bytecode::OpCode::OpOr(_) => Some(OpCode::OpOr(inner)),
        hashlink_bytecode::OpCode::OpXor(_) => Some(OpCode::OpXor(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    operand: ValueIndex,
) -> Option<OpCode> {
    let inner = Unop { assigns, operand };
    match op {
        hashlink_bytecode::OpCode::OpNeg(_) => Some(OpCode::OpNeg(inner)),
        hashlink_bytecode::OpCode::OpNot(_) => Some(OpCode::OpNot(inner)),
        hashlink_bytecode::OpCode::OpIncr(_) => Some(OpCode::OpIncr(inner)),
        hashlink_bytecode::OpCode::OpDecr(_) => Some(OpCode::OpDecr(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpCall0(_) => Some(OpCode::OpCall(inner)),
        hashlink_bytecode::OpCode::OpCall1(_) => Some(OpCode::OpCall(inner)),
        hashlink_bytecode::OpCode::OpCall2(_) => Some(OpCode::OpCall(inner)),
        hashlink_bytecode::OpCode::OpCall3(_) => Some(OpCode::OpCall(inner)),
        hashlink_bytecode::OpCode::OpCall4(_) => Some(OpCode::OpCall(inner)),
        hashlink_bytecode::OpCode::OpCallN(_) => Some(OpCode::OpCall(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpCallMethod(_) => Some(OpCode::OpCallMethod(inner)),
        hashlink_bytecode::OpCode::OpCallThis(_) => Some(OpCode::OpCallMethod(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpCallClosure(_) => Some(OpCode::OpCallClosure(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpSwitch(_) => Some(OpCode::OpSwitch(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpField(_) => Some(OpCode::OpGetField(inner)),
        hashlink_bytecode::OpCode::OpGetThis(_) => Some(OpCode::OpGetField(inner)),
        hashlink_bytecode::OpCode::OpDynGet(_) => Some(OpCode::OpDynGet(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpSetField(_) => Some(OpCode::OpSetField(inner)),
        hashlink_bytecode::OpCode::OpSetThis(_) => Some(OpCode::OpSetField(inner)),
        hashlink_bytecode::OpCode::OpDynSet(_) => Some(OpCode::OpDynSet(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpJTrue(_) => Some(OpCode::OpJTrue(inner)),
        hashlink_bytecode::OpCode::OpJFalse(_) => Some(OpCode::OpJFalse(inner)),
        hashlink_bytecode::OpCode::OpJNull(_) => Some(OpCode::OpJNull(inner)),
        hashlink_bytecode::OpCode::OpJNotNull(_) => Some(OpCode::OpJNotNull(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    lhs: ValueIndex,
    rhs: ValueIndex,
) -> Option<OpCode> {
    let comp_fn = match op {
        hashlink_bytecode::OpCode::OpJSLt(_) => Some(ComparisonFn::SignedLT),
        hashlink_bytecode::OpCode::OpJSGte(_) => Some(ComparisonFn::SignedGTE),
        hashlink_bytecode::OpCode::OpJSGt(_) => Some(ComparisonFn::SignedGT),
        hashlink_bytecode::OpCode::OpJSLte(_) => Some(ComparisonFn::SignedLTE),
        hashlink_bytecode::OpCode::OpJULt(_) => Some(ComparisonFn::UnsignedLT),
        hashlink_bytecode::OpCode::OpJUGte(_) => Some(ComparisonFn::UnsignedGTE),
        hashlink_bytecode::OpCode::OpJNotLt(_) => Some(ComparisonFn::SignedGTE),
        hashlink_bytecode::OpCode::OpJNotGte(_) => Some(ComparisonFn::SignedLT),
        hashlink_bytecode::OpCode::OpJEq(_) => Some(ComparisonFn::Eq),
        hashlink_bytecode::OpCode::OpJNotEq(_) => Some(ComparisonFn::Ne),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    function: FunctionIndex,
) -> Option<OpCode> {
    let inner = StaticClosure { assigns, function };
    match op {
        hashlink_bytecode::OpCode::OpStaticClosure(_) => Some(OpCode::OpStaticClosure(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpInstanceClosure(_) => Some(OpCode::OpInstanceClosure(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpVirtualClosure(_) => Some(OpCode::OpVirtualClosure(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = Cast { assigns, source };
    match op {
        hashlink_bytecode::OpCode::OpToDyn(_) => Some(OpCode::OpToDyn(inner)),
        hashlink_bytecode::OpCode::OpToSFloat(_) => Some(OpCode::OpToSFloat(inner)),
        hashlink_bytecode::OpCode::OpToUFloat(_) => Some(OpCode::OpToUFloat(inner)),
        hashlink_bytecode::OpCode::OpToInt(_) => Some(OpCode::OpToInt(inner)),
        hashlink_bytecode::OpCode::OpSafeCast(_) => Some(OpCode::OpSafeCast(inner)),
        hashlink_bytecode::OpCode::OpUnsafeCast(_) => Some(OpCode::OpUnsafeCast(inner)),
        hashlink_bytecode::OpCode::OpToVirtual(_) => Some(OpCode::OpToVirtual(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpGetI8(_) => Some(OpCode::OpGetI8(inner)),
        hashlink_bytecode::OpCode::OpGetI16(_) => Some(OpCode::OpGetI16(inner)),
        hashlink_bytecode::OpCode::OpGetMem(_) => Some(OpCode::OpGetMem(inner)),
        hashlink_bytecode::OpCode::OpGetArray(_) => Some(OpCode::OpGetArray(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpSetI8(_) => Some(OpCode::OpSetI8(inner)),
        hashlink_bytecode::OpCode::OpSetI16(_) => Some(OpCode::OpSetI16(inner)),
        hashlink_bytecode::OpCode::OpSetMem(_) => Some(OpCode::OpSetMem(inner)),
        hashlink_bytecode::OpCode::OpSetArray(_) => Some(OpCode::OpSetArray(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpMakeEnum(_) => Some(OpCode::OpMakeEnum(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    constructor: ConstructorIndex,
) -> Option<OpCode> {
    let inner = AllocEnum {
        assigns,
        constructor,
    };
    match op {
        hashlink_bytecode::OpCode::OpEnumAlloc(_) => Some(OpCode::OpEnumAlloc(inner)),
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
pub fn translate_value_index(op: &hashlink_bytecode::OpCode, value: ValueIndex) -> Option<OpCode> {
    match op {
        hashlink_bytecode::OpCode::OpNull(_) => Some(OpCode::OpNull(value)),
        hashlink_bytecode::OpCode::OpRet(_) => Some(OpCode::OpRet(value)),
        hashlink_bytecode::OpCode::OpThrow(_) => Some(OpCode::OpThrow(value)),
        hashlink_bytecode::OpCode::OpRethrow(_) => Some(OpCode::OpRethrow(value)),
        hashlink_bytecode::OpCode::OpNullCheck(_) => Some(OpCode::OpNullCheck(value)),
        hashlink_bytecode::OpCode::OpNew(_) => Some(OpCode::OpNew(value)),
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
pub fn translate_trap(
    op: &hashlink_bytecode::OpCode,
    destination: BasicBlockIndex,
) -> Option<OpCode> {
    let inner = Trap { destination };
    match op {
        hashlink_bytecode::OpCode::OpTrap(_) => Some(OpCode::OpTrap(inner)),
        _ => None,
    }
}

/// Translate an end trap type HashLink opcode into the corresponding form in Eon, built from
/// the provided values
///
/// # Errors
///
/// Will return `None` if the source HashLink opcode is not the correct type of instruction for this
/// function
///
pub fn translate_end_trap(op: &hashlink_bytecode::OpCode, inner: bool) -> Option<OpCode> {
    match op {
        hashlink_bytecode::OpCode::OpEndTrap(_) => Some(OpCode::OpEndTrap(inner)),
        _ => None,
    }
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
    op: &hashlink_bytecode::OpCode,
    inner: BasicBlockIndex,
) -> Option<OpCode> {
    match op {
        hashlink_bytecode::OpCode::OpJAlways(_) => Some(OpCode::OpJAlways(inner)),
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
    op: &hashlink_bytecode::OpCode,
    assigns: ValueIndex,
    source: ValueIndex,
) -> Option<OpCode> {
    let inner = RefData { assigns, source };
    match op {
        hashlink_bytecode::OpCode::OpRefData(_) => Some(OpCode::OpRefData(inner)),
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
    op: &hashlink_bytecode::OpCode,
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
        hashlink_bytecode::OpCode::OpRefOffset(_) => Some(OpCode::OpRefOffset(inner)),
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
pub fn translate_no_params(op: &hashlink_bytecode::OpCode) -> Option<OpCode> {
    match op {
        hashlink_bytecode::OpCode::OpAssert => Some(OpCode::OpAssert),
        hashlink_bytecode::OpCode::OpNop => Some(OpCode::OpNop),
        hashlink_bytecode::OpCode::OpLabel => Some(OpCode::OpNop),
        _ => None,
    }
}
