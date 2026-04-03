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

use std::sync::Arc;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;

use crate::internal::MgSystem;

/// This type manages a ring buffer of fences as well as a frame index counter.
///
/// This struct manages keeping track of which frame is currently being rendered. Asking for the
/// 'next frame' from this type will give out the index of that frame as well as a fence that should
/// be attached to the frame's queue submission.
pub struct FrameManager {
    /// GPU device handle
    device: Arc<dyn rhi::IDevice>,

    /// Number of frames we're allowed to render ahead by.
    render_ahead_frames: usize,

    /// Fences used for frame submissions, maintained in frame submission order as a ring buffer.
    frame_fences: BVec<Option<FenceEntry>, MgSystem>,

    /// The index of the _next_ frame. This will be what is returned on the next call to
    /// 'get_next_frame'.
    next_frame_index: usize,
}

impl FrameManager {
    pub fn new(device: Arc<dyn rhi::IDevice>, render_ahead_frames: usize) -> Self {
        Self {
            device,
            render_ahead_frames,
            frame_fences: aleph_alloc::vec![in system(); None; render_ahead_frames + 1],
            next_frame_index: 0,
        }
    }

    /// Ask for a new frame index. This must be called at the beginning of a new frame so we can
    /// get a frame index as well as maintain our ring buffer of fences.
    ///
    /// This will stall the render thread if we try and render too far ahead.
    ///
    /// Takes a 'fence' that _must_ be the fence that will be signaled in the subsequent frame's
    /// queue submission.
    pub fn get_next_frame(&mut self, fence: rhi::FenceHandle) -> FrameInfo {
        // The existing value in 'next_frame_index' is the index we give out. We increment it here
        // so that the next call will yield the next value in sequence.
        let frame_index = self.next_frame_index;
        self.next_frame_index = self
            .next_frame_index
            .checked_add(1)
            .expect("God help us if this overflows");

        let out = FrameInfo { frame_index };

        // We maintain a ring buffer of fences that were used for each frame. The size of the ring
        // buffer is 'render_ahead_frames + 1'. Because the index is monotonic, and we store in a
        // ring, this lookup will find the oldest fence.
        //
        // Once we have our fence, we wait on it. Configuring 'render_ahead_frames' changes the
        // number of frames we are allowed to submit to the GPU before we stall.
        //
        // A value of 0 will prevent any render ahead, stalling until all work for the last frame
        // is done on the GPU. A value of 1 will allow us to queue a second frame onto the GPU
        // before we start stalling.
        //
        // How this works is that we swap the old fence with a handle to the fence we'll be
        // signaling for this frame's work. This means the unsignalled fence takes the old one's
        // place. Then we wait on the old fence.
        let frame_fences_i = frame_index % self.render_ahead_frames;
        let mut old_entry = Some(FenceEntry {
            fence,
            value: out.signal_value(),
        });
        std::mem::swap(&mut old_entry, &mut self.frame_fences[frame_fences_i]);
        if let Some(old_entry) = old_entry {
            // The fences are optional as the pipe may be entry for the first frames we draw.
            let result =
                self.device
                    .wait_fences(&[&old_entry.fence], &[old_entry.value], true, u32::MAX);
            assert_eq!(result.unwrap(), rhi::FenceWaitResult::Complete);
        }

        out
    }

    /// Flush all fences from the ring buffer and wait for them all to be signaled.
    ///
    /// Once this returns there should be no render frame commands in flight on the GPU.
    pub fn wait_all_retired(&mut self) {
        for entry in self.frame_fences.iter_mut() {
            if let Some(entry) = entry {
                let result =
                    self.device
                        .wait_fences(&[&entry.fence], &[entry.value], true, u32::MAX);
                assert_eq!(result.unwrap(), rhi::FenceWaitResult::Complete);
            }
            *entry = None;
        }
    }
}

/// Result from [`FrameManager::get_next_frame`]
pub struct FrameInfo {
    /// The index of the current active frame. This is typically the frame the caller is about to
    /// begin recording commands for.
    pub frame_index: usize,
}

impl FrameInfo {
    pub const fn signal_value(&self) -> u64 {
        self.frame_index.saturating_add(1) as u64
    }
}

#[derive(Clone)]
struct FenceEntry {
    fence: rhi::FenceHandle,
    value: u64,
}
