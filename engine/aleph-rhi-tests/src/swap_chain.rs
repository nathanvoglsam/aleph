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

use aleph_rhi_api::{
    AcquiredImage, IQueue, ISwapChain, ISwapImage, ImageAcquireError, QueuePresentError,
};

/// Acquires a swap chain image, rebuilding and retrying if the swap chain is out of date.
///
/// A suboptimal image is returned and sets `needs_rebuild` so the caller can defer rebuilding
/// until the next acquisition.
///
/// # Safety
///
/// The caller must observe the safety requirements of [`ISwapChain::acquire_next_image`].
pub unsafe fn acquire_next_image(
    swap_chain: &dyn ISwapChain,
    needs_rebuild: &mut bool,
) -> Arc<dyn ISwapImage> {
    if *needs_rebuild {
        swap_chain
            .rebuild(None)
            .expect("Failed to rebuild swap chain");
        *needs_rebuild = false;
    }

    for attempt in 0..2 {
        match unsafe { swap_chain.acquire_next_image() } {
            Ok(AcquiredImage::Ok(image)) => return image,
            Ok(AcquiredImage::SubOptimal(image)) => {
                *needs_rebuild = true;
                return image;
            }
            Err(ImageAcquireError::OutOfDate) if attempt == 0 => {
                swap_chain
                    .rebuild(None)
                    .expect("Failed to rebuild out-of-date swap chain");
            }
            Err(error) => panic!("Failed to acquire swap chain image: {error}"),
        }
    }

    unreachable!("swap chain image acquisition attempts exhausted");
}

/// Presents a swap chain image, marking the swap chain for rebuild if it is out of date or
/// suboptimal.
///
/// # Safety
///
/// The caller must observe the safety requirements of [`IQueue::present`].
pub unsafe fn present(queue: &dyn IQueue, image: Arc<dyn ISwapImage>, needs_rebuild: &mut bool) {
    unsafe {
        match queue.present(image) {
            Ok(()) => {}
            Err(QueuePresentError::OutOfDate) | Err(QueuePresentError::SubOptimal) => {
                *needs_rebuild = true;
            }
            Err(err) => panic!("Failed to present image: {}", err),
        }
    }
}
