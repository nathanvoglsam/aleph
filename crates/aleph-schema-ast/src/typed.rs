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
/// A wrapper over `ItemVariant` that associates the position within the source file of the item
///
#[derive(Hash, Debug)]
pub struct Item {
    /// Position within the source text this item resides
    pub position: usize,

    /// The item variant itself
    pub item: ItemVariant,
}

///
/// Enumeration of all valid list items
///
#[derive(Hash, Debug)]
pub enum ItemVariant {
    /// A singular atom
    Atom(Atom),

    /// A list of items
    List(List),
}
///
/// Enumeration of all possible atom types
///
#[derive(Hash, Debug)]
pub enum Atom {
    /// A string literal, i.e `"Hello, World!"`
    LiteralString(String),

    /// An integer literal, i.e `56`
    LiteralInteger(String),

    /// A floating point literal, i.e `56.21` or `56.`
    LiteralFloat(String),

    /// An identifier, i.e `hello` or `defstruct`
    Ident(String),
}

///
/// Type alias for a list
///
pub type List = Vec<Item>;
