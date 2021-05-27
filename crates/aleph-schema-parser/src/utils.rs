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
        const SPECIAL_CHARS: [char; 25] = [
            '_', '<', '>', '+', '-', '[', ']', '{', '}', ':', ';', '.', ',', '!', '@', '#', '$',
            '%', '^', '&', '*', '?', '|', '/', '\\',
        ];
        self.is_ascii_alphanumeric() || SPECIAL_CHARS.contains(&self)
    }
}
