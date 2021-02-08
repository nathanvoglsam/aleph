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

use crate::raw::windows::win32::direct3d12::ID3D12Resource;
use crate::raw::windows::win32::dxgi::{
    IDXGISwapChain1, DXGI_ALPHA_MODE, DXGI_FORMAT, DXGI_PRESENT_PARAMETERS, DXGI_SAMPLE_DESC,
    DXGI_SCALING, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_EFFECT,
    DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use crate::raw::windows::win32::windows_and_messaging::HWND;
use crate::raw::windows::Error;
use crate::{CommandQueue, DXGIFactory};
use raw::windows::{Abi, Interface};
use raw_window_handle::windows::WindowsHandle;

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

pub struct SwapChainBuilder<'a, 'b> {
    pub(crate) queue: &'a CommandQueue,
    pub(crate) factory: &'b DXGIFactory,
    pub(crate) hwnd: WindowsHandle,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer_count: u32,
    pub(crate) allow_tearing: bool,
}

impl<'a, 'b> SwapChainBuilder<'a, 'b> {
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

    pub unsafe fn build(self) -> SwapChainCreateResult<SwapChain> {
        // Assert the back buffer count is valid
        if self.buffer_count < 2 || self.buffer_count > 16 {
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

        let buffer_count = self.buffer_count;

        // Accumulate flags based on configuration
        let mut flags = 0u32;
        if self.allow_tearing {
            flags |= DXGI_SWAP_CHAIN_FLAG::DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32
        }

        // Build the description
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            width: self.width,
            height: self.height,
            format,
            stereo,
            sample_desc,
            buffer_usage,
            buffer_count,
            scaling: DXGI_SCALING::DXGI_SCALING_NONE,
            swap_effect: DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_DISCARD,
            alpha_mode: DXGI_ALPHA_MODE::DXGI_ALPHA_MODE_IGNORE,
            flags: flags as _,
        };

        let hwnd = self.hwnd.hwnd as isize;
        let hwnd = HWND(hwnd);

        // Create the swapchain
        let mut swapchain: Option<IDXGISwapChain1> = None;
        self.factory
            .0
            .CreateSwapChainForHwnd(
                &self.queue.0,
                hwnd,
                &desc,
                std::ptr::null(),
                None,
                &mut swapchain,
            )
            .and_some(swapchain)
            .map(|v| {
                let swapchain = v;
                let views = get_buffers(&swapchain, buffer_count);
                SwapChain {
                    swapchain,
                    views,
                    flags,
                }
            })
            .map_err(|v| SwapChainCreateError::CreationFailed(v))
    }
}

#[derive(Clone)]
pub struct SwapChain {
    pub(crate) swapchain: IDXGISwapChain1,
    pub(crate) views: Vec<ID3D12Resource>,
    pub(crate) flags: u32,
}

impl SwapChain {
    pub unsafe fn resize_buffers(&mut self, width: u32, height: u32) -> raw::windows::Result<()> {
        // Empty views as we're holding on to resources from the swapchain in that array
        self.views.clear();

        self.swapchain
            .ResizeBuffers(
                0,
                width,
                height,
                DXGI_FORMAT::DXGI_FORMAT_UNKNOWN,
                self.flags,
            )
            .ok()
    }

    pub unsafe fn present(
        &self,
        sync_interval: u32,
        present_flags: u32,
    ) -> crate::raw::windows::Result<()> {
        let presentation_params = DXGI_PRESENT_PARAMETERS {
            dirty_rects_count: 0,
            p_dirty_rects: std::ptr::null_mut(),
            p_scroll_rect: std::ptr::null_mut(),
            p_scroll_offset: std::ptr::null_mut(),
        };
        self.swapchain
            .Present1(sync_interval, present_flags, &presentation_params)
            .ok()
    }

    pub fn get_description(&self) -> raw::windows::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut desc = Default::default();
        unsafe { self.swapchain.GetDesc1(&mut desc).ok().map(|_| desc) }
    }

    pub fn raw(&self) -> &IDXGISwapChain1 {
        &self.swapchain
    }

    pub fn views(&self) -> &[ID3D12Resource] {
        &self.views
    }
}

fn get_buffers(swapchain: &IDXGISwapChain1, count: u32) -> Vec<ID3D12Resource> {
    (0..count)
        .into_iter()
        .map(|buffer| unsafe {
            let mut resource: Option<ID3D12Resource> = None;
            let resource = swapchain
                .GetBuffer(buffer, &ID3D12Resource::IID, resource.set_abi())
                .and_some(resource)
                .unwrap();
            resource
        })
        .collect()
}
