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

use crate::bytecode::indexes::{StringIndex, TypeIndex};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Native {
    /// Index into the string table for library name
    pub lib: StringIndex,

    /// Index into the string table for the native name
    pub name: StringIndex,

    /// Index into the type table for the type
    pub type_: TypeIndex,

    /// ?
    pub f_index: usize,
}

impl From<hashlink_bytecode::Native> for Native {
    fn from(v: hashlink_bytecode::Native) -> Self {
        Self {
            lib: StringIndex(v.lib as usize),
            name: StringIndex(v.name as usize),
            type_: TypeIndex(v.type_ as usize),
            f_index: v.f_index as usize,
        }
    }
}
