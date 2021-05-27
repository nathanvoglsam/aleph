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
use combine::parser::char::newline;
use combine::parser::repeat::skip_until;
use combine::{any, eof, skip_count, Parser, token};
use combine::parser::choice::or;

///
/// Parser that parses out a line comment
///
pub fn comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    token('/').with(or(line_comment(), block_comment()))
}

fn line_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    let start_of_comment = token('/');
    let end_of_line = newline().map(|_| ());
    let end_of_file = eof();
    let terminator = end_of_line.or(end_of_file);
    let body = skip_until(terminator);
    let end = skip_count(1, any());
    start_of_comment.with(body).with(end).map(|_| ())
}

fn block_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    token('*').with(block_segment())
}

fn block_segment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    skip_until(block_segment_end()).with(skip_count(2, any()))
}

combine::parser! {
    fn block_segment_end[Input]()(Input) -> ()
    where [Input: MyStream]
    {
        token('*').with(token('/').map(|_| ()).or(block_segment()))
    }
}
