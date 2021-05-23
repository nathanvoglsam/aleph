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

use crate::utils::CharExtensions;
use combine::{many, satisfy, Parser, Stream};

///
/// Parser that attempts to parse out an identifier
///
pub fn ident<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    ident_first()
        .and(ident_rest())
        .map(|(first, rest): (char, String)| {
            let mut string = String::new();
            string.push(first);
            string.push_str(rest.as_str());
            ast::untyped::Atom::Ident(string)
        })
}

fn ident_first<Input: Stream<Token = char>>() -> impl Parser<Input, Output = char> {
    satisfy::<Input, _>(|v: char| v.is_identifier_first())
}

fn ident_rest<Input: Stream<Token = char>>() -> impl Parser<Input, Output = String> {
    many(satisfy::<Input, _>(|v: char| v.is_identifier_rest()))
}
