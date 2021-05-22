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

use crate::parsers::{ident, literal};
use combine::{choice, Parser, Stream};

///
/// A parser that attempts to parse out an atom
///
pub fn atom<Input: Stream<Token = char>>() -> impl Parser<Input, Output = ast::untyped::Item> {
    // The order to attempt to parse elements in
    let parsers = (
        // Try a string first as it is delimited
        literal::string(),
        // Float literal is superset of integer so parse first as an integer will parse first
        literal::float(),
        // Integer literal will start with number unlike ident so check this first
        literal::integer(),
        // Lastly check for an ident
        ident::ident(),
    );
    choice(parsers).map(|v| ast::untyped::Item::Atom(v))
}
