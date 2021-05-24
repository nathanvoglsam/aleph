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

use combine::easy::{Error, Info, ParseError};
use combine::stream::PointerOffset;
use combine::EasyParser;
use num_integer::Integer;
use unicode_width::UnicodeWidthStr;

///
/// A struct that defines a position inside a source code file
///
#[derive(Clone, Hash, Debug, Default)]
pub struct SourcePosition {
    /// The line the position points to
    pub line: usize,

    /// The column the position points to
    pub column: usize,
}

///
/// Trait alias for the stream type
///
pub trait MyStream: combine::Stream<Token = char, Position = PointerOffset<str>> {}

impl<T: combine::Stream<Token = char, Position = PointerOffset<str>>> MyStream for T {}

///
/// Will parse a source string into an untyped ast
///
pub fn parse(text: &str) -> Result<ast::untyped::List, ParseError<&str>> {
    let input_base = text.as_ptr() as usize;
    let result = crate::parsers::file::file(input_base).easy_parse(text)?;
    Ok(result.0)
}

///
/// Takes the source string and an offset within the source string and constructs a string of the
/// form:
/// ```ignore
/// 4|     )
/// 5|     (def-struct Vector3
/// 6|         (field x f32 (default 0.0!))
/// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^
/// ```
///
/// This function is intended to be used to highlight a position inside the source string, typically
/// for error reporting.
///
/// The function returns the line number
///
pub fn highlight_code(
    source: &str,
    position: usize,
) -> Result<(SourcePosition, String), std::fmt::Error> {
    use std::fmt::Write;

    // The string to write into and output
    let mut output = String::new();

    // Identify which line the error is on
    let line_history = produce_error_line_history(source, position);

    // Get the width in characters of the widest line number so we know how many characters to
    // justify to when printing line numbers
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

    // Print the preceding two lines, if they exist
    if let Some(v) = line_history[2] {
        writeln!(&mut output, "{:>width$}| {}", v.0, v.2, width = width)?
    }
    if let Some(v) = line_history[1] {
        writeln!(&mut output, "{:>width$}| {}", v.0, v.2, width = width)?
    }

    // Slots for outputing the line number and column number that is being highlighted
    let mut highlighted_line = 0;
    let mut highlighted_column = 0;

    // Print the line that contains the character we want to highlight as well as the arrow that
    // points to where we are highlighting
    if let Some((line_number, line_pos, line)) = line_history[0] {
        // Get the offset within the line of the error
        let err_offset = position - line_pos;

        // Get a slice that encompasses all characters in the line before the highlighted point
        let err_space = &line[0..err_offset];

        // Get the width of the above slice so we can generate the space for rendering the marker
        let err_width = err_space.width();

        // Create a the spacer string for our marker
        let space = {
            let num_spaces = err_width + width + 2;
            (0..num_spaces).into_iter().fold(String::new(), |mut v, _| {
                v.push('~');
                v
            })
        };

        // Print the line the highlighted point is on
        writeln!(&mut output, "{}| {}", line_number, line)?;

        // Print the marker
        writeln!(&mut output, "{}{}", space, '^')?;

        highlighted_line = line_number;
        highlighted_column = err_width + 1;
    }

    // Output or string, and the calculated source position
    let pos = SourcePosition {
        line: highlighted_line,
        column: highlighted_column,
    };
    Ok((pos, output))
}

pub fn print_error(source: &str, error: ParseError<&str>) {
    // Get the error
    let err_pos = error.position.0 as usize - source.as_ptr() as usize;

    let (pos, highlight) = highlight_code(source, err_pos).unwrap();

    println!("Error: Error on line {}, column {}", pos.line, pos.column);
    print!("{}", highlight);

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

fn produce_error_line_history(source: &str, position: usize) -> [Option<(usize, usize, &str)>; 3] {
    let mut line_history = [None; 3];
    for (line_number, line) in source.lines().enumerate() {
        // We need to know the span within the original string this line represents to compare with
        // our reported error pos
        let line_pos = line.as_ptr() as usize - source.as_ptr() as usize;
        let line_end = line_pos + line.len();

        // Append to the line history, pushing old entries out of the list
        line_history[2] = line_history[1].take();
        line_history[1] = line_history[0].take();
        line_history[0] = Some((line_number, line_pos, line));

        // Check if the error is on this line
        if position >= line_pos && position <= line_end {
            break;
        }
    }
    line_history
}
