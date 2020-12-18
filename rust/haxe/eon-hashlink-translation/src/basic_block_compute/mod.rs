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

mod opcode_branch_target_iter;

use crate::basic_block_compute::opcode_branch_target_iter::OpCodeBranchTargetIter;
use crate::error::TranspileResult;
use crate::utils::{is_begin_trap, is_block_terminator};
use eon_bytecode::indexes::{BasicBlockIndex, InstructionIndex};
use std::collections::{HashMap, HashSet};

/// Represents a range of instructions in a function. This span is inclusive, where the range should
/// represent to a slice of instructions that starts with `begin` and ends with `end` inclusively.
///
/// These instruction spans should map directly to an identified basic block.
///
/// Extra information about the basic block is held alongside the range. Currently that includes:
///
/// - Trap handler instruction index
pub struct InstructionSpan {
    /// The beginning of the instruction span's range
    pub begin: InstructionIndex,

    /// The final instruction of the span's range
    /// (this is an inclusive range, i.e `ops[begin..=end]`)
    pub end: InstructionIndex,

    /// This holds the instruction index of where this basic block should jump to if an exception
    /// is thrown inside it.
    ///
    /// `None` means this basic block does not catch exceptions (i.e is not inside a try/catch)
    ///
    /// This is important for the opcode translation stage as we need to know how to translate
    /// `Call` instructions (emit a regular `Call` or an `Invoke`).
    pub trap_handler: Option<InstructionIndex>,
}

/// This holds all useful information computed by basic block identification algorithm for use
/// later
pub struct BasicBlockSpans {
    /// Maps an instruction index to the set of instructions that branch to it
    pub destination_sources: HashMap<InstructionIndex, HashSet<InstructionIndex>>,

    /// A list of all of the branch instruction indexes in the function
    pub branches: HashSet<InstructionIndex>,

    /// The final output list of basic block spans
    pub spans: Vec<InstructionSpan>,
}

impl BasicBlockSpans {
    /// Find the span that holds the given instruction index
    pub fn find_source_span(&self, i: InstructionIndex) -> Option<BasicBlockIndex> {
        self.spans
            .iter()
            .enumerate()
            .find(|(_, v)| v.begin.0 <= i.0 && v.end.0 >= i.0)
            .map(|(i, _)| BasicBlockIndex(i))
    }

    /// This function will return the set of immediate successors. That is, it will return a set
    /// that contains all blocks that are jumped directly to from the given basic block.
    ///
    /// This essentially returns all outward edges of the given "node" in the basic block graph.
    pub fn get_block_immediate_successor_set(
        &self,
        func: &hashlink::Function,
        block: BasicBlockIndex,
    ) -> HashSet<BasicBlockIndex> {
        let span = &self.spans[block.0];

        let last_op_index = span.end.0;
        let last_op = &func.ops[last_op_index];

        let mut successor_set: HashSet<BasicBlockIndex> = HashSet::new();

        // When a trap handler is in scope we need to handle `OpThrow` and `OpCall` in a special
        // way
        if let Some(handler) = span.trap_handler {
            // OpCall when inside a trap handler will terminate a block with two potential targets,
            // it will either jump to what was originally the next instruction, or to the trap
            // handler's block
            if last_op.is_call() {
                let successor = last_op_index + 1;
                let successor = self.find_source_span(successor.into()).unwrap();

                let exceptional = handler.0;
                let exceptional = self.find_source_span(exceptional.into()).unwrap();

                successor_set.insert(successor.into());
                successor_set.insert(exceptional.into());

                return successor_set;
            }

            // When directly inside a trap handler an `OpThrow` is for all intents and purposes a
            // fancy unconditional jump and so we treat it as such
            if last_op.is_throw() {
                let exceptional = handler.0;
                let exceptional = self.find_source_span(exceptional.into()).unwrap();
                successor_set.insert(exceptional.into());

                return successor_set;
            }
        }

        // If the above set of checks fail then we just use our good friend `OpCodeBranchTargetIter`
        // to iterate over all immediate successor branches and add them to the set.
        if let Some(targets) = OpCodeBranchTargetIter::new(func, last_op_index) {
            for target in targets {
                let target = target.unwrap();
                let target = self.find_source_span(target).unwrap();
                successor_set.insert(target.into());
            }
        }

        successor_set
    }

    /// This function returns the set of all basic blocks that are reachable from the input block,
    /// either immediately or recursively through any successor.
    ///
    /// The result of the function is essentially a "flood fill" of all basic blocks that can be
    /// reached from the given basic block.
    ///
    /// The results also implicitly form a second set which is defined by all blocks that are not in
    /// the returned data structure. This "conceptual" second set, which is the 'set difference' of
    /// the return value with the set of all basic blocks in the function, represents the set of all
    /// basic blocks which can *not* be reached from the given source block.
    ///
    /// It is possible to identify a graph with cycles by checking whether the resulting set of this
    /// function contains the source block itself. If it does, then there are cycles as there must
    /// exist a path `source -> n -> ... -> source` for the original block to be within its own
    /// successor set.
    pub fn get_block_full_successor_set(
        &self,
        func: &hashlink::Function,
        block: BasicBlockIndex,
    ) -> HashSet<BasicBlockIndex> {
        // This is the final output set we'll accumulate into
        let mut successor_set = HashSet::new();

        // This is our queue of blocks we'll work though until empty
        let mut next_stack = Vec::new();

        // We start with our initial block, obviously
        next_stack.push(block);

        // Keep popping elements off the queue and handling them until the queue is empty
        while let Some(current) = next_stack.pop() {
            // Get the current block's immediate successors and add them to the set we're
            // accumulating
            let immediate_successors = self.get_block_immediate_successor_set(func, current);

            // Now we need to add those successors to the queue, while also adding them to the set
            // we're building
            for successor in immediate_successors {
                // If the successor set already contains the thing we're trying to queue then we
                // shouldn't try and process this a second time. This should correctly terminate
                // any cyclic graphs that would cause this function to otherwise loop infinitely and
                // correctly produce the desired set
                if successor_set.insert(successor) {
                    next_stack.push(successor);
                }
            }
        }

        successor_set
    }
}

struct NextItem {
    pub start: usize,
    pub trap_stack: Vec<InstructionIndex>,
}

///
///
///
pub fn compute_bb(old_fn: &hashlink::Function) -> TranspileResult<BasicBlockSpans> {
    // This stack holds the queue of instruction indices to be handled. This is the core part of
    // our recursive tree walk of the source instruction stream
    let mut next_stack = Vec::new();

    // This holds the set of all branch targets that have already been handled and so should not
    // be handled a second time
    let mut handled_targets = HashSet::new();

    // This holds the current state of the "trap stack". HashLink bytecode uses nested trap
    // sections to encode try/catch blocks. This is very, very incompatible with an SSA graph. As
    // such we need to be able to translate this nesting into a valid SSA graph.
    //
    // Thankfully we can use this nesting information in our algorithm for translating the HashLink
    // trap/endtrap into eon's SSA graph.
    //
    // This stack represents the nesting of trap handlers which we send off to later stages of the
    // translation to flatten into our Invoke+ReceiveException which maps much closer to LLVM.
    //let mut trap_stack = Vec::new();

    // The output of this function
    let mut out = BasicBlockSpans {
        destination_sources: HashMap::new(),
        branches: HashSet::new(),
        spans: Vec::new(),
    };

    // Insert the first instruction into the queue, this is always where we should begin
    let next = NextItem {
        start: 0,
        trap_stack: Vec::new(),
    };
    next_stack.push(next);

    // Continuously loop until the queue is empty
    while let Some(current) = next_stack.pop() {
        // Skip already handled branch targets
        if handled_targets.contains(&current.start) {
            continue;
        }

        // Iterate over all instructions beginning from the start index until we either hit a block
        // terminator or hit the end of the function
        for (i, op) in old_fn.ops[current.start..].iter().enumerate() {
            // Identify an `OpTrap` and get its handler block target
            if let Some(handler) = is_begin_trap(i, op, old_fn) {
                let handler = handler?;
                handle_op_trap(&mut out, &mut next_stack, &current, i, handler);

                // Mark as handled
                handled_targets.insert(current.start);

                // Continue to the next item in the queue
                break;
            }

            // Identify an `OpEndTrap`
            if matches!(op, hashlink::OpCode::OpEndTrap(_)) {
                handle_op_end_trap(&mut out, &mut next_stack, &current, i);

                // Mark as handled
                handled_targets.insert(current.start);

                // Continue to the next item in the queue
                break;
            }

            // We need to handle some instructions specially when there is a trap handler in scope.
            if !current.trap_stack.is_empty() {
                if op.is_call() {
                    handle_op_call(&mut out, &mut next_stack, &current, i);

                    // Mark as handled
                    handled_targets.insert(current.start);

                    // Continue to the next item in the queue
                    break;
                }
            }

            // The final case for regular block terminators like `OpRet` and branches
            if is_block_terminator(op) {
                handle_terminator(&mut out, &mut next_stack, old_fn, &current, i)?;

                // Mark as handled
                handled_targets.insert(current.start);

                // Continue to the next item in the queue
                break;
            }
        }
    }

    Ok(out)
}

fn handle_op_trap(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    current: &NextItem,
    i: usize,
    handler: InstructionIndex,
) {
    // The current instruction index
    let current_i = current.start + i;

    // The target instruction index for this "branch"
    let target = current_i + 1;

    // Add the current instruction as a branch
    out.branches.insert(current_i.into());

    // Add this instruction as a branch source for the targe instruction
    out.destination_sources
        .entry(target.into())
        .or_default()
        .insert(current_i.into());

    // Clone the current trap stack and push our new handler on top
    let mut next_trap_stack = current.trap_stack.clone();
    next_trap_stack.push(handler);

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let span = InstructionSpan {
        begin: current.start.into(),
        end: current_i.into(),
        trap_handler: current.trap_stack.last().cloned(),
    };
    out.spans.push(span);

    // We push the trap handler on to the queue as we need to produce a span for it later.
    let next = NextItem {
        start: handler.0,
        trap_stack: current.trap_stack.clone(),
    };
    next_stack.push(next);

    // A begin trap signals the end of a block, with an unconditional jump to the
    // immediately following instruction. The exception handler target is only
    // considered a branch on invoke and throw instructions so there is only a single
    // branch here.
    let next = NextItem {
        start: target,
        trap_stack: next_trap_stack,
    };
    next_stack.push(next);
}

fn handle_op_end_trap(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    current: &NextItem,
    i: usize,
) {
    // The current instruction index
    let current_i = current.start + i;

    // The target instruction index for this "branch"
    let target = current_i + 1;

    // Add the current instruction as a branch
    out.branches.insert(current_i.into());

    // Add this instruction as a branch source for the targe instruction
    out.destination_sources
        .entry(target.into())
        .or_default()
        .insert(current_i.into());

    // Clone the current stack and pop the current trap handler off the top
    let mut next_trap_stack = current.trap_stack.clone();
    next_trap_stack.pop();

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let span = InstructionSpan {
        begin: current.start.into(),
        end: current_i.into(),
        trap_handler: current.trap_stack.last().cloned(),
    };
    out.spans.push(span);

    // An end trap signals the end of a block, with an unconditional jump to the
    // immediately following instruction. This is needed for our exception translation
    // algorithm to work
    let next = NextItem {
        start: target,
        trap_stack: next_trap_stack,
    };
    next_stack.push(next);
}

fn handle_op_call(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    current: &NextItem,
    i: usize,
) {
    // The current instruction index
    let current_i = current.start + i;

    // The target instruction index for this "branch"
    let target = current_i + 1;

    // Add the current instruction as a branch
    out.branches.insert(current_i.into());

    // Add this instruction as a branch source for the targe instruction
    out.destination_sources
        .entry(target.into())
        .or_default()
        .insert(current_i.into());

    // Clone the current stack
    let next_trap_stack = current.trap_stack.clone();

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let span = InstructionSpan {
        begin: current.start.into(),
        end: current_i.into(),
        trap_handler: current.trap_stack.last().cloned(),
    };
    out.spans.push(span);

    // A call signals the end of a block, with an unconditional jump to the
    // immediately following instruction. This is needed for our exception translation
    // algorithm to work
    //
    // THE ABOVE IS ONLY TRUE WHEN A TRAP HANDLER IS IN SCOPE
    let next = NextItem {
        start: target,
        trap_stack: next_trap_stack,
    };
    next_stack.push(next);
}

fn handle_terminator(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    old_fn: &hashlink::Function,
    current: &NextItem,
    i: usize,
) -> TranspileResult<()> {
    // The current instruction index
    let current_i = current.start + i;

    let targets = OpCodeBranchTargetIter::new(old_fn, current_i);
    if let Some(targets) = targets {
        // Add the current instruction as a branch
        out.branches.insert(current_i.into());

        for target in targets {
            // Propagate error
            let target = target?;

            // Add this instruction as a branch source for the targe instruction
            out.destination_sources
                .entry(target)
                .or_default()
                .insert(current_i.into());

            let next = NextItem {
                start: target.0,
                trap_stack: current.trap_stack.clone(),
            };
            next_stack.push(next);
        }
    }

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let span = InstructionSpan {
        begin: current.start.into(),
        end: current_i.into(),
        trap_handler: current.trap_stack.last().cloned(),
    };
    out.spans.push(span);

    Ok(())
}
