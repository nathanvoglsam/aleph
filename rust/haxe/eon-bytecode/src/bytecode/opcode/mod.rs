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
    pub source: ValueIndex,

    /// The index into the global table to store source into
    pub assigns: ValueIndex,
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

/// Layout for loading from `this`. A less general form of `FieldLoad` where `object` is implicitly
/// the first function parameter
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ThisFieldLoad {
    /// The SSA value to store the result of the load into
    pub assigns: ValueIndex,

    /// The index of the field to load from
    pub field: FieldIndex,
}

/// Layout for storing to `this`. A less general form of `FieldStore` where object is implicitly the
/// first function parameter
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct ThisFieldStore {
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
    OpCallMethod(Call),
    OpCallThis(Call),
    OpCallClosure(Call),

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
    OpGetThis(ThisFieldLoad),
    OpSetThis(ThisFieldStore),
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
    OpTrap([i32; 2]),
    OpEndTrap(bool),
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
    OpUnref(Load),
    OpSetRef(Store),

    // Enum opcodes
    OpMakeEnum(MakeEnum),
    OpEnumAlloc(AllocEnum),
    OpEnumIndex(Load),
    OpEnumField(LoadEnumField),
    OpSetEnumField([i32; 3]),

    // Not really sure at the moment
    OpAssert,
    OpRefData([i32; 2]),
    OpRefOffset([i32; 3]),

    // Noop
    OpNop,
}
