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

use combine::error::StreamError;
use combine::parser::char::{hex_digit, spaces};
use combine::parser::choice::or;
use combine::{
    between, choice, count_min_max, many, satisfy, token, value, ParseError, Parser, StreamOnce,
};
use combine_utils::{CharExtensions, MyStream};
use smartstring::alias::CompactString;

///
/// A parser that parses out a string literal
///
pub fn string<Input: MyStream>() -> impl Parser<Input, Output = CompactString> {
    let open = token(char::quote());
    let close = token(char::quote());
    let inner = string_content();
    between(open, close, inner).expected("string literal")
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
    or(token('\n'), token('\r'))
        .with(spaces())
        .with(value(CompactString::new()))
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
