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
use unicode_width::UnicodeWidthStr;

///
/// A set of extensions for the char type for shorthand functions for checking characters
///
//noinspection RsSelfConvention
pub trait CharExtensions {
    /// Returns whether the char is the list open char
    fn is_list_open(self) -> bool;

    /// Returns the list open char
    fn list_open() -> char {
        '('
    }

    /// Returns whether the char is the list close char
    fn is_list_close(self) -> bool;

    /// Returns the list close char
    fn list_close() -> char {
        ')'
    }

    /// Returns whether the char is the quote char (for string literal)
    fn is_quote(self) -> bool;

    /// Returns the quote char
    fn quote() -> char {
        '"'
    }

    /// Returns whether the char is the negation prefix (for negative number literals)
    fn is_negation_prefix(self) -> bool;

    /// Returns the negation prefix char
    fn negation_prefix() -> char {
        '-'
    }

    /// Returns whether the char is the escape prefix (for escape sequence in string literal)
    fn is_escape_prefix(self) -> bool;

    /// Returns the negation prefix char
    fn escape_prefix() -> char {
        '\\'
    }

    /// Returns whether the char is the decimal point char
    fn is_decimal_point(self) -> bool;

    /// Returns the decimal point char
    fn decimal_point() -> char {
        '.'
    }

    /// Returns whether the char is the thousands separator
    fn is_thousands_separator(self) -> bool;

    /// Returns the thousands separator
    fn thousands_separator() -> char {
        '_'
    }

    /// Returns whether this character is the beginning of an item
    fn is_item_token(self) -> bool;

    /// Assuming the char is a hex digit, convert the digit to the corresponding number the digit
    /// represents
    fn hex_value(self) -> u8;
}

impl CharExtensions for char {
    #[inline]
    fn is_list_open(self) -> bool {
        self == char::list_open()
    }

    #[inline]
    fn is_list_close(self) -> bool {
        self == char::list_close()
    }

    #[inline]
    fn is_quote(self) -> bool {
        self == char::quote()
    }

    #[inline]
    fn is_negation_prefix(self) -> bool {
        self == char::negation_prefix()
    }

    #[inline]
    fn is_escape_prefix(self) -> bool {
        self == char::escape_prefix()
    }

    #[inline]
    fn is_decimal_point(self) -> bool {
        self == char::decimal_point()
    }

    fn is_thousands_separator(self) -> bool {
        self == char::thousands_separator()
    }

    #[inline]
    fn is_item_token(self) -> bool {
        const SPECIAL_CHARS: [char; 26] = [
            '!', '@', '#', '$', '%', '^', '&', '*', '[', ']', '{', '}', '<', '>', '.', ',', ':',
            ';', '+', '=', '-', '_', '~', '`', '?', '|',
        ];
        self.is_ascii_alphanumeric() || SPECIAL_CHARS.contains(&self)
    }

    fn hex_value(self) -> u8 {
        match self {
            '0' => 0x0,
            '1' => 0x1,
            '2' => 0x2,
            '3' => 0x3,
            '4' => 0x4,
            '5' => 0x5,
            '6' => 0x6,
            '7' => 0x7,
            '8' => 0x8,
            '9' => 0x9,
            'a' => 0xa,
            'b' => 0xb,
            'c' => 0xc,
            'd' => 0xd,
            'e' => 0xe,
            'f' => 0xf,
            'A' => 0xA,
            'B' => 0xB,
            'C' => 0xC,
            'D' => 0xD,
            'E' => 0xE,
            'F' => 0xF,
            _ => panic!("\'{}\' is not a hex digit", self),
        }
    }
}

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
            let new_width = 1 + (*line_number as f64).log10() as usize;
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
        writeln!(&mut output, "{:>width$}| {}", v.0 + 1, v.2, width = width)?
    }
    if let Some(v) = line_history[1] {
        writeln!(&mut output, "{:>width$}| {}", v.0 + 1, v.2, width = width)?
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
        //
        // We need to handle a special case where the err_offset is trying to point at a newline
        // character. The slices yielded by the `.lines()` iterator strips them so when we try to
        // take a sub slice we get an out of bounds index panic. We can detect this case and just
        // yield the whole line.
        let err_space = if err_offset >= line.len() {
            line
        } else {
            &line[0..err_offset]
        };

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
        writeln!(
            &mut output,
            "{:>width$}| {}",
            line_number + 1,
            line,
            width = width
        )?;

        // Print the marker
        writeln!(&mut output, "{}{}", space, '^')?;

        // Write out the highlighted line and column, +1 to use 1 indexing not 0 indexing
        highlighted_line = line_number + 1;
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

///
/// Trait alias for the stream type
///
pub trait MyStream:
    combine::Stream<Token = char, Position = combine::stream::PointerOffset<str>>
{
}

impl<T: combine::Stream<Token = char, Position = combine::stream::PointerOffset<str>>> MyStream
    for T
{
}
