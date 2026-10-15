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

//! This module provide a low-level, toggleable wrapper over the tracy GPU api.
//!
//! The wrapper provided by the tracy-client crate doesn't suit the low level integrations we use
//! in aleph-engine, so instead we provide our own wrapper.
//!
//! If compiled without the "profile-with-tracy" feature this will instead re-export a no-op
//! implementation that is API compatible.
//!
//! However, depending on your integration it may be required to employ some dynamic checks to
//! ensure that profiling information isn't passed through opaque interfaces where the compiler
//! can't prove the code is unused.
//!
//! PIX also provides its own, D3D12 exclusive API for getting profile hooks on the GPU timeline.
//! These, however, require passing handles to D3D12 objects so can't easily be abstracted over.

pub mod impl_empty;
pub mod impl_tracy;

/// Interface exposed for profiling a GPU queue with tracy.
pub trait TracyQueue {
    /// Constructs a new `Self`, which represents a GPU queue identifier for the tracy profiler.
    ///
    /// Associates the given name with the queue, which will be displayed in the profiler.
    ///
    /// - `context_type` flags to tracy what GPU API is in use, and should be set appropriately.
    /// - `calibrated` flags whether the CPU and GPU have a calibrated time domain.
    /// - `gpu_time` is an initial timestamp captured from the GPU. This is captured from the GPU
    ///   using the APIs timestamp calibration mechanism and should have been captured as close in
    ///   time as possible to the call to this function.
    /// - `period`
    fn new_queue_id(
        name: &std::ffi::CStr,
        context_type: ContextType,
        calibrated: bool,
        gpu_time: u64,
        period: f32,
    ) -> Self;

    /// Sends a time domain calibration event to the profiler for the given queue.
    fn time_domain_calibration(queue_id: Self, gpu_time: u64, delta: u64);

    /// Marks the beginning of a span on the CPU and associates a query_id value with it. Eventually
    /// the client must provide a gpu timestamp for this call. The tracy profiler will then be able
    /// to show the CPU and GPU timelines together.
    ///
    /// # Safety
    ///
    /// Each call to [`TracyQueue::begin_span_alloc`] _must_ have exactly one matching call to
    /// [`TracyQueue::end_span`] and [`TracyQueue::publish_gpu_time`] with a matching query_id
    /// value. You must also call `publish_gpu_time` for the `end_span` event too.
    ///
    /// I can't find much documentation that describes this requirement, but if any one of those 3
    /// calls is missing either the profiler will crash or the app will crash. What this means is
    /// that once a span has been opened with some query_id, you _must_ also end the span and you
    /// _must_ eventually publish a gpu time value before reusing the id.
    ///
    /// If you don't you're in for sadness.
    unsafe fn begin_span_alloc(
        name: &std::ffi::CStr,
        function: &std::ffi::CStr,
        file: &std::ffi::CStr,
        line: u32,
        queue_id: Self,
        query_id: u16,
    );

    /// Marks the end of a span on the CPU and associates a query_id value with it. Eventually
    /// the client must provide a gpu timestamp for this call. The tracy profiler will then be able
    /// to show the CPU and GPU timelines together.
    ///
    /// # Safety
    ///
    /// Each call to [`TracyQueue::begin_span_alloc`] _must_ have exactly one matching call to
    /// [`TracyQueue::end_span`] and [`TracyQueue::publish_gpu_time`] with a matching query_id
    /// value. You must also call `publish_gpu_time` for the `end_span` event too.
    ///
    /// I can't find much documentation that describes this requirement, but if any one of those 3
    /// calls is missing either the profiler will crash or the app will crash. What this means is
    /// that once a span has been opened with some query_id, you _must_ also end the span and you
    /// _must_ eventually publish a gpu time value before reusing the id.
    ///
    /// If you don't you're in for sadness.
    unsafe fn end_span(queue_id: Self, query_id: u16);

    /// Must be called for every begin/end span to upload the gpu timestamp for the event.
    ///
    /// # Safety
    ///
    /// Each call to [`TracyQueue::begin_span_alloc`] _must_ have exactly one matching call to
    /// [`TracyQueue::end_span`] and [`TracyQueue::publish_gpu_time`] with a matching query_id
    /// value. You must also call `publish_gpu_time` for the `end_span` event too.
    ///
    /// I can't find much documentation that describes this requirement, but if any one of those 3
    /// calls is missing either the profiler will crash or the app will crash. What this means is
    /// that once a span has been opened with some query_id, you _must_ also end the span and you
    /// _must_ eventually publish a gpu time value before reusing the id.
    ///
    /// If you don't you're in for sadness.
    unsafe fn publish_gpu_time(queue_id: Self, query_id: u16, gpu_time: u64);
}

#[cfg(not(feature = "profile-with-tracy"))]
#[allow(unused_imports)]
pub use impl_empty::*;
#[cfg(feature = "profile-with-tracy")]
#[allow(unused_imports)]
pub use impl_tracy::*;

/// Flag used to notify the connected profiler what GPU API is being profiled with the GPU
/// integration.
///
/// This is just a mirror of Tracy's identically named type.
pub enum ContextType {
    Invalid = 0,
    OpenGl = 1,
    Vulkan = 2,
    OpenCL = 3,
    Direct3D12 = 4,
    Direct3D11 = 5,
    Metal = 6,
    Custom = 7,
    CUDA = 8,
    Rocprof = 9,
    WebGPU = 10,
}
