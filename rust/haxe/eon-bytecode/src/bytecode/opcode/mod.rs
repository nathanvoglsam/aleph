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

/// Layout for loading type instructions that load something into an SSA value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Load {
    /// SSA value to load into
    pub target: usize,

    /// What to load from, precise meaning depends on the exact opcode
    pub load: usize,
}

/// Layout for storing type instructions that store an SSA value into some destination
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Store {
    /// SSA value to store into target
    pub source: usize,

    /// Where to store to, precise meaning depends on the exact opcode
    pub target: usize,
}

/// Layout for the various binop arithmetic instructions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Binop {
    /// SSA value to store the result of the operation into
    pub target: usize,

    /// Op left hand side
    pub lhs: usize,

    /// Op right hand side
    pub rhs: usize,
}

/// Layout for the various unop arithmetic instructions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unop {
    /// SSA value to store the result of the operation into
    pub target: usize,

    /// The single operand for this operation
    pub operand: usize,
}

/// Layout for a function call. Our representation collapses HashLink's Call0, Call1, ..., etc into
/// a single representation as I don't see any benefit to this.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Call {
    /// SSA value to store the result of the operation into
    pub target: usize,

    /// The ID of the function to call
    pub function: usize,

    /// The list of function arguments
    pub fn_params: Vec<i32>,
}

/// Layout for the switch instruction.
///
/// HashLink's encoding represents a jump table where the instruction encodes a list of instruction
/// indexes to jump to based on the input register, which indexes into the table. If the index
/// register is out of bounds it jumps to the fallback instruction index.
///
/// Because we need to move to SSA form we change this instruction's meaning slightly. Instead of
/// jumping to an instruction index, we specify a table of *basic block* indexes to jump to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Switch {
    /// The SSA value to use as the index into the jump table
    pub input: usize,

    /// A list of basic block indexes to map the input to which basic block to jump to
    pub jump_table: Vec<usize>,

    /// The fallback basic block for if the input does not match any of our jump table entires
    pub fallback: usize,
}

/// Layout for a field load from an object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldLoad {
    /// The SSA value to store the result of the load into
    pub target: usize,

    /// The SSA value that holds the object to load the field from
    pub object: usize,

    /// The index of the field to load from
    pub field: usize,
}

/// Layout for a field store to an object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldStore {
    /// The SSA value that holds the object to store into
    pub object: usize,

    /// The field index on the object to store into
    pub field: usize,

    /// The SSA value to store into the field
    pub source: usize,
}

/// Layout for loading from `this`. A less general form of `FieldLoad` where `object` is implicitly
/// the first function parameter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThisFieldLoad {
    /// The SSA value to store the result of the load into
    pub target: usize,

    /// The index of the field to load from
    pub field: usize,
}

/// Layout for storing to `this`. A less general form of `FieldStore` where object is implicitly the
/// first function parameter
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThisFieldStore {
    /// The field index on the object to store into
    pub field: usize,

    /// The SSA value to store into the field
    pub source: usize,
}

/// Layout for a conditional branch where the comparison value is implicit to the opcode itself.
///
/// Once again, like with `Switch` we have to change the meaning slightly. HashLink specifies jumps
/// in relation to instruction indexes, we need to specify them in relation to basic blocks.
///
/// As such the destination will now refer to the basic block to jump to instead of the instruction
/// index.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CondBranch {
    /// Value to check
    pub check: usize,

    /// Basic block to jump to upon success
    pub destination: usize,
}

/// Layout for a comparison branch where we compare two provided values to decide whether to branch
/// or not.
///
/// This is a more general form of `CondBranch` where the value to compare against can also be
/// be provided.
///
/// See `CondBranch` docs for an explanation to how this differs from HashLink
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompBranch {
    pub lhs: usize,
    pub rhs: usize,
    pub destination: usize,
}

/// Layout for our phi instruction.
///
/// This is an opcode we add to the bytecode ourselves during the translation process.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Phi {
    /// A list of value pairs for loading specific values from other basic blocks when they branch
    /// into the basic block the phi instruction is in
    pub block_values: Vec<(usize, usize)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpCode {
    // Type and value initialization op codes
    OpMov(Load),
    OpInt(Load),
    OpFloat(Load),
    OpBool(Load),
    OpBytes(Load),
    OpString(Load),
    OpNull(usize),

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
    OpStaticClosure([i32; 2]),
    OpInstanceClosure([i32; 3]),
    OpVirtualClosure([i32; 3]),

    // Global getting and setting opcodes
    OpGetGlobal(Load),
    OpSetGlobal(Store),

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
    OpJAlways(usize),
    OpRet(usize),
    OpSwitch(Switch),
    OpPhi(Phi),

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

    OpNew(usize),
    OpArraySize([i32; 2]),
    OpType([i32; 2]),
    OpGetType([i32; 2]),
    OpGetTID([i32; 2]),

    // Reference opcodes
    OpRef([i32; 2]),
    OpUnref([i32; 2]),
    OpSetRef([i32; 2]),

    // Enum opcodes
    OpMakeEnum(Call),
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
