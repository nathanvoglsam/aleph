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

#![allow(unused)]

use std::ops::Range;
use std::str::CharIndices;

/// The error type the lexer emits
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    InvalidEscapeCharacter {
        span: Range<usize>,
    },
    InvalidAsciiEscapeSequence {
        span: Range<usize>,
        expected: Vec<&'static str>,
    },
    InvalidUnicodeEscapeSequence {
        span: Range<usize>,
        expected: Vec<&'static str>,
    },
    UnexpectedEndOfInput {
        expected: Vec<&'static str>,
    },
}

/// Type alias for our parser's iterator yield value
pub type Spanned<Tok> = Result<(usize, Tok, usize), Error>;

/// Our custom lexer implementation
pub struct StringLiteralParser<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
    state: State,
}

impl<'input> StringLiteralParser<'input> {
    /// Constructs a new lexer from the given text input
    pub fn new(input: &'input str) -> Self {
        Self {
            chars: input.char_indices().peekable(),
            input,
            state: State::Default,
        }
    }
}

impl<'input> Iterator for StringLiteralParser<'input> {
    type Item = Spanned<char>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.peek().cloned() {
                Some((i, c)) => match self.state.take() {
                    State::Default => {
                        if c == '\\' {
                            self.state = State::EscapeStart(i);
                            self.chars.next().unwrap();
                            continue;
                        } else {
                            self.chars.next().unwrap();
                            return Some(Ok((i, c, i + 1)));
                        }
                    }
                    State::EscapeStart(span_start) => {
                        if c == 'n'
                            || c == 'r'
                            || c == 't'
                            || c == '\\'
                            || c == '"'
                            || c == '\''
                            || c == '0'
                        {
                            self.chars.next().unwrap();
                            return Some(Ok((span_start, c, i + 1)));
                        }

                        if c == 'x' {
                            self.state = State::EscapeAscii(span_start, 0);
                            self.chars.next().unwrap();
                            continue;
                        }

                        if c == 'u' {
                            self.state = State::EscapeUnicode(span_start, 0);
                            self.chars.next().unwrap();
                            continue;
                        }

                        if c == '\n' || c == '\r' {
                            self.state = State::EscapeNewline(span_start);
                            self.chars.next().unwrap();
                            continue;
                        }

                        self.chars.next().unwrap();
                        let span = i..i + 1;
                        return Some(Err(Error::InvalidEscapeCharacter { span }));
                    }
                    State::EscapeAscii(span_start, tokens_consumed) => match tokens_consumed {
                        0 | 1 => {
                            if c.is_ascii_hexdigit() {
                                self.state = State::EscapeAscii(span_start, tokens_consumed + 1);
                                self.chars.next().unwrap();
                                continue;
                            } else {
                                self.chars.next().unwrap();
                                let span = span_start..span_start + tokens_consumed + 1;
                                let expected = vec!["hex digit"];
                                return Some(Err(Error::InvalidAsciiEscapeSequence {
                                    span,
                                    expected,
                                }));
                            }
                        }
                        2 => {
                            let begin = span_start + 1;
                            let end = begin + 2;
                            let digits = &self.input[begin..end];
                            let code = u8::from_str_radix(digits, 16).unwrap();

                            return if code > 0x7F {
                                let span = span_start..end;
                                let expected = vec!["ascii code <= 0x7F"];
                                self.chars.next().unwrap();
                                Some(Err(Error::InvalidAsciiEscapeSequence { span, expected }))
                            } else {
                                let tok = char::from(code);
                                self.chars.next().unwrap();
                                Some(Ok((span_start, tok, end)))
                            };
                        }
                        _ => unreachable!(),
                    },
                    State::EscapeUnicode(span_start, tokens_consumed) => match tokens_consumed {
                        0 => {
                            if c == '{' {
                                self.state = State::EscapeUnicode(span_start, tokens_consumed + 1);
                                self.chars.next().unwrap();
                                continue;
                            } else {
                                let span = span_start..i;
                                let expected = vec!["{"];
                                self.chars.next().unwrap();
                                return Some(Err(Error::InvalidUnicodeEscapeSequence {
                                    span,
                                    expected,
                                }));
                            }
                        }
                        1 => {
                            if c.is_ascii_hexdigit() {
                                self.state = State::EscapeUnicode(span_start, tokens_consumed + 1);
                                self.chars.next().unwrap();
                                continue;
                            }

                            let span = span_start..i;
                            let expected = vec!["hex digit"];
                            self.chars.next().unwrap();
                            return Some(Err(Error::InvalidUnicodeEscapeSequence {
                                span,
                                expected,
                            }));
                        }
                        2 | 3 | 4 | 5 | 6 => {
                            if c == '}' {
                                return self.unicode_token(i, span_start);
                            }
                            if c.is_ascii_hexdigit() {
                                self.state = State::EscapeUnicode(span_start, tokens_consumed + 1);
                                self.chars.next().unwrap();
                                continue;
                            }
                            let span = span_start..i;
                            let expected = vec!["hex digit", "}"];
                            self.chars.next().unwrap();
                            return Some(Err(Error::InvalidUnicodeEscapeSequence {
                                span,
                                expected,
                            }));
                        }
                        7 => {
                            if c == '}' {
                                return self.unicode_token(i, span_start);
                            }

                            let span = span_start..i;
                            let expected = vec!["}"];
                            self.chars.next().unwrap();
                            return Some(Err(Error::InvalidUnicodeEscapeSequence {
                                span,
                                expected,
                            }));
                        }
                        _ => unreachable!(),
                    },
                    State::EscapeNewline(span_start) => {
                        if c.is_whitespace() {
                            self.state = State::EscapeNewline(span_start);
                            self.chars.next().unwrap();
                        }
                        continue;
                    }
                },
                None => {
                    return match self.state.take() {
                        State::Default => None,
                        State::EscapeStart(_) => unreachable!(),
                        State::EscapeAscii(span_start, tokens_consumed) => match tokens_consumed {
                            0 | 1 => Some(Err(Error::UnexpectedEndOfInput {
                                expected: vec!["complete ascii escape sequence"],
                            })),
                            2 => {
                                let begin = span_start + 1;
                                let end = begin + 2;
                                let digits = &self.input[begin..end];
                                let code = u8::from_str_radix(digits, 16).unwrap();

                                if code > 0x7F {
                                    let span = span_start..end;
                                    let expected = vec!["ascii code <= 0x7F"];
                                    Some(Err(Error::InvalidAsciiEscapeSequence { span, expected }))
                                } else {
                                    let tok = char::from(code);
                                    Some(Ok((span_start, tok, end)))
                                }
                            }
                            _ => unreachable!(),
                        },
                        State::EscapeUnicode(_, _) => Some(Err(Error::UnexpectedEndOfInput {
                            expected: vec!["complete unicode escape sequence"],
                        })),
                        State::EscapeNewline(_) => None,
                    };
                }
            }
        }
    }
}

impl<'input> StringLiteralParser<'input> {
    fn unicode_token(&mut self, i: usize, span_start: usize) -> Option<Spanned<char>> {
        let begin = span_start + 3;
        let end = i;
        let sequence = &self.input[begin..end];
        let num = u32::from_str_radix(sequence, 16).unwrap();
        if let Some(result) = char::from_u32(num) {
            self.chars.next().unwrap();
            Some(Ok((span_start, result, i)))
        } else {
            let span = span_start..i;
            let expected = vec!["valid unicode codepoint"];
            self.chars.next().unwrap();
            Some(Err(Error::InvalidUnicodeEscapeSequence { span, expected }))
        }
    }
}

enum State {
    Default,
    EscapeStart(usize),
    EscapeAscii(usize, usize),
    EscapeUnicode(usize, usize),
    EscapeNewline(usize),
}

impl Default for State {
    fn default() -> Self {
        Self::Default
    }
}

impl State {
    fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
