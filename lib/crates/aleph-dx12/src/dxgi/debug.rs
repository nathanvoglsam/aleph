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
use windows::core::Interface;
use windows::core::{GUID, HRESULT};
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Dxgi::{IDXGIDebug1, DXGI_DEBUG_RLO_FLAGS};

type CreateFn = extern "system" fn(
    //flags: u32,
    riid: *const GUID,
    p_debug: *mut *mut ::std::ffi::c_void,
) -> HRESULT;

static CREATE_FN: DynamicLoadCell<CreateFn> =
    DynamicLoadCell::new(&utf16_null!("dxgidebug.dll"), "DXGIGetDebugInterface\0");

#[repr(transparent)]
pub struct Debug(pub(crate) IDXGIDebug1);

impl Debug {
    #[inline]
    pub fn new() -> windows::core::Result<Debug> {
        unsafe {
            let create_fn = *CREATE_FN.get().expect("Failed to load dxgidebug.dll");
            let mut dxgi_debug: Option<IDXGIDebug1> = None;
            let ptr = &mut dxgi_debug;
            let ptr = ptr as *mut Option<IDXGIDebug1>;
            let ptr = ptr as *mut *mut ::std::ffi::c_void;
            create_fn(&IDXGIDebug1::IID, ptr)
                .and_some(dxgi_debug)
                .map(Self)
        }
    }

    #[inline]
    pub fn report_live_objects(
        &mut self,
        debug_id: DebugID,
        flags: DebugRLOFlags,
    ) -> crate::windows::core::Result<()> {
        let debug_id: GUID = debug_id.into();
        unsafe { self.0.ReportLiveObjects(debug_id, flags.into()) }
    }
}

/// A rustier wrapper around `DXGI_DEBUG_ID`
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum DebugID {
    All,
    DirectX,
    DXGI,
    App,
    Direct3D11,
}

impl From<DebugID> for GUID {
    #[inline]
    fn from(v: DebugID) -> Self {
        match v {
            DebugID::All => GUID::from_values(
                0xe48ae283,
                0xda80,
                0x490b,
                [0x87, 0xe6, 0x43, 0xe9, 0xa9, 0xcf, 0xda, 0x8],
            ),
            DebugID::DirectX => GUID::from_values(
                0x35cdd7fc,
                0x13b2,
                0x421d,
                [0xa5, 0xd7, 0x7e, 0x44, 0x51, 0x28, 0x7d, 0x64],
            ),
            DebugID::DXGI => GUID::from_values(
                0x25cddaa4,
                0xb1c6,
                0x47e1,
                [0xac, 0x3e, 0x98, 0x87, 0x5b, 0x5a, 0x2e, 0x2a],
            ),
            DebugID::App => GUID::from_values(
                0x6cd6e01,
                0x4219,
                0x4ebd,
                [0x87, 0x9, 0x27, 0xed, 0x23, 0x36, 0xc, 0x62],
            ),
            DebugID::Direct3D11 => GUID::from_values(
                0x4b99317b,
                0xac39,
                0x4aa6,
                [0xbb, 0xb, 0xba, 0xa0, 0x47, 0x84, 0x79, 0x8f],
            ),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct DebugRLOFlags(pub u32);

windows::flags_bitwise_impl!(DebugRLOFlags);

impl DebugRLOFlags {
    pub const NONE: Self = Self(0u32);
    pub const SUMMARY: Self = Self(1u32);
    pub const DETAIL: Self = Self(2u32);
    pub const IGNORE_INTERNAL: Self = Self(4u32);
    pub const ALL: Self = Self(7u32);
}

impl From<DebugRLOFlags> for DXGI_DEBUG_RLO_FLAGS {
    #[inline]
    fn from(v: DebugRLOFlags) -> Self {
        v.0
    }
}
