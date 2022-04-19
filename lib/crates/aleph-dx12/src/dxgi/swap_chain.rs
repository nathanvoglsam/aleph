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
use crate::{CommandQueue, WeakRef};
use std::mem::transmute;
use windows::core::IUnknown;
use windows::Win32::Graphics::Dxgi::{
    IDXGISwapChain4, DXGI_MAX_SWAP_CHAIN_BUFFERS, DXGI_PRESENT_PARAMETERS,
};

#[repr(transparent)]
pub struct SwapChain(pub(crate) IDXGISwapChain4);

impl SwapChain {
    /// `IDXGISwapChain3::ResizeBuffers1`
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn resize_buffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: Format,
        flags: SwapChainFlags,
        node_masks: Option<&[u32]>,
        queues: &[WeakRef<CommandQueue>],
    ) -> windows::core::Result<()> {
        // Input validation
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

        // Input validation + arg unpacking
        let p_creation_node_mask = if let Some(node_masks) = node_masks {
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

        // Arg unpack
        let pp_present_queue = queues.as_ptr() as *mut WeakRef<CommandQueue>;
        let pp_present_queue = pp_present_queue as *mut Option<IUnknown>;

        self.0.ResizeBuffers1(
            buffer_count,
            width,
            height,
            format.into(),
            flags.0,
            p_creation_node_mask,
            pp_present_queue,
        )
    }

    /// `IDXGISwapChain1::Present1`
    #[inline]
    pub unsafe fn present(
        &self,
        sync_interval: u32,
        present_flags: u32,
    ) -> windows::core::Result<()> {
        let presentation_params = DXGI_PRESENT_PARAMETERS {
            DirtyRectsCount: 0,
            pDirtyRects: std::ptr::null_mut(),
            pScrollRect: std::ptr::null_mut(),
            pScrollOffset: std::ptr::null_mut(),
        };
        self.0
            .Present1(sync_interval, present_flags, &presentation_params)
    }

    /// `IDXGISwapChain3::GetCurrentBackBufferIndex`
    #[inline]
    pub fn get_current_back_buffer_index(&self) -> u32 {
        unsafe { self.0.GetCurrentBackBufferIndex() }
    }

    #[inline]
    pub fn get_buffers(&self, buffer_count: u32) -> windows::core::Result<Vec<crate::Resource>> {
        let mut out = Vec::with_capacity(buffer_count as usize);
        for i in 0..buffer_count {
            out.push(self.get_buffer(i)?);
        }
        Ok(out)
    }

    #[inline]
    pub fn get_buffer(&self, buffer: u32) -> windows::core::Result<crate::Resource> {
        unsafe { self.0.GetBuffer(buffer).map(crate::Resource) }
    }

    #[inline]
    pub fn get_description(&self) -> windows::core::Result<SwapChainDesc1> {
        unsafe { self.0.GetDesc1().map(|v| transmute(v)) }
    }
}

impl Clone for SwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
