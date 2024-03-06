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

// use std::any::TypeId;
use std::hash::{Hash, Hasher};

/// Internal utility for extracting a u64 from small values like integers. Uses the 'write_<int>'
/// specializations to extract a u64 that  gives us.
///
/// This wrapper should be zero cost, and functionally should just amount to transmuting the bits
/// of the type into the hash we output
///
/// Not zero-cost in a debug build though :/
pub struct IdentityHasher {
    result: u64,
}

impl IdentityHasher {
    #[inline(always)]
    pub fn hash<T: Copy + Sized + Hash>(v: T) -> u64 {
        let mut hasher = Self::default();
        v.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for IdentityHasher {
    #[inline(always)]
    fn default() -> Self {
        Self { result: 0 }
    }
}

impl Hasher for IdentityHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.result
    }

    #[inline(always)]
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.result = i;
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_i128(&mut self, i: i128) {
        self.result = i as u64;
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        self.result = i as u64;
    }
}
