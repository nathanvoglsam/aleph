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

use syn::punctuated::Punctuated;
use syn::{Expr, ExprTuple, Item, Local, Stmt};

///
/// A set of utility functions for creating one of the variants of the `Stmt` enum
///
#[derive(Clone)]
pub struct Statement {}

impl Statement {
    ///
    /// Creates a null statement
    ///
    /// The "null" statement is defined as `();`. This calls no code, produces an empty tuple and
    /// doesn't try to return it to an outer scope.
    ///
    /// This statement does not produce any effects at all so can be considered a "null" statement
    ///
    pub fn null() -> Stmt {
        // Make an empty tuple expression. This means a `()` expression
        let tuple = ExprTuple {
            attrs: Vec::new(),
            paren_token: Default::default(),
            elems: Punctuated::new(),
        };

        // Make a statement with the tuple expression that is terminated with a semicolon.
        // This is the rust equivalent of a "null" statement that has no direct effects or side
        // effects
        Statement::semi_expression(Expr::Tuple(tuple))
    }

    ///
    /// Produces a let binding statement
    ///
    pub fn local(local: Local) -> Stmt {
        Stmt::Local(local)
    }

    ///
    /// Produces an item statement
    ///
    pub fn item(item: Item) -> Stmt {
        Stmt::Item(item)
    }

    ///
    /// Produces an expression statement
    ///
    pub fn expression(expression: Expr) -> Stmt {
        Stmt::Expr(expression)
    }

    ///
    /// Produces a statement that consists of an expression terminated by a semicolon
    ///
    pub fn semi_expression(expression: Expr) -> Stmt {
        Stmt::Semi(expression, Default::default())
    }
}
