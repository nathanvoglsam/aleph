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

use aleph_any::AnyArc;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;

use crate::device::Device;

pub struct Semaphore {
    pub(crate) _device: AnyArc<Device>,
}

unsafe_impl_iobject!(Semaphore, "01980753-5c4f-7ae3-be3b-9730007ecfaf");

impl Semaphore {
    pub(crate) fn get(v: &SemaphoreHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Semaphore implementation!")
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        todo!()
    }
}
