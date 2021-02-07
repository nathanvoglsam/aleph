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

use crate::raw::windows::win32::direct3d12::{D3D12GetDebugInterface, ID3D12Debug, ID3D12Debug1};
use crate::raw::windows::{Abi, Interface};

#[derive(Clone)]
#[repr(transparent)]
pub struct Debug(pub(crate) ID3D12Debug);

impl Debug {
    pub unsafe fn new() -> raw::windows::Result<Self> {
        let mut debug: Option<ID3D12Debug> = None;
        D3D12GetDebugInterface(&ID3D12Debug::IID, debug.set_abi())
            .and_some(debug)
            .map(|v| Self(v))
    }

    pub unsafe fn enable_debug_layer(&self) {
        self.0.EnableDebugLayer()
    }

    pub unsafe fn set_enable_gpu_validation(&self, enable: bool) -> raw::windows::Result<()> {
        let casted = self.0.cast::<ID3D12Debug1>()?;
        casted.SetEnableGPUBasedValidation(enable.into());
        Ok(())
    }

    pub unsafe fn set_enable_synchronized_command_queue_validation(
        &self,
        enable: bool,
    ) -> raw::windows::Result<()> {
        let casted = self.0.cast::<ID3D12Debug1>()?;
        casted.SetEnableSynchronizedCommandQueueValidation(enable.into());
        Ok(())
    }

    pub fn raw(&self) -> &ID3D12Debug {
        &self.0
    }
}
