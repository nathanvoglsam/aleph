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

use crate::parsers::comment::{block_comment, line_comment};
use crate::parsers::MyStream;
use combine::parser::char::space;
use combine::{attempt, choice, skip_many, Parser};

///
/// Parser that attempts to parse out an identifier
///
pub fn empty_space<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    let line_comment = attempt(line_comment());
    let block_comment = attempt(block_comment());
    let space = space().map(|_| ());
    choice((space, block_comment, line_comment)).expected("whitespace or comment")
}

///
/// Parser that attempts to parse out an identifier
///
pub fn empty_spaces<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    skip_many(empty_space()).expected("whitespace or comment")
}
