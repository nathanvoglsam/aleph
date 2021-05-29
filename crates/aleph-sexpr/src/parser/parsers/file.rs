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

use crate::parser::{empty_spaces, list};
use combine::stream::PointerOffset;
use combine::{eof, position, sep_end_by, Parser};
use combine_utils::MyStream;

///
/// Parser that will parse a whole file out to an AST.
///
pub fn file<Input: MyStream>(input_base: usize) -> impl Parser<Input, Output = crate::ast::List> {
    // Parser for an individual list item
    let parser = position().and(list(input_base)).map(
        move |(pos, item): (PointerOffset<str>, crate::ast::ItemVariant)| crate::ast::Item {
            position: pos.0 - input_base,
            item,
        },
    );

    // Parser for a sequence of 0 or more lists at the file root
    let main = sep_end_by(parser, empty_spaces()).and(eof()).map(|v| v.0);

    empty_spaces().with(main)
}
