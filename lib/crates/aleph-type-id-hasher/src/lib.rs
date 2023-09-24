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

use std::any::TypeId;
use std::hash::{Hash, Hasher};

/// Internal utility for extracting a u64 from TypeId. Uses the 'write_u64' specialization to
/// extract the u64 that TypeId::hash gives us.
///
/// This wrapper should be zero cost, and functionally should just amount to grabbing one of the
/// upper/lower 64bit blocks of the underlying u128 used in the TypeId. This will just optimize to
/// nothing (ideally) as nothing actually happens.
///
/// Not zero-cost in a debug build though :/
pub struct TypeIdHasher(pub u64);

impl TypeIdHasher {
    #[inline(always)]
    pub fn hash(v: TypeId) -> u64 {
        let mut hasher = Self(0);
        v.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for TypeIdHasher {
    #[inline(always)]
    fn default() -> Self {
        Self(0)
    }
}

impl Hasher for TypeIdHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!()
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
