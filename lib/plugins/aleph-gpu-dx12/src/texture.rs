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
use dx12::{dxgi, D3D12Object};
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{
    Format, INamedObject, ITexture, TextureCopyAspect, TextureDesc, TextureDimension,
    TextureSubResourceSet,
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

pub enum TextureInner {
    Plain(PlainTexture),
    Swap(SwapTexture),
}

impl TextureInner {
    #[inline]
    pub fn get_raw_handle(&self) -> dx12::Resource {
        match self {
            TextureInner::Plain(v) => v.resource.clone(),
            TextureInner::Swap(v) => v.resource.clone(),
        }
    }

    #[inline]
    pub fn get_raw_format(&self) -> dxgi::Format {
        match self {
            TextureInner::Plain(v) => v.dxgi_format,
            TextureInner::Swap(v) => texture_format_to_dxgi(v.desc.format),
        }
    }

    #[inline]
    pub fn set_name(&self, name: &str) {
        match self {
            TextureInner::Plain(v) => v.resource.set_name(name).unwrap(),
            TextureInner::Swap(v) => v.resource.set_name(name).unwrap(),
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
    pub fn resource(&self) -> &dx12::Resource {
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
    fn get_raw_handle(&self) -> dx12::Resource;

    fn get_raw_format(&self) -> dxgi::Format;

    fn get_raw_rtv(&self) -> Option<dx12::CPUDescriptorHandle>;
}

impl ITextureExt for Texture {
    fn get_raw_handle(&self) -> dx12::Resource {
        self.inner.get_raw_handle()
    }

    fn get_raw_format(&self) -> dxgi::Format {
        self.inner.get_raw_format()
    }

    fn get_raw_rtv(&self) -> Option<dx12::CPUDescriptorHandle> {
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

type CacheViewCPU = HashMap<(Format, TextureSubResourceSet), dx12::CPUDescriptorHandle>;

pub struct PlainTexture {
    pub(crate) device: AnyArc<Device>,
    pub(crate) resource: dx12::Resource,
    pub(crate) desc: TextureDesc,
    pub(crate) dxgi_format: dxgi::Format,
    pub(crate) rtv_cache: RwLock<CacheViewCPU>,
    pub(crate) dsv_cache: RwLock<CacheViewCPU>,
}

impl PlainTexture {
    #[inline]
    pub fn get_or_create_rtv_for_usage(
        &self,
        format: Option<Format>,
        sub_resources: &TextureSubResourceSet,
    ) -> Option<dx12::CPUDescriptorHandle> {
        let init = |view, format, sub_resources| unsafe {
            let desc = self.make_rtv_desc_for_format_and_sub_resources(format, &sub_resources);
            self.device
                .device
                .create_render_target_view(&self.resource, &desc, view);
        };
        self.get_or_create_view_for_usage(
            &self.rtv_cache,
            &self.device.rtv_heap,
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
    ) -> Option<dx12::CPUDescriptorHandle> {
        let init = |view, format, sub_resources| unsafe {
            let desc = self.make_dsv_desc_for_format_and_sub_resources(format, &sub_resources);
            self.device
                .device
                .create_depth_stencil_view(&self.resource, &desc, view);
        };
        self.get_or_create_view_for_usage(
            &self.dsv_cache,
            &self.device.dsv_heap,
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
        init: impl FnOnce(dx12::CPUDescriptorHandle, Format, TextureSubResourceSet),
    ) -> Option<dx12::CPUDescriptorHandle> {
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

    #[inline]
    pub fn make_rtv_desc_for_format_and_sub_resources(
        &self,
        format: Format,
        sub_resources: &TextureSubResourceSet,
    ) -> dx12::RenderTargetViewDesc {
        let is_array = self.desc.array_size > 1;
        let is_ms = self.desc.sample_count > 1;

        match (self.desc.dimension, is_array, is_ms) {
            (TextureDimension::Texture1D, true, _) => dx12::RenderTargetViewDesc::Texture1DArray {
                format: texture_format_to_dxgi(format),
                texture_1d_array: dx12::Tex1DArrayRtv {
                    mip_slice: sub_resources.base_mip_level,
                    first_array_slice: sub_resources.base_array_slice,
                    array_size: sub_resources.num_array_slices,
                },
            },
            (TextureDimension::Texture1D, false, _) => dx12::RenderTargetViewDesc::Texture1D {
                format: texture_format_to_dxgi(format),
                texture_1d: dx12::Tex1DRtv {
                    mip_slice: sub_resources.base_mip_level,
                },
            },
            (TextureDimension::Texture2D, true, false) => {
                dx12::RenderTargetViewDesc::Texture2DArray {
                    format: texture_format_to_dxgi(format),
                    texture_2d_array: dx12::Tex2DArrayRtv {
                        mip_slice: sub_resources.base_mip_level,
                        first_array_slice: sub_resources.base_array_slice,
                        array_size: sub_resources.num_array_slices,
                        plane_slice: 0,
                    },
                }
            }
            (TextureDimension::Texture2D, false, false) => dx12::RenderTargetViewDesc::Texture2D {
                format: texture_format_to_dxgi(format),
                texture_2d: dx12::Tex2DRtv {
                    mip_slice: sub_resources.base_mip_level,
                    plane_slice: 0,
                },
            },
            (TextureDimension::Texture2D, true, true) => {
                dx12::RenderTargetViewDesc::Texture2DMSArray {
                    format: texture_format_to_dxgi(format),
                    texture_2dms_array: dx12::Tex2DMSArrayRtv {
                        first_array_slice: sub_resources.base_array_slice,
                        array_size: sub_resources.num_array_slices,
                    },
                }
            }
            (TextureDimension::Texture2D, false, true) => dx12::RenderTargetViewDesc::Texture2DMS {
                format: texture_format_to_dxgi(format),
                texture_2dms: dx12::Tex2DMSRtv { _unused: 0 },
            },
            (TextureDimension::Texture3D, _, _) => dx12::RenderTargetViewDesc::Texture3D {
                format: texture_format_to_dxgi(format),
                texture_3d: dx12::Tex3DRtv {
                    mip_slice: sub_resources.base_mip_level,
                    first_w_slice: sub_resources.base_array_slice,
                    w_size: sub_resources.num_array_slices,
                },
            },
        }
    }

    #[inline]
    pub fn make_dsv_desc_for_format_and_sub_resources(
        &self,
        format: Format,
        sub_resources: &TextureSubResourceSet,
    ) -> dx12::DepthStencilViewDesc {
        let is_array = self.desc.array_size > 1;
        let is_ms = self.desc.sample_count > 1;

        match (self.desc.dimension, is_array, is_ms) {
            (TextureDimension::Texture1D, true, _) => dx12::DepthStencilViewDesc::Texture1DArray {
                format: texture_format_to_dxgi(format),
                texture_1d_array: dx12::Tex1DArrayDsv {
                    mip_slice: sub_resources.base_mip_level,
                    first_array_slice: sub_resources.base_array_slice,
                    array_size: sub_resources.num_array_slices,
                },
            },
            (TextureDimension::Texture1D, false, _) => dx12::DepthStencilViewDesc::Texture1D {
                format: texture_format_to_dxgi(format),
                texture_1d: dx12::Tex1DDsv {
                    mip_slice: sub_resources.base_mip_level,
                },
            },
            (TextureDimension::Texture2D, true, false) => {
                dx12::DepthStencilViewDesc::Texture2DArray {
                    format: texture_format_to_dxgi(format),
                    texture_2d_array: dx12::Tex2DArrayDsv {
                        mip_slice: sub_resources.base_mip_level,
                        first_array_slice: sub_resources.base_array_slice,
                        array_size: sub_resources.num_array_slices,
                    },
                }
            }
            (TextureDimension::Texture2D, false, false) => dx12::DepthStencilViewDesc::Texture2D {
                format: texture_format_to_dxgi(format),
                texture_2d: dx12::Tex2DDsv {
                    mip_slice: sub_resources.base_mip_level,
                },
            },
            (TextureDimension::Texture2D, true, true) => {
                dx12::DepthStencilViewDesc::Texture2DMSArray {
                    format: texture_format_to_dxgi(format),
                    texture_2dms_array: dx12::Tex2DMSArrayDsv {
                        first_array_slice: sub_resources.base_array_slice,
                        array_size: sub_resources.num_array_slices,
                    },
                }
            }
            (TextureDimension::Texture2D, false, true) => dx12::DepthStencilViewDesc::Texture2DMS {
                format: texture_format_to_dxgi(format),
                texture_2dms: dx12::Tex2DMSDsv { _unused: 0 },
            },
            (TextureDimension::Texture3D, _, _) => unreachable!(),
        }
    }
}

impl Drop for PlainTexture {
    #[inline]
    fn drop(&mut self) {
        // Free all RTVs associated with this texture
        let rtvs = self.rtv_cache.read();
        for (_, rtv) in rtvs.iter() {
            self.device.rtv_heap.free(*rtv);
        }
    }
}

pub struct SwapTexture {
    pub(crate) swap_chain: AnyArc<SwapChain>,
    pub(crate) resource: dx12::Resource,
    pub(crate) view: dx12::CPUDescriptorHandle,
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
