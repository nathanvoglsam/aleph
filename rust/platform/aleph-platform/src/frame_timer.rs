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

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct FrameTimerState {
    first: AtomicU64,
    current: AtomicU64,
    last: AtomicU64,
    freq: AtomicU64,
}

pub(crate) static FRAME_TIMER_STATE: Lazy<FrameTimerState> = Lazy::new(|| FrameTimerState {
    first: AtomicU64::new(0),
    current: AtomicU64::new(0),
    last: AtomicU64::new(0),
    freq: AtomicU64::new(0),
});

pub struct FrameTimer {}

impl FrameTimer {
    pub(crate) fn init(timer: &sdl2::TimerSubsystem) {
        aleph_log::trace!("Initializing the Frame Timer");
        let state = &FRAME_TIMER_STATE;

        state
            .freq
            .store(timer.performance_frequency(), Ordering::Relaxed);
        state
            .first
            .store(timer.performance_counter(), Ordering::Relaxed);
        state
            .last
            .store(timer.performance_counter(), Ordering::Relaxed);
        state
            .current
            .store(timer.performance_counter() + 1, Ordering::Relaxed);
    }

    pub(crate) fn frame(timer: &sdl2::TimerSubsystem) {
        let state = &FRAME_TIMER_STATE;

        state
            .last
            .store(state.current.load(Ordering::Relaxed), Ordering::Relaxed);
        state
            .current
            .store(timer.performance_counter(), Ordering::Relaxed);
    }

    pub fn delta_time() -> f64 {
        let state = &FRAME_TIMER_STATE;

        let current = state.current.load(Ordering::Relaxed) as f64;
        let last = state.last.load(Ordering::Relaxed) as f64;
        let freq = state.freq.load(Ordering::Relaxed) as f64;

        let delta = current - last;
        let delta = delta / freq;

        delta
    }

    pub fn elapsed_time() -> f64 {
        let state = &FRAME_TIMER_STATE;

        let first = state.first.load(Ordering::Relaxed) as f64;
        let current = state.current.load(Ordering::Relaxed) as f64;
        let freq = state.freq.load(Ordering::Relaxed) as f64;

        let elapsed = current - first;
        let elapsed = elapsed / freq;

        elapsed
    }
}
