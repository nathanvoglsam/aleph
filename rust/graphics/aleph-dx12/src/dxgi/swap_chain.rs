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
use crate::CommandQueue;
use std::mem::transmute;
use std::ops::Deref;
use std::sync::RwLockReadGuard;
use windows_raw::win32::direct3d12::{ID3D12CommandQueue, ID3D12Resource};
use windows_raw::win32::dxgi::{
    IDXGISwapChain4, DXGI_MAX_SWAP_CHAIN_BUFFERS, DXGI_PRESENT_PARAMETERS,
};
use windows_raw::IUnknown;
use windows_raw::{Abi, Interface};

pub struct SwapChain(pub(crate) IDXGISwapChain4);

impl SwapChain {
    /// `IDXGISwapChain3::ResizeBuffers1`
    pub unsafe fn resize_buffers(
        &mut self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: Format,
        flags: SwapChainFlags,
        node_masks: Option<&[u32]>,
        queues: &[&CommandQueue],
    ) -> crate::Result<()> {
        assert!(
            queues.len() <= DXGI_MAX_SWAP_CHAIN_BUFFERS as usize,
            "queues len must be <= 16"
        );
        assert_eq!(
            queues.len(),
            buffer_count as usize,
            "queues len must == buffer count if buffer count != 0"
        );
        assert!(
            buffer_count <= DXGI_MAX_SWAP_CHAIN_BUFFERS,
            "can't have more than 16 swap chain buffers"
        );

        let node_masks = if let Some(node_masks) = node_masks {
            assert!(!node_masks.iter().any(|v| v.count_ones() > 1));
            assert!(
                node_masks.len() <= DXGI_MAX_SWAP_CHAIN_BUFFERS as usize,
                "node masks len must be <= 16"
            );
            assert_eq!(
                node_masks.len(),
                buffer_count as usize,
                "node masks len must == buffer count"
            );
            node_masks.as_ptr()
        } else {
            static DEFAULT_NODE_MASKS: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            DEFAULT_NODE_MASKS.as_ptr()
        };

        let mut locks: [Option<RwLockReadGuard<ID3D12CommandQueue>>; 16] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        let mut unpacked_queues: [Option<IUnknown>; 16] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        queues.iter().enumerate().for_each(|(index, queue)| {
            let lock = queue.get_raw_shared();
            let ptr = lock.deref().clone();
            unpacked_queues[index] = transmute(ptr);
            locks[index] = Some(lock);
        });

        self.0
            .ResizeBuffers1(
                buffer_count,
                width,
                height,
                format.into(),
                flags.0,
                node_masks,
                unpacked_queues.as_mut_ptr(),
            )
            .ok()
    }

    /// `IDXGISwapChain1::Present1`
    pub unsafe fn present(&mut self, sync_interval: u32, present_flags: u32) -> crate::Result<()> {
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

    /// `IDXGISwapChain3::GetCurrentBackBufferIndex`
    pub fn get_current_back_buffer_index(&mut self) -> u32 {
        unsafe { self.0.GetCurrentBackBufferIndex() }
    }

    pub fn get_buffers(&mut self, buffer_count: u32) -> crate::Result<Vec<crate::Resource>> {
        let mut out = Vec::with_capacity(buffer_count as usize);
        for i in 0..buffer_count {
            out.push(self.get_buffer(i)?);
        }
        Ok(out)
    }

    pub fn get_buffer(&mut self, buffer: u32) -> crate::Result<crate::Resource> {
        unsafe {
            let mut resource: Option<ID3D12Resource> = None;
            self.0
                .GetBuffer(buffer, &ID3D12Resource::IID, resource.set_abi())
                .and_some(resource)
                .map(|v| crate::Resource(v))
        }
    }

    pub fn get_description(&mut self) -> crate::Result<SwapChainDesc1> {
        unsafe {
            let mut desc = Default::default();
            self.0.GetDesc1(&mut desc).ok().map(|_| transmute(desc))
        }
    }
}
