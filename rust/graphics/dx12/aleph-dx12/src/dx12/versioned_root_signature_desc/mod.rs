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

use crate::raw::windows::win32::direct3d12::D3D_ROOT_SIGNATURE_VERSION;
use crate::{RootSignatureDesc, RootSignatureDesc1};

pub(crate) mod raw;

#[derive(Clone)]
pub enum VersionedRootSignatureDesc<'a> {
    Version10(RootSignatureDesc<'a>),
    Version11(RootSignatureDesc1<'a>),
}

impl<'a> Into<raw::D3D12_VERSIONED_ROOT_SIGNATURE_DESC> for VersionedRootSignatureDesc<'a> {
    fn into(self) -> raw::D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        let (version, desc) = match self {
            VersionedRootSignatureDesc::Version10(v) => (
                D3D_ROOT_SIGNATURE_VERSION::D3D_ROOT_SIGNATURE_VERSION_1_0,
                v.into(),
            ),
            VersionedRootSignatureDesc::Version11(v) => (
                D3D_ROOT_SIGNATURE_VERSION::D3D_ROOT_SIGNATURE_VERSION_1_1,
                v.into(),
            ),
        };
        raw::D3D12_VERSIONED_ROOT_SIGNATURE_DESC { version, desc }
    }
}
