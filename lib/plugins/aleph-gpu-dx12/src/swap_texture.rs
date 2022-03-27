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

use crate::internal::conv::texture_format_to_dxgi;
use crate::swap_chain::SwapChain;
use crate::ITextureExt;
use dx12::{dxgi, D3D12Object};
use interfaces::gpu::{INamedObject, ITexture, TextureDesc};
use interfaces::ref_ptr::{ref_ptr_object, RefPtr};
use std::sync::atomic::Ordering;

ref_ptr_object! {
    pub struct SwapTexture: ITexture, ITextureExt, ISwapTextureExt {
        pub(crate) resource: dx12::Resource,
        pub(crate) view: dx12::CPUDescriptorHandle,
        pub(crate) swap_chain: RefPtr<SwapChain>,
        pub(crate) desc: TextureDesc,
    }
}

impl Drop for SwapTexture {
    fn drop(&mut self) {
        self.swap_chain
            .images_in_flight
            .fetch_sub(1, Ordering::Release);
    }
}

impl ITexture for SwapTexture {
    fn desc(&self) -> &TextureDesc {
        &self.desc
    }
}

// SAFETY: The reference to the swap chain is never used, it is only present to extend the lifetime
//         of the swap chain
unsafe impl Send for SwapTexture {}

// SAFETY: See above
unsafe impl Sync for SwapTexture {}

impl ITextureExt for SwapTexture {
    fn get_raw_handle(&self) -> dx12::Resource {
        self.resource.clone()
    }

    fn get_raw_format(&self) -> dxgi::Format {
        texture_format_to_dxgi(self.desc.format)
    }
}

pub trait ISwapTextureExt: ITextureExt {
    fn get_raw_rtv(&self) -> dx12::CPUDescriptorHandle;
}

impl ISwapTextureExt for SwapTexture {
    fn get_raw_rtv(&self) -> dx12::CPUDescriptorHandle {
        self.view
    }
}

impl INamedObject for SwapTexture {
    fn set_name(&self, name: &str) {
        self.resource.set_name(name).unwrap()
    }
}
