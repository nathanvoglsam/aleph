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

use std::alloc::{handle_alloc_error, Layout};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_object_system::uuid::Uuid;
use aleph_object_system::ObjectDescription;
use allocator_api2::alloc::{Allocator, Global};

use crate::Resource;

#[derive(Default)]
pub struct Resources {
    pub(crate) resources: HashMap<Uuid, UnsafeCell<ResourceBox>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: Resource>(&mut self, v: T) {
        let _ = self
            .resources
            .insert(T::ID, UnsafeCell::new(ResourceBox::new(v)));
    }

    pub fn get_ref<T: Resource>(&self) -> Option<&T> {
        let cell = self.resources.get(&T::ID)?;
        unsafe {
            let out = (&*cell.get()).get_ref::<T>();
            out
        }
    }

    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        let cell = self.resources.get_mut(&T::ID)?;
        let out = cell.get_mut().get_mut::<T>();
        out
    }

    pub fn take<T: Resource>(&mut self) -> Option<T> {
        let cell = self.resources.remove(&T::ID)?;
        cell.into_inner().into_inner::<T>()
    }

    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

unsafe impl Send for Resources {}
unsafe impl Sync for Resources {}

pub(crate) struct ResourceBox {
    ptr: NonNull<()>,
    desc: ObjectDescription,
}

unsafe impl Send for ResourceBox {}
unsafe impl Sync for ResourceBox {}

impl ResourceBox {
    fn new<T: Resource>(v: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = Global.allocate(layout);
        match ptr {
            Ok(ptr) => unsafe {
                let ptr: NonNull<MaybeUninit<T>> = ptr.cast();
                ptr.write(MaybeUninit::new(v));
                let ptr: NonNull<()> = ptr.cast();
                Self {
                    ptr,
                    desc: ObjectDescription::get::<T>(),
                }
            },
            Err(_) => handle_alloc_error(layout),
        }
    }

    pub(crate) fn into_inner<T: Resource>(mut self) -> Option<T> {
        let object = self.get_ptr::<T>()?;
        let out = unsafe { object.read() };

        // Prevent the box from calling a drop function by nulling the drop fn. Leave freeing the
        // buffer to the drop implementation
        self.desc.destructor = None;

        Some(out)
    }

    pub(crate) fn get_ref<T: Resource>(&self) -> Option<&T> {
        self.get_ptr::<T>().map(|v| unsafe { v.as_ref() })
    }

    pub(crate) fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.get_ptr::<T>().map(|mut v| unsafe { v.as_mut() })
    }

    pub(crate) fn get_ptr<T: Resource>(&self) -> Option<NonNull<T>> {
        if T::ID == self.desc.id {
            Some(self.ptr.cast::<T>())
        } else {
            None
        }
    }
}

impl Drop for ResourceBox {
    fn drop(&mut self) {
        if let Some(f) = self.desc.destructor {
            // Safety: It is unsafe to create a ResourceBox where this isn't safe to call. So this
            //         is safe as the real unsafety is elsewhere
            unsafe { f(self.ptr, 1) }
        }

        let layout = Layout::from_size_align(self.desc.size, self.desc.align).unwrap();
        // Safety: It is unsafe to construct a ResourceBox where this isn't safe to call, much like
        //         the above destructor call.
        unsafe {
            Global.deallocate(self.ptr.cast(), layout);
        }
    }
}
