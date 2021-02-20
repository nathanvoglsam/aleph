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
    IDXGISwapChain4, DXGI_FORMAT, DXGI_PRESENT_PARAMETERS, DXGI_SWAP_CHAIN_DESC1,
};
use raw::windows::{Abi, Interface};

pub struct SwapChain(pub(crate) IDXGISwapChain4);

impl SwapChain {
    pub unsafe fn resize_buffers(
        &mut self,
        width: u32,
        height: u32,
        flags: u32,
    ) -> raw::windows::Result<()> {
        self.0
            .ResizeBuffers(0, width, height, DXGI_FORMAT::DXGI_FORMAT_UNKNOWN, flags)
            .ok()
    }

    pub unsafe fn get_current_back_buffer_index(&self) -> u32 {
        self.0.GetCurrentBackBufferIndex()
    }

    pub unsafe fn get_buffers(
        &self,
        buffer_count: u32,
    ) -> raw::windows::Result<Vec<ID3D12Resource>> {
        let mut out = Vec::with_capacity(buffer_count as usize);
        for i in 0..buffer_count {
            out.push(self.get_buffer(i)?);
        }
        Ok(out)
    }

    pub unsafe fn get_buffer(&self, buffer: u32) -> raw::windows::Result<ID3D12Resource> {
        let mut resource: Option<ID3D12Resource> = None;
        self.0
            .GetBuffer(buffer, &ID3D12Resource::IID, resource.set_abi())
            .and_some(resource)
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
        self.0
            .Present1(sync_interval, present_flags, &presentation_params)
            .ok()
    }

    pub unsafe fn get_description(&self) -> raw::windows::Result<DXGI_SWAP_CHAIN_DESC1> {
        let mut desc = Default::default();
        self.0.GetDesc1(&mut desc).ok().map(|_| desc)
    }
}
