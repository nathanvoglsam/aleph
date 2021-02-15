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

use crate::raw::windows::win32::direct3d11::ID3DBlob;
use crate::raw::windows::win32::direct3d12::PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE;
use crate::utils::DynamicLoadCell;
use utf16_lit::utf16_null;

pub(crate) static CREATE_FN: DynamicLoadCell<PFN_D3D12_SERIALIZE_VERSIONED_ROOT_SIGNATURE> =
    DynamicLoadCell::new(
        &utf16_null!("d3d12.dll"),
        "D3D12SerializeVersionedRootSignature\0",
    );

#[derive(Clone)]
#[repr(transparent)]
pub struct RootSignatureBlob(pub(crate) ID3DBlob);

impl RootSignatureBlob {
    pub unsafe fn new(
        desc: &VersionedRootSignatureDesc,
    ) -> Result<Self, (raw::windows::Error, String)> {
        // Wrap the types to shorter names to keep the code readable
        type MyDesc =
            crate::dx12::versioned_root_signature_desc::D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
        type MyV10 = crate::dx12::root_signature_desc::D3D12_ROOT_SIGNATURE_DESC;
        type MyV11 = crate::dx12::root_signature_desc::D3D12_ROOT_SIGNATURE_DESC1;
        type MyPV10 = crate::dx12::root_parameter::D3D12_ROOT_PARAMETER;
        type MyPV11 = crate::dx12::root_parameter::D3D12_ROOT_PARAMETER1;
        type MyVersions = crate::dx12::versioned_root_signature_desc::D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS;
        type FFIDesc = crate::raw::windows::win32::direct3d12::D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
        type FFISS = crate::raw::windows::win32::direct3d12::D3D12_STATIC_SAMPLER_DESC;

        enum Buffers {
            V10(Vec<MyPV10>, Vec<FFISS>),
            V11(Vec<MyPV11>, Vec<FFISS>),
        }

        let (buffers, desc) = desc.clone().into_ffi();
        let desc_ptr = &desc as *const MyDesc as *const FFIDesc;

        let create_fn = *CREATE_FN.get().expect("Failed to load d3d12.dll");
        let mut blob: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None;
        create_fn(desc_ptr, &mut blob, &mut err)
            .and_some(blob)
            .map(|v| RootSignatureBlob(v))
            .map_err(|v| {
                if let Some(err) = err {
                    // TODO: See if this returns a string and wrap it into the error type
                    let _ptr = err.GetBufferPointer() as *const u8;
                    let _sze = err.GetBufferSize();
                    (v, String::new())
                } else {
                    (v, String::new())
                }
            })
    }
}
