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

use crate::parsers::untyped::comment::{comment};
use crate::parsers::MyStream;
use combine::parser::char::space;
use combine::{choice, skip_many, Parser};

///
/// Parser that skips over a single unit of empty space. This could either be a single whitespace
/// character or an entire line comment or an entire block comment, all are equivalent.
///
pub fn empty_space<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    let space = space().map(|_| ());
    choice((space, comment())).expected("whitespace or comment")
}

///
/// A wrapper that skips over 0 or more `empty_space` tokens
///
pub fn empty_spaces<Input: MyStream>() -> impl Parser<Input, Output = ()> {
    skip_many(empty_space())
}
