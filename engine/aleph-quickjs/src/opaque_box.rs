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

use std::any::{Any, TypeId};
use std::ptr::NonNull;

#[repr(C)]
pub(crate) struct UntypedOpaqueBox {
    pub(crate) type_id: TypeId,
    pub(crate) dropper: unsafe fn(NonNull<UntypedOpaqueBox>),
}

impl UntypedOpaqueBox {
    pub(crate) fn try_to_typed<T: Any + Sized>(&self) -> Option<&OpaqueBox<T>> {
        if self.type_id == TypeId::of::<T>() {
            let out: &OpaqueBox<T> = unsafe { std::mem::transmute(self) };
            Some(out)
        } else {
            None
        }
    }

    pub(crate) unsafe fn drop_inner(v: NonNull<Self>) {
        unsafe {
            let dropper = v.as_ref().dropper;
            (dropper)(v)
        }
    }
}

#[repr(C)]
pub(crate) struct OpaqueBox<T> {
    pub(crate) type_id: TypeId,
    pub(crate) dropper: unsafe fn(NonNull<UntypedOpaqueBox>),
    pub(crate) v: T,
}

impl<T: Any + Sized> OpaqueBox<T> {
    pub(crate) fn new(v: T) -> NonNull<UntypedOpaqueBox> {
        let opaque = Self {
            type_id: TypeId::of::<T>(),
            dropper: Self::dropper_fn,
            v,
        };
        let opaque = Box::new(opaque);
        let opaque = Box::leak(opaque);
        NonNull::from(opaque).cast::<UntypedOpaqueBox>()
    }

    pub(crate) unsafe fn dropper_fn(v: NonNull<UntypedOpaqueBox>) {
        unsafe {
            let v = v.cast::<OpaqueBox<T>>();
            let _ = Box::from_raw(v.as_ptr());
        }
    }
}
