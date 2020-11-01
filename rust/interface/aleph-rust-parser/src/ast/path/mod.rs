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

use crate::interner::{Interner, StrId};

/// Internal struct for interning a rust path
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Path {
    pub segments: Vec<StrId>,
    pub absolute: bool,
}

impl Path {
    pub fn new<T: IntoIterator<Item = StrId>>(segments: T, absolute: bool) -> Path {
        Path {
            segments: segments.into_iter().collect(),
            absolute,
        }
    }

    pub fn from_syn(interner: &mut Interner, path: &syn::Path) -> Path {
        let segments: Vec<StrId> = path
            .segments
            .iter()
            .map(|v| interner.intern(v.ident.to_string()))
            .collect();
        Self {
            segments,
            absolute: false,
        }
    }

    pub fn to_string(&self, interner: &Interner) -> String {
        let mut out = String::new();
        if self.absolute {
            out.push_str("::");
        }
        if self.segments.is_empty() {
            out
        } else {
            self.segments[0..self.segments.len() - 1]
                .iter()
                .for_each(|v| {
                    out.push_str(interner.lookup(*v));
                    out.push_str("::");
                });
            out.push_str(interner.lookup(*self.segments.last().unwrap()));
            out
        }
    }
}
