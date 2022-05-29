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

use dx12::D3D12Object;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{INamedObject, IPipelineLayout};

pub struct PipelineLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) root_signature: dx12::RootSignature,
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
    fn get_raw_handle(&self) -> dx12::RootSignature;
}

impl IPipelineLayoutExt for PipelineLayout {
    fn get_raw_handle(&self) -> dx12::RootSignature {
        self.root_signature.clone()
    }
}

impl INamedObject for PipelineLayout {
    fn set_name(&self, name: &str) {
        self.root_signature.set_name(name).unwrap()
    }
}
