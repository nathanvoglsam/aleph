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

use crate::indexes::TypeIndex;
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};

/// This struct maps very directly to a "register" in terms of the raw HashLink bytecode. We hold
/// on to the information the "registers" provide because it makes some analysis passes easier as we
/// don't need to reconstruct this information from the SSA graph every time we need it
#[derive(Clone, Debug, Hash, Default, Serialize, Deserialize)]
pub struct Register {
    /// Does the allocated value outlive the function. Used for optimizing allocations.
    ///
    /// This only really has meaning for allocated types. Value types like plain integers and floats
    /// will never outlive a function as they don't have the concept of a lifetime. Value types are
    /// always copied when they assign to something else so they will only ever live as long as the
    /// scope they are defined in.
    ///
    /// An allocated type (something created with `new`) can have the lifetime extended beyond the
    /// scope it was created in by passing the pointer around. The pointer itself is a value type
    /// but what it points to will always be alive as long as a pointer to it exists.
    ///
    /// We can do some analysis to decide if the allocated object will outlive the function it was
    /// created in so we leave a spot here to fill the information in later.
    pub outlives_function: Option<bool>,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct SSAValue {
    /// The type this ssa value holds
    pub type_: TypeIndex,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct BasicBlock {
    /// This is just a flat, sequential list of opcodes
    pub ops: Vec<OpCode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    /// Index into the type table for the type signature of this function
    pub type_: TypeIndex,

    /// ?
    pub f_index: i32,

    /// This is the list of SSA values that get referred to by the
    pub ssa_values: Vec<SSAValue>,

    /// The list of basic blocks within the function
    pub basic_blocks: Vec<BasicBlock>,
}
