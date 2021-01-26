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

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{AttrStyle, Attribute, AttributeArgs, Path};

///
/// A builder wrapper around the `syn` crate's `Attribute` type that makes building attributes less
/// verbose.
///
pub struct AttributeBuilder {
    inner: Attribute,
}

impl AttributeBuilder {
    ///
    /// Creates a new `AttributeBuilder` for an attribute with the name given by `path`
    ///
    pub fn new(path: Path) -> Self {
        Self {
            inner: Attribute {
                pound_token: Default::default(),
                style: AttrStyle::Outer,
                bracket_token: Default::default(),
                path,
                tokens: Default::default(),
            },
        }
    }

    ///
    /// Consumes the builder and returns the created `Attribute`
    ///
    pub fn build(self) -> Attribute {
        self.inner
    }

    ///
    /// Marks this as an inner attribute
    ///
    pub fn inner(&mut self) -> &mut Self {
        self.inner.style = AttrStyle::Inner(Default::default());
        self
    }

    ///
    /// Marks this as an outer attribute
    ///
    pub fn outer(&mut self) -> &mut Self {
        self.inner.style = AttrStyle::Outer;
        self
    }

    ///
    /// Add a raw stream of tokens to the
    ///
    pub fn raw_tokens(&mut self, tokens: TokenStream) -> &mut Self {
        self.inner.tokens = tokens;
        self
    }

    ///
    /// Convert the given `AttributeArgs` struct into a token stream and use it as the attributes
    /// token stream.
    ///
    pub fn arguments(&mut self, mut args: AttributeArgs) -> &mut Self {
        let mut stream = TokenStream::new();
        args.drain(..).for_each(|token| {
            token.to_tokens(&mut stream);
        });
        self.raw_tokens(stream)
    }
}
