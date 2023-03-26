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
use crate::internal::unwrap;
use crate::swap_chain::{SwapChain, SwapChainState};
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use parking_lot::Mutex;
use std::any::TypeId;

pub struct Surface {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) context: AnyArc<Context>,
    pub(crate) surface: vk::SurfaceKHR,
}

declare_interfaces!(Surface, [ISurface]);

impl IGetPlatformInterface for Surface {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<vk::SurfaceKHR>(&self.surface, out, target)
    }
}

impl Surface {
    /// Internal function for querying present support for a given surface
    unsafe fn get_queue_support(
        device: &Device,
        surface: vk::SurfaceKHR,
    ) -> Result<QueuePresentSupportFlags, vk::Result> {
        let mut flags = QueuePresentSupportFlags::empty();

        if let Some(queue) = device.general_queue.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.info.family_index,
                    surface,
                )
                .result()?;
            if supported {
                flags |= QueuePresentSupportFlags::GENERAL;
            }
        }
        if let Some(queue) = device.compute_queue.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.info.family_index,
                    surface,
                )
                .result()?;
            if supported {
                flags |= QueuePresentSupportFlags::COMPUTE;
            }
        }
        if let Some(queue) = device.transfer_queue.as_ref() {
            let supported = device
                .context
                .instance_loader
                .get_physical_device_surface_support_khr(
                    device.adapter.physical_device,
                    queue.info.family_index,
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
        let device = unwrap::device(device);

        let queue_support = unsafe { Surface::get_queue_support(device, self.surface).unwrap() };

        let inner = SwapChainState {
            swap_chain: vk::SwapchainKHR::null(),
            format: Format::Bgra8Unorm,
            vk_format: Default::default(),
            color_space: Default::default(),
            present_mode: Default::default(),
            vk_present_mode: Default::default(),
            extent: Default::default(),
            images: Vec::new(),
            acquired: false,
        };
        let swap_chain = AnyArc::new_cyclic(move |v| SwapChain {
            this: v.clone(),
            device: device.this.upgrade().unwrap(),
            surface: self.this.upgrade().unwrap(),
            inner: Mutex::new(inner),
            queue_support,
        });

        // TODO: This is unsound and wrong, no checks have been made yet
        unsafe {
            let mut inner = swap_chain.inner.lock();
            swap_chain.build(&mut inner, config)?;
        }

        Ok(AnyArc::map::<dyn ISwapChain, _>(swap_chain, |v| v))
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.context
                .instance_loader
                .destroy_surface_khr(self.surface, None);
        }
    }
}

// SAFETY: RawWindowHandle is an opaque handle and can the only purpose is for some other object to
//         consume it. The consumer constrains thread sharing so this is safe.
unsafe impl Send for Surface {}
