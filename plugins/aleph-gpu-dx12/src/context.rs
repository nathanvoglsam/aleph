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

use crate::adapter::Adapter;
use crate::surface::Surface;
use dx12::dxgi;
use interfaces::any::{declare_interfaces, QueryInterface, QueryInterfaceBox};
use interfaces::gpu::{
    AdapterPowerClass, AdapterRequestOptions, IGpuAdapter, IGpuContext, IGpuSurface,
};
use interfaces::platform::HasRawWindowHandle;

pub struct Context {
    pub(crate) _debug: Option<dx12::Debug>,
    pub(crate) factory: dxgi::Factory,
}

impl Context {
    /// Checks if a surface is compatible with an adapter by performing a full device initialization
    /// in order to check if it would succeed.
    ///
    /// There's no other way to check if the surface can be used on the device so we just eat some
    /// overhead at init time to do this.
    fn check_surface_compatibility(
        &self,
        adapter: &dxgi::Adapter,
        surface: &Surface,
    ) -> Option<()> {
        let device = dx12::Device::new(adapter, dx12::FeatureLevel::Level_11_0).ok()?;

        // Create a direct queue so we can create a swapchain
        let desc = dx12::CommandQueueDesc::builder()
            .queue_type(dx12::CommandListType::Direct)
            .priority(0)
            .build();
        let queue = device.create_command_queue(&desc).ok()?;

        // Create our swap chain to check if the surface is compatible
        let desc = dxgi::SwapChainDesc1::builder()
            .width(128) // Dummy values, shouldn't be important?
            .height(128) // Dummy values, shouldn't be important?
            .format(dxgi::Format::R8G8B8A8Unorm) // Guaranteed supported format
            .buffer_count(3)
            .usage_flags(dxgi::UsageFlags::BACK_BUFFER)
            .usage_flags(dxgi::UsageFlags::RENDER_TARGET_OUTPUT)
            .build();
        self.factory
            .create_swap_chain(&queue, &surface, &desc)
            .ok()?;

        Some(())
    }
}

impl IGpuContext for Context {
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<Box<dyn IGpuAdapter>> {
        let power_preference = match options.power_class {
            AdapterPowerClass::LowPower => dxgi::GpuPreference::MinimumPower,
            AdapterPowerClass::HighPower => dxgi::GpuPreference::HighPerformance,
        };
        if let Some(adapter) = self
            .factory
            .select_hardware_adapter(dx12::FeatureLevel::Level_11_0, power_preference)
        {
            if let Some(surface) = options.surface.as_ref() {
                let surface = surface.query_interface::<Surface>().unwrap();
                self.check_surface_compatibility(&adapter, &surface)?;
            }

            let desc = adapter
                .get_adapter_desc()
                .expect("Failed to get adapter description. Something very wrong");
            let name = desc.description_string().unwrap_or("Unknown".to_string());
            let adapter = Adapter { name, adapter };
            let adapter = Box::new(adapter);
            Some(adapter)
        } else {
            None
        }
    }

    fn create_surface(&self, window: &dyn HasRawWindowHandle) -> Box<dyn IGpuSurface> {
        let surface = Surface {
            factory: self.factory.clone(),
            handle: window.raw_window_handle(),
        };

        let surface = Box::new(surface);
        surface.query_interface().ok().unwrap()
    }
}

// SAFETY: Can't be auto marked because of the COM pointers. COM pointers are just Arc, which is
// fine to send across thread boundaries
unsafe impl Send for Context {}

pub trait IGpuContextExt: IGpuContext {
    fn get_raw_handle(&self) -> &dxgi::Factory;
}

impl IGpuContextExt for Context {
    fn get_raw_handle(&self) -> &dxgi::Factory {
        &self.factory
    }
}

declare_interfaces!(Context, [IGpuContext, IGpuContextExt]);