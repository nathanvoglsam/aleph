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

//!
//! This module holds the definitions of the various driver functions that actually perform the
//! translation from HashLink into Eon
//!

use crate::basic_block_build::build_bb;
use crate::basic_block_graph::compute_bb_graph;
use crate::basic_block_spans::compute_bb_spans;
use crate::error::{TranspileError, TranspileResult};
use crate::translators::{
    translate_constants, translate_globals, translate_natives, translate_types,
};
use eon_bytecode::function::{Function, Metadata};
use eon_bytecode::indexes::{FunctionIndex, TypeIndex};
use eon_bytecode::module::Module;

///
/// Takes ownership of the given HashLink module and translate/transpile it into a new Eon module.
///
/// This is the primary point of contact for this library and, for the most part, should be the only
/// function you actually need to call. Everything else implements some aspect of the translation
/// algorithm that will be driven by this function.
///
/// # Information
///
/// The translation is pretty straight forward as Eon was designed to be very similar to the
/// HashLink VM's bytecode. Everything except the functions are pretty much translated verbatim,
/// only casting various values into more appropriate integer types (mostly just making indexes all
/// use `usize` instead of `i32`)
///
/// The "meat and potatoes" of the translation is in the algorithm for transpiling the function
/// opcodes from one format to the other. HashLink is a statically typed, register based VM. It maps
/// pretty close to plain C code, where each register corresponds to a local variable in the
/// function.
///
/// This is *very* different to Eon. Eon uses an SSA form representation of the function's values as
/// it was originally intended to be a intermediate representation of the HashLink bytecode while
/// compiling it to LLVM-IR. To translate the function we need to perform similar analysis to the
/// `mem2reg` optimization layer of LLVM to raise the lower level mutable register form into a valid
/// SSA graph.
///
/// We implement this at a higher level layer where the semantics of the code are much simpler and
/// so we'll almost certainly do this faster and more reliably than LLVM could've.
///
pub fn translate_hashlink_module(code: hashlink_bytecode::Code) -> TranspileResult<Module> {
    // First we translate all the direct stuff
    //
    // The only thing we massively change is the actual function instructions as we move that
    // into SSA form.
    let mut module = Module {
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
        if let Some(new) = transpile_hashlink_function(&module, f) {
            functions.push(new);
        } else {
            return Err(TranspileError::InvalidFunction);
        }
    }

    module.functions = functions;

    // Finally output our finished module
    Ok(module)
}

pub fn transpile_hashlink_function(
    module: &Module,
    f: hashlink_bytecode::Function,
) -> Option<Function> {
    let mut out = Function {
        type_: TypeIndex(f.type_ as usize),
        f_index: f.f_index,
        ssa_values: vec![],
        basic_blocks: vec![],
        metadata: Metadata {
            value_data: None,
            reg_data: None,
        },
    };

    // First we need to find all branch instructions and where they branch to
    let bb_graph = compute_bb_graph(&f)?;

    // Now we need to compute a list of spans for all the basic blocks in the bytecode
    let spans = compute_bb_spans(&f, &bb_graph)?;

    build_bb(&mut out, &module, &f, bb_graph, spans)?;

    Some(out)
}
