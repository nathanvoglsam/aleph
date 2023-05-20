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
use windows::core::{ComInterface, GUID};
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Dxgi::*;

type CreateFn =
    extern "system" fn(u32, *const GUID, *mut *mut ::std::ffi::c_void) -> windows::core::HRESULT;

pub static CREATE_FN: DynamicLoadCell<CreateFn> =
    DynamicLoadCell::new(&utf16_null!("dxgi.dll"), "CreateDXGIFactory2\0");

#[inline]
pub fn create_dxgi_factory(debug: bool) -> windows::core::Result<IDXGIFactory2> {
    unsafe {
        let create_fn = *CREATE_FN.get().expect("Failed to load dxgi.dll");

        let flags = if debug { 0x1 } else { 0x0 };

        let mut dxgi_factory: Option<IDXGIFactory2> = None;

        let ptr = &mut dxgi_factory;
        let ptr = ptr as *mut Option<IDXGIFactory2>;
        let ptr = ptr as *mut *mut ::std::ffi::c_void;

        create_fn(flags, &IDXGIFactory2::IID, ptr).and_some(dxgi_factory)
    }
}
