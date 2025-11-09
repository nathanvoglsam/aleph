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

use std::sync::atomic::{AtomicU64, Ordering};

use aleph_any::AnyArc;
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use crate::device::Device;

pub struct Fence {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) objects: FenceObjects,

    /// Monotonically increasing counter that tracks what value a should be signalled or waited
    /// on when a fence is used.
    ///
    /// This does mean eventually the counter will overflow, but overflowing the u64 counter here
    /// in practice would require a single renderer instance to run for millions of years. We do
    /// panic if you somehow manage it, but nobody will ever be alive to see it happen.
    pub(crate) value: AtomicU64,
}

unsafe_impl_iobject!(Fence, "01980753-5c4f-7ae3-be3b-96ea6487c813");

impl Fence {
    pub(crate) fn create(
        device: &Device,
        signalled: bool,
    ) -> Result<FenceHandle, FenceCreateError> {
        let event = match device.device.newSharedEvent() {
            Some(v) => v,
            None => return Err(FenceCreateError::Platform),
        };

        if signalled {
            event.setSignaledValue(1);
        }

        let fence = Fence {
            _device: device.this.upgrade().unwrap(),
            objects: FenceObjects { event },
            value: AtomicU64::new(2),
        };
        let fence = Object::new_arc_opaque(fence);
        unsafe { Ok(FenceHandle::new(fence)) }
    }

    pub(crate) fn get(v: &FenceHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Fence implementation!")
    }

    /// Returns what value the fence is currently waiting to become signalled via the attached
    /// MTLEvent object.
    ///
    /// This is used internally for [`IDevice::wait_fences`].
    pub fn get_wait_value(&self) -> u64 {
        // We use a saturating sub to protect against an underflow leading to big time sadness. A
        // default 'value' of 0 would lead to an underflow. If someone were to attempt to wait on
        // a fence that hadn't been queued to be signalled then the wait value would be u64::MAX.
        //
        // u64::MAX will never be signalled (on a human timeframe anyway), so you will just deadlock
        // the thread waiting on this value.
        //
        // It isn't legal to wait on a fence in this way in aleph-rhi, but the cost of this is low
        // so I think it's better to protect against a guaranteed deadlock here.
        self.value.load(Ordering::Relaxed).saturating_sub(1)
    }

    /// Increment the internal counter and return the value that should be signalled on a queue
    /// in order to mark this fence complete.
    ///
    /// This is used internally for [`IQueue::submit`].
    pub fn get_next_signal_value(&self) -> u64 {
        // Fetch add means we get the value we want to signal + increment to the next value fully
        // atomically. The subsequent 'wait' operation will use 'value - 1'.
        let signal_val = self.value.fetch_add(1, Ordering::Relaxed);

        // If we somehow managed to run a single renderer instance for 243 million years (assuming
        // you signalled the same fence 2400 times per second) then this will overflow.
        //
        // If you see this panic message, let me know how humanity is going.
        assert_ne!(signal_val, u64::MAX, "Semaphore internal value overflow!");

        signal_val
    }

    /// Returns if the fence has been signalled
    pub fn poll_signalled(&self) -> bool {
        let value = self.get_wait_value();
        self.objects.event.signaledValue() >= value
    }
}

pub struct FenceObjects {
    pub event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
}
