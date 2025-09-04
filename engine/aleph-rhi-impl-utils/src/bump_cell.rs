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

use aleph_alloc::{Blink, BlinkAlloc};

use crate::RhiSystem;

pub type BlinkCellAlloc = Blink<BlinkAlloc<RhiSystem>>;

pub struct BlinkCell(Cell<Option<Box<BlinkCellAlloc>>>);

impl BlinkCell {
    #[inline]
    pub fn new() -> Self {
        let v = RhiSystem::default();
        let v = BlinkAlloc::new_in(v);
        let v = Blink::new_in(v);
        let v = Box::new(v);
        Self(Cell::new(Some(v)))
    }

    #[inline]
    pub fn scope(&self) -> BlinkScope<'_> {
        let bump = self
            .0
            .take()
            .expect("A BlinkScope for this BlinkCell already exists.");
        BlinkScope {
            cell: &self.0,
            bump: Some(bump),
        }
    }
}

impl Default for BlinkCell {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlinkScope<'a> {
    cell: &'a Cell<Option<Box<BlinkCellAlloc>>>,
    bump: Option<Box<BlinkCellAlloc>>,
}

impl<'a> Drop for BlinkScope<'a> {
    #[inline]
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

impl<'a> Deref for BlinkScope<'a> {
    type Target = BlinkCellAlloc;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // Safety: We statically guarantee 'bump' is always Some, except in Drop where it's
        //         impossible to observe as the object is no longer accessible.
        unsafe { self.bump.as_ref().unwrap_unchecked() }
    }
}

impl<'a> DerefMut for BlinkScope<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: We statically guarantee 'bump' is always Some, except in Drop where it's
        //         impossible to observe as the object is no longer accessible.
        unsafe { self.bump.as_mut().unwrap_unchecked() }
    }
}
