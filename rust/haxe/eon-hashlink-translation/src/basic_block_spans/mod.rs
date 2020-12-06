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

use crate::basic_block_graph::BasicBlockGraph;
use eon_bytecode::indexes::InstructionIndex;

pub fn compute_bb_spans(
    old_fn: &hashlink_bytecode::Function,
    bb_graph: &BasicBlockGraph,
) -> Option<Vec<(InstructionIndex, InstructionIndex)>> {
    // Now we need to compute a list of spans for all the basic blocks in the bytecode
    let mut spans = Vec::new();

    // Go over all the points that are branched to and find the next branch so we can produce a
    // list of spans for all the basic blocks
    for start in bb_graph.destination_sources.keys().map(|v| *v) {
        let start = start.0;
        let mut found_branch = false;
        for (i, op) in old_fn.ops[start..].iter().enumerate() {
            if is_block_terminator(op) {
                let a = InstructionIndex(start);
                let b = InstructionIndex(start + i);
                let span = (a, b);
                spans.push(span);
                found_branch = true;
                break;
            }
        }
        if !found_branch {
            return None;
        }
    }

    // There's no guarantee that the first instruction is in the above list so we have to handle
    // the first instruction separately if it isn't. The first instruction is an implicit
    // beginning of a basic block.
    if !bb_graph
        .destination_sources
        .contains_key(&InstructionIndex(0))
    {
        let mut found_branch = false;
        for (i, op) in old_fn.ops.iter().enumerate() {
            if is_block_terminator(op) {
                let a = InstructionIndex(0);
                let b = InstructionIndex(i);
                let span = (a, b);
                spans.push(span);
                found_branch = true;
                break;
            }
        }
        if !found_branch {
            return None;
        }
    }

    Some(spans)
}

fn is_block_terminator(op: &hashlink_bytecode::OpCode) -> bool {
    op.is_branch() || op.is_ret() || op.is_throw()
}
