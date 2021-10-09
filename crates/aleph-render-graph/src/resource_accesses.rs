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

use dx12::dxgi;

pub struct RenderTargetCreateDesc {
    /// The width of the render target in pixels
    pub width: u32,

    /// The height of the render target in pixels
    pub height: u32,

    /// The pixel format of the render target
    pub format: dxgi::Format,

    /// ?
    pub initial_state: (),
}

impl Into<ResourceCreateDesc> for RenderTargetCreateDesc {
    fn into(self) -> ResourceCreateDesc {
        ResourceCreateDesc::RenderTarget(self)
    }
}

pub struct BufferCreateDesc {
    /// The size, in bytes, of the buffer
    pub size: u32,
}

impl Into<ResourceCreateDesc> for BufferCreateDesc {
    fn into(self) -> ResourceCreateDesc {
        ResourceCreateDesc::Buffer(self)
    }
}

pub enum ResourceCreateDesc {
    RenderTarget(RenderTargetCreateDesc),
    Buffer(BufferCreateDesc),
}

pub struct ResourceAccessDesc {
    /// The state that this resource will be used in. This is specified so the render graph can
    /// generate barriers that handle transitioning the resource into the required state.
    pub state: dx12::ResourceStates,
}
