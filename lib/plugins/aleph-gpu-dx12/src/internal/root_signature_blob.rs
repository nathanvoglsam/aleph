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

use std::ops::Deref;
use utf16_lit::utf16_null;
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

pub(crate) static CREATE_FN: DynamicLoadCell<PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE> =
    DynamicLoadCell::new(
        &utf16_null!("d3d12.dll"),
        "D3D12SerializeVersionedRootSignature\0",
    );

#[repr(transparent)]
pub struct RootSignatureBlob(pub(crate) ID3DBlob);

impl RootSignatureBlob {
    #[inline]
    pub unsafe fn new(desc: &D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> windows::core::Result<Self> {
        let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();
        let mut blob: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None; // TODO: Find a sane way to expose this
        create_fn(desc, &mut blob, &mut err)
            .and_some(blob)
            .map(RootSignatureBlob)
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.GetBufferPointer() as *const u8,
                self.0.GetBufferSize(),
            )
        }
    }
}

impl Deref for RootSignatureBlob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
