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
        let out = base.checked_sub(offset)?;
        let out = out.checked_add(1)?;

        Some(out)
    } else {
        // If the offset is positive we can just cast it straight to usize and add
        let out = base.checked_add(offset as usize)?;
        let out = out.checked_add(1)?;

        Some(out)
    }
}
