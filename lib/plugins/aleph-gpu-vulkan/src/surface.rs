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
use crate::format::texture_format_to_vk;
use crate::swap_chain::SwapChain;
use interfaces::any::declare_interfaces;
use interfaces::gpu::{
    IDevice, ISurface, ISwapChain, PresentationMode, SwapChainConfiguration, SwapChainCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};

pub struct Surface {
    pub(crate) handle: RawWindowHandle,
}

impl ISurface for Surface {
    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<Box<dyn ISwapChain>, SwapChainCreateError> {
        todo!()
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

pub trait ISurfaceExt: ISurface {}

impl ISurfaceExt for Surface {}

declare_interfaces!(Surface, [ISurface, ISurfaceExt]);
