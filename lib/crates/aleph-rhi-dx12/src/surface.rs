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

use std::any::TypeId;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use parking_lot::Mutex;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

use crate::context::Context;
use crate::internal::conv::texture_format_to_dxgi;
use crate::internal::swap_chain_creation::dxgi_create_swap_chain;
use crate::internal::unwrap;
use crate::swap_chain::{SwapChain, SwapChainState};

pub struct Surface {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) handle: RawWindowHandle,
    pub(crate) has_swap_chain: Mutex<bool>,
}

// SAFETY: RawWindowHandle is an opaque handle and can the only purpose is for some other object to
//         consume it. The consumer constrains thread sharing so this is safe.
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

declare_interfaces!(Surface, [ISurface]);

impl IGetPlatformInterface for Surface {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl Surface {
    unsafe fn inner_create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError> {
        let device = unwrap::device(device);

        // Translate our high level present mode into terms that make sense to d3d12
        let buffer_count = config.buffer_count;
        let flags = match config.present_mode {
            PresentationMode::Immediate => DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING,
            PresentationMode::Mailbox => DXGI_SWAP_CHAIN_FLAG::default(),
            PresentationMode::Fifo => DXGI_SWAP_CHAIN_FLAG::default(),
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
        let (queue, queue_type) = match config.present_queue {
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
            let factory = self.context.factory.as_ref().unwrap().lock();
            dxgi_create_swap_chain(&factory, &queue, self, &desc)
                .map_err(|e| log::error!("Platform Error: {:#?}", e))?
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
            acquired: Default::default(),
        });

        {
            let mut state = swap_chain.inner.lock();
            swap_chain
                .recreate_swap_images(&mut state, desc.BufferCount)
                .map_err(|e| log::error!("Platform Error: {:#?}", e))?;
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
        let mut has_swap_chain = self.has_swap_chain.lock();
        if *has_swap_chain {
            return Err(SwapChainCreateError::SurfaceAlreadyOwned);
        }

        let result = unsafe { self.inner_create_swap_chain(device, config) };

        // If we successfully created the swap chain then we update the owned flag to prevent
        // creating more.
        if result.is_ok() {
            *has_swap_chain = true;
        }

        result
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle
    }
}
