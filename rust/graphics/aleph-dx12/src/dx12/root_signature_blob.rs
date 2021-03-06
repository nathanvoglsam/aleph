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

use crate::dx12::versioned_root_signature_desc::D3D12_VERSIONED_ROOT_SIGNATURE_DESC as MyDesc;
use crate::windows_raw::win32::direct3d11::ID3DBlob;
use crate::windows_raw::win32::direct3d12::D3D12_VERSIONED_ROOT_SIGNATURE_DESC as Desc;
use crate::windows_raw::win32::direct3d12::PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE;
use crate::{Abi, VersionedRootSignatureDesc};
use utf16_lit::utf16_null;
use windows_raw::utils::DynamicLoadCell;

pub(crate) static CREATE_FN: DynamicLoadCell<PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE> =
    DynamicLoadCell::new(
        &utf16_null!("d3d12.dll"),
        "D3D12SerializeVersionedRootSignature\0",
    );

#[derive(Clone)]
#[repr(transparent)]
pub struct RootSignatureBlob(pub(crate) ID3DBlob);

impl RootSignatureBlob {
    pub unsafe fn new(desc: &VersionedRootSignatureDesc) -> crate::Result<Self> {
        let desc: MyDesc = desc.clone().into();
        let desc_ptr = &desc as *const MyDesc as *const Desc;

        let create_fn = *CREATE_FN.get().expect("Failed to load d3d12.dll");
        let mut blob: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None; // TODO: Find a sane way to expose this
        create_fn(desc_ptr, blob.set_abi(), err.set_abi())
            .and_some(blob)
            .map(|v| RootSignatureBlob(v))
    }
}
