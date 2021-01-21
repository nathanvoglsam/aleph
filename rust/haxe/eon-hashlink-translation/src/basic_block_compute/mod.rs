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
use crate::indexes::InstructionIndex;
use crate::utils::{is_begin_trap, is_block_terminator};
use eon_bytecode::BasicBlockIndex;
use std::collections::{HashMap, HashSet};

/// Represents a range of instructions in a function. This span is inclusive, where the range should
/// represent to a slice of instructions that starts with `begin` and ends with `end` inclusively.
///
/// These instruction spans should map directly to an identified basic block.
///
/// Extra information about the basic block is held alongside the range. Currently that includes:
///
/// - Trap handler instruction index
#[derive(Debug)]
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
#[derive(Debug)]
pub struct BasicBlockSpans {
    /// The final output list of basic block spans
    pub spans: Vec<InstructionSpan>,

    /// Predecessor set for each block
    pub predecessors: Vec<HashSet<BasicBlockIndex>>,

    /// Successor set for each block
    pub successors: Vec<HashSet<BasicBlockIndex>>,
}

impl BasicBlockSpans {
    #[must_use]
    fn insert_span(&mut self, current: &NextItem, end: InstructionIndex) -> BasicBlockIndex {
        // Build the span
        let span = InstructionSpan {
            begin: current.start.into(),
            end,
            trap_handler: current.trap_stack.last().cloned(),
        };

        // Push into span list
        let block_index = self.spans.len();
        self.spans.push(span);

        // Get either an empty set or a set initialized with the current span's predecessor
        let predecessors = if let Some(predecessor) = current.predecessor {
            let mut set = HashSet::new();
            set.insert(predecessor);
            set
        } else {
            HashSet::new()
        };

        // Insert the initial predecessor and successor set
        self.predecessors.push(predecessors);
        self.successors.push(HashSet::new());

        block_index.into()
    }

    pub fn last_block_index(&self) -> BasicBlockIndex {
        (self.spans.len() - 1).into()
    }

    /// Returns an iterator over all "real" basic blocks. Some basic blocks are generated, but have
    /// no matching span within the source bytecode so these often need to be skipped in many parts
    /// of the program as an edge case
    pub fn real_block_iter(&self) -> impl Iterator<Item = &InstructionSpan> {
        self.spans[0..self.last_block_index().0].iter()
    }

    pub fn find_source_span_starting_with(&self, i: InstructionIndex) -> Option<BasicBlockIndex> {
        let mut iter = self.spans.iter().enumerate().filter(|(_, v)| v.begin == i);
        if let Some((block, _)) = iter.next() {
            // Assert that there are no other blocks that start with the same instruction
            debug_assert!(iter.count() == 0);
            Some(BasicBlockIndex(block))
        } else {
            None
        }
    }
}

struct NextItem {
    pub predecessor: Option<BasicBlockIndex>,
    pub start: usize,
    pub trap_stack: Vec<InstructionIndex>,
}

impl NextItem {
    fn next(&self, predecessor: BasicBlockIndex, start: InstructionIndex) -> NextItem {
        Self {
            predecessor: Some(predecessor),
            start: start.into(),
            trap_stack: self.trap_stack.clone(),
        }
    }

    fn next_no_predecessor(&self, start: InstructionIndex) -> NextItem {
        Self {
            predecessor: None,
            start: start.into(),
            trap_stack: self.trap_stack.clone(),
        }
    }

    fn next_popped(&self, predecessor: BasicBlockIndex, start: InstructionIndex) -> NextItem {
        let mut trap_stack = self.trap_stack.clone();
        trap_stack.pop();

        Self {
            predecessor: Some(predecessor),
            start: start.into(),
            trap_stack,
        }
    }

    fn next_pushed(
        &self,
        predecessor: BasicBlockIndex,
        start: InstructionIndex,
        handler: InstructionIndex,
    ) -> NextItem {
        let mut trap_stack = self.trap_stack.clone();
        trap_stack.push(handler);

        Self {
            predecessor: Some(predecessor),
            start: start.into(),
            trap_stack,
        }
    }
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
    let mut handled_targets = HashMap::new();

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
        spans: Vec::new(),
        predecessors: Vec::new(),
        successors: Vec::new(),
    };

    // Insert the first instruction into the queue, this is always where we should begin
    let next = NextItem {
        predecessor: None,
        start: 0,
        trap_stack: Vec::new(),
    };
    next_stack.push(next);

    // Continuously loop until the queue is empty
    while let Some(current) = next_stack.pop() {
        // Skip already handled branch targets
        if check_is_handled(&mut out, &handled_targets, &current) {
            continue;
        }

        // Iterate over all instructions beginning from the start index until we either hit a block
        // terminator or hit the end of the function
        for (i, op) in old_fn.ops[current.start..].iter().enumerate() {
            let current_i = current.start + i;

            // Identify an `OpTrap` and get its handler block target
            if let Some(handler) = is_begin_trap(current_i, op, old_fn) {
                let (_, handler) = handler?;
                handle_op_trap(
                    &mut out,
                    &mut next_stack,
                    &mut handled_targets,
                    &current,
                    current_i,
                    handler,
                );

                // Continue to the next item in the queue
                break;
            }

            // Identify an `OpEndTrap`
            if matches!(op, hashlink::OpCode::OpEndTrap(_)) {
                handle_op_end_trap(
                    &mut out,
                    &mut next_stack,
                    &mut handled_targets,
                    &current,
                    current_i,
                );

                // Continue to the next item in the queue
                break;
            }

            // We need to handle some instructions specially when there is a trap handler in scope.
            if !current.trap_stack.is_empty() {
                if op.is_call() {
                    handle_op_call(
                        &mut out,
                        &mut next_stack,
                        &mut handled_targets,
                        &current,
                        current_i,
                    );

                    // Continue to the next item in the queue
                    break;
                }

                if op.throws() {
                    if op.is_throw() {
                        handle_op_throw(
                            &mut out,
                            &mut next_stack,
                            &mut handled_targets,
                            &current,
                            current_i,
                        );
                    } else {
                        handle_op_maybe_throws(
                            &mut out,
                            &mut next_stack,
                            &mut handled_targets,
                            &current,
                            current_i,
                        );
                    }
                    // Continue to the next item in the queue
                    break;
                }
            }

            // The final case for regular block terminators like `OpRet` and branches
            if is_block_terminator(op) {
                handle_terminator(
                    &mut out,
                    &mut next_stack,
                    &mut handled_targets,
                    old_fn,
                    &current,
                    current_i,
                )?;

                // Continue to the next item in the queue
                break;
            }
        }
    }

    // Insert a dummy span for our "unreachable" marker
    out.spans.push(InstructionSpan {
        begin: Default::default(),
        end: Default::default(),
        trap_handler: None,
    });
    out.predecessors.push(Default::default());
    let last_block_index = out.last_block_index();

    let mut successors: Vec<HashSet<BasicBlockIndex>> = vec![Default::default(); out.spans.len()];
    for (i, span) in out.spans.iter().enumerate() {
        // If not handling the last block we check if the block ends with an unconditional throw.
        // We'll need to patch in the unreachable block as a successor
        if i != out.spans.len() {
            if old_fn.ops[span.end.0].is_throw() {
                successors[i].insert(last_block_index);
                out.predecessors[last_block_index.0].insert(i.into());
            }
        }

        // Mark the current block as a successor to all it's successors
        for predecessor in out.predecessors[i].iter().cloned() {
            successors[predecessor.0].insert(BasicBlockIndex(i));
        }
    }
    out.successors = successors;

    Ok(out)
}

fn handle_op_trap(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    current_i: usize,
    handler: InstructionIndex,
) {
    // The target instruction index for this "branch"
    let target = InstructionIndex(current_i + 1);

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    // Push the trap handler but with not predecessor so we guarantee that the trap handler will
    // have a basic block, even if it is unreachable
    next_stack.push(current.next_no_predecessor(handler));

    next_stack.push(current.next_pushed(block, target, handler));
    mark_handled(handled_targets, current, block);
}

fn handle_op_end_trap(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    current_i: usize,
) {
    // The target instruction index for this "branch"
    let target = InstructionIndex(current_i + 1);

    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    next_stack.push(current.next_popped(block, target));
    mark_handled(handled_targets, current, block);
}

fn handle_op_call(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    current_i: usize,
) {
    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    if let Some(trap_handler) = current.trap_stack.last().cloned() {
        next_stack.push(current.next_popped(block, trap_handler));
    }

    next_stack.push(current.next(block, InstructionIndex(current_i + 1)));
    mark_handled(handled_targets, current, block);
}

fn handle_op_throw(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    current_i: usize,
) {
    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    if let Some(trap_handler) = current.trap_stack.last().cloned() {
        next_stack.push(current.next_popped(block, trap_handler));
    }

    mark_handled(handled_targets, current, block);
}

fn handle_op_maybe_throws(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    current_i: usize,
) {
    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    if let Some(trap_handler) = current.trap_stack.last().cloned() {
        next_stack.push(current.next_popped(block, trap_handler));
    }

    next_stack.push(current.next(block, InstructionIndex(current_i + 1)));
    mark_handled(handled_targets, current, block);
}

fn handle_terminator(
    out: &mut BasicBlockSpans,
    next_stack: &mut Vec<NextItem>,
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    old_fn: &hashlink::Function,
    current: &NextItem,
    current_i: usize,
) -> TranspileResult<()> {
    // As this terminates a block we also should emit a span for the block we just
    // created.
    let block = out.insert_span(current, current_i.into());

    let targets = OpCodeBranchTargetIter::new(old_fn, current_i);
    if let Some(targets) = targets {
        for target in targets {
            next_stack.push(current.next(block, target?));
        }
    }

    mark_handled(handled_targets, current, block);

    Ok(())
}

#[deprecated]
fn mark_handled(
    handled_targets: &mut HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
    block: BasicBlockIndex,
) {
    let previous = handled_targets.insert(current.start.into(), block);
    debug_assert!(previous.is_none());
}

#[deprecated]
fn check_is_handled(
    out: &mut BasicBlockSpans,
    handled_targets: &HashMap<InstructionIndex, BasicBlockIndex>,
    current: &NextItem,
) -> bool {
    if let Some(successor) = handled_targets
        .get(&InstructionIndex(current.start))
        .cloned()
    {
        if let Some(predecessor) = current.predecessor {
            out.predecessors[successor.0].insert(predecessor);
        }
        true
    } else {
        false
    }
}
