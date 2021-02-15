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

use crate::{dxgi, InputClassification};
use raw::windows::win32::direct3d12::D3D12_INPUT_ELEMENT_DESC;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Clone, Debug, Hash)]
pub struct InputElementDesc<'a> {
    pub semantic_name: &'a CStr,
    pub semantic_index: u32,
    pub format: dxgi::Format,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: InputClassification,
    pub instance_data_step_rate: u32,
}

impl<'a> Into<D3D12_INPUT_ELEMENT_DESC> for InputElementDesc<'a> {
    fn into(self) -> D3D12_INPUT_ELEMENT_DESC {
        D3D12_INPUT_ELEMENT_DESC {
            semantic_name: self.semantic_name.as_ptr() as *mut c_char as *mut _,
            semantic_index: self.semantic_index,
            format: self.format.into(),
            input_slot: self.input_slot,
            aligned_byte_offset: self.aligned_byte_offset,
            input_slot_class: self.input_slot_class.into(),
            instance_data_step_rate: self.instance_data_step_rate,
        }
    }
}
