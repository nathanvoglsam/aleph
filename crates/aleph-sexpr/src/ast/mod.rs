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

use std::fmt::{Display, Formatter};
use std::ops::Range;

///
/// A wrapper over `ItemVariant` that associates the position within the source file of the item
///
#[derive(Clone, Hash, Debug)]
pub struct Item<'input> {
    /// Position within the source text this item resides
    pub span: Range<usize>,

    /// The item variant itself
    pub item: ItemVariant<'input>,
}

impl<'input> PartialEq for Item<'input> {
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

impl<'input> Eq for Item<'input> {}

impl<'input> Item<'input> {
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
    pub fn atom<A: Into<Atom<'input>>>(atom: A, span: Option<Range<usize>>) -> Item<'input> {
        Item {
            span: span.unwrap_or(Range::default()),
            item: ItemVariant::Atom(atom.into()),
        }
    }

    #[inline]
    pub fn list<L: Into<List<'input>>>(list: L, span: Option<Range<usize>>) -> Item<'input> {
        Item {
            span: span.unwrap_or(Range::default()),
            item: ItemVariant::List(list.into()),
        }
    }
}

///
/// Enumeration of all valid list item types
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ItemVariantType {
    Atom,
    List,
}

///
/// Enumeration of all valid list items
///
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ItemVariant<'input> {
    /// A singular atom
    Atom(Atom<'input>),

    /// A list of items
    List(List<'input>),
}

impl<'input> ItemVariant<'input> {
    pub fn variant_type(&self) -> ItemVariantType {
        match self {
            Self::Atom(_) => ItemVariantType::Atom,
            Self::List(_) => ItemVariantType::List,
        }
    }
}

impl<'input> From<Vec<Item<'input>>> for ItemVariant<'input> {
    #[inline]
    fn from(v: Vec<Item<'input>>) -> Self {
        Self::List(v)
    }
}

impl<'input> ItemVariant<'input> {
    #[inline]
    pub fn list(&self) -> Option<&List<'input>> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    #[inline]
    pub fn list_mut(&mut self) -> Option<&mut List<'input>> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    #[inline]
    pub fn atom(&self) -> Option<&Atom<'input>> {
        match self {
            ItemVariant::Atom(atom) => Some(atom),
            ItemVariant::List(_) => None,
        }
    }

    #[inline]
    pub fn atom_mut(&mut self) -> Option<&mut Atom<'input>> {
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
pub enum Atom<'input> {
    /// A string literal, i.e `"Hello, World!"`
    String(&'input str),

    /// Anything that isn't a string literal
    Word(&'input str),
}

impl<'input> Atom<'input> {
    #[inline]
    pub fn string(string: &'input str) -> Self {
        Atom::String(string)
    }

    #[inline]
    pub fn word(word: &'input str) -> Self {
        Atom::Word(word)
    }

    #[inline]
    pub fn as_string(&self) -> Option<&'input str> {
        match self {
            Atom::String(v) => Some(v),
            Atom::Word(_) => None,
        }
    }

    #[inline]
    pub fn as_word(&self) -> Option<&'input str> {
        match self {
            Atom::String(_) => None,
            Atom::Word(v) => Some(v),
        }
    }
}

impl<'input> Into<ItemVariant<'input>> for Atom<'input> {
    #[inline]
    fn into(self) -> ItemVariant<'input> {
        ItemVariant::Atom(self)
    }
}

impl<'input> Display for Atom<'input> {
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
pub type List<'input> = Vec<Item<'input>>;
