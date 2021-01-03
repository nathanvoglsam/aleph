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

use crate::error::TranspileResult;

pub struct FunctionTranslator<'module, 'callable_table> {
    pub module: &'module eon::Module,
    pub callable_table: &'callable_table [hashlink::Callable],
}

impl<'module, 'callable_table> FunctionTranslator<'module, 'callable_table> {
    pub fn new(
        module: &'module eon::Module,
        callable_table: &'callable_table [hashlink::Callable],
    ) -> Self {
        Self {
            module,
            callable_table,
        }
    }

    pub fn translate_function(
        &self,
        mut source: hashlink::Function,
    ) -> TranspileResult<eon::Function> {
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
        source.ops.insert(0, noop_jump);

        Ok(eon::Function::default())
    }
}
