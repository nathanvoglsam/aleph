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

use std::cell::Cell;
use std::ops::{Deref, DerefMut};

use bumpalo::Bump;

pub struct BumpCell(Cell<Option<Bump>>);

impl BumpCell {
    pub fn new() -> Self {
        Self(Cell::new(Some(Bump::new())))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Cell::new(Some(Bump::with_capacity(capacity))))
    }

    pub fn scope(&self) -> BumpScope {
        let bump = self
            .0
            .take()
            .expect("A BumpScope for this BumpCell already exists.");
        BumpScope {
            cell: &self.0,
            bump: Some(bump),
        }
    }
}

impl Default for BumpCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BumpScope<'a> {
    cell: &'a Cell<Option<Bump>>,
    bump: Option<Bump>,
}

impl<'a> Drop for BumpScope<'a> {
    fn drop(&mut self) {
        let mut cell = self.bump.take();

        // Safety: We statically guarantee 'bump' is always Some, except in Drop where it's
        //         impossible to observe as the object is no longer accessible.
        unsafe {
            let bump = cell.as_mut().unwrap_unchecked();
            bump.reset()
        }

        self.cell.set(cell)
    }
}

impl<'a> Deref for BumpScope<'a> {
    type Target = Bump;

    fn deref(&self) -> &Self::Target {
        // Safety: We statically guarantee 'bump' is always Some, except in Drop where it's
        //         impossible to observe as the object is no longer accessible.
        unsafe { self.bump.as_ref().unwrap_unchecked() }
    }
}

impl<'a> DerefMut for BumpScope<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: We statically guarantee 'bump' is always Some, except in Drop where it's
        //         impossible to observe as the object is no longer accessible.
        unsafe { self.bump.as_mut().unwrap_unchecked() }
    }
}
