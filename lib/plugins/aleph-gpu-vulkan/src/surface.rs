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
use crate::internal::queue_present_support::QueuePresentSupportFlags;
use crate::swap_chain::{SwapChain, SwapChainState};
use erupt::vk;
use erupt::vk::SurfaceKHR;
use interfaces::any::declare_interfaces;
use interfaces::anyhow::anyhow;
use interfaces::gpu::{
    IDevice, ISurface, ISwapChain, SwapChainConfiguration, SwapChainCreateError, TextureFormat,
};
use interfaces::ref_ptr::{ref_ptr_init, ref_ptr_object, RefPtr, RefPtrObject, WeakRefPtr};
use std::sync::Mutex;

ref_ptr_object! {
    pub struct Surface: ISurface, ISurfaceExt {
        pub(crate) surface: vk::SurfaceKHR,
        pub(crate) context: RefPtr<Context>,
    }
}

impl Surface {
    /// Internal function for querying present support for a given surface
    unsafe fn get_queue_support(
        device: &Device,
        surface: vk::SurfaceKHR,
    ) -> Result<QueuePresentSupportFlags, vk::Result> {
        let mut flags = QueuePresentSupportFlags::empty();

        if let Some(queue) = device.queues.general.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.index,
                    surface,
                )
                .result()?;
            if supported {
                flags |= QueuePresentSupportFlags::GENERAL;
            }
        }
        if let Some(queue) = device.queues.compute.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.index,
                    surface,
                )
                .result()?;
            if supported {
                flags |= QueuePresentSupportFlags::COMPUTE;
            }
        }
        if let Some(queue) = device.queues.transfer.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.index,
                    surface,
                )
                .result()?;
            if supported {
                flags |= QueuePresentSupportFlags::TRANSFER;
            }
        }

        Ok(flags)
    }
}

impl ISurface for Surface {
    fn create_swap_chain(
        &self,
        device: WeakRefPtr<dyn IDevice>,
        config: &SwapChainConfiguration,
    ) -> Result<RefPtr<dyn ISwapChain>, SwapChainCreateError> {
        let device = device.query_interface::<Device>().unwrap().to_strong();

        let queue_support = unsafe { Surface::get_queue_support(&device, self.surface).unwrap() };

        let fence = unsafe {
            let fence_info = vk::FenceCreateInfoBuilder::new();
            device
                .device_loader
                .create_fence(&fence_info, None)
                .result()
                .map_err(|e| anyhow!("Failed to create wait image fence with code {}", e))?
        };
        let inner = SwapChainState {
            swap_chain: vk::SwapchainKHR::null(),
            acquire_fence: fence,
            acquired: false,
            format: TextureFormat::Bgra8Unorm,
            vk_format: Default::default(),
            color_space: Default::default(),
            present_mode: Default::default(),
            extent: Default::default(),
            images: Vec::new(),
            queued_resize: None, // TODO: This likely needs to be initialized to something
        };
        let swap_chain = ref_ptr_init! {
            SwapChain {
                inner: Mutex::new(inner),
                queue_support: queue_support,
                device: device,
                surface: self.as_ref_ptr(),
            }
        };
        let swap_chain: RefPtr<SwapChain> = RefPtr::new(swap_chain);

        // TODO: This is unsound and wrong, no checks have been made yet
        unsafe {
            let mut inner = swap_chain.inner.lock().unwrap();
            swap_chain.build(&mut inner, config)?;
        }

        Ok(swap_chain.query_interface().unwrap())
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.context
                .instance_loader
                .destroy_surface_khr(Some(self.surface), None);
        }
    }
}

// SAFETY: RawWindowHandle is an opaque handle and can the only purpose is for some other object to
//         consume it. The consumer constrains thread sharing so this is safe.
unsafe impl Send for Surface {}

pub trait ISurfaceExt: ISurface {
    fn get_raw_handle(&self) -> vk::SurfaceKHR;
}

impl ISurfaceExt for Surface {
    fn get_raw_handle(&self) -> SurfaceKHR {
        self.surface
    }
}

declare_interfaces!(Surface, [ISurface, ISurfaceExt]);
