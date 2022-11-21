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
use crate::internal::conv::texture_format_to_dxgi;
use crate::internal::descriptor_allocator_cpu::DescriptorAllocatorCPU;
use crate::internal::{calc_subresource_index, plane_layer_for_aspect};
use crate::swap_chain::SwapChain;
use crate::CPUDescriptorHandle;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{
    Format, INamedObject, ITexture, TextureCopyAspect, TextureDesc, TextureDimension,
    TextureSubResourceSet,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

pub enum TextureInner {
    Plain(PlainTexture),
    Swap(SwapTexture),
}

impl TextureInner {
    #[inline]
    pub fn get_raw_handle(&self) -> ID3D12Resource {
        match self {
            TextureInner::Plain(v) => v.resource.clone(),
            TextureInner::Swap(v) => v.resource.clone(),
        }
    }

    #[inline]
    pub fn get_raw_format(&self) -> DXGI_FORMAT {
        match self {
            TextureInner::Plain(v) => v.dxgi_format,
            TextureInner::Swap(v) => texture_format_to_dxgi(v.desc.format),
        }
    }

    #[inline]
    pub fn set_name(&self, name: &str) {
        unsafe {
            let utf16: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let name = PCWSTR::from_raw(utf16.as_ptr());

            match self {
                TextureInner::Plain(v) => v.resource.SetName(name).unwrap(),
                TextureInner::Swap(v) => v.resource.SetName(name).unwrap(),
            }
        }
    }

    pub const fn desc(&self) -> &TextureDesc {
        match self {
            TextureInner::Plain(v) => &v.desc,
            TextureInner::Swap(v) => &v.desc,
        }
    }
}

pub struct Texture {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) inner: TextureInner,
}

declare_interfaces!(Texture, [ITexture, ITextureExt]);

impl Texture {
    #[inline]
    pub fn resource(&self) -> &ID3D12Resource {
        match &self.inner {
            TextureInner::Plain(v) => &v.resource,
            TextureInner::Swap(v) => &v.resource,
        }
    }

    pub const fn plane_slice_for(&self, aspect: TextureCopyAspect) -> Option<u32> {
        let desc = self.inner.desc();
        plane_layer_for_aspect(desc.format, aspect)
    }

    pub const fn subresource_index_for(
        &self,
        mip_level: u32,
        array_layer: u32,
        aspect: TextureCopyAspect,
    ) -> Option<u32> {
        if let Some(plane_slice) = self.plane_slice_for(aspect) {
            let desc = self.inner.desc();
            Some(calc_subresource_index(
                mip_level,
                array_layer,
                plane_slice,
                desc.mip_levels,
                desc.array_size,
            ))
        } else {
            None
        }
    }
}

impl ITexture for Texture {
    fn upgrade(&self) -> AnyArc<dyn ITexture> {
        AnyArc::map::<dyn ITexture, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn desc(&self) -> &TextureDesc {
        self.inner.desc()
    }
}

pub trait ITextureExt: ITexture {
    fn get_raw_handle(&self) -> ID3D12Resource;

    fn get_raw_format(&self) -> DXGI_FORMAT;

    fn get_raw_rtv(&self) -> Option<CPUDescriptorHandle>;
}

impl ITextureExt for Texture {
    fn get_raw_handle(&self) -> ID3D12Resource {
        self.inner.get_raw_handle()
    }

    fn get_raw_format(&self) -> DXGI_FORMAT {
        self.inner.get_raw_format()
    }

    fn get_raw_rtv(&self) -> Option<CPUDescriptorHandle> {
        if let TextureInner::Swap(v) = &self.inner {
            Some(v.view)
        } else {
            None
        }
    }
}

impl INamedObject for Texture {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}

type CacheViewCPU = HashMap<(Format, TextureSubResourceSet), CPUDescriptorHandle>;

pub struct PlainTexture {
    pub(crate) device: AnyArc<Device>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) desc: TextureDesc,
    pub(crate) dxgi_format: DXGI_FORMAT,
    pub(crate) rtv_cache: RwLock<CacheViewCPU>,
    pub(crate) dsv_cache: RwLock<CacheViewCPU>,
}

impl PlainTexture {
    #[inline]
    pub fn get_or_create_rtv_for_usage(
        &self,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
    ) -> Option<CPUDescriptorHandle> {
        let init = |view: CPUDescriptorHandle, format, sub_resources| unsafe {
            let desc = self.make_rtv_desc_for_format_and_sub_resources(format, &sub_resources);
            self.device
                .device
                .CreateRenderTargetView(&self.resource, &desc, view.into());
        };
        self.get_or_create_view_for_usage(
            &self.rtv_cache,
            &self.device.descriptor_heaps.cpu_rtv_heap(),
            format,
            sub_resources,
            false,
            init,
        )
    }

    #[inline]
    pub fn get_or_create_dsv_for_usage(
        &self,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
    ) -> Option<CPUDescriptorHandle> {
        let init = |view: CPUDescriptorHandle, format, sub_resources| unsafe {
            let desc = self.make_dsv_desc_for_format_and_sub_resources(format, &sub_resources);
            self.device
                .device
                .CreateDepthStencilView(&self.resource, &desc, view.into());
        };
        self.get_or_create_view_for_usage(
            &self.dsv_cache,
            &self.device.descriptor_heaps.cpu_dsv_heap(),
            format,
            sub_resources,
            false,
            init,
        )
    }

    #[inline]
    pub fn get_or_create_view_for_usage(
        &self,
        cache: &RwLock<CacheViewCPU>,
        allocator: &DescriptorAllocatorCPU,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
        allow_multiple_mips: bool,
        init: impl FnOnce(CPUDescriptorHandle, Format, TextureSubResourceSet),
    ) -> Option<CPUDescriptorHandle> {
        // First see if we already have a compatible view
        //
        // We intentionally take a read lock optimistically as very likely the view is already in
        // the cache. If it isn't then we hit the slow path of initializing the view so we need to
        // take a write lock.
        let views = cache.read();

        // Whether more than a single mip level is valid for this view
        let sub_resources = if !allow_multiple_mips && sub_resources.num_mip_levels > 1 {
            let mut sub_resources = sub_resources.clone();
            sub_resources.num_mip_levels = 1;
            sub_resources
        } else {
            sub_resources.clone()
        };

        // Zero mip levels would imply no image data so is also invalid
        if sub_resources.num_mip_levels < 1 {
            return None;
        }

        let format = format.unwrap_or(self.desc.format);
        let key = (format, sub_resources.clone());
        if let Some(view) = views.get(&key) {
            Some(*view)
        } else {
            // Otherwise we need to create a new one. We drop the old read lock and take a new write
            // lock so we can get exclusive access to the map
            drop(views);
            let mut views = cache.write();

            // Allocate a descriptor and write the view into it
            let view = allocator.allocate().unwrap();

            // Call the initializer to write the descriptor
            (init)(view, format, sub_resources);

            // Add the view to our cache
            views.insert(key, view);

            Some(view)
        }
    }

    pub fn make_rtv_desc_for_format_and_sub_resources(
        &self,
        format: Format,
        sub_resources: &TextureSubResourceSet,
    ) -> D3D12_RENDER_TARGET_VIEW_DESC {
        let is_array = self.desc.array_size > 1;
        let is_ms = self.desc.sample_count > 1;

        let (view_dimension, anonymous) = match (self.desc.dimension, is_array, is_ms) {
            (TextureDimension::Texture1D, true, _) => (
                D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_RTV {
                        MipSlice: sub_resources.base_mip_level,
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                    },
                },
            ),
            (TextureDimension::Texture1D, false, _) => (
                D3D12_RTV_DIMENSION_TEXTURE1D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_RTV {
                        MipSlice: sub_resources.base_mip_level,
                    },
                },
            ),
            (TextureDimension::Texture2D, true, false) => (
                D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                        MipSlice: sub_resources.base_mip_level,
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                        PlaneSlice: 0,
                    },
                },
            ),
            (TextureDimension::Texture2D, false, false) => (
                D3D12_RTV_DIMENSION_TEXTURE2D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_RTV {
                        MipSlice: sub_resources.base_mip_level,
                        PlaneSlice: 0,
                    },
                },
            ),
            (TextureDimension::Texture2D, true, true) => (
                D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV {
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                    },
                },
            ),
            (TextureDimension::Texture2D, false, true) => (
                D3D12_RTV_DIMENSION_TEXTURE2DMS,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DMS: D3D12_TEX2DMS_RTV {
                        UnusedField_NothingToDefine: 0,
                    },
                },
            ),
            (TextureDimension::Texture3D, _, _) => (
                D3D12_RTV_DIMENSION_TEXTURE3D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture3D: D3D12_TEX3D_RTV {
                        MipSlice: sub_resources.base_mip_level,
                        FirstWSlice: sub_resources.base_array_slice,
                        WSize: sub_resources.num_array_slices,
                    },
                },
            ),
        };

        D3D12_RENDER_TARGET_VIEW_DESC {
            Format: texture_format_to_dxgi(format).into(),
            ViewDimension: view_dimension,
            Anonymous: anonymous,
        }
    }

    pub fn make_dsv_desc_for_format_and_sub_resources(
        &self,
        format: Format,
        sub_resources: &TextureSubResourceSet,
    ) -> D3D12_DEPTH_STENCIL_VIEW_DESC {
        let is_array = self.desc.array_size > 1;
        let is_ms = self.desc.sample_count > 1;

        let (view_dimension, anonymous) = match (self.desc.dimension, is_array, is_ms) {
            (TextureDimension::Texture1D, true, _) => (
                D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_DSV {
                        MipSlice: sub_resources.base_mip_level,
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                    },
                },
            ),
            (TextureDimension::Texture1D, false, _) => (
                D3D12_DSV_DIMENSION_TEXTURE1D,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_DSV {
                        MipSlice: sub_resources.base_mip_level,
                    },
                },
            ),
            (TextureDimension::Texture2D, true, false) => (
                D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                        MipSlice: sub_resources.base_mip_level,
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                    },
                },
            ),
            (TextureDimension::Texture2D, false, false) => (
                D3D12_DSV_DIMENSION_TEXTURE2D,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_DSV {
                        MipSlice: sub_resources.base_mip_level,
                    },
                },
            ),
            (TextureDimension::Texture2D, true, true) => (
                D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV {
                        FirstArraySlice: sub_resources.base_array_slice,
                        ArraySize: sub_resources.num_array_slices,
                    },
                },
            ),
            (TextureDimension::Texture2D, false, true) => (
                D3D12_DSV_DIMENSION_TEXTURE2DMS,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DMS: D3D12_TEX2DMS_DSV {
                        UnusedField_NothingToDefine: 0,
                    },
                },
            ),
            (TextureDimension::Texture3D, _, _) => unreachable!(),
        };

        D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: texture_format_to_dxgi(format).into(),
            ViewDimension: view_dimension,
            Flags: Default::default(),
            Anonymous: anonymous,
        }
    }
}

impl Drop for PlainTexture {
    #[inline]
    fn drop(&mut self) {
        // Free all RTVs associated with this texture
        let rtvs = self.rtv_cache.read();
        for (_, rtv) in rtvs.iter() {
            self.device.descriptor_heaps.cpu_rtv_heap().free(*rtv);
        }
    }
}

pub struct SwapTexture {
    pub(crate) swap_chain: AnyArc<SwapChain>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) view: CPUDescriptorHandle,
    pub(crate) desc: TextureDesc,
}

impl Drop for SwapTexture {
    fn drop(&mut self) {
        // TODO: This potentially violates the 'Send' / 'Sync' bits, need to figure this out
        self.swap_chain
            .images_in_flight
            .fetch_sub(1, Ordering::Release);
    }
}

// SAFETY: The reference to the swap chain is never used, it is only present to extend the lifetime
//         of the swap chain
unsafe impl Send for SwapTexture {}

// SAFETY: See above
unsafe impl Sync for SwapTexture {}
