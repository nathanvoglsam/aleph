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

use crate::constant::Constant;
use crate::function::Function;
use crate::indexes::{FunctionIndex, TypeIndex};
use crate::native::Native;
use crate::type_::Type;
use serde::{Deserialize, Serialize};

pub mod dump;

/// This struct is a direct representation of a hashlink module *as read from disk*. The original C
/// hashlink code deserializes directly into the datastructures used by the JIT and runtime. This
/// implementation is completely distinct from any runtime and serves purely as a utility for
/// reading, operating on and writing hashlink modules so any information that is not read directly
/// from a hashlink file or is only used by the runtime is not stored here.
///
/// This struct can be used as a component for reading hashlink modules to be consumed by a JIT
/// runtime but is not appropriate to be consumed directly by the runtime.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    /// The file's integer table
    pub ints: Vec<i32>,

    /// The file's float table
    pub floats: Vec<f64>,

    /// The file's string table
    pub strings: Vec<String>,

    /// The file's bytes blob
    pub bytes: Vec<u8>,

    /// The file's byte offsets table
    pub byte_offsets: Vec<usize>,

    /// The file's debug file table
    pub debug_files: Vec<String>,

    /// The file's type table
    pub types: Vec<Type>,

    /// The file's natives table
    pub natives: Vec<Native>,

    /// The file's global table (list of indices into type table)
    pub globals: Vec<TypeIndex>,

    /// The file's function table
    pub functions: Vec<Function>,

    /// The file's constants table
    pub constants: Vec<Constant>,

    /// Index into the functions table for specifying which function is the entrypoint
    pub entrypoint: FunctionIndex,
}
