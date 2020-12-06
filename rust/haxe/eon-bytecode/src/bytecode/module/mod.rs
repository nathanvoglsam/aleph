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

use crate::bytecode::constant::Constant;
use crate::bytecode::function::Function;
use crate::bytecode::indexes::{FunctionIndex, TypeIndex};
use crate::bytecode::native::Native;
use crate::bytecode::type_::Type;
use serde::{Deserialize, Serialize};

/// Set of all errors that can occur when transpiling from hashlink bytecode
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum TranspileError {
    /// This occurs when there is an error when translating the type definitions. Generally this
    /// error will never actually happen as it's not possible to encode an invalid type in the
    /// on-disk hashlink format but one could be made after being loaded from disk.
    InvalidType,

    /// This error occurs when transpiling a function from the hashlink module fails
    InvalidFunction,
}

pub type TranspileResult<T> = Result<T, TranspileError>;

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

    /// The file's byte offets table
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

impl Module {
    pub fn from_hashlink(code: hashlink_bytecode::Code) -> TranspileResult<Self> {
        // First we translate all the direct stuff
        //
        // The only thing we massively change is the actual function instructions as we move that
        // into SSA form.
        let mut module = Self {
            ints: code.ints,
            floats: code.floats,
            strings: code.strings,
            bytes: code.bytes,
            byte_offsets: code.byte_offsets,
            debug_files: code.debug_files,
            types: translate_types(code.types),
            natives: translate_natives(code.natives),
            globals: translate_globals(code.globals),
            functions: Vec::new(),
            constants: translate_constants(code.constants),
            entrypoint: FunctionIndex(code.entrypoint as usize),
        };

        // Now we do the fun part, we transpile the hashlink bytecode to our own bytecode form.
        //
        // We don't do any optimizations yet, we save that for later
        let mut functions = Vec::new();
        for f in code.functions.into_iter() {
            if let Some(new) = Function::transpile_hashlink(&module, f) {
                functions.push(new);
            } else {
                return Err(TranspileError::InvalidFunction);
            }
        }

        module.functions = functions;

        // Finally output our finished module
        Ok(module)
    }
}

fn translate_types(input: Vec<hashlink_bytecode::Type>) -> Vec<Type> {
    input.into_iter().map(Type::from).collect()
}

fn translate_natives(input: Vec<hashlink_bytecode::Native>) -> Vec<Native> {
    input.into_iter().map(Native::from).collect()
}

fn translate_globals(input: Vec<i32>) -> Vec<TypeIndex> {
    input.into_iter().map(|v| TypeIndex(v as usize)).collect()
}

fn translate_constants(input: Vec<hashlink_bytecode::Constant>) -> Vec<Constant> {
    input.into_iter().map(Constant::from).collect()
}
