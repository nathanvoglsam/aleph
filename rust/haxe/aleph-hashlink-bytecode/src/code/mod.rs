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

mod result;

pub use result::Result;
pub use result::CodeReadError;

use std::io::Read;
use byteorder::{ReadBytesExt, LittleEndian};

const HEADER_H: u8 = 0x48;
const HEADER_L: u8 = 0x4C;
const HEADER_B: u8 = 0x42;

pub struct Code {
    version: u8,
    ints: Vec<i32>,
    floats: Vec<f64>,
}

impl Code {
    pub fn read(stream: &mut impl Read) -> Result<Code> {
        //
        // READ AND VALIDATE FLE HEADER
        //
        let h = stream.read_u8()?;
        let l = stream.read_u8()?;
        let b = stream.read_u8()?;
        if h != HEADER_H || l != HEADER_L || b != HEADER_B {
            return Err(CodeReadError::InvalidFileHeader);
        }
        let version = stream.read_u8()?;

        // Load file description that lists flags and the sizes of various tables
        let flags = read_uindex(stream)?;
        let num_ints = read_uindex(stream)?;
        let num_floats = read_uindex(stream)?;
        let num_strings = read_uindex(stream)?;
        let num_bytes = if version >= 5 { read_uindex(stream)? } else { 0 };
        let num_types = read_uindex(stream)?;
        let num_globals = read_uindex(stream)?;
        let num_natives = read_uindex(stream)?;
        let num_functions = read_uindex(stream)?;
        let num_constants = if version >= 4 { read_uindex(stream)? } else { 0 };
        let entrypoint = read_uindex(stream)?;
        let has_debug = flags & 1;

        let mut ints = vec![0i32; num_ints as usize];
        stream.read_i32_into::<LittleEndian>(&mut ints)?;

        let mut floats = vec![0f64; num_floats as usize];
        stream.read_f64_into::<LittleEndian>(&mut floats)?;

        let out = Code {
            version,
            ints,
            floats,
        };
        Ok(out)
    }
}

fn read_index(stream: &mut impl Read) -> Result<i64> {
    let b = stream.read_u8()? as i64;
    if (b & 0x80) == 0 {
        let out = b & 0x7F;
        Ok(out)
    } else if (b & 0x40) == 0 {
        let out = stream.read_u8()? as i64;
        let out = out | (b & 31) << 8;
        return if (b & 0x20) == 0 {
            Ok(out)
        } else {
            Ok(-out)
        }
    } else {
        let c = stream.read_u8()? as i64;
        let d = stream.read_u8()? as i64;
        let e = stream.read_u8()? as i64;
        let v = ((b & 31) << 24) | (c << 16) | (d << 8) | e;
        if (b & 0x20) == 0 {
            Ok(v)
        } else {
            Ok(-v)
        }
    }
}

fn read_uindex(stream: &mut impl Read) -> Result<u64> {
    let i = read_index(stream)?;
    if i < 0 {
        Err(CodeReadError::InvalidIndexUnsignedLessThanOne)
    } else {
        Ok(i as u64)
    }
}
