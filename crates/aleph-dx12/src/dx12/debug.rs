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

use crate::windows::core::Interface;
use utf16_lit::utf16_null;
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Direct3D12::{
    ID3D12Debug, ID3D12Debug1, PFN_D3D12_GET_DEBUG_INTERFACE,
};

pub(crate) static CREATE_FN: DynamicLoadCell<PFN_D3D12_GET_DEBUG_INTERFACE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12GetDebugInterface\0");

#[repr(transparent)]
pub struct Debug(pub(crate) ID3D12Debug);

impl Debug {
    #[inline]
    pub unsafe fn new() -> windows::core::Result<Self> {
        let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();
        let mut debug: Option<ID3D12Debug> = None;
        let ptr = &mut debug;
        let ptr = ptr as *mut Option<ID3D12Debug>;
        let ptr = ptr as *mut *mut ::std::ffi::c_void;
        create_fn(&ID3D12Debug::IID, ptr)
            .and_some(debug)
            .map(|v| Self(v))
    }

    #[inline]
    pub unsafe fn enable_debug_layer(&self) {
        self.0.EnableDebugLayer()
    }

    #[inline]
    pub unsafe fn set_enable_gpu_validation(&self, enable: bool) -> windows::core::Result<()> {
        let casted = self.0.cast::<ID3D12Debug1>()?;
        casted.SetEnableGPUBasedValidation(enable);
        Ok(())
    }

    #[inline]
    pub unsafe fn set_enable_synchronized_command_queue_validation(
        &self,
        enable: bool,
    ) -> windows::core::Result<()> {
        let casted = self.0.cast::<ID3D12Debug1>()?;
        casted.SetEnableSynchronizedCommandQueueValidation(enable);
        Ok(())
    }
}

crate::owned_object!(Debug);
