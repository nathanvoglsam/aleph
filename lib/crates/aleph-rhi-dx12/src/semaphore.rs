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

use crate::device::Device;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use std::any::TypeId;
use std::sync::atomic::{AtomicU64, Ordering};
use windows::Win32::Graphics::Direct3D12::*;

pub struct Semaphore {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) fence: ID3D12Fence,

    /// Monotonically increasing counter that tracks what value a should be signalled or waited
    /// on when a semaphore is used.
    ///
    /// A semaphore is implemented with `ID3D12Fence`. It can only be used on the GPU. 'value' is
    /// used to track the next signal value. A semaphore will signal 'value' on a queue and then
    /// increment 'value'. It will then wait on 'value - 1' after being signalled. The counter
    /// always increases.
    ///
    /// This does mean eventually the counter will overflow, but overflowing the u64 counter here
    /// in practice would require a single renderer instance to run for millions of years. We do
    /// panic if you somehow manage it, but nobody will ever be alive to see it happen.
    pub(crate) value: AtomicU64,
}

declare_interfaces!(Semaphore, [ISemaphore]);

impl IGetPlatformInterface for Semaphore {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.fence, out, target)
    }
}

impl ISemaphore for Semaphore {
    fn upgrade(&self) -> AnyArc<dyn ISemaphore> {
        AnyArc::map::<dyn ISemaphore, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }
}

impl Semaphore {
    ///
    /// Internal implementation for the 'wait_semaphore' case.
    ///
    /// # Safety
    ///
    /// FFI to ID3D12CommandQueue::Wait
    ///
    pub unsafe fn wait_on_queue(&self, queue: &ID3D12CommandQueue) -> windows::core::Result<()> {
        // We subtract one, saturating to 0. It is invalid to use a semaphore in
        // 'wait_semaphores' before first being used as a 'signal_semaphore' in a previous
        // submission so valid usage guarantees this won't underflow.
        //
        // We saturate instead to simply avoid adding UB for no reason as this would trigger an
        // integer underflow in this case (as well as a GPU deadlock).
        //
        // The validation layers should catch this
        let wait_val = self.value.load(Ordering::Relaxed).saturating_sub(1);
        queue.Wait(&self.fence, wait_val)
    }

    ///
    /// Internal implementation for the 'signal_semaphore' case.
    ///
    /// # Safety
    ///
    /// FFI to ID3D12CommandQueue::Signal
    ///
    pub unsafe fn signal_on_queue(&self, queue: &ID3D12CommandQueue) -> windows::core::Result<()> {
        // Fetch add means we get the value we want to signal + increment to the next value fully
        // atomically. The subsequent 'wait' operation will use 'value - 1'.
        let signal_val = self.value.fetch_add(1, Ordering::Relaxed);

        // If we somehow managed to run a single renderer instance for 243 million years (assuming
        // you signalled the same semaphore 2400 times per second) then this will overflow.
        //
        // If you see this panic message, let me know how humanity is going.
        assert_ne!(signal_val, u64::MAX, "Semaphore internal value overflow!");

        queue.Signal(&self.fence, signal_val)
    }

    ///
    /// Signals the semaphore from the CPU timeline. This is used to emulate operations that should
    /// signal semaphores, but D3D12 doesn't. Namely swap image acquisition.
    ///
    /// # Safety
    ///
    /// FFI to ID3D12CommandQueue::Signal
    ///
    pub unsafe fn signal_from_cpu(&self) -> windows::core::Result<()> {
        // Fetch add means we get the value we want to signal + increment to the next value fully
        // atomically. The subsequent 'wait' operation will use 'value - 1'.
        let signal_val = self.value.fetch_add(1, Ordering::Relaxed);

        // If we somehow managed to run a single renderer instance for 243 million years (assuming
        // you signalled the same semaphore 2400 times per second) then this will overflow.
        //
        // If you see this panic message, let me know how humanity is going.
        assert_ne!(signal_val, u64::MAX, "Semaphore internal value overflow!");

        self.fence.Signal(signal_val)
    }
}
