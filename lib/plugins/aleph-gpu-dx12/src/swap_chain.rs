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

use crate::surface::Surface;
use dx12::dxgi;
use interfaces::gpu::{AcquireImageError, ISwapChain, QueueType};
use interfaces::ref_ptr::{ref_ptr_object, RefPtr};
use std::sync::atomic::Ordering;

ref_ptr_object! {
    pub struct SwapChain: ISwapChain, ISwapChainExt {
        pub(crate) swap_chain: dxgi::SwapChain,
        pub(crate) surface: RefPtr<Surface>,
        pub(crate) queue_support: QueueType,
    }
}

impl ISwapChain for SwapChain {
    fn present_supported_on_queue(&self, queue: QueueType) -> bool {
        queue == self.queue_support
    }

    fn acquire_image(&self) -> Result<(), AcquireImageError> {
        todo!()
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        // Release the surface as the swap chain no longer owns it
        debug_assert!(self.surface.has_swap_chain.swap(false, Ordering::SeqCst));

        // TODO: We will need to manually extend the lifetime of the SwapChain so we can perform the
        //       above operation when we know that the swap chain is no longer in use
    }
}

pub trait ISwapChainExt: ISwapChain {
    fn get_raw_handle(&self) -> &dxgi::SwapChain;
}

impl ISwapChainExt for SwapChain {
    fn get_raw_handle(&self) -> &dxgi::SwapChain {
        &self.swap_chain
    }
}
