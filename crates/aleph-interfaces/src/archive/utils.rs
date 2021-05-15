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

use std::fmt::{Formatter, Result};
use std::num::NonZeroU64;

pub fn display_id(f: &mut Formatter<'_>, val: NonZeroU64) -> Result {
    let chunks = chunks(val.get());
    f.write_fmt(format_args!(
        "{:X}-{:X}-{:X}-{:X}",
        chunks[0].clone(),
        chunks[1].clone(),
        chunks[2].clone(),
        chunks[3].clone()
    ))
}

fn chunks(val: u64) -> [u16; 4] {
    let b7 = ((val >> 56) & 0xFF) as u16;
    let b6 = ((val >> 48) & 0xFF) as u16;
    let b5 = ((val >> 40) & 0xFF) as u16;
    let b4 = ((val >> 32) & 0xFF) as u16;
    let b3 = ((val >> 24) & 0xFF) as u16;
    let b2 = ((val >> 16) & 0xFF) as u16;
    let b1 = ((val >> 8) & 0xFF) as u16;
    let b0 = ((val >> 0) & 0xFF) as u16;
    let c3 = b6 | (b7 << 8);
    let c2 = b4 | (b5 << 8);
    let c1 = b2 | (b3 << 8);
    let c0 = b0 | (b1 << 8);
    [c3, c2, c1, c0]
}
