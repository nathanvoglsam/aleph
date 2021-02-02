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

use crate::raw::windows::win32::direct3d12::ID3D12CommandQueue;
use crate::raw::windows::win32::dxgi::{
    IDXGISwapChain1, DXGI_ALPHA_MODE, DXGI_FORMAT, DXGI_SAMPLE_DESC, DXGI_SCALING,
    DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT, DXGI_USAGE_BACK_BUFFER,
    DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use crate::raw::windows::win32::windows_and_messaging::HWND;
use crate::raw::windows::Error;
use crate::Device;
use raw_window_handle::windows::WindowsHandle;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

/// Represents the set of errors that can be encountered from swapchain creation
#[derive(Clone, Debug, PartialEq)]
pub enum SwapChainCreateError {
    DidNotProvideQueue,
    DidNotProvideHWND,
    InvalidBackBufferCount,
    CreationFailed(Error),
}

/// A `Result` wrapper type used for swapchain initialization
pub type SwapChainCreateResult<T> = Result<T, SwapChainCreateError>;

pub struct SwapChainBuilder {
    width: u32,
    height: u32,
    buffer_count: u32,
    allow_tearing: bool,
    queue: Option<ID3D12CommandQueue>,
    hwnd: Option<WindowsHandle>,
}

impl SwapChainBuilder {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            buffer_count: 2,
            allow_tearing: false,
            queue: None,
            hwnd: None,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.buffer_count = buffer_count;
        self
    }

    pub fn allow_tearing(mut self, allow_tearing: bool) -> Self {
        self.allow_tearing = allow_tearing;
        self
    }

    pub fn queue(mut self, queue: ID3D12CommandQueue) -> Self {
        self.queue = Some(queue);
        self
    }

    pub fn hwnd(mut self, hwnd: &impl HasRawWindowHandle) -> Self {
        if let RawWindowHandle::Windows(hwnd) = hwnd.raw_window_handle() {
            self.hwnd = Some(hwnd);
            self
        } else {
            panic!();
        }
    }

    pub fn build(self, device: &Device) -> SwapChainCreateResult<SwapChain> {
        aleph_log::trace!("Initializing swapchain");

        // Assert the back buffer count is valid
        if self.buffer_count < 2 || self.buffer_count > 16 {
            aleph_log::error!("Invalid back buffer count requested");
            return Err(SwapChainCreateError::InvalidBackBufferCount);
        }

        // Just choose a standard RGBA32 back buffer format
        let format = DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_UNORM;

        // This will always be false
        let stereo = false.into();

        // This is the only valid form under dx12
        let sample_desc = DXGI_SAMPLE_DESC {
            count: 1,
            quality: 0,
        };

        // These are the valid usages we support
        let buffer_usage = DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT;

        // Accumulate flags based on configuration
        let mut flags = 0;
        if self.allow_tearing {
            flags |= DXGI_SWAP_CHAIN_FLAG::DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0
        }

        // Build the description
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            width: self.width,
            height: self.height,
            format,
            stereo,
            sample_desc,
            buffer_usage,
            buffer_count: self.buffer_count,
            scaling: DXGI_SCALING::DXGI_SCALING_NONE,
            swap_effect: DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_DISCARD,
            alpha_mode: DXGI_ALPHA_MODE::DXGI_ALPHA_MODE_IGNORE,
            flags: flags as _,
        };

        let queue = self.queue.ok_or(SwapChainCreateError::DidNotProvideQueue)?;
        let hwnd = self.hwnd.ok_or(SwapChainCreateError::DidNotProvideHWND)?;
        let hwnd = hwnd.hwnd as isize;
        let hwnd = HWND(hwnd);

        // Create the swapchain
        aleph_log::trace!("Initializing IDXGISwapChain");
        let mut swapchain: Option<IDXGISwapChain1> = None;
        device
            .dxgi_factory
            .CreateSwapChainForHwnd(queue, hwnd, &desc, std::ptr::null(), None, &mut swapchain)
            .and_some(swapchain)
            .map(|v| SwapChain { swapchain: v })
            .map_err(|v| SwapChainCreateError::CreationFailed(v))
    }
}

#[derive(Clone)]
pub struct SwapChain {
    pub swapchain: IDXGISwapChain1,
}

impl SwapChain {
    pub fn builder() -> SwapChainBuilder {
        SwapChainBuilder::new()
    }
}
