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

#![allow(non_camel_case_types)]

//!
//! This provides a set of wrappers over some variant of the 'DXGIGetDebugInterface' function.
//!
//! PC Windows can dynamically lookup the symbol from DXGIDebug.dll, so we do exactly that as it
//! doesn't impose any linking requirements.
//!
//! UWP supposedly requires linking directly to DXGIGetDebugInterface1. While PC can do it as well
//! I would prefer to avoid it so we use a special case on UWP.
//!
//! Other platforms get an 'unimplemented!' stub.
//!

#[cfg(all(target_os = "windows", not(target_vendor = "uwp")))]
pub use self::pc::dxgi_get_debug_interface;

#[cfg(all(target_os = "windows", target_vendor = "uwp"))]
pub use self::uwp::dxgi_get_debug_interface;

#[cfg(not(target_os = "windows"))]
pub use self::unimplemented::dxgi_get_debug_interface;

#[cfg(all(target_os = "windows", not(target_vendor = "uwp")))]
mod pc {
    use utf16_lit::utf16_null;
    use windows::core::Interface;
    use windows::utils::DynamicLoadCell;
    use windows::Win32::Foundation::*;

    type PFN_DXGI_GET_DEBUG_INTERFACE = Option<
        extern "system" fn(
            riid: *const windows::core::GUID,
            pdebug: *mut *mut core::ffi::c_void,
        ) -> windows::core::HRESULT,
    >;

    pub static CREATE_FN: DynamicLoadCell<PFN_DXGI_GET_DEBUG_INTERFACE> =
        DynamicLoadCell::new(&utf16_null!("DXGIDebug.dll"), "DXGIGetDebugInterface\0");

    pub unsafe fn dxgi_get_debug_interface<T: Interface>() -> windows::core::Result<T> {
        let create_fn = CREATE_FN.get()?;
        let create_fn = create_fn
            .as_ref()
            .ok_or(windows::core::Error::from(E_FAIL))?;

        let mut result__ = None;
        create_fn(&<T as Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}

#[cfg(all(target_os = "windows", target_vendor = "uwp"))]
mod uwp {
    use windows::core::Interface;
    use windows::Win32::Graphics::Dxgi::*;

    pub unsafe fn dxgi_get_debug_interface<T: Interface>() -> windows::core::Result<T> {
        DXGIGetDebugInterface1(0)
    }
}

#[cfg(not(target_os = "windows"))]
mod unimplemented {
    use windows::core::Interface;

    pub unsafe fn dxgi_get_debug_interface<T: Interface>() -> windows::core::Result<T> {
        unimplemented!("Unsupported on non windows platforms")
    }
}
