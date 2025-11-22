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

use std::ptr::NonNull;

use raw::JSPropertyEnum;

use crate::{Atom, Context};

pub struct PropertyEnum {
    pub is_enumerable: bool,
    pub atom: Option<Atom>,
}

/// Wrapper type over the result of [`raw::JS_GetOwnPropertyNames`]. Manages freeing the result
/// array when the 'Self' is dropped.
pub struct OwnPropertyNames {
    pub(crate) ctx: Context,

    /// Reference to the context to keep it alive
    pub(crate) props: NonNull<[JSPropertyEnum]>,
}

impl OwnPropertyNames {
    pub fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item = PropertyEnum> + 'a {
        unsafe {
            self.props.as_ref().iter().map(|v| PropertyEnum {
                is_enumerable: v.is_enumerable,
                atom: v.atom.map(|v| {
                    let v = raw::JS_DupAtom(self.ctx.0.ctx, v).unwrap();
                    Atom {
                        v,
                        c: self.ctx.clone(),
                    }
                }),
            })
        }
    }
}

impl Drop for OwnPropertyNames {
    fn drop(&mut self) {
        if !self.props.is_empty() {
            // Safety: if props is not empty then we're guanteed to have a valid pointer (job of
            //         GetOwnPropertyIter constructor) that needs to be freed with the provided
            //         function.
            unsafe {
                let props = &mut *self.props.as_ptr();

                // 'len' is guaranteed to fit in u32 as we got the length from quickjs in the first
                // place.
                raw::JS_FreePropertyEnum(
                    self.ctx.0.ctx,
                    props.as_mut_ptr() as *mut _,
                    self.props.len() as u32,
                );
            }
        }
    }
}
