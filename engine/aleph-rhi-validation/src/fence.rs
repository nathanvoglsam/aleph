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
use crossbeam::atomic::AtomicCell;

use crate::ValidationDevice;

pub struct ValidationFence {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: FenceHandle,
    pub(crate) state: AtomicCell<FenceState>,
}

unsafe_impl_iobject!(ValidationFence, "01944f8f-95b6-7d51-b2da-23931a5b3fc5");

impl ValidationFence {
    pub(crate) fn get(v: &FenceHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Fence implementation!")
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum FenceState {
    /// The state of a fence in the following conditions:
    /// - After creation when created as unsignalled
    /// - After the fence is reset but before it is used as a signal target again
    ///
    /// In this state it is valid to do the following:
    /// - Reset the fence. Simply does nothing.
    /// - Use the fence as a signal target.
    /// - Wait on the fence. But only with a timeout.
    /// - Poll the state of the fence as signalled or unsignalled.
    NotSignalled,

    /// The state a fence transitions to when used to wait on GPU work in a queue submission. This
    /// state is specifically only active before the GPU work is complete.
    ///
    /// In this state it is valid to do the following:
    /// - Wait on the fence.
    /// - Poll the state of the fence as signalled or unsignalled.
    Pending,

    /// The state a fence transitions to when the GPU work it is waiting on is completed. The
    /// transition specifically only happens when the fence is actually waited on. An outstanding
    /// fence only becomes 'waited' after use in 'wait_fences'.
    ///
    /// In this state it is valid to do the following:
    /// - Reset the fence. Returns to the NotSignalled state.
    /// - Wait on the fence.
    /// - Poll the state of the fence as signalled or unsignalled.
    ObservedAsSignalled,
}
