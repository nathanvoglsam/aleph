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

///
/// A struct to wrap resources that are created and destroyed once during the Imgui renderer's
/// lifecycle
///
pub struct ConstantObjects {
    pub rtv_heap: dx12::DescriptorHeap,
    pub sampler_heap: dx12::DescriptorHeap,
    pub root_signature: dx12::RootSignature,
}

impl ConstantObjects {
    pub fn init(device: &dx12::Device) -> Self {
        let desc = dx12::DescriptorHeapDesc::builder()
            .heap_type(dx12::DescriptorHeapType::RenderTargetView)
            .num_descriptors(3)
            .build();
        let rtv_heap = device.create_descriptor_heap(&desc).unwrap();

        let desc = dx12::DescriptorHeapDesc::builder()
            .heap_type(dx12::DescriptorHeapType::Sampler)
            .num_descriptors(1)
            .build();
        let sampler_heap = device.create_descriptor_heap(&desc).unwrap();

        let root_signature = Self::create_root_signature(device);

        unsafe {
            let dest = heap.get_cpu_descriptor_handle_for_heap_start();
            let desc = dx12::SamplerDesc::builder()
                .address_u(dx12::TextureAddressMode::Clamp)
                .address_v(dx12::TextureAddressMode::Clamp)
                .address_w(dx12::TextureAddressMode::Clamp)
                .build();
            device.create_sampler(&desc, dest);
        }

        Self {
            rtv_heap,
            sampler_heap,
            root_signature,
        }
    }

    pub fn create_root_signature(device: &dx12::Device) -> dx12::RootSignature {
        // TODO: The rest
        let desc_builder = dx12::RootSignatureDesc::builder();
        let desc = desc_builder.build();
        let desc = dx12::VersionedRootSignatureDesc::Desc(desc);
        let root_signature_blob = unsafe { dx12::RootSignatureBlob::new(&desc).unwrap() };
        let root_signature = device.create_root_signature(&root_signature_blob).unwrap();
        root_signature
    }
}
