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

use std::ops::Range;

use crate::ast::{Atom, Item, List};

pub struct ListBuilder<'input> {
    inner: List<'input>,
}

impl<'input> ListBuilder<'input> {
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[inline]
    pub fn add_atom<A: Into<Atom<'input>>>(mut self, atom: A, span: Option<Range<usize>>) -> Self {
        self.inner.push(Item::atom(atom, span));
        self
    }

    #[inline]
    pub fn add_string(self, string: &'input str, span: Option<Range<usize>>) -> Self {
        self.add_atom(Atom::string(string), span)
    }

    #[inline]
    pub fn add_word(self, word: &'input str, span: Option<Range<usize>>) -> Self {
        self.add_atom(Atom::word(word), span)
    }

    #[inline]
    pub fn add_list<L: Into<List<'input>>>(mut self, list: L, span: Option<Range<usize>>) -> Self {
        self.inner.push(Item::list(list, span));
        self
    }

    #[inline]
    pub fn build(self) -> List<'input> {
        self.inner
    }
}

impl<'input> From<ListBuilder<'input>> for List<'input> {
    #[inline]
    fn from(v: ListBuilder<'input>) -> Self {
        v.inner
    }
}

impl<'input> Default for ListBuilder<'input> {
    fn default() -> Self {
        Self::new()
    }
}
