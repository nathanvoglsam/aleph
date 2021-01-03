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

extern crate eon_bytecode;
extern crate hashlink_bytecode as hashlink;

mod basic_block_build;
mod basic_block_compute;
mod indexes;
mod utils;

#[cfg(test)]
mod tests;

pub mod error;
pub mod translation;

use crate::basic_block_build::build_bb;
use crate::basic_block_compute::compute_bb;
use crate::error::TranspileResult;
use crate::translation::function::FunctionTranslator;
use crate::translation::misc_translators::{
    translate_constants, translate_globals, translate_natives,
};
use crate::translation::type_translators::translate_types;
use eon_bytecode::{Function, FunctionIndex, Module};

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
pub fn translate_hashlink_module(code: hashlink::Code) -> TranspileResult<Module> {
    let callable_table = code.make_callable_table();

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
    for old_fn in code.functions.into_iter() {
        let new_fn = transpile_hashlink_function(&module, old_fn, &callable_table)?;
        functions.push(new_fn);
    }

    module.functions = functions;

    // Finally output our finished module
    Ok(module)
}

fn transpile_hashlink_function(
    module: &Module,
    mut old_fn: hashlink::Function,
    callable_table: &[hashlink::Callable],
) -> TranspileResult<Function> {
    // This is a very, very, very, very hacky thing we inject into the instruction stream of every
    // function we translate.
    //
    // An unconditional jump with an offset of 0 is a no-op as the execution semantics are exactly
    // equal to OpNop. This little hack serves one purpose, it guarantees that every function begins
    // with a basic block with no predecessors and ends with an unconditional jump.
    //
    // The purpose of this hack is to simplify our "mem2reg" implementation when dealing with
    // function arguments. HashLink implicitly uses the 0..n registers for 0..n function arguments.
    // There is no sane way to represent this kind of assignment semantics in the SSA graph we
    // calculate in the next step without forcing the entire algorithm to work around this one edge
    // case.
    //
    // The problem stems from the fact that it is invalid, in Eon, for the *entry* basic block to be
    // the target of a branch, as we can't encode phi instructions that import the values from the
    // function arguments without introducing more painful edge cases to the instruction encoding.
    // HashLink bytecode makes no guarantee, and actively has code generated, that violates this
    // guarantee of Eon bytecode (if we translated directly without this hack anyway).
    //
    // The solution, just add a no-op branch at the start of the function. This, essentially, just
    // explicitly encodes the edge of the execution graph that was otherwise only implicitly
    // represented. There is an implicit execution graph edge from the caller into the callee, which
    // is the execution edge that the function argument's SSA values are imported from. This empty
    // basic block will encode that edge explicitly so we don't need to handle any edge cases when
    // emitting phi instructions for branch target blocks (there's no sane way to handle having the
    // first instruction be a branch target otherwise).
    let noop_jump = hashlink::OpOneParam { param_1: 0 };
    let noop_jump = hashlink::OpCode::OpJAlways(noop_jump);
    old_fn.ops.insert(0, noop_jump);

    let spans = compute_bb(&old_fn)?;

    Ok(build_bb(spans, &old_fn, module, callable_table)?)
}

/// Attempt <large number> at translation with a far more robust algorithm
///
/// Takes ownership of the given HashLink module and translate it into a new Eon module.
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
pub fn translate_hashlink_module_2(code: hashlink::Code) -> TranspileResult<Module> {
    let callable_table = code.make_callable_table();

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

    let function_translator = FunctionTranslator::new(&module, &callable_table);
    for source in code.functions.into_iter() {
        let translated = function_translator.translate_function(source)?;
        functions.push(translated);
    }

    module.functions = functions;

    // Finally output our finished module
    Ok(module)
}
