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

use std::num::NonZeroU64;
use std::sync::atomic::{AtomicU64, Ordering};

/// A generic utility used for generating globally (within the same IDevice scope) unique IDs that
/// never get reused.
pub struct ObjectCounter {
    buffer: AtomicU64,
    texture: AtomicU64,
    sampler: AtomicU64,
    set_layout: AtomicU64,
    pipeline_layout: AtomicU64,
    graphics_pipeline: AtomicU64,
    compute_pipeline: AtomicU64,
}

impl ObjectCounter {
    pub fn new() -> Self {
        Self {
            buffer: AtomicU64::new(1),
            texture: AtomicU64::new(1),
            sampler: AtomicU64::new(1),
            set_layout: AtomicU64::new(1),
            pipeline_layout: AtomicU64::new(1),
            graphics_pipeline: AtomicU64::new(1),
            compute_pipeline: AtomicU64::new(1),
        }
    }

    pub fn next_buffer(&self) -> NonZeroU64 {
        Self::next_object(&self.buffer)
    }

    pub fn next_texture(&self) -> NonZeroU64 {
        Self::next_object(&self.texture)
    }

    pub fn next_sampler(&self) -> NonZeroU64 {
        Self::next_object(&self.sampler)
    }

    pub fn next_set_layout(&self) -> NonZeroU64 {
        Self::next_object(&self.set_layout)
    }

    pub fn next_pipeline_layout(&self) -> NonZeroU64 {
        Self::next_object(&self.pipeline_layout)
    }

    pub fn next_graphics_pipeline(&self) -> NonZeroU64 {
        Self::next_object(&self.graphics_pipeline)
    }

    pub fn next_compute_pipeline(&self) -> NonZeroU64 {
        Self::next_object(&self.compute_pipeline)
    }

    fn next_object(counter: &AtomicU64) -> NonZeroU64 {
        let id = counter.fetch_add(1, Ordering::Relaxed);
        NonZeroU64::new(id).expect("Overflowed object counter! HOW!!!!!")
    }
}
