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
use crate::raw::windows::win32::direct3d12::{
    D3D12SerializeVersionedRootSignature, ID3D12Device4, ID3D12RootSignature,
};
use crate::raw::windows::{Abi, Interface};
use crate::{D3D12DeviceChild, D3D12Object, Device, VersionedRootSignatureDesc};

#[derive(Clone)]
#[repr(transparent)]
pub struct RootSignatureBlob(pub(crate) ID3DBlob);

impl RootSignatureBlob {
    pub unsafe fn new(
        desc: &VersionedRootSignatureDesc,
    ) -> Result<Self, (raw::windows::Error, String)> {
        // Wrap the types to shorter names to keep the code readable
        type MyDesc =
            crate::versioned_root_signature_desc::raw::D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
        type FFIDesc = crate::raw::windows::win32::direct3d12::D3D12_VERSIONED_ROOT_SIGNATURE_DESC;

        let desc = desc.clone();
        let desc: MyDesc = desc.into();
        let desc_ptr = &desc as *const MyDesc as *const FFIDesc;

        let mut blob: Option<ID3DBlob> = None;
        let mut err: Option<ID3DBlob> = None;
        D3D12SerializeVersionedRootSignature(desc_ptr, &mut blob, &mut err)
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

#[derive(Clone)]
#[repr(transparent)]
pub struct RootSignature(pub(crate) ID3D12RootSignature);

impl RootSignature {
    //pub fn builder() -> RootSignatureBuilder {
    //    RootSignatureBuilder::new()
    //}
}

impl D3D12Object for RootSignature {
    unsafe fn set_name_raw(&self, name: &[u16]) -> raw::windows::Result<()> {
        self.0.SetName(name.as_ptr()).ok()
    }
}

impl D3D12DeviceChild for RootSignature {
    unsafe fn get_device(&self) -> raw::windows::Result<Device> {
        let mut device: Option<ID3D12Device4> = None;
        self.0
            .GetDevice(&ID3D12Device4::IID, device.set_abi())
            .and_some(device)
            .map(|v| Device(v))
    }
}
