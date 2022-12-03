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
use crate::internal::swap_chain_creation::dxgi_create_swap_chain;
use crate::swap_chain::{SwapChain, SwapChainState};
use crossbeam::atomic::AtomicCell;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    IDevice, ISurface, ISwapChain, PresentationMode, QueueType, SwapChainConfiguration,
    SwapChainCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct Surface {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) factory: IDXGIFactory2,
    pub(crate) handle: RawWindowHandle,
    pub(crate) has_swap_chain: AtomicBool,
}

declare_interfaces!(Surface, [ISurface, ISurfaceExt]);

impl Surface {
    unsafe fn inner_create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError> {
        let device = device.query_interface::<Device>().unwrap();

        // Translate our high level present mode into terms that make sense to d3d12
        let (buffer_count, flags) = match config.present_mode {
            PresentationMode::Immediate => (2, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING),
            PresentationMode::Mailbox => (3, DXGI_SWAP_CHAIN_FLAG::default()),
            PresentationMode::Fifo => (2, DXGI_SWAP_CHAIN_FLAG::default()),
        };

        // Translate our format
        let view_format = texture_format_to_dxgi(config.format);

        // Vulkan allows SRGB formats for textures in memory, d3d12 does not and instead you alias
        // a non SRGB texture of the same layout with an RTV with an SRGB format.
        let in_memory_format = match view_format {
            DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => DXGI_FORMAT_R8G8B8A8_UNORM,
            DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => DXGI_FORMAT_B8G8R8A8_UNORM,
            format => format,
        };

        // Fill out our description
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: config.width,
            Height: config.height,
            Format: in_memory_format,
            Stereo: BOOL::from(false),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: buffer_count,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: DXGI_ALPHA_MODE_IGNORE,
            Flags: flags.0 as u32,
        };

        // Select a queue to attach the swap chain to. If the preferred queue is not supported we
        // fallback directly to the general queue.
        let (queue, queue_type) = match config.preferred_queue {
            QueueType::General => {
                // Loading the general queue is handled after this match block as a fallback for
                // the other two cases. We can just re-use the same code for loading it if we
                // pretend we didn't find a queue here.
                (None, QueueType::General)
            }
            QueueType::Compute => {
                if let Some(queue) = device.compute_queue.as_ref() {
                    let queue = queue.handle.clone();
                    (Some(queue), QueueType::Compute)
                } else {
                    (None, QueueType::General)
                }
            }
            QueueType::Transfer => {
                if let Some(queue) = device.transfer_queue.as_ref() {
                    let queue = queue.handle.clone();
                    (Some(queue), QueueType::Transfer)
                } else {
                    (None, QueueType::General)
                }
            }
        };
        let (queue, queue_type) = if queue.is_none() {
            if let Some(queue) = device.general_queue.as_ref() {
                let queue = queue.handle.clone();
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
        let swap_chain = unsafe {
            dxgi_create_swap_chain(&self.factory, &queue, self, &desc).map_err(|e| anyhow!(e))?
        };

        let inner = SwapChainState {
            config: config.clone(),
            current: -1,
            textures: Vec::with_capacity(desc.BufferCount as usize),
            dxgi_format: in_memory_format,
            dxgi_view_format: view_format,
        };
        let swap_chain = AnyArc::new_cyclic(move |v| SwapChain {
            this: v.clone(),
            device: device.this.upgrade().unwrap(),
            surface: self.this.upgrade().unwrap(),
            swap_chain,
            queue_support: queue_type,
            inner: Mutex::new(inner),
            queued_resize: AtomicCell::new(None),
        });

        {
            let mut state = swap_chain.inner.lock();
            swap_chain
                .recreate_swap_images(&mut state, desc.BufferCount)
                .map_err(|e| anyhow!(e))?;
        }

        Ok(AnyArc::map::<dyn ISwapChain, _>(swap_chain, |v| v))
    }
}

impl ISurface for Surface {
    fn upgrade(&self) -> AnyArc<dyn ISurface> {
        AnyArc::map::<dyn ISurface, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError> {
        // Check if the surface is currently taken with an existing swap chain
        if self.has_swap_chain.swap(true, Ordering::SeqCst) {
            return Err(SwapChainCreateError::SurfaceAlreadyOwned);
        }

        unsafe {
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
    fn get_raw_handle(&self) -> IDXGIFactory2;
}

impl ISurfaceExt for Surface {
    fn get_raw_handle(&self) -> IDXGIFactory2 {
        self.factory.clone()
    }
}
