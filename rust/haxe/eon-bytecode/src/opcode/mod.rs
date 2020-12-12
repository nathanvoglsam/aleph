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

use crate::indexes::{
    BasicBlockIndex, BytesIndex, ConstructorIndex, FieldIndex, FloatIndex, FunctionIndex,
    GlobalIndex, IntegerIndex, StringIndex, TypeIndex, ValueIndex,
};
use crate::module::Module;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Layout for instructions that perform a load of some kind from a SSA value and assigns it to a
/// new SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Load {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The SSA value to read from from
    pub source: ValueIndex,
}

impl Load {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} %{}", mnemonic, self.assigns.0, self.source.0)
    }
}

/// Layout for the `OpInt` instruction for initializing an SSA value from the integer table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadInt {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The integer in the integer table to load
    pub integer: IntegerIndex,
}

impl LoadInt {
    pub fn opcode_dump(&self, module: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} {}",
            mnemonic, self.assigns.0, module.ints[self.integer.0]
        )
    }
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadFloat {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The float in the float table to load
    pub float: FloatIndex,
}

impl LoadFloat {
    pub fn opcode_dump(&self, module: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} {}",
            mnemonic, self.assigns.0, module.floats[self.float.0]
        )
    }
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadBool {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The boolean value to store into the destination
    pub value: bool,
}

impl LoadBool {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} {}", mnemonic, self.assigns.0, self.value)
    }
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadBytes {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The bytes index to use to load the bytes value from
    pub bytes: BytesIndex,
}

impl LoadBytes {
    pub fn opcode_dump(&self, module: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} offset[{}]",
            mnemonic, self.assigns.0, module.byte_offsets[self.bytes.0]
        )
    }
}

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadString {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The string index to use to load the string from
    pub string: StringIndex,
}

impl LoadString {
    pub fn opcode_dump(&self, module: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} \"{}\"",
            mnemonic, self.assigns.0, &module.strings[self.string.0]
        )
    }
}

/// Layout for the `OpGetGlobal` instruction for loading values from the global table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadGlobal {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// Index into the global table to load from
    pub source: GlobalIndex,
}

impl LoadGlobal {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} global[{}]", mnemonic, self.assigns.0, self.source.0)
    }
}

/// Layout for the `OpType` instruction for loading values from the type table
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct LoadType {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// Index into the global table to load from
    pub source: TypeIndex,
}

impl LoadType {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} type[{}]", mnemonic, self.assigns.0, self.source.0)
    }
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

impl LoadEnumField {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{}.c_{}.f_{}",
            mnemonic, self.assigns.0, self.source.0, self.constructor.0, self.field_index.0
        )
    }
}

/// Layout for `OpSetGlobal` for storing a value into a global
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct StoreGlobal {
    /// The index into the global table to store source into
    pub target: GlobalIndex,

    /// SSA value to store into target
    pub source: ValueIndex,
}

impl StoreGlobal {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} global[{}] %{}", mnemonic, self.target.0, self.source.0)
    }
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

impl Store {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} %{}", mnemonic, self.target.0, self.source.0)
    }
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

impl Binop {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{} %{}",
            mnemonic, self.assigns.0, self.lhs.0, self.rhs.0
        )
    }
}

/// Layout for the various unop arithmetic instructions
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Unop {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The single operand for this operation
    pub operand: ValueIndex,
}

impl Unop {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} %{}", mnemonic, self.assigns.0, self.operand.0)
    }
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

impl Call {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut args = String::new();
        for p in &self.fn_params {
            write!(&mut args, "%{}, ", p.0).unwrap();
        }
        format!(
            "{} %{} fn[{}]({})",
            mnemonic, self.assigns.0, self.function.0, args
        )
    }
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

impl CallMethod {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut args = String::new();
        for p in &self.fn_params {
            write!(&mut args, "%{}, ", p.0).unwrap();
        }
        format!(
            "{} %{} %{} fn[{}]({})",
            mnemonic, self.assigns.0, self.object.0, self.function.0, args
        )
    }
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

impl CallClosure {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut args = String::new();
        for p in &self.fn_params {
            write!(&mut args, "%{}, ", p.0).unwrap();
        }
        format!(
            "{} %{} %{}({})",
            mnemonic, self.assigns.0, self.closure.0, args
        )
    }
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

impl Switch {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut table = String::new();
        for p in &self.jump_table {
            write!(&mut table, "${}, ", p.0).unwrap();
        }
        format!(
            "{} %{} [{}] ${}",
            mnemonic, self.input.0, table, self.fallback.0
        )
    }
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

impl FieldLoad {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{}.f_{}",
            mnemonic, self.assigns.0, self.object.0, self.field.0
        )
    }
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

impl FieldStore {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{}.f_{} %{}",
            mnemonic, self.object.0, self.field.0, self.source.0
        )
    }
}

/// Represents the supported set of comparison types for a conditional branch/comparison
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum ComparisonFn {
    SignedLT,
    SignedGT,
    SignedGTE,
    SignedLTE,
    UnsignedLT,
    UnsignedGT,
    UnsignedLTE,
    UnsignedGTE,
    Eq,
    Ne,
}

/// Layout for a comparison instruction.
///
/// This instruction compares to values and assigns the given value with the result of the
/// comparison.
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Comparison {
    /// The type of comparison to perform on the two values
    pub comp_fn: ComparisonFn,

    /// The SSA value assigned with the result of the comparison
    pub assigns: ValueIndex,

    /// The left hand side of the comparison
    pub lhs: ValueIndex,

    /// The right hand side of the comparison
    pub rhs: ValueIndex,
}

impl Comparison {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let text = match self.comp_fn {
            ComparisonFn::SignedLT => "slt",
            ComparisonFn::SignedGT => "sgt",
            ComparisonFn::SignedGTE => "sge",
            ComparisonFn::SignedLTE => "sle",
            ComparisonFn::UnsignedLT => "ult",
            ComparisonFn::UnsignedGT => "ugt",
            ComparisonFn::UnsignedLTE => "ule",
            ComparisonFn::UnsignedGTE => "uge",
            ComparisonFn::Eq => "eq",
            ComparisonFn::Ne => "ne",
        };
        format!(
            "{} {} %{} %{} %{}",
            mnemonic, text, self.assigns.0, self.lhs.0, self.rhs.0
        )
    }
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

impl CondBranch {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} ${} ${}",
            mnemonic, self.check.0, self.success.0, self.failure.0
        )
    }
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

impl Phi {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut table = String::new();
        for (v, b) in &self.block_values {
            write!(&mut table, "[%{} ${}], ", v.0, b.0).unwrap();
        }
        format!("{} %{} {}", mnemonic, self.assigns.0, table)
    }
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

impl StaticClosure {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} fn[{}]", mnemonic, self.assigns.0, self.function.0)
    }
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

impl InstanceClosure {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} fn[{}] %{}",
            mnemonic, self.assigns.0, self.function.0, self.object.0
        )
    }
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

impl VirtualClosure {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{}.f_{}",
            mnemonic, self.assigns.0, self.object.0, self.field.0
        )
    }
}

/// Layout for the conversion instructions that convert one type to another
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Cast {
    /// SSA value to move into
    pub assigns: ValueIndex,

    /// The SSA value to load from and convert into the target's type
    pub source: ValueIndex,
}

impl Cast {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} %{}", mnemonic, self.assigns.0, self.source.0)
    }
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

impl ReadMemory {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{}[%{}]",
            mnemonic, self.assigns.0, self.source.0, self.offset.0
        )
    }
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

impl WriteMemory {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{}[%{}] %{}",
            mnemonic, self.target.0, self.offset.0, self.source.0
        )
    }
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

impl MakeEnum {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        let mut table = String::new();
        for a in &self.args {
            write!(&mut table, "%{}, ", a.0).unwrap();
        }
        format!(
            "{} %{} c_{}({})",
            mnemonic, self.assigns.0, self.constructor.0, table
        )
    }
}

/// Layout for `OpEnumAlloc`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct AllocEnum {
    /// SSA value to store the result of the operation into
    pub assigns: ValueIndex,

    /// The enum constructor/variant to build
    pub constructor: ConstructorIndex,
}

impl AllocEnum {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} c_{}", mnemonic, self.assigns.0, self.constructor.0)
    }
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

impl RefData {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{} %{}", mnemonic, self.assigns.0, self.source.0)
    }
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

impl RefOffset {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{} %{}[%{}]",
            mnemonic, self.assigns.0, self.source.0, self.offset.0
        )
    }
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

impl StoreEnumField {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!(
            "{} %{}.f_{} %{}",
            mnemonic, self.target.0, self.field.0, self.source.0
        )
    }
}

/// Layout for `OpSetEnumField`
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Trap {
    /// The basic block index to jump to if an exception is thrown
    pub destination: BasicBlockIndex,
}

impl Trap {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} ${}", mnemonic, self.destination.0)
    }
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

impl ReceiveException {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{}", mnemonic, self.assigns.0)
    }
}

fn bool_opcode_dump(v: &bool, _: &Module, mnemonic: &str) -> String {
    format!("{} {}", mnemonic, v)
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

    // Comparison opcode
    OpCmp(Comparison),

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
    pub fn opcode_dump(&self, module: &Module) -> String {
        match self {
            OpCode::OpMov(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpInt(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpFloat(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpBool(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpBytes(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpString(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNull(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpAdd(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSub(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpMul(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSDiv(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpUDiv(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSMod(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpUMod(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpShl(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSShr(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpUShr(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpAnd(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpOr(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpXor(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNeg(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNot(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpIncr(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpDecr(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpCall(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpCallMethod(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpCallClosure(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpStaticClosure(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpInstanceClosure(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpVirtualClosure(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetGlobal(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetGlobal(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetField(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetField(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpDynGet(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpDynSet(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpJTrue(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpJFalse(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpJNull(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpJNotNull(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpCmp(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpJAlways(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpRet(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSwitch(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpPhi(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpToDyn(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpToSFloat(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpToUFloat(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpToInt(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSafeCast(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpUnsafeCast(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpToVirtual(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpThrow(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpRethrow(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpTrap(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpEndTrap(v) => bool_opcode_dump(v, module, self.get_mnemonic()),
            OpCode::OpReceiveException(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNullCheck(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetI8(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetI16(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetMem(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetArray(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetI8(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetI16(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetMem(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetArray(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNew(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpArraySize(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpType(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetType(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpGetTID(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpRef(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpUnRef(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetRef(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpMakeEnum(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpEnumAlloc(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpEnumIndex(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpEnumField(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpSetEnumField(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpAssert => self.get_mnemonic().to_string(),
            OpCode::OpRefData(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpRefOffset(v) => v.opcode_dump(module, self.get_mnemonic()),
            OpCode::OpNop => self.get_mnemonic().to_string(),
        }
    }

    pub fn get_mnemonic(&self) -> &'static str {
        match self {
            OpCode::OpMov(_) => "mov",
            OpCode::OpInt(_) => "int",
            OpCode::OpFloat(_) => "float",
            OpCode::OpBool(_) => "bool",
            OpCode::OpBytes(_) => "bytes",
            OpCode::OpString(_) => "string",
            OpCode::OpNull(_) => "null",
            OpCode::OpAdd(_) => "add",
            OpCode::OpSub(_) => "sub",
            OpCode::OpMul(_) => "mul",
            OpCode::OpSDiv(_) => "sdiv",
            OpCode::OpUDiv(_) => "udiv",
            OpCode::OpSMod(_) => "smod",
            OpCode::OpUMod(_) => "umod",
            OpCode::OpShl(_) => "shl",
            OpCode::OpSShr(_) => "sshr",
            OpCode::OpUShr(_) => "ushr",
            OpCode::OpAnd(_) => "and",
            OpCode::OpOr(_) => "or",
            OpCode::OpXor(_) => "xor",
            OpCode::OpNeg(_) => "neg",
            OpCode::OpNot(_) => "not",
            OpCode::OpIncr(_) => "incr",
            OpCode::OpDecr(_) => "decr",
            OpCode::OpCall(_) => "call",
            OpCode::OpCallMethod(_) => "call_method",
            OpCode::OpCallClosure(_) => "call_closure",
            OpCode::OpStaticClosure(_) => "static_closure",
            OpCode::OpInstanceClosure(_) => "instance_closure",
            OpCode::OpVirtualClosure(_) => "virtual_closure",
            OpCode::OpGetGlobal(_) => "get_global",
            OpCode::OpSetGlobal(_) => "set_global",
            OpCode::OpGetField(_) => "get_field",
            OpCode::OpSetField(_) => "set_field",
            OpCode::OpDynGet(_) => "dyn_get",
            OpCode::OpDynSet(_) => "dyn_set",
            OpCode::OpJTrue(_) => "j_true",
            OpCode::OpJFalse(_) => "j_false",
            OpCode::OpJNull(_) => "j_null",
            OpCode::OpJNotNull(_) => "j_not_null",
            OpCode::OpCmp(_) => "cmp",
            OpCode::OpJAlways(_) => "j_always",
            OpCode::OpRet(_) => "ret",
            OpCode::OpSwitch(_) => "switch",
            OpCode::OpPhi(_) => "phi",
            OpCode::OpToDyn(_) => "to_dyn",
            OpCode::OpToSFloat(_) => "to_sfloat",
            OpCode::OpToUFloat(_) => "to_ufloat",
            OpCode::OpToInt(_) => "to_int",
            OpCode::OpSafeCast(_) => "safe_cast",
            OpCode::OpUnsafeCast(_) => "unsafe_cast",
            OpCode::OpToVirtual(_) => "to_virtual",
            OpCode::OpThrow(_) => "throw",
            OpCode::OpRethrow(_) => "rethrow",
            OpCode::OpTrap(_) => "trap",
            OpCode::OpEndTrap(_) => "end_trap",
            OpCode::OpReceiveException(_) => "receive_exception",
            OpCode::OpNullCheck(_) => "null_check",
            OpCode::OpGetI8(_) => "get_i8",
            OpCode::OpGetI16(_) => "get_i16",
            OpCode::OpGetMem(_) => "get_mem",
            OpCode::OpGetArray(_) => "get_array",
            OpCode::OpSetI8(_) => "set_i8",
            OpCode::OpSetI16(_) => "set_i16",
            OpCode::OpSetMem(_) => "set_mem",
            OpCode::OpSetArray(_) => "set_array",
            OpCode::OpNew(_) => "new",
            OpCode::OpArraySize(_) => "array_size",
            OpCode::OpType(_) => "type",
            OpCode::OpGetType(_) => "get_type",
            OpCode::OpGetTID(_) => "get_tid",
            OpCode::OpRef(_) => "ref",
            OpCode::OpUnRef(_) => "un_ref",
            OpCode::OpSetRef(_) => "set_ref",
            OpCode::OpMakeEnum(_) => "make_enum",
            OpCode::OpEnumAlloc(_) => "enum_alloc",
            OpCode::OpEnumIndex(_) => "enum_index",
            OpCode::OpEnumField(_) => "enum_field",
            OpCode::OpSetEnumField(_) => "set_enum_field",
            OpCode::OpAssert => "assert",
            OpCode::OpRefData(_) => "ref_data",
            OpCode::OpRefOffset(_) => "ref_offset",
            OpCode::OpNop => "nop",
        }
    }
}
