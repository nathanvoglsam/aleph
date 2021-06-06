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

use crate::ast::{HasAttributes, Struct, Table};
use smartstring::alias::CompactString;
use std::collections::HashMap;

/// AST node that adds a name context for all it's child elements
#[derive(Debug)]
pub struct Module<'input> {
    /// Position within the source text this item resides
    pub position: usize,

    /// The list of items inside the module
    pub children: HashMap<CompactString, ModuleItem<'input>>,

    /// A list of arbitrary attributes attached to this item. These are simply arbitrary list
    /// s-expressions that can be freely interpreted.
    pub attributes: Vec<sexpr::ast::List<'input>>,
}

impl<'input> Module<'input> {
    ///
    /// A custom eq implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_eq(&self, other: &Self) -> bool {
        self.eq(other) && self.position.eq(&other.position)
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

impl<'input> PartialEq for Module<'input> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // The default implementation ignores the position as it has no semantic meaning and is only
        // used for generating error messages
        self.children.eq(&other.children) && self.attributes.eq(&other.attributes)
    }
}

impl<'input> Eq for Module<'input> {}

impl<'input> HasAttributes for Module<'input> {
    #[inline]
    fn attributes(&self) -> &[sexpr::ast::List] {
        self.attributes.as_slice()
    }
}

#[derive(Debug)]
pub enum ModuleItemType {
    Module,
    Struct,
    Table,
}

#[derive(Debug)]
pub enum ModuleItem<'input> {
    Module(Module<'input>),
    Struct(Struct<'input>),
    Table(Table<'input>),
}

impl<'input> ModuleItem<'input> {
    ///
    /// Returns the type of module item this is, without the attached `Module`/`Struct`/`Table` item
    ///
    #[inline]
    pub fn item_type(&self) -> ModuleItemType {
        match self {
            ModuleItem::Module(_) => ModuleItemType::Module,
            ModuleItem::Struct(_) => ModuleItemType::Struct,
            ModuleItem::Table(_) => ModuleItemType::Table,
        }
    }

    ///
    /// A custom eq implementation that performs a full equality check, including comparing the
    /// `position` field which is ignored in the `PartialEq` implementation
    ///
    #[inline]
    pub fn full_eq(&self, other: &Self) -> bool {
        match self {
            ModuleItem::Module(v) => {
                if let ModuleItem::Module(o) = other {
                    v.full_eq(o)
                } else {
                    false
                }
            }
            ModuleItem::Struct(v) => {
                if let ModuleItem::Struct(o) = other {
                    v.full_eq(o)
                } else {
                    false
                }
            }
            ModuleItem::Table(v) => {
                if let ModuleItem::Table(o) = other {
                    v.full_eq(o)
                } else {
                    false
                }
            }
        }
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

impl<'input> PartialEq for ModuleItem<'input> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // The default implementation ignores the position as it has no semantic meaning and is only
        // used for generating error messages
        match self {
            ModuleItem::Module(v) => {
                if let ModuleItem::Module(o) = other {
                    v.eq(o)
                } else {
                    false
                }
            }
            ModuleItem::Struct(v) => {
                if let ModuleItem::Struct(o) = other {
                    v.eq(o)
                } else {
                    false
                }
            }
            ModuleItem::Table(v) => {
                if let ModuleItem::Table(o) = other {
                    v.eq(o)
                } else {
                    false
                }
            }
        }
    }
}

impl<'input> Eq for ModuleItem<'input> {}

impl<'input> HasAttributes for ModuleItem<'input> {
    #[inline]
    fn attributes(&self) -> &[sexpr::ast::List] {
        match self {
            ModuleItem::Module(v) => v.attributes(),
            ModuleItem::Struct(v) => v.attributes(),
            ModuleItem::Table(v) => v.attributes(),
        }
    }
}
