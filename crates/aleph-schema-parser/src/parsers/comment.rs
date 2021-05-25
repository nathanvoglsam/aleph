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

use crate::parsers::MyStream;
use combine::parser::char::{newline, string};
use combine::parser::repeat::skip_until;
use combine::{eof, Parser, attempt, skip_count, any};

///
/// Parser that attempts to parse out an identifier
///
pub fn line_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    let start_of_comment = string("//").map(|_| ());
    let end_of_line = newline().map(|_| ());
    let end_of_file = eof();
    let terminator = end_of_line.or(end_of_file);
    let body = skip_until(attempt(terminator));
    let end = skip_count(1, any());
    start_of_comment.and(body).and(end).map(|_| ())
}

///
/// Parser that attempts to parse out an identifier
///
pub fn block_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    let start_of_block = string("/*").map(|_| ());
    let end_of_block = string("*/").map(|_| ());
    let end_of_file = eof();
    let terminator = end_of_block.or(end_of_file);
    let body = skip_until(attempt(terminator));
    let end = skip_count(2, any());
    start_of_block.and(body).and(end).map(|_| ())
}
