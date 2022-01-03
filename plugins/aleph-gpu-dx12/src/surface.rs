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

use crate::device::Device;
use crate::format::texture_format_to_dxgi;
use crate::swap_chain::SwapChain;
use dx12::dxgi;
use dx12::dxgi::SwapChainFlags;
use interfaces::any::{declare_interfaces, AnyArc, QueryInterface};
use interfaces::gpu::{
    IGpuDevice, IGpuSurface, IGpuSwapChain, PresentationMode, SwapChainConfiguration,
    SwapChainCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};

pub struct Surface {
    pub(crate) factory: dxgi::Factory,
    pub(crate) handle: RawWindowHandle,
}

impl IGpuSurface for Surface {
    fn create_swap_chain(
        &self,
        device: &dyn IGpuDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn IGpuSwapChain>, SwapChainCreateError> {
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
        let swap_chain = self
            .factory
            .create_swap_chain(&device.queue, self, &desc)
            .map_err(|e| {
                let e = Box::new(e);
                SwapChainCreateError::Platform(e)
            })?;
        let swap_chain = SwapChain { swap_chain };
        let swap_chain = AnyArc::new(swap_chain);
        Ok(swap_chain.query_interface().unwrap())
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.handle
    }
}

pub trait IGpuSurfaceExt: IGpuSurface {
    fn get_raw_handle(&self) -> dxgi::Factory;
}

impl IGpuSurfaceExt for Surface {
    fn get_raw_handle(&self) -> dxgi::Factory {
        self.factory.clone()
    }
}

declare_interfaces!(Surface, [IGpuSurface, IGpuSurfaceExt]);
