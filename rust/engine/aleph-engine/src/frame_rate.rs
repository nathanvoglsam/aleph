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

use platform::FrameTimer;

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
        let window_len = self.frame_times.len() / 4;
        let window = self.frame_times.len() - window_len;
        let frame_time: f32 = self.frame_times[window..].iter().sum();
        frame_time / window_len as f32
    }

    pub fn frame_time_history(&self) -> &[f32] {
        &self.frame_times
    }
}
