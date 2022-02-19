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
use crate::format::texture_format_to_dxgi;
use crate::swap_chain::SwapChain;
use dx12::dxgi;
use dx12::dxgi::SwapChainFlags;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    IDevice, ISurface, ISwapChain, PresentationMode, QueueType, SwapChainConfiguration,
    SwapChainCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject, WeakRefPtr};

ref_ptr_object! {
    pub struct Surface: ISurface, ISurfaceExt {
        pub(crate) factory: dxgi::Factory,
        pub(crate) handle: RawWindowHandle,
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

        let (buffer_count, flags) = match config.present_mode {
            PresentationMode::Immediate => (2, SwapChainFlags::ALLOW_TEARING),
            PresentationMode::Mailbox => (3, SwapChainFlags::NONE),
            PresentationMode::Fifo => (2, SwapChainFlags::NONE),
        };

        let format = texture_format_to_dxgi(config.format);

        // Create our swap chain to check if the surface is compatible
        let desc = dxgi::SwapChainDesc1::builder()
            .width(config.width) // Dummy values, shouldn't be important?
            .height(config.height) // Dummy values, shouldn't be important?
            .format(format) // Guaranteed supported format
            .usage_flags(dxgi::UsageFlags::BACK_BUFFER)
            .usage_flags(dxgi::UsageFlags::RENDER_TARGET_OUTPUT)
            .buffer_count(buffer_count)
            .swap_effect(dxgi::SwapEffect::FlipDiscard)
            .flags(flags)
            .build();

        let (queue, queue_type) = match config.preferred_queue {
            QueueType::General => (device.queues.general.as_ref(), QueueType::General),
            QueueType::Compute => {
                if let Some(queue) = device.queues.compute.as_ref() {
                    (Some(queue), QueueType::Compute)
                } else {
                    (device.queues.general.as_ref(), QueueType::General)
                }
            }
            QueueType::Transfer => {
                if let Some(queue) = device.queues.transfer.as_ref() {
                    (Some(queue), QueueType::Transfer)
                } else {
                    (device.queues.general.as_ref(), QueueType::General)
                }
            }
        };

        let swap_chain = self
            .factory
            .create_swap_chain(queue.unwrap(), self, &desc)
            .map_err(|e| anyhow!(e))?;

        let swap_chain = ref_ptr_init! {
            SwapChain {
                swap_chain: swap_chain,
                surface: self.as_ref_ptr(),
                queue_support: queue_type,
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

        match self.inner_create_swap_chain(device, config) {
            v @ Ok(_) => v,
            v @ Err(_) => {
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
