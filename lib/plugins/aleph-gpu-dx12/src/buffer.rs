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
use interfaces::gpu::{BufferDesc, IBuffer, INamedObject};

pub struct Buffer {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) resource: dx12::Resource,
    pub(crate) desc: BufferDesc,
}

declare_interfaces!(Buffer, [IBuffer, IBufferExt]);

impl IBuffer for Buffer {
    fn upgrade(&self) -> AnyArc<dyn IBuffer> {
        self.this.upgrade().unwrap().query_interface().unwrap()
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &BufferDesc {
        &self.desc
    }
}

pub trait IBufferExt: IBuffer {
    fn get_raw_handle(&self) -> dx12::Resource;
}

impl IBufferExt for Buffer {
    fn get_raw_handle(&self) -> dx12::Resource {
        self.resource.clone()
    }
}

impl INamedObject for Buffer {
    fn set_name(&self, name: &str) {
        self.resource.set_name(name).unwrap()
    }
}
