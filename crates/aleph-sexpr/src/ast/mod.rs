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

mod builders;

pub use builders::ListBuilder;

use smartstring::alias::CompactString;
use std::fmt::{Display, Formatter};
use std::ops::Range;

///
/// A wrapper over `ItemVariant` that associates the position within the source file of the item
///
#[derive(Clone, Hash, Debug)]
pub struct Item {
    /// Position within the source text this item resides
    pub span: Range<usize>,

    /// The item variant itself
    pub item: ItemVariant,
}

impl PartialEq for Item {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // The default implementation ignores the position as it has no semantic meaning and is only
        // used for generating error messages
        self.item.eq(&other.item)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        // The default implementation ignores the position as it has no semantic meaning and is only
        // used for generating error messages
        self.item.ne(&other.item)
    }
}

impl Eq for Item {}

impl Item {
    ///
    /// A custom eq implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_eq(&self, other: &Self) -> bool {
        self.span.clone().eq(other.span.clone()) && self.item.eq(&other.item)
    }

    ///
    /// A custom ne implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_ne(&self, other: &Self) -> bool {
        !self.full_eq(other)
    }

    #[inline]
    pub fn atom<A: Into<Atom>>(atom: A, span: Option<Range<usize>>) -> Item {
        Item {
            span: span.unwrap_or(Range::default()),
            item: ItemVariant::Atom(atom.into()),
        }
    }

    #[inline]
    pub fn list<L: Into<List>>(list: L, span: Option<Range<usize>>) -> Item {
        Item {
            span: span.unwrap_or(Range::default()),
            item: ItemVariant::List(list.into()),
        }
    }
}

///
/// Enumeration of all valid list items
///
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ItemVariant {
    /// A singular atom
    Atom(Atom),

    /// A list of items
    List(List),
}

impl From<Vec<Item>> for ItemVariant {
    #[inline]
    fn from(v: Vec<Item>) -> Self {
        Self::List(v)
    }
}

impl ItemVariant {
    #[inline]
    pub fn list(&self) -> Option<&List> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    #[inline]
    pub fn list_mut(&mut self) -> Option<&mut List> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    #[inline]
    pub fn atom(&self) -> Option<&Atom> {
        match self {
            ItemVariant::Atom(atom) => Some(atom),
            ItemVariant::List(_) => None,
        }
    }

    #[inline]
    pub fn atom_mut(&mut self) -> Option<&mut Atom> {
        match self {
            ItemVariant::Atom(atom) => Some(atom),
            ItemVariant::List(_) => None,
        }
    }
}

///
/// Enumeration of all possible atom types
///
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Atom {
    /// A string literal, i.e `"Hello, World!"`
    String(CompactString),

    /// Anything that isn't a string literal
    Word(CompactString),
}

impl Atom {
    #[inline]
    pub fn string<S: Into<CompactString>>(string: S) -> Self {
        Atom::String(string.into())
    }

    #[inline]
    pub fn word<S: Into<CompactString>>(word: S) -> Self {
        Atom::Word(word.into())
    }
}

impl Into<ItemVariant> for Atom {
    #[inline]
    fn into(self) -> ItemVariant {
        ItemVariant::Atom(self)
    }
}

impl Display for Atom {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(v) => f.write_fmt(format_args!("\"{}\"", v)),
            Atom::Word(v) => f.write_str(v),
        }
    }
}

///
/// Type alias for a list
///
pub type List = Vec<Item>;
