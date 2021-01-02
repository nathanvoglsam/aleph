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

use crate::error::{InvalidFunctionReason, TranspileError, TranspileResult};
use crate::indexes::InstructionIndex;
use eon_bytecode::indexes::RegisterIndex;
use std::collections::HashSet;
use std::hash::Hash;

// We need to apply the offset to the current instruction index. We do it in this
// convoluted way so that we don't discard the full bit width of a usize in order
// to apply the offset. If we cast the index to isize and applied using a simple add
// then we could only represent offsets up to `isize::max`.
//
// Because we're going to this effort I may as well make it panic on overflow
pub fn offset_from(base: usize, offset: i32) -> Option<usize> {
    if offset.is_negative() {
        // Convert negative to positive so it will fit into a usize
        let offset = -offset;
        let offset = offset as usize;

        // Subtract the inverted negative offset. This is mathematically identical to just
        // adding a signed offset but does not discard the precision of the base value
        let out = base.checked_add(1)?;
        let out = out.checked_sub(offset)?;

        Some(out)
    } else {
        // If the offset is positive we can just cast it straight to usize and add
        let out = base.checked_add(offset as usize)?;
        let out = out.checked_add(1)?;

        Some(out)
    }
}

/// Applies the given `offset` to the provided `i_index` while checking for any potential errors in
/// the provided offset.
pub fn offset_to_index(
    i_index: usize,
    offset: i32,
    old_fn: &hashlink::Function,
) -> TranspileResult<InstructionIndex> {
    // Compute the offset, producing an error if the offset is invalid
    let target = if let Some(target) = offset_from(i_index, offset) {
        target
    } else {
        return Err(offset_error(i_index, old_fn));
    };

    // Check if a negative index offset branch does not branch to a label opcode
    if offset < 0 {
        if !matches!(&old_fn.ops[target], hashlink::OpCode::OpLabel) {
            //negative offset not targeting label
            let err = negative_offset_error(i_index, old_fn);
            return Err(err);
        }
    }

    // Check if the target is in bounds
    bounds_check_target(target, old_fn, i_index)?;

    Ok(InstructionIndex(target))
}

/// Bounds checks the given jump target against the provided HashLink source function
pub fn bounds_check_target(
    target: usize,
    old_fn: &hashlink::Function,
    i_index: usize,
) -> TranspileResult<()> {
    if target >= old_fn.ops.len() {
        let reason = InvalidFunctionReason::JumpOffsetOutOfBounds {
            i_index,
            func: old_fn.clone(),
        };
        let err = TranspileError::InvalidFunction(reason);
        return Err(err); // Out of bounds
    }
    Ok(())
}

/// This function, when given an iterator that yields iterators of `T`, will produce a `HashSet<T>`
/// that holds the intersection of all iterators of `T` that the outermost iterator yields.
///
/// The input should be considered an iterator of sets, where the sets themselves are represented as
/// iterators for flexibility.
pub fn intersect_hash_sets<T: Hash + Eq, I: Iterator<Item = T>>(
    mut sets: impl Iterator<Item = I>,
) -> HashSet<T> {
    // We have to handle the first case specially so we manually pop it off the iterator first
    if let Some(v) = sets.next() {
        // We pre-allocate 3 sets here which get consumed in the following loop.
        //
        // By pre-allocating these sets we can massively reduce how many times we need to hit
        // the memory allocator. For most inputs we'll never have to allocate in the loop
        let mut acc = HashSet::with_capacity(24);
        let mut scratch = HashSet::with_capacity(24);
        let mut next = HashSet::with_capacity(24);

        // Initialize the accumulator set with the initial set we're intersecting
        v.for_each(|v| {
            acc.insert(v);
        });

        // Intersect every set in `sets`
        sets.for_each(|set| {
            // Clear `scratch` and `next`
            next.clear();
            scratch.clear();

            // Make our "scratch" set so it is exactly equal to the union iterator we get from
            // the `sets` iterator
            set.for_each(|v| {
                scratch.insert(v);
            });

            // With this iteration's live set collected into the `scratch` set we can now
            // intersect our accumulator (`acc`) with our temp set and fill the `next` set with
            // that intersection
            for v in acc.drain() {
                if scratch.contains(&v) {
                    next.insert(v);
                }
            }

            // The next set now contains the intersection of `acc` and `scratch`, which is what
            // we need to be in `acc` for the next intersection, or as the final output of this
            // loop
            //
            // We swap `acc` and `next` to prepare for output or the next iteration
            //
            // `acc` now contains the intersection we want to output or use in the next
            // iteration
            //
            // `next` now contains the old accumulator, the contents of which can now be
            // discarded
            std::mem::swap(&mut acc, &mut next);
        });

        acc
    } else {
        // If the iterator was empty just return the empty set
        HashSet::new()
    }
}

/// Returns whether the given instruction is an `OpTrap`, the register the exception will be
/// received in, and the trap handler target index.
/// is.
pub fn is_begin_trap(
    i_index: usize,
    op: &hashlink::OpCode,
    old_fn: &hashlink::Function,
) -> Option<TranspileResult<(RegisterIndex, InstructionIndex)>> {
    match op {
        hashlink::OpCode::OpTrap(v) => {
            let out = offset_to_index(i_index, v.param_2, old_fn)
                .map(|i| (RegisterIndex(v.param_1 as usize), i));
            Some(out)
        }
        _ => None,
    }
}

/// Returns whether this opcode is unconditionally (i.e always) a block terminator
///
/// # Warning
///
/// Some instructions only terminate a block depending on the trap handler stack. For example, an
/// OpCall will terminate a block only if inside a trap handler section as it must be translated to
/// an OpInvoke.
///
/// This function will need to be used alongside further contextual checks to correctly identify all
/// block terminator opcodes
pub fn is_block_terminator(op: &hashlink::OpCode) -> bool {
    op.is_branch() || op.is_ret() || op.is_throw()
}

/// Utility for producing an error variant
pub fn negative_offset_error(i_index: usize, old_fn: &hashlink::Function) -> TranspileError {
    let reason = InvalidFunctionReason::JumpNegativeOffsetNotTargetingLabel {
        i_index,
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}

/// Utility for producing an error variant
pub fn offset_error(i_index: usize, old_fn: &hashlink::Function) -> TranspileError {
    let reason = InvalidFunctionReason::JumpInvalidOffset {
        i_index,
        func: old_fn.clone(),
    };
    let err = TranspileError::InvalidFunction(reason);
    err
}
