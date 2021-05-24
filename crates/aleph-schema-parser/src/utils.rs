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
pub trait CharExtensions {
    /// Returns whether this character is the beginning of an identifier
    fn is_identifier_first(&self) -> bool;

    /// Returns whether this character is a valid identifier character
    fn is_identifier_rest(&self) -> bool;
}

impl CharExtensions for char {
    fn is_identifier_first(&self) -> bool {
        self.is_ascii_alphabetic() || self.is_identifier_special()
    }

    fn is_identifier_rest(&self) -> bool {
        self.is_alphanumeric() || self.is_identifier_special()
    }
}

trait InternalCharExtensions {
    /// Returns whether this character is a special character for an identifier ('+', '-', etc)
    fn is_identifier_special(&self) -> bool;
}

impl InternalCharExtensions for char {
    fn is_identifier_special(&self) -> bool {
        const SPECIAL_CHARS: [char; 26] = [
            '_', '-', '<', '>', '+', '-', '[', ']', '{', '}', ':', ';', '.', ',', '!', '@', '#',
            '$', '%', '^', '&', '*', '?', '/', '\\', '|',
        ];
        SPECIAL_CHARS.contains(self)
    }
}
