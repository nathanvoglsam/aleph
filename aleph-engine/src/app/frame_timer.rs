//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct FrameTimerState {
    first: AtomicU64,
    current: AtomicU64,
    last: AtomicU64,
    freq: AtomicU64,
}

pub static FRAME_TIMER_STATE: Lazy<FrameTimerState> = Lazy::new(|| FrameTimerState {
    first: AtomicU64::new(0),
    current: AtomicU64::new(0),
    last: AtomicU64::new(0),
    freq: AtomicU64::new(0),
});

pub struct FrameTimer {}

impl FrameTimer {
    pub(crate) fn init(timer: &sdl2::TimerSubsystem) {
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
