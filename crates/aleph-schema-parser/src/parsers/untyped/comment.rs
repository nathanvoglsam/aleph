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
use combine::parser::choice::or;
use combine::parser::repeat::skip_until;
use combine::{eof, token, Parser};

///
/// Parser that parses out a line comment
///
pub fn comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    token('/').with(or(line_comment(), block_comment()))
}

fn line_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    token('/')
        .with(skip_until(line_comment_end()))
        .skip(line_comment_end())
}

fn line_comment_end<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    or(token('\n'), token('\r')).map(|_| ()).or(eof())
}

fn block_comment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    token('*').with(block_segment())
}

fn block_segment<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    skip_until(token('*'))
        .skip(token('*'))
        .with(block_segment_end())
}

combine::parser! {
    fn block_segment_end[Input]()(Input) -> ()
    where [Input: MyStream]
    {
        token('/').map(|_| ()).or(block_segment())
    }
}
