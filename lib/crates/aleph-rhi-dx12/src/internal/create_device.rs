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

use utf16_lit::utf16_null;
use windows::core::{IUnknown, Interface};
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::*;

pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

pub fn create_device<'a>(
    adapter: impl Into<Option<&'a IDXGIAdapter1>>,
    minimum_feature_level: D3D_FEATURE_LEVEL,
) -> windows::core::Result<ID3D12Device10> {
    unsafe {
        let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();

        // create_fn won't decrement the reference counter if we clone the adapter (which
        // increments the counter). To work around this we use transmute_copy to make a copy without
        // incrementing the reference count. Otherwise we leak a reference to the adapter
        let adapter: Option<IUnknown> = adapter.into().map(|v| std::mem::transmute_copy(v));

        let mut device: Option<ID3D12Device10> = None;

        let ptr = &mut device;
        let ptr = ptr as *mut Option<ID3D12Device10>;
        let ptr = ptr as *mut *mut ::std::ffi::c_void;
        create_fn(adapter, minimum_feature_level, &ID3D12Device10::IID, ptr).and_some(device)
    }
}
