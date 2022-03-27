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

use crate::context::Context;
use crate::device::Device;
use crate::internal::conv::texture_format_to_dxgi;
use crate::swap_chain::{SwapChain, SwapChainState};
use crossbeam::atomic::AtomicCell;
use dx12::dxgi;
use dx12::dxgi::{Format, SwapChainFlags};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    IDevice, ISurface, ISwapChain, PresentationMode, QueueType, SwapChainConfiguration,
    SwapChainCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject, WeakRefPtr};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

ref_ptr_object! {
    pub struct Surface: ISurface, ISurfaceExt {
        pub(crate) factory: dxgi::Factory,
        pub(crate) handle: RawWindowHandle,
        pub(crate) has_swap_chain: AtomicBool,
        pub(crate) context: RefPtr<Context>,
    }
}

impl Surface {
    fn inner_create_swap_chain(
        &self,
        device: WeakRefPtr<dyn IDevice>,
        config: &SwapChainConfiguration,
    ) -> Result<RefPtr<dyn ISwapChain>, SwapChainCreateError> {
        let device = device.query_interface::<Device>().unwrap();

        // Translate our high level present mode into terms that make sense to d3d12
        let (buffer_count, flags) = match config.present_mode {
            PresentationMode::Immediate => (2, SwapChainFlags::ALLOW_TEARING),
            PresentationMode::Mailbox => (3, SwapChainFlags::NONE),
            PresentationMode::Fifo => (2, SwapChainFlags::NONE),
        };

        // Translate our format
        let view_format = texture_format_to_dxgi(config.format);

        // Vulkan allows SRGB formats for textures in memory, d3d12 does not and instead you alias
        // a non SRGB texture of the same layout with an RTV with an SRG format.
        let in_memory_format = match view_format {
            Format::R8G8B8A8UnormSRGB => Format::R8G8B8A8Unorm,
            Format::B8G8R8A8UnormSRGB => Format::B8G8R8A8Unorm,
            format => format,
        };

        // Fill out our description
        let desc = dxgi::SwapChainDesc1::builder()
            .width(config.width)
            .height(config.height)
            .format(in_memory_format)
            .usage_flags(dxgi::UsageFlags::BACK_BUFFER)
            .usage_flags(dxgi::UsageFlags::RENDER_TARGET_OUTPUT)
            .buffer_count(buffer_count)
            .swap_effect(dxgi::SwapEffect::FlipDiscard)
            .flags(flags)
            .build();

        // Select a queue to attach the swap chain to. If the preferred queue is not supported we
        // fallback directly to the general queue.
        let queues = &device.queues;
        let (queue, queue_type) = match config.preferred_queue {
            QueueType::General => {
                // Loading the general queue is handled after this match block as a fallback for
                // the other two cases. We can just re-use the same code for loading it if we
                // pretend we didn't find a queue here.
                (None, QueueType::General)
            }
            QueueType::Compute => {
                if let Some(queue) = queues.compute.as_ref() {
                    let queue = queue.read().handle.clone();
                    (Some(queue), QueueType::Compute)
                } else {
                    (None, QueueType::General)
                }
            }
            QueueType::Transfer => {
                if let Some(queue) = queues.transfer.as_ref() {
                    let queue = queue.read().handle.clone();
                    (Some(queue), QueueType::Transfer)
                } else {
                    (None, QueueType::General)
                }
            }
        };
        let (queue, queue_type) = if queue.is_none() {
            if let Some(queue) = queues.general.as_ref() {
                let queue = queue.read().handle.clone();
                (Some(queue), QueueType::General)
            } else {
                (None, QueueType::General)
            }
        } else {
            (queue, queue_type)
        };

        // If the preferred queue and fallback queue are not supported we return an error
        let (queue, queue_type) = if let Some(queue) = queue {
            (queue, queue_type)
        } else {
            return Err(SwapChainCreateError::NoQueueAvailable);
        };

        // Create the actual swap chain object
        let swap_chain = self
            .factory
            .create_swap_chain(&queue, self, &desc)
            .map_err(|e| anyhow!(e))?;

        let images = unsafe {
            device.create_views_for_swap_images(&swap_chain, view_format, desc.buffer_count)?
        };

        let inner = SwapChainState {
            config: config.clone(),
            acquired: false,
            images,
            dxgi_format: in_memory_format,
            dxgi_view_format: view_format,
        };
        let swap_chain = ref_ptr_init! {
            SwapChain {
                swap_chain: swap_chain,
                device: device.as_ref_ptr(),
                surface: self.as_ref_ptr(),
                queue_support: queue_type,
                inner: Mutex::new(inner),
                queued_resize: AtomicCell::new(None),
            }
        };
        let swap_chain: RefPtr<SwapChain> = RefPtr::new(swap_chain);
        Ok(swap_chain.query_interface().unwrap())
    }
}

impl ISurface for Surface {
    fn create_swap_chain(
        &self,
        device: WeakRefPtr<dyn IDevice>,
        config: &SwapChainConfiguration,
    ) -> Result<RefPtr<dyn ISwapChain>, SwapChainCreateError> {
        // Check if the surface is currently taken with an existing swap chain
        if self.has_swap_chain.swap(true, Ordering::SeqCst) {
            return Err(SwapChainCreateError::SurfaceAlreadyOwned);
        }

        match self.inner_create_swap_chain(device, config) {
            v @ Ok(_) => v,
            v @ Err(_) => {
                // Release the surface if we failed to actually create the swap chain
                debug_assert!(self.has_swap_chain.swap(false, Ordering::SeqCst));
                v
            }
        }
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle
    }
}

// SAFETY: RawWindowHandle is an opaque handle and can the only purpose is for some other object to
//         consume it. The consumer constrains thread sharing so this is safe.
unsafe impl Send for Surface {}

pub trait ISurfaceExt: ISurface {
    fn get_raw_handle(&self) -> dxgi::Factory;
}

impl ISurfaceExt for Surface {
    fn get_raw_handle(&self) -> dxgi::Factory {
        self.factory.clone()
    }
}
