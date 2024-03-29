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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use crossbeam::atomic::AtomicCell;

use crate::ValidationDevice;

pub struct ValidationFence {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IFence>,
    pub(crate) state: AtomicCell<FenceState>,
}

declare_interfaces!(ValidationFence, [IFence]);

crate::impl_platform_interface_passthrough!(ValidationFence);

impl IFence for ValidationFence {
    fn upgrade(&self) -> AnyArc<dyn IFence> {
        AnyArc::map::<dyn IFence, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
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
