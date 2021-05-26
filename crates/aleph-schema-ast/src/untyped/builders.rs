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

use crate::untyped::{Atom, Item, List};

pub struct ListBuilder {
    inner: List,
}

impl ListBuilder {
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[inline]
    pub fn add_atom<A: Into<Atom>>(mut self, atom: A, position: Option<usize>) -> Self {
        self.inner.push(Item::atom(atom, position));
        self
    }

    #[inline]
    pub fn add_string<S: Into<String>>(self, string: S, position: Option<usize>) -> Self {
        self.add_atom(Atom::string(string), position)
    }

    #[inline]
    pub fn add_int_number(self, integer: i128, position: Option<usize>) -> Self {
        self.add_atom(Atom::int_number(integer), position)
    }

    #[inline]
    pub fn add_float_number(self, float: f64, position: Option<usize>) -> Self {
        self.add_atom(Atom::float_number(float), position)
    }

    #[inline]
    pub fn add_string_number<S: Into<String>>(self, number: S, position: Option<usize>) -> Self {
        self.add_atom(Atom::string_number(number), position)
    }

    #[inline]
    pub fn add_ident<S: Into<String>>(self, ident: S, position: Option<usize>) -> Self {
        self.add_atom(Atom::ident(ident), position)
    }

    #[inline]
    pub fn add_list<L: Into<List>>(mut self, list: L, position: Option<usize>) -> Self {
        self.inner.push(Item::list(list, position));
        self
    }

    #[inline]
    pub fn build(self) -> List {
        self.inner
    }
}

impl Into<List> for ListBuilder {
    #[inline]
    fn into(self) -> List {
        self.inner
    }
}
