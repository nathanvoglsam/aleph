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

mod types;

pub use types::PrimitiveType;
pub use types::SizedType;
pub use types::Structure;

use std::fmt::{Display, Formatter};

///
/// A wrapper over `ItemVariant` that associates the position within the source file of the item
///
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Item {
    /// Position within the source text this item resides
    pub position: usize,

    /// The item variant itself
    pub item: ItemVariant,
}

///
/// Enumeration of all valid list items
///
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ItemVariant {
    /// A singular atom
    Atom(Atom),

    /// A list of items
    List(List),
}

impl ItemVariant {
    pub fn list(&self) -> Option<&List> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    pub fn list_mut(&mut self) -> Option<&mut List> {
        match self {
            ItemVariant::Atom(_) => None,
            ItemVariant::List(list) => Some(list),
        }
    }

    pub fn atom(&self) -> Option<&Atom> {
        match self {
            ItemVariant::Atom(atom) => Some(atom),
            ItemVariant::List(_) => None,
        }
    }

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
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Atom {
    /// A string literal, i.e `"Hello, World!"`
    LiteralString(String),

    /// An number literal, i.e `56` or `56.21` or `56.`
    LiteralNumber(String),

    /// An identifier, i.e `hello` or `defstruct`
    Ident(String),
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::LiteralString(v) => f.write_fmt(format_args!("\"{}\"", v)),
            Atom::LiteralNumber(v) => f.write_str(v),
            Atom::Ident(v) => f.write_str(v),
        }
    }
}

///
/// Type alias for a list
///
pub type List = Vec<Item>;
