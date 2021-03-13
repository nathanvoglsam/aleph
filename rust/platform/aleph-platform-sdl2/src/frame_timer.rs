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

use interfaces::platform::IFrameTimer;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FrameTimerImpl {
    freq: AtomicU64,
    first: AtomicU64,
    last: AtomicU64,
    current: AtomicU64,
}

impl FrameTimerImpl {
    pub fn new(timer: &sdl2::TimerSubsystem) -> Self {
        aleph_log::trace!("Initializing the Frame Timer");
        Self {
            freq: AtomicU64::new(timer.performance_frequency()),
            first: AtomicU64::new(timer.performance_counter()),
            last: AtomicU64::new(timer.performance_counter()),
            current: AtomicU64::new(timer.performance_counter() + 1),
        }
    }

    pub fn update(&self, timer: &sdl2::TimerSubsystem) {
        self.last
            .store(self.current.load(Ordering::Relaxed), Ordering::Relaxed);
        self.current
            .store(timer.performance_counter(), Ordering::Relaxed);
    }
}

impl IFrameTimer for FrameTimerImpl {
    fn delta_time(&self) -> f64 {
        let current = self.current.load(Ordering::Relaxed) as f64;
        let last = self.last.load(Ordering::Relaxed) as f64;
        let freq = self.freq.load(Ordering::Relaxed) as f64;

        let delta = current - last;
        let delta = delta / freq;

        delta
    }

    fn elapsed_time(&self) -> f64 {
        let first = self.first.load(Ordering::Relaxed) as f64;
        let current = self.current.load(Ordering::Relaxed) as f64;
        let freq = self.freq.load(Ordering::Relaxed) as f64;

        let elapsed = current - first;
        let elapsed = elapsed / freq;

        elapsed
    }
}

interfaces::any::declare_interfaces!(FrameTimerImpl, [IFrameTimer]);
