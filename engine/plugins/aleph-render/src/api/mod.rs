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

use std::any::Any;
use std::sync::Arc;

use aleph_nstr::nstr;
use aleph_rhi_api::*;
use interfaces::any::AnyArc;

use crate::render::{
    BufferHandle, BufferLoader, BufferPool, DeletionPool, TextureHandle, TextureLoader, TexturePool,
};

pub trait IRenderSurface: Any {
    fn get_render_extent(&self) -> Extent2D;
    fn get_swap_chain(&self) -> &dyn ISwapChain;
    fn needs_rebuild(&self) -> bool;
}

pub struct Renderer {
    device: AnyArc<dyn IDevice>,
    queue: AnyArc<dyn IQueue>,
    swap_manager: SwapManager,
    texture_loader: Arc<TextureLoader>,
    buffer_loader: Arc<BufferLoader>,
    texture_pool: TexturePool,
    buffer_pool: BufferPool,
    frame_manager: FrameManager,
}

impl Renderer {
    pub fn get_texture_loader(&self) -> Arc<TextureLoader> {
        self.texture_loader.clone()
    }

    pub fn get_buffer_loader(&self) -> Arc<BufferLoader> {
        self.buffer_loader.clone()
    }

    pub fn create_texture(&mut self) -> Option<TextureHandle> {
        unimplemented!()
    }

    pub fn create_buffer(&mut self) -> Option<BufferHandle> {
        unimplemented!()
    }

    pub unsafe fn draw_next_frame(&mut self) {
        let FrameResources {
            acquire_semaphore,
            present_semaphore,
            done_fence,
            deletion_pool,
        } = self.frame_manager.get_next_frame();

        // If we're producing frames faster than the GPU is producing them and we run out of frames
        // in flight then we need to wait for the oldest frame to complete before we can start
        // doing anything.
        //
        // This will stall until the oldest frame is complete, applying back pressure up the
        // pipeline.
        assert_eq!(
            self.device
                .wait_fences(&[done_fence.as_ref()], true, u32::MAX),
            FenceWaitResult::Complete
        );
        self.device.reset_fences(&[done_fence.as_ref()]);

        // We are now definitely recording the frame, we've proven it has been retired on the GPU
        // timeline and that means we can purge anything that was being held alive for that GPU
        // frame in the deletion pool.
        deletion_pool.purge();

        let acquired_image = self
            .swap_manager
            .acquire_next_image(acquire_semaphore.as_ref());

        let mut list = self
            .device
            .create_command_list(&CommandListDesc {
                queue_type: QueueType::General,
                name: None,
            })
            .unwrap();

        {
            let mut encoder = list.begin_general().unwrap();

            encoder.begin_event(Color::BLUE, nstr!("Upload Streaming Requests"));
            // TODO: we want to batch all of these, so we need a better interface so we can bundle
            //       the barriers and copy commands for all our loaders into a single batch.
            //
            //       either that or we unify to a single loader type.
            self.buffer_loader.upload_requests(
                &mut self.buffer_pool,
                deletion_pool,
                self.device.as_ref(),
                encoder.as_mut(),
                usize::MAX,
            );
            self.texture_loader.upload_requests(
                &mut self.texture_pool,
                deletion_pool,
                self.device.as_ref(),
                encoder.as_mut(),
                usize::MAX,
            );
            encoder.end_event();
        }

        self.queue
            .submit(&QueueSubmitDesc {
                command_lists: &[Some(list).into()],
                wait_semaphores: &[acquire_semaphore.as_ref()],
                signal_semaphores: &[present_semaphore.as_ref()],
                fence: Some(done_fence.as_ref()),
            })
            .unwrap();

        self.swap_manager.present(
            self.queue.as_ref(),
            &[present_semaphore.as_ref()],
            acquired_image,
        );
    }
}

struct SwapManager {
    surface: Box<dyn IRenderSurface>,
    images: Vec<AnyArc<dyn ITexture>>,
    needs_rebuild: bool,
}

impl SwapManager {
    pub unsafe fn acquire_next_image(
        &mut self,
        signal_semaphore: &dyn ISemaphore,
    ) -> AcquiredImage {
        // Query if the surface wants to rebuild this frame and coerce the 'needs_rebuild' flag to
        // true if it wasn't already flagged.
        self.needs_rebuild = self.needs_rebuild || self.surface.needs_rebuild();

        let mut attempts_remaining = 2;
        while attempts_remaining != 0 {
            attempts_remaining -= 1;

            if self.needs_rebuild {
                self.rebuild();
            }

            let acquired_index = match self
                .surface
                .get_swap_chain()
                .acquire_next_image(&AcquireDesc { signal_semaphore })
            {
                Ok(i) => i,
                Err(ImageAcquireError::SubOptimal(i)) => {
                    // We should queue a rebuild for the next frame, but we can still render with
                    // this attachment so rather than stalling immediately we render now with the
                    // suboptimal attachment
                    self.needs_rebuild = true;
                    i
                }
                Err(ImageAcquireError::OutOfDate) => {
                    // If the swapchain is out of date then we have to rebuild immediately, so
                    // that's exactly what we do by forcing the rebuild flag on and looping around
                    // again.
                    //
                    // We can only try to acquire twice. If we fail a second time then we panic as
                    // we could end up stuck in a loop.
                    self.needs_rebuild = true;
                    continue;
                }
                v => v.unwrap(),
            };
            let acquired_image = self.images[acquired_index as usize].clone();

            return AcquiredImage {
                image: acquired_image,
                index: acquired_index as usize,
            };
        }
        panic!("Unable to acquire swap chain image!");
    }

    pub unsafe fn present(
        &mut self,
        queue: &dyn IQueue,
        wait_semaphores: &[&dyn ISemaphore],
        image: AcquiredImage,
    ) {
        let submit_result = queue.present(&QueuePresentDesc {
            swap_chain: self.surface.get_swap_chain(),
            image_index: image.index as u32,
            wait_semaphores,
        });
        match submit_result {
            Ok(_) => {}
            Err(QueuePresentError::OutOfDate) | Err(QueuePresentError::SubOptimal) => {
                self.needs_rebuild = true;
            }
            v @ Err(_) => v.unwrap(),
        }
    }

    fn rebuild(&mut self) {
        let swap_chain = self.surface.get_swap_chain();

        self.images.clear();

        let drawable_size = self.surface.get_render_extent();
        let new_config = swap_chain.rebuild(Some(drawable_size)).unwrap();

        let mut images: Vec<_> = (0..new_config.buffer_count).map(|_| None).collect();
        swap_chain.get_images(&mut images);

        self.images = images.into_iter().map(|v| v.unwrap()).collect();
        self.needs_rebuild = false;
    }
}

struct AcquiredImage {
    /// Handle to the image we ended up acquiring
    image: AnyArc<dyn ITexture>,

    /// The image index of the image we acquired
    index: usize,
}

struct FrameManager {
    frames: Vec<FrameResources>,
    current: usize,
}

impl FrameManager {
    pub fn get_next_frame(&mut self) -> &mut FrameResources {
        self.current = (self.current + 1) % self.frames.len();
        &mut self.frames[self.current]
    }
}

struct FrameResources {
    /// Used for syncing on the swap chain acquisition.
    acquire_semaphore: AnyArc<dyn ISemaphore>,

    /// Used for syncing the present operation on the completion of the frame's final submission.
    present_semaphore: AnyArc<dyn ISemaphore>,

    /// Used for notifying the CPU when the GPU frame is complete.
    done_fence: AnyArc<dyn IFence>,

    /// Pool for placing any resource that was deleted within the frame but must remain alive until
    /// that frame is finally retired on the GPU.
    deletion_pool: DeletionPool,
}
