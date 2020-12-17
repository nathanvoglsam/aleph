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
use eon_bytecode::indexes::InstructionIndex;

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

/// Returns whether the given instruction is an `OpTrap`, and the trap handler target index if it
/// is.
pub fn is_begin_trap(
    i_index: usize,
    op: &hashlink::OpCode,
    old_fn: &hashlink::Function,
) -> Option<TranspileResult<InstructionIndex>> {
    match op {
        hashlink::OpCode::OpTrap(v) => Some(offset_to_index(i_index, v.param_2, old_fn)),
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
