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
use windows_raw::utils::DynamicLoadCell;
use windows_raw::{Abi, Interface};

static CREATE_FN: DynamicLoadCell<dxc_raw::DxcCreateInstanceProc> =
    DynamicLoadCell::new(&utf16_null!("dxcompiler.dll"), "DxcCreateInstance\0");

#[derive(Clone, Debug)]
pub enum DxcCompilerCreateError {
    FailedToLoadLibrary,
    CreateCallFailed(dx12::Error),
}

#[repr(transparent)]
pub struct DxcCompiler(pub(crate) dxc_raw::IDxcCompiler);

impl DxcCompiler {
    pub unsafe fn new() -> Result<Self, DxcCompilerCreateError> {
        let create_fn = CREATE_FN
            .get()
            .ok_or(DxcCompilerCreateError::FailedToLoadLibrary)?;
        let clsid = windows_raw::Guid::from(dxc_raw::CLSID_DxcCompiler);
        let riid = &dxc_raw::IDxcCompiler::IID;
        let mut out: Option<dxc_raw::IDxcCompiler> = None;
        create_fn(&clsid, riid, out.set_abi())
            .and_some(out)
            .map(|v| Self(v))
            .map_err(|v| DxcCompilerCreateError::CreateCallFailed(v))
    }
}
