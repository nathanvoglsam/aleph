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

use crate::ast::{HasAttributes, Struct};
use smartstring::alias::CompactString;
use std::ops::Range;

#[derive(Debug)]
pub struct Enum<'input> {
    /// Position within the source text this item resides
    pub position: Range<usize>,

    /// The list of fields on the struct
    pub variants: Vec<(CompactString, Option<Struct<'input>>)>,

    /// A list of arbitrary attributes attached to this item. These are simply arbitrary list
    /// s-expressions that can be freely interpreted.
    pub attributes: Vec<sexpr::ast::List<'input>>,
}

impl<'input> Enum<'input> {
    ///
    /// A custom eq implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_eq(&self, other: &Self) -> bool {
        self.eq(other)
            && self.position.start == other.position.start
            && self.position.end == other.position.end
    }

    ///
    /// A custom ne implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_ne(&self, other: &Self) -> bool {
        !self.full_eq(other)
    }
}

impl<'input> PartialEq for Enum<'input> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // The default implementation ignores the position as it has no semantic meaning and is only
        // used for generating error messages
        self.variants.eq(&other.variants) && self.attributes.eq(&other.attributes)
    }
}

impl<'input> Eq for Enum<'input> {}

impl<'input> HasAttributes for Enum<'input> {
    #[inline]
    fn attributes(&self) -> &[sexpr::ast::List] {
        self.attributes.as_slice()
    }
}
