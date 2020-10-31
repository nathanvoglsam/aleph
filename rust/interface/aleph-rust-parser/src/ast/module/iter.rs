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

use crate::ast::Module;
use crate::interner::StrId;

/// Internal iterator type used for walking a module graph in a depth first traversal.
pub enum IterUnionMut<'a> {
    Root(Option<(StrId, &'a mut Module)>),
    Map(std::collections::hash_map::IterMut<'a, StrId, Module>),
}

impl<'a> Iterator for IterUnionMut<'a> {
    type Item = (StrId, &'a mut Module);

    fn next(&mut self) -> Option<(StrId, &'a mut Module)> {
        match self {
            IterUnionMut::Root(item) => item.take(),
            IterUnionMut::Map(iter) => iter.next().map(|v| (*v.0, v.1)),
        }
    }
}

/// Internal iterator type used for walking a module graph in a depth first traversal.
pub enum IterUnion<'a> {
    Root(Option<(StrId, &'a Module)>),
    Map(std::collections::hash_map::Iter<'a, StrId, Module>),
}

impl<'a> Iterator for IterUnion<'a> {
    type Item = (StrId, &'a Module);

    fn next(&mut self) -> Option<(StrId, &'a Module)> {
        match self {
            IterUnion::Root(item) => item.take(),
            IterUnion::Map(iter) => iter.next().map(|v| (*v.0, v.1)),
        }
    }
}
