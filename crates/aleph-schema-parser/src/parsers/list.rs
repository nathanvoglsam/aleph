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

use crate::parsers::empty_space::empty_spaces;
use crate::parsers::{item, MyStream};
use crate::utils::CharExtensions;
use combine::{attempt, between, sep_end_by1, token, Parser};

///
/// Parser that will attempt to parse out a list
///
pub fn list<Input: MyStream>(
    input_base: usize,
) -> impl Parser<Input, Output = ast::untyped::ItemVariant> {
    let open = token(char::list_open());
    let close = token(char::list_close());
    let inner = list_body(input_base);
    between(open, close, inner)
        .map(|v| ast::untyped::ItemVariant::List(v))
        .expected("list")
}

pub fn list_body<Input: MyStream>(
    input_base: usize,
) -> impl Parser<Input, Output = ast::untyped::List> {
    attempt(non_empty_list(input_base)).or(attempt(empty_list()))
}

fn empty_list<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::List> {
    empty_spaces().map(|_| Vec::new())
}

fn non_empty_list<Input: MyStream>(
    input_base: usize,
) -> impl Parser<Input, Output = ast::untyped::List> {
    empty_spaces().with(sep_end_by1(item::item(input_base), empty_spaces()))
}
