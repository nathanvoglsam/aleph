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

use crate::raw::windows::win32::direct3d11::D3D_FEATURE_LEVEL;
use crate::raw::windows::win32::direct3d12::{D3D12CreateDevice, ID3D12Device};
use crate::raw::windows::win32::dxgi::{
    IDXGIAdapter1, IDXGIFactory1, IDXGIFactory6, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG,
    DXGI_GPU_PREFERENCE,
};
use crate::raw::windows::{Abi, Interface};
use std::char::{decode_utf16, DecodeUtf16};
use std::ffi::OsString;

pub unsafe fn select_adapter(
    factory: &IDXGIFactory1,
    minimum_feature_level: D3D_FEATURE_LEVEL,
) -> Option<IDXGIAdapter1> {
    // If possible we can explicitly ask for a "high performance" device.
    let factory_6 = factory.cast::<IDXGIFactory6>().ok();

    // Loop over all the available adapters
    let mut i = 0;
    loop {
        // Use the newest available interface to enumerate the adapter
        let adapter = if let Some(factory) = factory_6.as_ref() {
            enum_edapter_new(factory, i)
        } else {
            enum_edapter_old(factory, i)
        };

        // Check if we've gotten an adapter, or break from the loop if we don't as we've either hit
        // a big error or enumerated all of them already
        if let Ok(adapter) = adapter {
            // Get the adapter description so we can decide if we want to use it or not
            let mut desc = DXGI_ADAPTER_DESC1::default();
            adapter.GetDesc1(&mut desc).unwrap();

            // We want to skip software adapters as they're going to be *very* slow
            if (desc.flags & DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE.0) != 0 {
                continue;
            }

            // Check if the device supports the feature level we want by trying to create a device
            let result = D3D12CreateDevice(
                Some(adapter.cast().unwrap()),
                minimum_feature_level,
                &ID3D12Device::IID,
                std::ptr::null_mut(),
            );

            // If we succeeded then use the adapter
            if result.is_ok() {
                return Some(adapter);
            }
        } else {
            break;
        }

        i += 1;
    }

    None
}

unsafe fn enum_edapter_new(factory: &IDXGIFactory6, i: u32) -> raw::windows::Result<IDXGIAdapter1> {
    let mut ppv_adapter: Option<IDXGIAdapter1> = None;
    factory
        .EnumAdapterByGpuPreference(
            i,
            DXGI_GPU_PREFERENCE::DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            &IDXGIAdapter1::IID,
            ppv_adapter.set_abi(),
        )
        .and_some(ppv_adapter)
}

unsafe fn enum_edapter_old(factory: &IDXGIFactory1, i: u32) -> raw::windows::Result<IDXGIAdapter1> {
    let mut pp_adapter: Option<IDXGIAdapter1> = None;
    factory
        .EnumAdapters1(i, &mut pp_adapter)
        .and_some(pp_adapter)
}

pub fn extract_null_terminated(slice: &[u16]) -> &[u16] {
    let mut i = 0;
    for v in slice {
        if *v == 0 {
            break;
        }
        i += 1;
    }
    &slice[0..i]
}
