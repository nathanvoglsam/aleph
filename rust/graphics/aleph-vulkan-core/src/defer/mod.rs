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

use crossbeam::queue::SegQueue;

///
/// Type alias for the box used for storing deferral objects
///
pub type DeferBox<T> = Box<T>;

///
/// A generic defer list
///
/// This should not be used as a direct public interface. It should be used behind a wrapper type
/// that provides more concrete rules for the lifetime of the DeferList object.
///
pub struct DeferList<T: ?Sized> {
    list: SegQueue<DeferBox<T>>,
}

impl<T: ?Sized> DeferList<T> {
    ///
    /// Creates a new device defer list
    ///
    pub fn new() -> Self {
        Self {
            list: Default::default(),
        }
    }

    ///
    /// Adds a defer item into
    ///
    pub fn add(&self, item: DeferBox<T>) {
        self.list.push(item);
    }

    ///
    /// Consume all deferred items by iterating over the list and calling the functor for each item.
    ///
    pub fn consume(&self, mut func: impl FnMut(DeferBox<T>)) {
        while let Some(item) = self.list.pop() {
            func(item);
        }
    }
}
