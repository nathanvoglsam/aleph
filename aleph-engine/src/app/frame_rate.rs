//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::FrameTimer;

///
/// A struct that holds a short history window of frame times to produce a frame rate counter that
/// is stable and resilient to single frame fluctuations
///
pub struct FrameRate {
    frame_times: [f32; 128],
}

impl FrameRate {
    ///
    /// Constructs a new FrameRate object with the array initialized to contain a frametime that
    /// represents 60fps, just so the counter doesn't start reporting an infinite FPS (0 frame time)
    ///
    pub fn new() -> Self {
        Self {
            frame_times: [1.0 / 60.0; 128],
        }
    }

    ///
    /// Updates the internal frame rate window to remove the oldest sample and insert a new sample.
    ///
    /// Call this once per frame
    ///
    pub fn update(&mut self) {
        for i in 1..self.frame_times.len() {
            self.frame_times[i - 1] = self.frame_times[i];
        }
        *self.frame_times.last_mut().unwrap() = FrameTimer::delta_time() as f32;
    }

    ///
    /// Gets the frame rate
    ///
    pub fn frame_rate(&self) -> f32 {
        1.0 / self.frame_time()
    }

    ///
    /// Gets the average frame time
    ///
    pub fn frame_time(&self) -> f32 {
        let mut frame_time = 0.0;
        let window_len = self.frame_times.len() / 4;
        let window = self.frame_times.len() - window_len;
        self.frame_times[window..]
            .iter()
            .for_each(|v| frame_time += *v);
        frame_time / window_len as f32
    }

    pub fn frame_time_history(&self) -> &[f32] {
        &self.frame_times
    }
}
