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

use crate::bytecode::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, IntegerIndex, StringIndex, TypeIndex, ValueIndex,
};
use serde::{Deserialize, Serialize};

/// Layout for instructions that perform a load of some kind from a SSA value and assigns it to a
/// new SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Load {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The SSA value to read from from
    pub source: ValueIndex,
}

/// Layout for the `OpInt` instruction for initializing an SSA value from the integer table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadInt {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The integer in the integer table to load
    pub integer: IntegerIndex,
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadFloat {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The float in the float table to load
    pub float: FloatIndex,
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadBool {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The boolean value to store into the destination
    pub value: bool,
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadBytes {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The bytes index to use to load the bytes value from
    pub bytes: BytesIndex,
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadString {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The string index to use to load the string from
    pub string: StringIndex,
}

/// Layout for the `OpGetGlobal` instruction for loading values from the global table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadGlobal {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// Index into the global table to load from
    pub source: GlobalIndex,
}

/// Layout for the `OpType` instruction for loading values from the type table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadType {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// Index into the global table to load from
    pub source: TypeIndex,
}

/// Layout for `OpEnumField`. Loads a given field from an enum.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadEnumField {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The SSA value to read from from
    pub source: ValueIndex,

    /// The index of the constructor/variant of the enum to load the field from
    pub constructor: ConstructorIndex,

    /// The index of the field on the enum to load from
    pub field_index: FieldIndex,
}

/// Layout for `OpSetGlobal` for storing a value into a global
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct StoreGlobal {
    /// SSA value to store into target
    pub source: ValueIndex,

    /// The index into the global table to store source into
    pub target: GlobalIndex,
}

/// Layout for instructions that perform a store from one register into another
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Store {
    /// SSA value to store into target
    pub target: ValueIndex,

    /// The index of the value that should be stored through the reference. This is a *read* not
    /// a write
    pub source: ValueIndex,
}

/// Layout for the various binop arithmetic instructions
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Binop {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// Op left hand side
    pub lhs: ValueIndex,

    /// Op right hand side
    pub rhs: ValueIndex,
}

/// Layout for the various unop arithmetic instructions
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Unop {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The single operand for this operation
    pub operand: ValueIndex,
}

/// Layout for a function call. Our representation collapses HashLink's Call0, Call1, ..., etc into
/// a single representation as I don't see any benefit to this.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Call {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The ID of the function to call
    pub function: FunctionIndex,

    /// The list of function arguments
    pub fn_params: Vec<ValueIndex>,
}

/// Layout for a member method call.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct CallMethod {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The object that the method should be called on
    pub object: ValueIndex,

    /// The field that contains the function to call
    pub function: FieldIndex,

    /// The list of function arguments
    pub fn_params: Vec<ValueIndex>,
}

/// Layout for a closure call.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct CallClosure {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The value that contains the closure to call
    pub closure: ValueIndex,

    /// The list of function arguments
    pub fn_params: Vec<ValueIndex>,
}

/// Layout for the switch instruction.
///
/// HashLink's encoding represents a jump table where the instruction encodes a list of instruction
/// indexes to jump to based on the input register, which indexes into the table. If the index
/// register is out of bounds it just continues past the switch statement.
///
/// Because we need to move to SSA form we change this instruction's meaning slightly. Instead of
/// jumping to an instruction index, we specify a table of *basic block* indexes to jump to. And we
/// also need to specify the basic block we jump to explicitly as the fallback destination as a
/// branch always terminates a basic block
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Switch {
    /// The SSA value to use as the index into the jump table
    pub input: ValueIndex,

    /// A list of basic block indexes to map the input to which basic block to jump to
    pub jump_table: Vec<BasicBlockIndex>,

    /// The basic block to jump to if the read index is out of bounds of the jump table
    pub fallback: BasicBlockIndex,
}

/// Layout for a field load from an object
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct FieldLoad {
    /// The SSA value to store the result of the load into
    pub assigns: ValueIndex,

    /// The SSA value that holds the object to load the field from
    pub object: ValueIndex,

    /// The index of the field to load from
    pub field: FieldIndex,
}

/// Layout for a field store to an object
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct FieldStore {
    /// The SSA value that holds the object to store into
    pub object: ValueIndex,

    /// The field index on the object to store into
    pub field: FieldIndex,

    /// The SSA value to store into the field
    pub source: ValueIndex,
}

/// Layout for a conditional branch where the comparison value is implicit to the opcode itself.
///
/// Once again, like with `Switch` we have to change the meaning slightly. HashLink specifies jumps
/// in relation to instruction indexes, we need to specify them in relation to basic blocks.
///
/// As such the destination will now refer to the basic block to jump to instead of the instruction
/// index.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct CondBranch {
    /// Value to check
    pub check: ValueIndex,

    /// The basic block to jump to upon the check succeeding
    pub success: BasicBlockIndex,

    /// The basic block to jump to upon the check failing
    pub failure: BasicBlockIndex,
}

/// Layout for a comparison branch where we compare two provided values to decide whether to branch
/// or not.
///
/// This is a more general form of `CondBranch` where the value to compare against can also be
/// be provided.
///
/// See `CondBranch` docs for an explanation to how this differs from HashLink
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct CompBranch {
    /// Left hand side of comparison
    pub lhs: ValueIndex,

    /// Right hand side of comparison
    pub rhs: ValueIndex,

    /// The basic block to jump to upon the check succeeding
    pub success: BasicBlockIndex,

    /// The basic block to jump to upon the check failing
    pub failure: BasicBlockIndex,
}

/// Layout for our phi instruction.
///
/// This is an opcode we add to the bytecode ourselves during the translation process.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Phi {
    /// The value that the phi instruction assigns to
    pub assigns: ValueIndex,

    /// A list of value pairs for loading specific values from other basic blocks when they branch
    /// into the basic block the phi instruction is in
    pub block_values: Vec<(ValueIndex, BasicBlockIndex)>,
}

/// Layout for the `OpStaticClosure` instruction. This creates a "static" closure from the given
/// function. This essentially just creates a bare function pointer
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct StaticClosure {
    /// The SSA value to store the result of creating the static closure into
    pub assigns: ValueIndex,

    /// The index of the function to produce a static closure for
    pub function: FunctionIndex,
}

/// Layout for the `OpInstanceClosure` instruction. This creates a closure that carries an object
/// pointer with it which should be applied as the first argument when the closure is called.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct InstanceClosure {
    /// The SSA value to store the result of the instruction into
    pub assigns: ValueIndex,

    /// The index of the function to produce a closure for
    pub function: FunctionIndex,

    /// The object to store alongside the function pointer that should be applied as the first arg
    /// when the closure is called
    pub object: ValueIndex,
}

/// Layout for the `OpVirtualClosure` instruction.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct VirtualClosure {
    /// The SSA value to store the result of the instruction into
    pub assigns: ValueIndex,

    /// The object to store alongside the function pointer that should be applied as the first arg
    /// when the closure is called
    pub object: ValueIndex,

    /// The index of the field to load as a function to use for the closure
    pub field: FieldIndex,
}

/// Layout for the conversion instructions that convert one type to another
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Cast {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The SSA value to load from and convert into the target's type
    pub source: ValueIndex,
}

/// Layout used for the memory read style instructions
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ReadMemory {
    /// SSA value to read into
    pub assigns: ValueIndex,

    /// The value which is used as the source for reading memory from
    pub source: ValueIndex,

    /// An extra register which is used for applying an offset to read from
    pub offset: ValueIndex,
}

/// Layout used for the memory write style instructions
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct WriteMemory {
    /// The value which is used as the source for reading memory from
    pub target: ValueIndex,

    /// An extra register which is used for applying an offset to read from
    pub offset: ValueIndex,

    /// SSA value to store to memory
    pub source: ValueIndex,
}

/// Layout for `OpMakeEnum`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct MakeEnum {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The enum constructor/variant to build
    pub constructor: ConstructorIndex,

    /// The list of arguments
    pub args: Vec<ValueIndex>,
}

/// Layout for `OpEnumAlloc`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AllocEnum {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The enum constructor/variant to build
    pub constructor: ConstructorIndex,
}

/// Layout for `OpRefData`
///
/// The instruction essentially converts an array into a pointer to the first element of the array
/// + the type
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct RefData {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The array to make the reference from
    pub source: ValueIndex,
}

/// Layout for `OpRefOffset`
///
/// This is almost exactly the same as `OpRefData` except it also takes an extra register argument
/// which will hold an integer offset into the array which should be used as the base for the
/// reference to be created
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct RefOffset {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The array to make the reference from
    pub source: ValueIndex,

    /// The register which holds the offset index which should be used as the base element
    pub offset: ValueIndex,
}

/// Layout for `OpSetEnumField`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct StoreEnumField {
    /// The enum to store into
    pub target: ValueIndex,

    /// The field index to set which field to set
    pub field: FieldIndex,

    /// The register to set on the enum
    pub source: ValueIndex,
}

/// Layout for `OpSetEnumField`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Trap {
    /// The basic block index to jump to if an exception is thrown
    pub destination: BasicBlockIndex,
}

/// Layout for `OpReceiveException`
///
/// This instruction *must* be the first instruction of an exception handling basic block. It serves
/// to assign the SSA value that holds the exception value.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ReceiveException {
    /// The SSA value that should be assigned with the value of the exception if the handler block
    /// is jumped to
    pub assigns: ValueIndex,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum OpCode {
    // Type and value initialization op codes
    OpMov(Load),
    OpInt(LoadInt),
    OpFloat(LoadFloat),
    OpBool(LoadBool),
    OpBytes(LoadBytes),
    OpString(LoadString),
    OpNull(ValueIndex),

    // Arithmetic opcodes
    OpAdd(Binop),
    OpSub(Binop),
    OpMul(Binop),
    OpSDiv(Binop),
    OpUDiv(Binop),
    OpSMod(Binop),
    OpUMod(Binop),
    OpShl(Binop),
    OpSShr(Binop),
    OpUShr(Binop),
    OpAnd(Binop),
    OpOr(Binop),
    OpXor(Binop),
    OpNeg(Unop),
    OpNot(Unop),

    OpIncr(Unop),
    OpDecr(Unop),

    // Function calling opcodes
    OpCall(Call),
    OpCallMethod(CallMethod),
    OpCallClosure(CallClosure),

    // No idea what the specifics of these are, but I'm guessing allocate closures
    OpStaticClosure(StaticClosure),
    OpInstanceClosure(InstanceClosure),
    OpVirtualClosure(VirtualClosure),

    // Global getting and setting opcodes
    OpGetGlobal(LoadGlobal),
    OpSetGlobal(StoreGlobal),

    // Object field access
    OpGetField(FieldLoad),
    OpSetField(FieldStore),
    OpDynGet(FieldLoad),
    OpDynSet(FieldStore),

    // Branching opcodes
    OpJTrue(CondBranch),
    OpJFalse(CondBranch),
    OpJNull(CondBranch),
    OpJNotNull(CondBranch),
    OpJSLt(CompBranch),
    OpJSGte(CompBranch),
    OpJSGt(CompBranch),
    OpJSLte(CompBranch),
    OpJULt(CompBranch),
    OpJUGte(CompBranch),
    OpJNotLt(CompBranch),
    OpJNotGte(CompBranch),
    OpJEq(CompBranch),
    OpJNotEq(CompBranch),
    OpJAlways(BasicBlockIndex),
    OpRet(ValueIndex),
    OpSwitch(Switch),
    OpPhi(Phi),

    // Casting opcodes
    OpToDyn(Cast),
    OpToSFloat(Cast),
    OpToUFloat(Cast),
    OpToInt(Cast),

    // Coercions opcodes
    OpSafeCast(Cast),
    OpUnsafeCast(Cast),
    OpToVirtual(Cast),

    // Exception opcodes
    OpThrow(ValueIndex),
    OpRethrow(ValueIndex),
    OpTrap(Trap),
    OpEndTrap(bool),
    OpReceiveException(ReceiveException),
    OpNullCheck(ValueIndex),

    // Bytes section reading opcodes
    OpGetI8(ReadMemory),
    OpGetI16(ReadMemory),
    OpGetMem(ReadMemory),
    OpGetArray(ReadMemory),
    OpSetI8(WriteMemory),
    OpSetI16(WriteMemory),
    OpSetMem(WriteMemory),
    OpSetArray(WriteMemory),

    OpNew(ValueIndex),
    OpArraySize(Load),
    OpType(LoadType),
    OpGetType(Load),
    OpGetTID(Load),

    // Reference opcodes
    OpRef(Load),
    OpUnRef(Load),
    OpSetRef(Store),

    // Enum opcodes
    OpMakeEnum(MakeEnum),
    OpEnumAlloc(AllocEnum),
    OpEnumIndex(Load),
    OpEnumField(LoadEnumField),
    OpSetEnumField(StoreEnumField),

    // Not really sure at the moment
    OpAssert,
    OpRefData(RefData),
    OpRefOffset(RefOffset),

    // Noop
    OpNop,
}

impl OpCode {
    pub fn translate_load(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_load_int(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        integer: IntegerIndex,
    ) -> Option<Self> {
        let inner = LoadInt { assigns, integer };
        match op {
            hashlink_bytecode::OpCode::OpInt(_) => Some(OpCode::OpInt(inner)),
            _ => None,
        }
    }

    pub fn translate_load_float(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        float: FloatIndex,
    ) -> Option<Self> {
        let inner = LoadFloat { assigns, float };
        match op {
            hashlink_bytecode::OpCode::OpFloat(_) => Some(OpCode::OpFloat(inner)),
            _ => None,
        }
    }

    pub fn translate_load_bool(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        value: bool,
    ) -> Option<Self> {
        let inner = LoadBool { assigns, value };
        match op {
            hashlink_bytecode::OpCode::OpBool(_) => Some(OpCode::OpBool(inner)),
            _ => None,
        }
    }

    pub fn translate_load_bytes(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        bytes: BytesIndex,
    ) -> Option<Self> {
        let inner = LoadBytes { assigns, bytes };
        match op {
            hashlink_bytecode::OpCode::OpBytes(_) => Some(OpCode::OpBytes(inner)),
            _ => None,
        }
    }

    pub fn translate_load_string(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        string: StringIndex,
    ) -> Option<Self> {
        let inner = LoadString { assigns, string };
        match op {
            hashlink_bytecode::OpCode::OpString(_) => Some(OpCode::OpString(inner)),
            _ => None,
        }
    }

    pub fn translate_load_global(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: GlobalIndex,
    ) -> Option<Self> {
        let inner = LoadGlobal { assigns, source };
        match op {
            hashlink_bytecode::OpCode::OpGetGlobal(_) => Some(OpCode::OpGetGlobal(inner)),
            _ => None,
        }
    }

    pub fn translate_load_type(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: TypeIndex,
    ) -> Option<Self> {
        let inner = LoadType { assigns, source };
        match op {
            hashlink_bytecode::OpCode::OpType(_) => Some(OpCode::OpType(inner)),
            _ => None,
        }
    }

    pub fn translate_load_enum_field(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
        constructor: ConstructorIndex,
        field_index: FieldIndex,
    ) -> Option<Self> {
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

    pub fn translate_store_enum_field(
        op: &hashlink_bytecode::OpCode,
        target: ValueIndex,
        field: FieldIndex,
        source: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_store_global(
        op: &hashlink_bytecode::OpCode,
        source: ValueIndex,
        target: GlobalIndex,
    ) -> Option<Self> {
        let inner = StoreGlobal { source, target };
        match op {
            hashlink_bytecode::OpCode::OpSetGlobal(_) => Some(OpCode::OpSetGlobal(inner)),
            _ => None,
        }
    }

    pub fn translate_store(
        op: &hashlink_bytecode::OpCode,
        target: ValueIndex,
        source: ValueIndex,
    ) -> Option<Self> {
        let inner = Store { target, source };
        match op {
            hashlink_bytecode::OpCode::OpSetRef(_) => Some(OpCode::OpSetRef(inner)),
            _ => None,
        }
    }

    pub fn translate_binop(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        lhs: ValueIndex,
        rhs: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_unop(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        operand: ValueIndex,
    ) -> Option<Self> {
        let inner = Unop { assigns, operand };
        match op {
            hashlink_bytecode::OpCode::OpNeg(_) => Some(OpCode::OpNeg(inner)),
            hashlink_bytecode::OpCode::OpNot(_) => Some(OpCode::OpNot(inner)),
            hashlink_bytecode::OpCode::OpIncr(_) => Some(OpCode::OpIncr(inner)),
            hashlink_bytecode::OpCode::OpDecr(_) => Some(OpCode::OpDecr(inner)),
            _ => None,
        }
    }

    pub fn translate_call(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        function: FunctionIndex,
        fn_params: Vec<ValueIndex>,
    ) -> Option<Self> {
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

    pub fn translate_call_field(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        object: ValueIndex,
        function: FieldIndex,
        fn_params: Vec<ValueIndex>,
    ) -> Option<Self> {
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

    pub fn translate_call_closure(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        closure: ValueIndex,
        fn_params: Vec<ValueIndex>,
    ) -> Option<Self> {
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

    pub fn translate_switch(
        op: &hashlink_bytecode::OpCode,
        input: ValueIndex,
        jump_table: Vec<BasicBlockIndex>,
        fallback: BasicBlockIndex,
    ) -> Option<Self> {
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

    pub fn translate_field_load(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        object: ValueIndex,
        field: FieldIndex,
    ) -> Option<Self> {
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

    pub fn translate_field_store(
        op: &hashlink_bytecode::OpCode,
        object: ValueIndex,
        field: FieldIndex,
        source: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_cond_branch(
        op: &hashlink_bytecode::OpCode,
        check: ValueIndex,
        success: BasicBlockIndex,
        failure: BasicBlockIndex,
    ) -> Option<Self> {
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

    pub fn translate_comp_branch(
        op: &hashlink_bytecode::OpCode,
        lhs: ValueIndex,
        rhs: ValueIndex,
        success: BasicBlockIndex,
        failure: BasicBlockIndex,
    ) -> Option<Self> {
        let inner = CompBranch {
            lhs,
            success,
            failure,
            rhs,
        };
        match op {
            hashlink_bytecode::OpCode::OpJSLt(_) => Some(OpCode::OpJSLt(inner)),
            hashlink_bytecode::OpCode::OpJSGte(_) => Some(OpCode::OpJSGte(inner)),
            hashlink_bytecode::OpCode::OpJSGt(_) => Some(OpCode::OpJSGt(inner)),
            hashlink_bytecode::OpCode::OpJSLte(_) => Some(OpCode::OpJSLte(inner)),
            hashlink_bytecode::OpCode::OpJULt(_) => Some(OpCode::OpJULt(inner)),
            hashlink_bytecode::OpCode::OpJUGte(_) => Some(OpCode::OpJUGte(inner)),
            hashlink_bytecode::OpCode::OpJNotLt(_) => Some(OpCode::OpJNotLt(inner)),
            hashlink_bytecode::OpCode::OpJNotGte(_) => Some(OpCode::OpJNotGte(inner)),
            hashlink_bytecode::OpCode::OpJEq(_) => Some(OpCode::OpJEq(inner)),
            hashlink_bytecode::OpCode::OpJNotEq(_) => Some(OpCode::OpJNotEq(inner)),
            _ => None,
        }
    }

    pub fn translate_static_closure(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        function: FunctionIndex,
    ) -> Option<Self> {
        let inner = StaticClosure { assigns, function };
        match op {
            hashlink_bytecode::OpCode::OpStaticClosure(_) => Some(OpCode::OpStaticClosure(inner)),
            _ => None,
        }
    }

    pub fn translate_instance_closure(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        function: FunctionIndex,
        object: ValueIndex,
    ) -> Option<Self> {
        let inner = InstanceClosure {
            assigns,
            function,
            object,
        };
        match op {
            hashlink_bytecode::OpCode::OpInstanceClosure(_) => {
                Some(OpCode::OpInstanceClosure(inner))
            }
            _ => None,
        }
    }

    pub fn translate_virtual_closure(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        object: ValueIndex,
        field: FieldIndex,
    ) -> Option<Self> {
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

    pub fn translate_cast(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_read_memory(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
        offset: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_write_memory(
        op: &hashlink_bytecode::OpCode,
        target: ValueIndex,
        offset: ValueIndex,
        source: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_make_enum(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        constructor: ConstructorIndex,
        args: Vec<ValueIndex>,
    ) -> Option<Self> {
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

    pub fn translate_alloc_enum(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        constructor: ConstructorIndex,
    ) -> Option<Self> {
        let inner = AllocEnum {
            assigns,
            constructor,
        };
        match op {
            hashlink_bytecode::OpCode::OpEnumAlloc(_) => Some(OpCode::OpEnumAlloc(inner)),
            _ => None,
        }
    }

    pub fn translate_value_index(
        op: &hashlink_bytecode::OpCode,
        value: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_trap(
        op: &hashlink_bytecode::OpCode,
        destination: BasicBlockIndex,
    ) -> Option<Self> {
        let inner = Trap { destination };
        match op {
            hashlink_bytecode::OpCode::OpTrap(_) => Some(OpCode::OpTrap(inner)),
            _ => None,
        }
    }

    pub fn translate_end_trap(op: &hashlink_bytecode::OpCode, inner: bool) -> Option<Self> {
        match op {
            hashlink_bytecode::OpCode::OpEndTrap(_) => Some(OpCode::OpEndTrap(inner)),
            _ => None,
        }
    }

    pub fn translate_unconditional_branch(
        op: &hashlink_bytecode::OpCode,
        inner: BasicBlockIndex,
    ) -> Option<Self> {
        match op {
            hashlink_bytecode::OpCode::OpJAlways(_) => Some(OpCode::OpJAlways(inner)),
            _ => None,
        }
    }

    pub fn translate_ref_data(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
    ) -> Option<Self> {
        let inner = RefData { assigns, source };
        match op {
            hashlink_bytecode::OpCode::OpRefData(_) => Some(OpCode::OpRefData(inner)),
            _ => None,
        }
    }

    pub fn translate_ref_offset(
        op: &hashlink_bytecode::OpCode,
        assigns: ValueIndex,
        source: ValueIndex,
        offset: ValueIndex,
    ) -> Option<Self> {
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

    pub fn translate_no_params(op: &hashlink_bytecode::OpCode) -> Option<Self> {
        match op {
            hashlink_bytecode::OpCode::OpAssert => Some(OpCode::OpAssert),
            hashlink_bytecode::OpCode::OpNop => Some(OpCode::OpNop),
            hashlink_bytecode::OpCode::OpLabel => Some(OpCode::OpNop),
            _ => None,
        }
    }
}
