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

use crate::parsers::untyped::atom::atom;
use crate::parsers::untyped::list::list;
use crate::parsers::MyStream;
use combine::parser::choice::or;
use combine::stream::PointerOffset;
use combine::{position, Parser};

combine::parser! {
    fn item_inner[Input](input_base: usize)(Input) -> ast::untyped::ItemVariant
    where [Input: MyStream]
    {
        or(list(*input_base), atom())
    }
}

///
/// Parser that will try to parse out a list or an atom to create an item
///
pub fn item<Input: MyStream>(input_base: usize) -> impl Parser<Input, Output = ast::untyped::Item> {
    position().and(item_inner(input_base)).map(
        move |(pos, item): (PointerOffset<str>, ast::untyped::ItemVariant)| ast::untyped::Item {
            position: pos.0 - input_base,
            item,
        },
    )
}
