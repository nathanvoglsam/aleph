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

#![cfg(feature = "profile-with-tracy")]

use tracy_client::sys;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct QueueId {
    id: u8,
}

impl crate::tracy_gpu::TracyQueue for QueueId {
    fn new_queue_id(
        name: &std::ffi::CStr,
        context_type: crate::tracy_gpu::ContextType,
        calibrated: bool,
        gpu_time: u64,
        period: f32,
    ) -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering};

        tracy_client::Client::start();

        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        if id > u8::MAX as usize {
            panic!("Too many profiler GPU queue IDs");
        }

        // Safety: We've just generated a queue id that we guarantee to be unique so we're the first
        //         and only callers of the profiler hooks.
        unsafe {
            sys::___tracy_emit_gpu_new_context_serial(sys::___tracy_gpu_new_context_data {
                gpuTime: gpu_time as i64,
                period,
                context: id as u8,
                flags: if calibrated { 1 } else { 0 },
                type_: context_type as u8,
            });
            sys::___tracy_emit_gpu_context_name_serial(sys::___tracy_gpu_context_name_data {
                context: id as u8,
                name: name.as_ptr(),
                len: name.to_bytes_with_nul().len().min(u16::MAX as usize) as u16,
            });
        }

        Self { id: id as u8 }
    }

    fn time_domain_calibration(queue_id: Self, gpu_time: u64, delta: u64) {
        tracy_client::Client::start();

        if delta <= 0 {
            return;
        }
        unsafe {
            sys::___tracy_emit_gpu_calibration_serial(sys::___tracy_gpu_calibration_data {
                gpuTime: gpu_time as i64,
                cpuDelta: delta as i64,
                context: queue_id.id,
            });
        }
    }

    unsafe fn begin_span_alloc(
        name: &std::ffi::CStr,
        function: &std::ffi::CStr,
        file: &std::ffi::CStr,
        line: u32,
        queue_id: Self,
        query_id: u16,
    ) {
        tracy_client::Client::start();
        unsafe {
            let srcloc = sys::___tracy_alloc_srcloc_name(
                line,
                file.as_ptr().cast(),
                file.to_bytes_with_nul().len(),
                function.as_ptr().cast(),
                function.to_bytes_with_nul().len(),
                name.as_ptr().cast(),
                name.to_bytes_with_nul().len(),
                0,
            );
            sys::___tracy_emit_gpu_zone_begin_alloc_serial(sys::___tracy_gpu_zone_begin_data {
                srcloc,
                queryId: query_id,
                context: queue_id.id,
            });
        };
    }

    unsafe fn end_span(queue_id: Self, query_id: u16) {
        tracy_client::Client::start();
        unsafe {
            sys::___tracy_emit_gpu_zone_end_serial(sys::___tracy_gpu_zone_end_data {
                queryId: query_id,
                context: queue_id.id,
            });
        };
    }

    unsafe fn publish_gpu_time(queue_id: Self, query_id: u16, gpu_time: u64) {
        tracy_client::Client::start();
        unsafe {
            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: gpu_time as i64,
                queryId: query_id,
                context: queue_id.id,
            });
        }
    }
}
