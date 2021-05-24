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
use combine::parser::char::digit;
use combine::{between, many, optional, satisfy, skip_many, token, Parser};

///
/// A parser that attempts to parse out a string literal
///
pub fn string<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    let open = token('"');
    let close = token('"');
    let inner = many(string_char());
    between(open, close, inner).map(|v| ast::untyped::Atom::LiteralString(v))
}

///
/// This parser will succeed and yield a token only for tokens valid inside a string literal
///
pub fn string_char<Input: MyStream>() -> impl Parser<Input, Output = char> {
    satisfy::<Input, _>(|v: char| v != '"')
}

///
/// A parser that attempts to parse out an integer literal
///
pub fn integer<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    number().map(|number| ast::untyped::Atom::LiteralInteger(number))
}

///
/// A parser that attempts to parse out a float literal
///
pub fn float<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    decimal().map(|v| {
        ast::untyped::Atom::LiteralFloat(v)
    })
}

///
/// A parser that parses out an entire decimal number. This includes a negation prefix, the whole
/// number part, the decimal point and the fractional part.
///
pub fn decimal<Input: MyStream>() -> impl Parser<Input, Output = String> {
    // A parser that parses the fractional part of the float, taking a decimal point and optionally
    // after another string of digits
    let fractional = token('.').and(optional(number_body()));

    // Combine all the parsers
    number()
        .and(fractional)
        .map(|(mut first, (_, rest)): (String, (_, Option<String>))| {
            first.push('.');
            if let Some(rest) = rest {
                first.push_str(rest.as_str());
            }
            first
        })
}

///
/// A parser that parses out an entire number, including a negation prefix and all the numbers of
/// a base10 whole number.
///
pub fn number<Input: MyStream>() -> impl Parser<Input, Output = String> {
    number_prefix().and(number_body()).map(|(prefix, body)| {
        if let Some(prefix) = prefix {
            let mut out = String::new();
            out.push(prefix);
            out.push_str(&body);
            out
        } else {
            body
        }
    })
}

///
/// A parser that parses out a base10 whole number.
///
/// # Note
///
/// This parser does not handle a negation prefix and will fail if it encounters one.
///
pub fn number_body<Input: MyStream>() -> impl Parser<Input, Output = String> {
    number_first().and(optional(many(number_rest()))).map(
        |(first, rest): (char, Option<String>)| {
            let mut out = String::new();
            out.push(first);
            if let Some(rest) = rest {
                out.push_str(&rest);
            }
            out
        },
    )
}

///
/// This parser handles an optional negation prefix for a number. It is intended to be used to parse
/// a leading prefix for a number literal
///
pub fn number_prefix<Input: MyStream>() -> impl Parser<Input, Output = Option<char>> {
    optional(token('-'))
}

///
/// This parser will parse the first token after an optional prefix in a number literal. The first
/// token after a prefix is special as it must not be an '_' token.
///
pub fn number_first<Input: MyStream>() -> impl Parser<Input, Output = char> {
    digit()
}

///
/// This parser will parse a single token which is valid to find as part of a number after the
/// optional prefix and the first number part.
///
/// This parser will yield the next digit, skipping over any '_' tokens which are considered
/// thousands separators. The '_' tokens are discarded as they are purely visual aids for humans
/// and have no meaning.
///
pub fn number_rest<Input: MyStream>() -> impl Parser<Input, Output = char> {
    skip_many(token('_')).and(digit()).map(|v| v.1)
}
