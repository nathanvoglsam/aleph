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
use crate::utils::CharExtensions;
use combine::error::StreamError;
use combine::parser::char::{digit, hex_digit, spaces};
use combine::{between, choice, count_min_max, many, optional, satisfy, skip_many, token, ParseError, Parser, StreamOnce, value};
use smartstring::alias::CompactString;
use combine::parser::choice::or;

///
/// A parser that parses out a string literal
///
pub fn string<Input: MyStream>() -> impl Parser<Input, Output = ast::untyped::Atom> {
    let open = token(char::quote());
    let close = token(char::quote());
    let inner = string_content();
    between(open, close, inner)
        .map(|v| ast::untyped::Atom::LiteralString(v.to_string()))
        .expected("string literal")
}

fn string_content<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    many(string_char())
}

fn string_char<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    let not_quote = satisfy::<Input, _>(|v: char| !v.is_quote()).map(|v: char| {
        let mut output = CompactString::new();
        output.push(v);
        output
    });

    escape_sequence().or(not_quote)
}

fn escape_sequence<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    let choices = (
        token('n').map(|_| CompactString::from("\n")),
        token('r').map(|_| CompactString::from("\r")),
        token('t').map(|_| CompactString::from("\t")),
        token('r').map(|_| CompactString::from("\r")),
        token('\\').map(|_| CompactString::from("\\")),
        token('"').map(|_| CompactString::from("\"")),
        token('\'').map(|_| CompactString::from("\'")),
        token('0').map(|_| CompactString::from("\0")),
        ascii_escape_sequence(),
        unicode_escape_sequence(),
        newline_escape_sequence(),
    );
    token('\\').with(choice(choices))
}

fn newline_escape_sequence<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    or(token('\n'), token('\r')).with(spaces()).with(value(CompactString::new()))
}

fn ascii_escape_sequence<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    let sequence = hex_digit().and(hex_digit());
    let sequence = sequence.and_then(|(f, s)| {
        let first = f.hex_value() << 4;
        let second = s.hex_value();
        let combined = first | second;
        if combined > 0x7F {
            let err = format!("\\x{}{} is not a valid ASCII character", f, s);
            let err = <<Input as StreamOnce>::Error as ParseError<
                Input::Token,
                Input::Range,
                Input::Position,
            >>::StreamError::message_format(err);
            Err(err)
        } else {
            let mut output = CompactString::new();
            output.push(char::from(combined));
            Ok(output)
        }
    });
    token('x').with(sequence)
}

fn unicode_escape_sequence<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    let sequence_char = hex_digit();
    let sequence = count_min_max(1, 6, sequence_char);
    let sequence = sequence.and_then(|v: String| {
        let num = u32::from_str_radix(&v, 16).unwrap();
        if let Some(result) = char::from_u32(num) {
            let mut output = CompactString::new();
            output.push(result);
            Ok(output)
        } else {
            let err = format!("\\x{{{}}} is not a valid Unicode codepoint", v);
            let err = <<Input as StreamOnce>::Error as ParseError<
                Input::Token,
                Input::Range,
                Input::Position,
            >>::StreamError::message_format(err);
            Err(err)
        }
    });
    token('u').with(between(token('{'), token('}'), sequence))
}

///
/// A parser that parses out an entire decimal number. This includes a negation prefix, the whole
/// number part, the decimal point and the fractional part.
///
pub fn decimal<Input: MyStream>() -> impl Parser<Input, Output = String> {
    // A parser that parses the fractional part of the float, taking a decimal point and optionally
    // after another string of digits
    let fractional = token(char::decimal_point()).and(optional(whole_number_body()));

    // Combine all the parsers
    whole_number()
        .and(fractional)
        .map(|(mut first, (_, rest)): (String, (_, Option<String>))| {
            first.push(char::decimal_point());
            if let Some(rest) = rest {
                first.push_str(rest.as_str());
            }
            first
        })
        .expected("decimal number")
}

///
/// A parser that parses out an entire number, including a negation prefix and all the numbers of
/// a base10 whole number.
///
pub fn whole_number<Input: MyStream>() -> impl Parser<Input, Output = String> {
    whole_number_prefix()
        .and(whole_number_body())
        .map(|(prefix, body)| {
            if let Some(prefix) = prefix {
                let mut out = String::new();
                out.push(prefix);
                out.push_str(&body);
                out
            } else {
                body
            }
        })
        .expected("whole number")
}

///
/// A parser that parses out a base10 whole number.
///
/// # Note
///
/// This parser does not handle a negation prefix and will fail if it encounters one.
///
pub fn whole_number_body<Input: MyStream>() -> impl Parser<Input, Output = String> {
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
pub fn whole_number_prefix<Input: MyStream>() -> impl Parser<Input, Output = Option<char>> {
    optional(token(char::negation_prefix()))
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
    skip_many(token(char::thousands_separator()))
        .and(digit())
        .map(|v| v.1)
}
