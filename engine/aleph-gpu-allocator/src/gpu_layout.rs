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

use std::num::NonZero;

/// Like [`std::mem::Layout`], but for the GPU.
///
/// - `size`, must be non-zero.
/// - `align`, must be non-zero.
/// - `align`, must be a power of two.
/// - `size`, when rounded up to the nearest multiple of `align`, must not overflow `isize` (i.e.,
///   the rounded value must be less than or equal to `isize::MAX`).
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GpuLayout {
    /// Size, in bytes.
    size: NonZero<u64>,

    /// Alignment, in bytes. Must be a power of two.
    alignment: NonZero<u64>,
}

impl GpuLayout {
    pub const fn new(size: u64, alignment: u64) -> Option<Self> {
        let size = match NonZero::new(size) {
            None => return None,
            Some(v) => v,
        };
        let alignment = match NonZero::new(alignment) {
            None => return None,
            Some(v) => v,
        };
        if Self::is_size_align_valid(size.get(), alignment.get()) {
            Some(Self { size, alignment })
        } else {
            None
        }
    }

    const fn is_size_align_valid(size: u64, align: u64) -> bool {
        if !align.is_power_of_two() {
            return false;
        }
        if size > Self::max_size_for_align(align) {
            return false;
        }
        true
    }

    #[inline(always)]
    const fn max_size_for_align(align: u64) -> u64 {
        // (power-of-two implies align != 0.)

        // Rounded up size is:
        //   size_rounded_up = (size + align - 1) & !(align - 1);
        //
        // We know from above that align != 0. If adding (align - 1)
        // does not overflow, then rounding up will be fine.
        //
        // Conversely, &-masking with !(align - 1) will subtract off
        // only low-order-bits. Thus if overflow occurs with the sum,
        // the &-mask cannot subtract enough to undo that overflow.
        //
        // Above implies that checking for summation overflow is both
        // necessary and sufficient.

        // SAFETY: the maximum possible alignment is `isize::MAX + 1`,
        // so the subtraction cannot overflow.
        unsafe { u64::unchecked_sub(isize::MAX as u64 + 1, align) }
    }

    /// Size, in bytes, this layout encodes.
    pub const fn size(&self) -> u64 {
        self.size.get()
    }

    /// Alignment, in bytes, this layout encodes. Will always be a power of two.
    pub const fn alignment(&self) -> u64 {
        self.alignment.get()
    }
}
