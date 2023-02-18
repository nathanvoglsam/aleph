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
use crate::internal::{calc_subresource_index, plane_layer_for_aspect, try_clone_value_into_slot};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use parking_lot::RwLock;
use std::any::TypeId;
use std::collections::HashMap;
use windows::utils::CPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

type CacheViewCPU = HashMap<(Format, TextureSubResourceSet), CPUDescriptorHandle>;

pub struct Texture {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) desc: TextureDesc<'static>,
    pub(crate) name: Option<String>,
    pub(crate) dxgi_format: DXGI_FORMAT,
    pub(crate) rtv_cache: RwLock<CacheViewCPU>,
    pub(crate) dsv_cache: RwLock<CacheViewCPU>,
    pub(crate) srv_cache: RwLock<CacheViewCPU>,
    pub(crate) uav_cache: RwLock<CacheViewCPU>,
}

declare_interfaces!(Texture, [ITexture]);

impl Texture {
    pub const fn plane_slice_for(&self, aspect: TextureCopyAspect) -> Option<u32> {
        plane_layer_for_aspect(self.desc.format, aspect)
    }

    pub const fn subresource_index_for(
        &self,
        mip_level: u32,
        array_layer: u32,
        aspect: TextureCopyAspect,
    ) -> Option<u32> {
        if let Some(plane_slice) = self.plane_slice_for(aspect) {
            Some(calc_subresource_index(
                mip_level,
                array_layer,
                plane_slice,
                self.desc.mip_levels,
                self.desc.array_size,
            ))
        } else {
            None
        }
    }

    pub unsafe fn get_or_create_rtv_for_usage(
        &self,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
    ) -> Option<CPUDescriptorHandle> {
        let init = |view: CPUDescriptorHandle, format, sub_resources| {
            let desc = self.make_rtv_desc_for_format_and_sub_resources(format, sub_resources);
            self.device
                .device
                .CreateRenderTargetView(&self.resource, &desc, view.into());
        };
        self.get_or_create_view_for_usage(
            &self.rtv_cache,
            &self.device.descriptor_heaps.cpu_rtv_heap(),
            format,
            sub_resources,
            init,
        )
    }

    pub unsafe fn get_or_create_dsv_for_usage(
        &self,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
    ) -> Option<CPUDescriptorHandle> {
        let init = |view: CPUDescriptorHandle, format, sub_resources| {
            let desc = self.make_dsv_desc_for_format_and_sub_resources(format, sub_resources);
            self.device
                .device
                .CreateDepthStencilView(&self.resource, &desc, view.into());
        };
        self.get_or_create_view_for_usage(
            &self.dsv_cache,
            &self.device.descriptor_heaps.cpu_dsv_heap(),
            format,
            sub_resources,
            init,
        )
    }

    pub fn get_or_create_view_for_usage<'a>(
        &self,
        cache: &RwLock<CacheViewCPU>,
        allocator: &DescriptorAllocatorCPU,
        format: Option<Format>,
        sub_resources: &'a TextureSubResourceSet,
        init: impl FnOnce(CPUDescriptorHandle, Format, &'a TextureSubResourceSet),
    ) -> Option<CPUDescriptorHandle> {
        // First see if we already have a compatible view
        //
        // We intentionally take a read lock optimistically as very likely the view is already in
        // the cache. If it isn't then we hit the slow path of initializing the view so we need to
        // take a write lock.
        let views = cache.read();

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

    fn desc(&self) -> TextureDesc {
        let mut desc = self.desc.clone();
        desc.name = self.name.as_ref().map(String::as_str);
        desc
    }
}

impl IGetPlatformInterface for Texture {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        if try_clone_value_into_slot::<ID3D12Resource>(&self.resource, out, target).is_some() {
            return Some(());
        }
        if try_clone_value_into_slot::<DXGI_FORMAT>(&self.dxgi_format, out, target).is_some() {
            return Some(());
        }
        None
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        // Free all RTVs associated with this texture
        let rtvs = self.rtv_cache.read();
        for (_, rtv) in rtvs.iter() {
            self.device.descriptor_heaps.cpu_rtv_heap().free(*rtv);
        }

        // Free all DSVs associated with this texture
        let dsvs = self.dsv_cache.read();
        for (_, dsv) in dsvs.iter() {
            self.device.descriptor_heaps.cpu_dsv_heap().free(*dsv);
        }
    }
}
