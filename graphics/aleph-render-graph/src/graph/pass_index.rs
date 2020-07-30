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

///
/// Wrapper type for pass index to make it a little more safe and explicit
///
#[derive(Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct PassIndex {
    index: usize,
}

impl PassIndex {
    ///
    /// A marker value for specifying an "external" pass. An external pass refers to any use of a
    /// resource outside of the render graph
    ///
    pub const EXTERNAL: PassIndex = Self::new(usize::max_value());

    ///
    /// Creates a new PassIndex
    ///
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    ///
    /// Function wrapper for checking if this PassIndex refers to an external pass
    ///
    pub fn is_external_pass(self) -> bool {
        self == Self::EXTERNAL
    }

    ///
    /// Wrapper for getting the actual index. Returns `None` if `self == PassIndex::EXTERNAL` as
    /// this is not a valid pass index if used to index into the pass list
    ///
    pub fn get(self) -> Option<usize> {
        if self != Self::EXTERNAL {
            Some(self.index)
        } else {
            None
        }
    }
}
