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

use crate::{Context, CtxString, RefValue};

pub struct Atom {
    pub(crate) v: raw::JSAtom,
    pub(crate) c: Context,
}

impl Atom {
    pub const fn to_raw(&self) -> raw::JSAtom {
        self.v
    }

    #[inline]
    pub fn to_value(&self) -> RefValue {
        unsafe {
            let string = raw::JS_AtomToValue(self.c.0.ctx, self.v);
            RefValue::from_raw(&self.c, string)
        }
    }

    #[inline]
    pub fn to_string(&self) -> RefValue {
        unsafe {
            let string = raw::JS_AtomToString(self.c.0.ctx, self.v);
            RefValue::from_raw(&self.c, string)
        }
    }

    #[inline]
    pub fn to_c_str(&self) -> Option<CtxString> {
        let string = self.to_string();
        if !string.is_exception() {
            string.to_c_str()
        } else {
            None
        }
    }
}

impl Clone for Atom {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let atom = raw::JS_DupAtom(self.c.0.ctx, self.v).unwrap();
            Self {
                v: atom,
                c: self.c.clone(),
            }
        }
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe {
            raw::JS_FreeAtom(self.c.0.ctx, self.v);
        }
    }
}
