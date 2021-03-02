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

use crate::dxgi::{Format, SwapChainDesc1, SwapChainFlags};
use crate::raw::windows::win32::direct3d12::{ID3D12CommandQueue, ID3D12Resource};
use crate::raw::windows::win32::dxgi::{
    IDXGISwapChain4, DXGI_MAX_SWAP_CHAIN_BUFFERS, DXGI_PRESENT_PARAMETERS,
};
use crate::CommandQueue;
use raw::windows::{Abi, Interface};
use std::mem::{forget, transmute};
use crate::utils::optional_slice_to_num_ptr_pair;

pub struct SwapChain(pub(crate) IDXGISwapChain4);

impl SwapChain {
    pub fn resize_buffers(
        &mut self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: Format,
        flags: SwapChainFlags,
        node_masks: Option<&[u32]>,
        queues: &[CommandQueue],
    ) -> raw::windows::Result<()> {
        if let Some(node_masks) = &node_masks {
            assert!(node_masks.len() <= DXGI_MAX_SWAP_CHAIN_BUFFERS as usize);
        }
        assert!(buffer_count <= DXGI_MAX_SWAP_CHAIN_BUFFERS);

        let (_, node_masks) = optional_slice_to_num_ptr_pair(node_masks);

        unsafe {
            // This is a load of hacky crap to let the function call actually compile
            //
            // Fingers crossed this actually works
            //
            // TODO: Remove this when the bindings are generated correctly by windows-rs
            let pp_queues = queues.as_ptr();
            let pp_queues: ID3D12CommandQueue = transmute(pp_queues);

            self.0
                .ResizeBuffers1(
                    buffer_count,
                    width,
                    height,
                    format.into(),
                    flags.0,
                    node_masks,
                    &pp_queues,
                )
                .ok()?;

            forget(pp_queues);
        }
        Ok(())
    }

    pub fn get_current_back_buffer_index(&self) -> u32 {
        unsafe { self.0.GetCurrentBackBufferIndex() }
    }

    pub fn get_buffers(&mut self, buffer_count: u32) -> raw::windows::Result<Vec<crate::Resource>> {
        let mut out = Vec::with_capacity(buffer_count as usize);
        for i in 0..buffer_count {
            out.push(self.get_buffer(i)?);
        }
        Ok(out)
    }

    pub fn get_buffer(&mut self, buffer: u32) -> raw::windows::Result<crate::Resource> {
        unsafe {
            let mut resource: Option<ID3D12Resource> = None;
            self.0
                .GetBuffer(buffer, &ID3D12Resource::IID, resource.set_abi())
                .and_some(resource)
                .map(|v| crate::Resource(v))
        }
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

    pub unsafe fn get_description(&self) -> raw::windows::Result<SwapChainDesc1> {
        let mut desc = Default::default();
        self.0.GetDesc1(&mut desc).ok().map(|_| transmute(desc))
    }
}
