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

use combine_utils::CharExtensions;
use std::str::CharIndices;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    UnexpectedCharacter {
        character: char,
        expected: Vec<&'static str>,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: Vec<&'static str>,
    },
    UnclosedStringLiteral {
        begin: usize,
    },
}

/// Type alias for our lexer's iterator yield value
pub type Spanned<Tok> = Result<(usize, Tok, usize), Error>;

/// A enum over all token variants
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tok<'input> {
    Word(&'input str),
    StringLiteral(&'input str),
    ParenOpen,
    ParenClose,
}

/// Our custom lexer implementation
pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    /// Constructs a new lexer from the given text input
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>>;

    fn next(&mut self) -> Option<Self::Item> {
        enum LexerState {
            Default,
            Word(usize),
            StringLiteral(usize),
            StringLiteralEscape(usize),
            CommentStart(usize),
            LineComment(usize),
            BlockComment(usize),
            BlockCommentEnd(usize),
        }
        let mut lexer_state = LexerState::Default;

        loop {
            match self.chars.peek().cloned() {
                Some((i, c)) => match lexer_state {
                    LexerState::Default => {
                        // If we encounter an open paren in the default state we emit it directly
                        if c == '(' {
                            self.chars.next().unwrap();
                            return Some(Ok((i, Tok::ParenOpen, i + 1)));
                        }

                        // If we encounter a close paren in the default state we emit it directly
                        if c == ')' {
                            self.chars.next().unwrap();
                            return Some(Ok((i, Tok::ParenClose, i + 1)));
                        }

                        // If we encounter whitespace in the default space we skip it
                        if c.is_whitespace() {
                            self.chars.next().unwrap();
                            continue;
                        }

                        // If we encounter an open quote in the default state we move to the
                        // StringLiteral state and continue looping to consume the whole string
                        // token before yielding a value
                        if c == '"' {
                            // Skip the first index as we strip the quotes
                            lexer_state = LexerState::StringLiteral(i + 1);
                            self.chars.next().unwrap();
                            continue;
                        }

                        // A forward slash marks the beginning of a comment. We move to the
                        // CommentStart state and continue looping to determine what kind of comment
                        // is being read
                        if c == '/' {
                            lexer_state = LexerState::CommentStart(i);
                            self.chars.next().unwrap();
                            continue;
                        }

                        // If we encounter any character that is valid inside a word then we begin
                        // consuming the word, looping until the whole word has been consumed before
                        // yielding a value
                        if c.is_item_token() {
                            lexer_state = LexerState::Word(i);
                            self.chars.next().unwrap();
                            continue;
                        }

                        // If we reach this code we have failed to produce a valid token. This means
                        // we have encountered unexpected input and should emit an error
                        let expected = vec!["whitespace", "(", ")", "\"", "/", "word character"];
                        let err = Error::UnexpectedCharacter {
                            character: c,
                            expected,
                            position: i,
                        };
                        return Some(Err(err));
                    }
                    LexerState::StringLiteral(span_start) => {
                        // If we encounter a quote in the StringLiteral state it signifies the end
                        // of the string literal. We produce a string slice that encompasses the
                        // entire string literal's text and yield a StringLiteral token.
                        if c == '"' {
                            let span = &self.input[span_start..i];
                            let tok = Tok::StringLiteral(span);

                            self.chars.next().unwrap();
                            return Some(Ok((span_start, tok, i)));
                        }

                        // If we encounter a '\' character inside a string literal it marks the
                        // beginning of an escape sequence. We don't parse escape sequences in the
                        // lexer but we do need a special case to handle a quote escape \". If we
                        // didn't handle this specially a \" sequence would look like the string
                        // should be closed when it should not be.
                        if c == '\\' {
                            lexer_state = LexerState::StringLiteralEscape(span_start);
                        }

                        // All values other than a quote are valid inside a string literal so we
                        // must continue looping until one is reached to close the string
                        self.chars.next().unwrap();
                        continue;
                    }
                    LexerState::StringLiteralEscape(span_start) => {
                        // We don't have to actually parse an escape sequence here, just make sure
                        // we don't terminate the string early if there is a quote escape.
                        lexer_state = LexerState::StringLiteral(span_start);
                        self.chars.next().unwrap();
                        continue;
                    }
                    LexerState::Word(span_start) => {
                        // A word is defined as an unbroken sequence of purely word characters.
                        // Once we are in the Word state we consume every character until we reach
                        // an invalid item token
                        if !c.is_item_token() {
                            let span = &self.input[span_start..i];
                            let tok = Tok::Word(span);

                            return Some(Ok((span_start, tok, i)));
                        }

                        // If we reach this code we know `c.is_item_token()` is true, which means
                        // c is a valid word character and so we should commit the parse and keep
                        // looping
                        self.chars.next().unwrap();
                        continue;
                    }
                    LexerState::CommentStart(span_start) => {
                        // In the comment start state we have read off the start of a comment. Now
                        // we need to determine what type of comment is being read based on the
                        // next token and handle it accordingly.

                        // If we encounter a second '/' token (i.e "//") then we must be handling
                        // a line comment.
                        if c == '/' {
                            lexer_state = LexerState::LineComment(span_start);
                            self.chars.next().unwrap();
                            continue;
                        }

                        // If we encounter a '*' token (i.e "/*") then we must be handling a block
                        // comment.
                        if c == '*' {
                            lexer_state = LexerState::BlockComment(span_start);
                            self.chars.next().unwrap();
                            continue;
                        }

                        // If we don't encounter either a "/" or "*" then the comment prefix is
                        // invalid. An error should be thrown
                        let expected = vec!["/", "*"];
                        let err = Error::UnexpectedCharacter {
                            character: c,
                            expected,
                            position: i,
                        };
                        return Some(Err(err));
                    }
                    LexerState::LineComment(_span_start) => {
                        // We are now in the body of a line comment

                        // A line comment will endlessly skip characters until a newline character
                        // is encountered. As such we just keep looping and consuming, only breaking
                        // out of the line comment state when a newline is encountered.
                        if c == '\n' || c == '\r' {
                            lexer_state = LexerState::Default;
                        }

                        self.chars.next().unwrap();
                        continue;
                    }
                    LexerState::BlockComment(span_start) => {
                        // We are now in the body of a block comment.
                        //
                        // A block comment has a multi-token end sequence so requires multiple
                        // states to end

                        // The block comment will continuously consume tokens until it encounters
                        // a "*/" sequence. This is the first stage of this check
                        if c == '*' {
                            lexer_state = LexerState::BlockCommentEnd(span_start);
                        }

                        self.chars.next().unwrap();
                        continue;
                    }
                    LexerState::BlockCommentEnd(span_start) => {
                        // If a '/' is encountered in this state then it means the block comment has
                        // been terminated and we should resume normal parsing.
                        //
                        // If a non '/' token is found we assume we are still inside the block
                        // comment body and a plain '*' token had been read.
                        if c == '/' {
                            lexer_state = LexerState::Default;
                        } else {
                            lexer_state = LexerState::BlockComment(span_start);
                        }

                        self.chars.next().unwrap();
                        continue;
                    }
                },
                None => {
                    return match lexer_state {
                        // In the default state an end-of-stream is valid
                        LexerState::Default => None,

                        // A word token can be terminated by end-of-stream, so we consider this
                        // as a word terminator and emit a word.
                        //
                        // This is a valid lex but will not be a valid parse as a word must be
                        // enclosed within a list, which isn't possible as a word->end-of-stream
                        // would imply an unclosed list.
                        LexerState::Word(span_start) => {
                            let span = &self.input[span_start..];
                            let tok = Tok::StringLiteral(span);
                            Some(Ok((span_start, tok, self.input.len() - 1)))
                        }

                        // A string literal must be terminated by a quote token, so if we are
                        // expecting one when we get end-of-stream then we should emit an error
                        LexerState::StringLiteral(span_start) => {
                            let err = Error::UnclosedStringLiteral { begin: span_start };
                            Some(Err(err))
                        }
                        LexerState::StringLiteralEscape(span_start) => {
                            let err = Error::UnclosedStringLiteral { begin: span_start };
                            Some(Err(err))
                        }

                        // A bare '/' character is an invalid token
                        LexerState::CommentStart(span_start) => {
                            let err = Error::UnexpectedCharacter {
                                character: '/',
                                expected: Vec::new(),
                                position: span_start,
                            };
                            Some(Err(err))
                        }

                        // A line comment is terminated by either a new-line or an end-of-stream, so
                        // this is valid
                        LexerState::LineComment(_) => None,

                        // A block comment must be terminated by a "*/" token, so if we are
                        // expecting one when we get end-of-stream then we should emit an error
                        LexerState::BlockComment(_) => {
                            let expected = vec!["/", "*"];
                            let err = Error::UnexpectedEndOfInput { expected };
                            Some(Err(err))
                        }
                        LexerState::BlockCommentEnd(_) => {
                            let expected = vec!["/"];
                            let err = Error::UnexpectedEndOfInput { expected };
                            Some(Err(err))
                        }
                    };
                }
            }
        }
    }
}
