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

use crate::parsers::chars::CharExtensions;
use combine::parser::char::digit;
use combine::parser::choice::or;
use combine::{between, many, satisfy, token, Parser, Stream};

///
/// A parser that attempts to parse out a string literal
///
pub fn string<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    let open = token('"');
    let close = token('"');
    let inner = many(string_char());
    between(open, close, inner).map(|v| ast::untyped::Atom::LiteralString(v))
}

///
/// A parser that attempts to parse out an integer literal
///
pub fn integer<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    number_first()
        .and(many(digit()))
        .map(|(mut first, rest): (String, String)| {
            first.push_str(rest.as_str());
            ast::untyped::Atom::LiteralInteger(first)
        })
}

///
/// A parser that attempts to parse out a float literal
///
pub fn float<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    number_first()
        .and(decimal_point())
        .and(many(digit()))
        .map(|((mut first, _), rest): ((String, char), String)| {
            first.push('.');
            first.push_str(rest.as_str());
            ast::untyped::Atom::LiteralFloat(first)
        })
}

fn decimal_point<Input: Stream<Token = char>>() -> impl Parser<Input, Output = char> {
    satisfy::<Input, _>(|v: char| v.is_decimal_point())
}

fn string_char<Input: Stream<Token = char>>() -> impl Parser<Input, Output = char> {
    satisfy::<Input, _>(|v: char| v != '"')
}

fn number_first<Input: Stream<Token = char>>() -> impl Parser<Input, Output = String> {
    let prefixed = token::<Input>('-')
        .and(digit())
        .map(|(prefix, digit): (char, char)| {
            let mut string = String::new();
            string.push(prefix);
            string.push(digit);
            string
        });
    let bare = digit().map(|v| {
        let mut string = String::new();
        string.push(v);
        string
    });
    or(prefixed, bare)
}
