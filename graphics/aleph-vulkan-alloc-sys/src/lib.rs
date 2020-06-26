/*
 *
 * This file is a part of NovaEngine
 * https://gitlab.com/MindSpunk/NovaEngine
 *
 *
 * MIT License
 *
 * Copyright (c) 2020 Nathan Voglsam
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

pub mod raw {
    pub type size_t = usize;
    pub type __uint8_t = u8;
    pub type __uint16_t = u16;
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __int8_t = i8;
    pub type __int16_t = i16;
    pub type __int32_t = i32;
    pub type __int64_t = i64;
    include!("../raw.rs");
}

#[cfg(test)]
mod tests;
