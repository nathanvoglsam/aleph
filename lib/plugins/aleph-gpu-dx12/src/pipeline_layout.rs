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

use crate::device::Device;
use crate::internal::set_name::set_name;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{INamedObject, IPipelineLayout};
use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct3D12::*;

pub struct PipelineLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) root_signature: ID3D12RootSignature,
    pub(crate) push_constant_blocks: Vec<PushConstantBlockInfo>,
}

declare_interfaces!(PipelineLayout, [IPipelineLayout, IPipelineLayoutExt]);

impl IPipelineLayout for PipelineLayout {
    fn upgrade(&self) -> AnyArc<dyn IPipelineLayout> {
        AnyArc::map::<dyn IPipelineLayout, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }
}

pub trait IPipelineLayoutExt: IPipelineLayout {
    fn get_raw_handle(&self) -> ID3D12RootSignature;
}

impl IPipelineLayoutExt for PipelineLayout {
    fn get_raw_handle(&self) -> ID3D12RootSignature {
        self.root_signature.clone()
    }
}

impl INamedObject for PipelineLayout {
    fn set_name(&self, name: &str) {
        unsafe {
            set_name(&self.root_signature, name).unwrap();
        }
    }
}

/// Internal struct for caching information necessary for implementing command recording
pub struct PushConstantBlockInfo {
    pub size: u32,
    pub root_parameter_index: u32,
}
