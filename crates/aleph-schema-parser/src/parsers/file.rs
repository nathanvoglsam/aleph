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
use crate::parsers::{list, MyStream};
use combine::stream::PointerOffset;
use combine::{attempt, choice, eof, position, sep_end_by1, Parser};

///
/// Parser that will attempt to parse a whole file out to a list
///
pub fn file<Input: MyStream>(input_base: usize) -> impl Parser<Input, Output = ast::untyped::List> {
    choice((empty_file(), non_empty_file(input_base)))
}

fn empty_file<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::List> {
    attempt(empty_spaces().with(eof())).map(|_| Vec::new())
}

fn non_empty_file<Input: MyStream>(
    input_base: usize,
) -> impl Parser<Input, Output = ast::untyped::List> {
    let parser = position().and(list::list(input_base)).map(
        move |(pos, item): (PointerOffset<str>, ast::untyped::ItemVariant)| ast::untyped::Item {
            position: pos.0 - input_base,
            item,
        },
    );
    empty_spaces().with(sep_end_by1(parser, empty_spaces()))
}
