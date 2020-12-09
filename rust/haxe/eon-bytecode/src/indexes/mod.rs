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

use crate::module::Module;
use serde::{Deserialize, Serialize};

/// New-type for representing an index into the static "strings" table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct StringIndex(pub usize);

/// New-type for representing an index into the registers within a function. This is only used in
/// meta-data that is generated when transpiling from HashLink bytecode
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct RegisterIndex(pub usize);

/// New-type for representing an index that refers to a distinct SSA value within a function.
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct ValueIndex(pub usize);

impl ValueIndex {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} %{}", mnemonic, self.0)
    }
}

/// New-type for representing an index into the "globals" table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct GlobalIndex(pub usize);

/// New-type for representing an index into the "types" table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct TypeIndex(pub usize);

/// New-type for representing an index into the list of basic blocks inside a function
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct BasicBlockIndex(pub usize);

impl BasicBlockIndex {
    pub fn opcode_dump(&self, _: &Module, mnemonic: &str) -> String {
        format!("{} ${}", mnemonic, self.0)
    }
}

/// New-type for representing an index into the list of instructions within a function
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct InstructionIndex(pub usize);

/// New-type for representing a function index
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct FunctionIndex(pub usize);

/// New-type for representing a field index (indexes into list of fields on an object)
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct FieldIndex(pub usize);

/// New-type for an index into the integer table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct IntegerIndex(pub usize);

/// New-type for an index into the float table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct FloatIndex(pub usize);

/// New-type for an index into the bytes table
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct BytesIndex(pub usize);

/// New-type for an index into the list of constructors/variants associated with an enum
#[repr(transparent)]
#[derive(
    Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
pub struct ConstructorIndex(pub usize);
