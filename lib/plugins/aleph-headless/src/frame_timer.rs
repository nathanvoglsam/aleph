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

use interfaces::any::AnyArc;
use interfaces::platform::IFrameTimer;
use parking_lot::RwLock;
use std::time::{Duration, Instant};

struct Inner {
    first: Instant,
    last: Instant,
    current: Instant,
}

pub struct FrameTimerImpl {
    inner: RwLock<Inner>,
}

impl FrameTimerImpl {
    pub fn new() -> AnyArc<Self> {
        log::trace!("Initializing the Frame Timer");
        // Get the first two time stamps
        let first = Instant::now();
        let last = first;

        // Make prevent being able to observe a zero time step by offsetting this
        let current = last + Duration::from_nanos(100);

        let out = Self {
            inner: RwLock::new(Inner {
                first,
                last,
                current,
            }),
        };
        AnyArc::new(out)
    }

    pub fn update(&self) {
        let mut inner = self.inner.write();
        inner.last = inner.current;
        inner.current = Instant::now();
    }
}

impl IFrameTimer for FrameTimerImpl {
    fn delta_time(&self) -> f64 {
        let inner = self.inner.read();

        // Get the time between current and last
        let delta = inner.current.duration_since(inner.last);

        // Rescale to seconds
        let delta_micros = delta.as_micros() as f64;
        let delta_millis = delta_micros / 1000f64;
        delta_millis / 1000f64
    }

    fn elapsed_time(&self) -> f64 {
        let inner = self.inner.read();

        // Get the duration between current and first
        let elapsed = inner.current.duration_since(inner.first);

        // Rescale to seconds
        let elapsed_micros = elapsed.as_micros() as f64;
        let elapsed_millis = elapsed_micros / 1000f64;
        elapsed_millis / 1000f64
    }
}

interfaces::any::declare_interfaces!(FrameTimerImpl, [IFrameTimer]);
