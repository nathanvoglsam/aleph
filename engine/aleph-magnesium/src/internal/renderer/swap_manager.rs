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

use crate::internal::renderer::surface::SharedSurface;
use crate::renderer::surface_notify::SurfaceNotification;

pub struct SwapManager {
    pub surface: SharedSurface,
    pub desc: rhi::TextureDesc<'static>,
    pub needs_rebuild: bool,
    pub extent: rhi::Extent2D,
}

impl SwapManager {
    pub unsafe fn acquire_next_image(&mut self) -> AcquiredImage {
        let mut surface_resized = false;
        while let Ok(msg) = self.surface.notify.try_recv() {
            match msg {
                SurfaceNotification::Resized(extent) => {
                    surface_resized = true;
                    self.extent = extent;
                }
            }
        }

        // Query if the surface wants to rebuild this frame and coerce the 'needs_rebuild' flag to
        // true if it wasn't already flagged.
        self.needs_rebuild = self.needs_rebuild || surface_resized;

        let mut rebuilt = false;
        let mut attempts_remaining = 2;
        while attempts_remaining != 0 {
            attempts_remaining -= 1;

            if self.needs_rebuild {
                self.rebuild();
                rebuilt = true; // Flag if we had to rebuild before giving out the image.
            }

            let swap_image = unsafe { self.surface.swap_chain.acquire_next_image() };
            let swap_image = match swap_image {
                Ok(aleph_rhi_api::AcquiredImage::Ok(i)) => i,
                Ok(aleph_rhi_api::AcquiredImage::SubOptimal(i)) => {
                    // We should queue a rebuild for the next frame, but we can still render with
                    // this attachment so rather than stalling immediately we render now with the
                    // suboptimal attachment
                    self.needs_rebuild = true;
                    i
                }
                Err(rhi::ImageAcquireError::OutOfDate) => {
                    // If the swapchain is out of date then we have to rebuild immediately, so
                    // that's exactly what we do by forcing the rebuild flag on and looping around
                    // again.
                    //
                    // We can only try to acquire twice. If we fail a second time then we panic as
                    // we could end up stuck in a loop.
                    self.needs_rebuild = true;
                    continue;
                }
                v => v.unwrap().get(),
            };

            self.desc = swap_image.texture_desc().clone().strip_name();

            return AcquiredImage {
                swap_image,
                extent: self.extent,
                rebuilt,
            };
        }
        panic!("Unable to acquire swap chain image!");
    }

    pub unsafe fn present(&mut self, queue: &dyn rhi::IQueue, image: AcquiredImage) {
        let submit_result = unsafe { queue.present(image.swap_image) };
        match submit_result {
            Ok(_) => {}
            Err(rhi::QueuePresentError::OutOfDate) | Err(rhi::QueuePresentError::SubOptimal) => {
                self.needs_rebuild = true;
            }
            v @ Err(_) => v.unwrap(),
        }
    }

    pub fn rebuild(&mut self) {
        let _new_config = self.surface.swap_chain.rebuild(Some(self.extent)).unwrap();
        self.needs_rebuild = false;
    }
}

pub struct AcquiredImage {
    /// Handle to the acquire swap image
    pub swap_image: Arc<dyn rhi::ISwapImage>,

    /// The dimensions of the swap chain image
    pub extent: rhi::Extent2D,

    /// Flags whether the swap chain was rebuilt in order to acquire this image
    pub rebuilt: bool,
}
