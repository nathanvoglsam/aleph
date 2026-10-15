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

#![cfg(not(any(feature = "profile-with-tracy")))]

//! Fallback 'no-op' profiler implementation of the GPU API. This gets re-exported when one of the
//! real profiling integration libraries is not available.

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct QueueId {
    _private: (),
}

impl crate::tracy_gpu::TracyQueue for QueueId {
    fn new_queue_id(
        _name: &std::ffi::CStr,
        _context_type: crate::tracy_gpu::ContextType,
        _calibrated: bool,
        _gpu_time: u64,
        _period: f32,
    ) -> Self {
        Self { _private: () }
    }

    fn time_domain_calibration(_queue_id: Self, _gpu_time: u64, _delta: u64) {
        // Intentional no-op
    }

    unsafe fn begin_span_alloc(
        _name: &std::ffi::CStr,
        _function: &std::ffi::CStr,
        _file: &std::ffi::CStr,
        _line: u32,
        _queue_id: Self,
        _query_id: u16,
    ) {
        // Intentional no-op
    }

    unsafe fn end_span(_queue_id: Self, _query_id: u16) {
        // Intentional no-op
    }

    unsafe fn publish_gpu_time(_queue_id: Self, _query_id: u16, _gpu_time: u64) {
        // Intentional no-op
    }
}
