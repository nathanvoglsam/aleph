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

use crate::{IPixelStorage, ImageBuffer, PixelFormat};

pub trait IPixelAccess: IPixelStorage {
    type Result: PixelFormat;

    fn load(&self, x: u32, y: u32) -> Self::Result {
        match self.load_checked(x, y) {
            Some(v) => v,
            None => {
                let i =
                    calculate_index(x, y, self.width(), self.height(), Self::Result::COMPONENTS);
                let i_end = i + Self::Result::COMPONENTS;
                panic!(
                    "({x}, {y}) is out of bounds! Texture dimensions = ({}, {}). Tried loading range({}..{})",
                    self.width(),
                    self.height(),
                    i,
                    i_end
                );
            }
        }
    }

    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result>;

    unsafe fn load_unchecked(&self, x: u32, y: u32) -> Self::Result {
        self.load_checked(x, y).unwrap_unchecked()
    }

    fn store(&mut self, x: u32, y: u32, v: Self::Result) {
        match self.store_checked(x, y, v) {
            Some(v) => v,
            None => panic!(
                "({x}, {y}) is out of bounds! Texture dimensions = ({}, {})",
                self.width(),
                self.height()
            ),
        }
    }

    fn store_checked(&mut self, x: u32, y: u32, v: Self::Result) -> Option<()>;

    unsafe fn store_unchecked(&mut self, x: u32, y: u32, v: Self::Result) {
        self.store_checked(x, y, v).unwrap_unchecked()
    }
}

impl<T: PixelFormat> IPixelAccess for ImageBuffer<T> {
    type Result = T;

    #[inline]
    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(v) = self.data.get(i..i_end) {
            Some(T::from_storage(v))
        } else {
            None
        }
    }

    #[inline]
    fn store_checked(&mut self, x: u32, y: u32, v: T) -> Option<()> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(dst) = self.data.get_mut(i..i_end) {
            Some(v.write_at(dst))
        } else {
            None
        }
    }
}

const fn calculate_index(x: u32, y: u32, width: u32, _height: u32, components: usize) -> usize {
    let row_stride = width as usize * components;
    (row_stride * y as usize) + (x as usize * components)
}
