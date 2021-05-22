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

extern crate aleph_schema_ast as ast;

use combine::EasyParser;
use combine::easy::{ Error, Info};

mod parsers;

pub fn parse(text: &str) -> Result<ast::untyped::List, combine::easy::ParseError<&str>> {
    let result = parsers::item::item().easy_parse(text);
    let (out, _): (ast::untyped::Item, _) = result?;
    Ok(vec![out])
}

pub fn print_error(original: &str, error: combine::easy::ParseError<&str>) {
    // Get the error
    let err_pos = error.position.0 as usize - original.as_ptr() as usize;

    // Identify which line the error is on
    let mut err_line = None;
    for (line_number, line) in original.lines().enumerate() {
        // We need to know the span within the original string this line represents to compare with
        // our reported error pos
        let line_pos = line.as_ptr() as usize - original.as_ptr() as usize;
        let line_end = line_pos + line.len();

        // Check if the error is on this line
        if err_pos >= line_pos && err_pos < line_end {
            err_line = Some((line_number, line_pos, line));
            break;
        }
    }

    // If we found where the error is print it
    if let Some((line_number, line_pos, line)) = err_line {
        let err_offset = err_pos - line_pos;
        let space = {
            (0..err_offset).into_iter().fold(String::new(), |mut v, _| {
                v.push('~');
                v
            })
        };
        println!("Error in line {}:", line_number);
        println!("{}", line);
        println!("{}{}", space, '^');

    }

    for error in error.errors.into_iter() {
        match error {
            Error::Unexpected(info) => match info {
                Info::Token(token) => println!("Unexpected token '{}'", token),
                Info::Range(range) => println!("Unexpected range '{}'", range),
                Info::Owned(msg) => println!("Unexpected input \"{}\"", msg),
                Info::Static(msg) => println!("Unexpected input \"{}\"", msg),
            },
            Error::Expected(info) => match info {
                Info::Token(token) => println!("Expected token '{}'", token),
                Info::Range(range) => println!("Expected input '{}'", range),
                Info::Owned(msg) => println!("Expected input \"{}\"", msg),
                Info::Static(msg) => println!("Expected input \"{}\"", msg),
            },
            Error::Message(info) => match info {
                Info::Token(token) => println!("Message '{}'", token),
                Info::Range(range) => println!("Message '{}'", range),
                Info::Owned(msg) => println!("Message \"{}\"", msg),
                Info::Static(msg) => println!("Message \"{}\"", msg),
            },
            Error::Other(info) => println!("Unknown error \"{}\"", info),
        }
    }
}

#[test]
fn test1() {
    let text = std::fs::read_to_string("./schemas/test1.schema").unwrap();
    match parse(text.as_str()) {
        Ok(output) => println!("{:#?}", output),
        Err(error) => print_error(&text, error),
    }
}

#[test]
fn test2() {
    let text = std::fs::read_to_string("./schemas/test2.schema").unwrap();
    match parse(text.as_str()) {
        Ok(output) => println!("{:#?}", output),
        Err(error) => print_error(&text, error),
    }
}
