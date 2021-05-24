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

use combine::parser::char::digit;
use combine::{between, many, many1, optional, satisfy, token, Parser};
use crate::parsers::MyStream;

///
/// A parser that attempts to parse out a string literal
///
pub fn string<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    let open = token('"');
    let close = token('"');
    let inner = many(string_char());
    between(open, close, inner).map(|v| ast::untyped::Atom::LiteralString(v))
}

fn string_char<Input: MyStream>() -> impl Parser<Input, Output = char> {
    satisfy::<Input, _>(|v: char| v != '"')
}

///
/// A parser that attempts to parse out an integer literal
///
pub fn integer<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    // A parser that will parse an optional "negation" prefix
    let prefix = number_prefix();
    let number = prefix.and(many1(digit()));
    number.map(|(prefix, number): (Option<String>, String)| {
        if let Some(mut prefix) = prefix {
            prefix.push_str(number.as_str());
            ast::untyped::Atom::LiteralInteger(prefix)
        } else {
            ast::untyped::Atom::LiteralInteger(number)
        }
    })
}

///
/// A parser that attempts to parse out a float literal
///
pub fn float<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    // A parser that will parse an optional "negation" prefix
    let prefix = number_prefix();

    // A parser that combines the prefix parser and another parser that will consume the whole
    // number portion of the float
    let number = prefix.and(many1(digit()));

    // A parser that parses the fractional part of the float, taking a decimal point and optionally
    // after another string of digits
    let fractional = token('.').and(optional(many1(digit())));

    // Combine all the parsers
    number.and(fractional).map(
        |((prefix, first), (_, rest)): ((Option<String>, String), (char, Option<String>))| {
            let mut first = prefix
                .map(|mut v| {
                    v.push_str(first.as_str());
                    v
                })
                .unwrap_or(first);
            first.push('.');
            if let Some(rest) = rest {
                first.push_str(rest.as_str());
            }
            ast::untyped::Atom::LiteralFloat(first)
        },
    )
}

fn number_prefix<Input: MyStream>() -> impl Parser<Input, Output = Option<String>> {
    optional(token('-')).map(|v: Option<char>| v.map(|v| v.to_string()))
}
