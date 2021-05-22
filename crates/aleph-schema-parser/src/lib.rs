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

use combine::easy::{Error, Info, ParseError};
use combine::EasyParser;
use num_integer::Integer;
use unicode_width::UnicodeWidthStr;

mod parsers;

pub fn parse(text: &str) -> Result<ast::untyped::List, ParseError<&str>> {
    let result = parsers::item::item().easy_parse(text);
    let (out, _): (ast::untyped::Item, _) = result?;
    Ok(vec![out])
}

pub fn print_error(original: &str, error: ParseError<&str>) {
    // Get the error
    let err_pos = error.position.0 as usize - original.as_ptr() as usize;

    // Identify which line the error is on
    let line_history = produce_error_line_history(original, err_pos);

    // If we found where the error is print it
    print_lines(err_pos, line_history);

    print_errors(error)
}

fn produce_error_line_history(original: &str, err_pos: usize) -> [Option<(usize, usize, &str)>; 3] {
    let mut line_history = [None; 3];
    for (line_number, line) in original.lines().enumerate() {
        // We need to know the span within the original string this line represents to compare with
        // our reported error pos
        let line_pos = line.as_ptr() as usize - original.as_ptr() as usize;
        let line_end = line_pos + line.len();

        // Append to the line history, pushing old entries out of the list
        line_history[2] = line_history[1].take();
        line_history[1] = line_history[0].take();
        line_history[0] = Some((line_number, line_pos, line));

        // Check if the error is on this line
        if err_pos >= line_pos && err_pos <= line_end {
            break;
        }
    }
    line_history
}

fn print_lines(err_pos: usize, line_history: [Option<(usize, usize, &str)>; 3]) {
    let width: usize = line_history.iter().fold(1, |width, v| {
        if let Some((line_number, _, _)) = v {
            let new_width = line_number.div_ceil(&10);
            if width < new_width {
                new_width
            } else {
                width
            }
        } else {
            width
        }
    });
    line_history[2].map(|v| println!("{:>width$}| {}", v.0, v.2, width = width));
    line_history[1].map(|v| println!("{:>width$}| {}", v.0, v.2, width = width));
    line_history[0].map(|(line_number, line_pos, line)| {
        // Get the offset within the line of the error
        let err_offset = err_pos - line_pos;

        // Get a slice that encompasses all characters in the line before the error
        let err_space = &line[0..err_offset];

        // Get the width of the above slice so we can generate the space for rendering the error
        // marker
        let err_width = err_space.width();

        // Create a the spacer string for our marker
        let space = {
            let num_spaces = err_width + width + 2;
            (0..num_spaces).into_iter().fold(String::new(), |mut v, _| {
                v.push('~');
                v
            })
        };

        // Print the line the error is on
        println!("{}| {}", line_number, line);

        // Print the marker
        println!("{}{}", space, '^');

        // Print the line number for the error
        println!("Error in line {}:", line_number);
    });
}

fn print_errors(error: ParseError<&str>) {
    for error in error.errors.into_iter() {
        match error {
            Error::Unexpected(info) => match info {
                Info::Token(token) => {
                    if token.is_control() || token.is_ascii_control() {
                        println!(" - Unexpected token '{}'", token.escape_default())
                    } else {
                        println!(" - Unexpected token '{}'", token)
                    }
                }
                Info::Range(range) => println!(" - Unexpected range '{}'", range),
                Info::Owned(msg) => println!(" - Unexpected \"{}\"", msg),
                Info::Static(msg) => println!(" - Unexpected \"{}\"", msg),
            },
            Error::Expected(info) => match info {
                Info::Token(token) => {
                    if token.is_control() || token.is_ascii_control() {
                        println!(" - Expected token '{}'", token.escape_default())
                    } else {
                        println!(" - Expected token '{}'", token)
                    }
                }
                Info::Range(range) => println!(" - Expected input '{}'", range),
                Info::Owned(msg) => println!(" - Expected \"{}\"", msg),
                Info::Static(msg) => println!(" - Expected \"{}\"", msg),
            },
            Error::Message(info) => match info {
                Info::Token(token) => {
                    if token.is_control() || token.is_ascii_control() {
                        println!(" - Message '{}'", token.escape_default())
                    } else {
                        println!(" - Message '{}'", token)
                    }
                }
                Info::Range(range) => println!(" - Message '{}'", range),
                Info::Owned(msg) => println!(" - Message \"{}\"", msg),
                Info::Static(msg) => println!(" - Message \"{}\"", msg),
            },
            Error::Other(info) => println!(" - Unknown error \"{}\"", info),
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
