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

use crate::dx12::root_signature_desc::raw::{
    D3D12_ROOT_SIGNATURE_DESC, D3D12_ROOT_SIGNATURE_DESC1,
};
use crate::raw::windows::win32::direct3d12::D3D_ROOT_SIGNATURE_VERSION;
use std::mem::ManuallyDrop;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    pub version: D3D_ROOT_SIGNATURE_VERSION,
    pub desc: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub union D3D12_VERSIONED_ROOT_SIGNATURE_DESC_VERSIONS {
    pub version_1_0: ManuallyDrop<D3D12_ROOT_SIGNATURE_DESC>,
    pub version_1_1: ManuallyDrop<D3D12_ROOT_SIGNATURE_DESC1>,
}
