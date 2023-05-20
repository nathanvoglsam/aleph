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
use crate::internal::{
    calc_subresource_index, plane_layer_for_aspect, plane_layer_for_aspect_flag,
};
use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use parking_lot::Mutex;
use std::any::TypeId;
use std::collections::HashMap;
use windows::utils::CPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

pub struct Texture {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) device: AnyArc<Device>,
    pub(crate) resource: ID3D12Resource,
    pub(crate) desc: TextureDesc<'static>,
    pub(crate) name: Option<String>,
    pub(crate) dxgi_format: DXGI_FORMAT,
    pub(crate) views: Mutex<HashMap<ImageViewDesc, CPUDescriptorHandle>>,
    pub(crate) rtvs: Mutex<HashMap<ImageViewDesc, CPUDescriptorHandle>>,
    pub(crate) dsvs: Mutex<HashMap<ImageViewDesc, CPUDescriptorHandle>>,
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

    pub fn make_rtv_desc_for_view_desc(desc: &ImageViewDesc) -> D3D12_RENDER_TARGET_VIEW_DESC {
        let (view_dimension, anonymous) = match desc.view_type {
            ImageViewType::TexArray1D => (
                D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_RTV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            ImageViewType::Tex1D => (
                D3D12_RTV_DIMENSION_TEXTURE1D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_RTV {
                        MipSlice: desc.sub_resources.base_mip_level,
                    },
                },
            ),
            ImageViewType::TexArray2D => (
                D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                        PlaneSlice: 0,
                    },
                },
            ),
            ImageViewType::Tex2D => (
                D3D12_RTV_DIMENSION_TEXTURE2D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_RTV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        PlaneSlice: 0,
                    },
                },
            ),
            // ImageViewType::Texture2D => (
            //     D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
            //     D3D12_RENDER_TARGET_VIEW_DESC_0 {
            //         Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV {
            //             FirstArraySlice: desc.sub_resources.base_array_slice,
            //             ArraySize: desc.sub_resources.num_array_slices,
            //         },
            //     },
            // ),
            // ImageViewType::Texture2D => (
            //     D3D12_RTV_DIMENSION_TEXTURE2DMS,
            //     D3D12_RENDER_TARGET_VIEW_DESC_0 {
            //         Texture2DMS: D3D12_TEX2DMS_RTV {
            //             UnusedField_NothingToDefine: 0,
            //         },
            //     },
            // ),
            ImageViewType::Tex3D => (
                D3D12_RTV_DIMENSION_TEXTURE3D,
                D3D12_RENDER_TARGET_VIEW_DESC_0 {
                    Texture3D: D3D12_TEX3D_RTV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstWSlice: desc.sub_resources.base_array_slice,
                        WSize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            _ => unimplemented!("Can't make RTVs for texture cubes"),
        };

        D3D12_RENDER_TARGET_VIEW_DESC {
            Format: texture_format_to_dxgi(desc.format),
            ViewDimension: view_dimension,
            Anonymous: anonymous,
        }
    }

    pub fn make_dsv_desc_for_view_desc(desc: &ImageViewDesc) -> D3D12_DEPTH_STENCIL_VIEW_DESC {
        let (view_dimension, anonymous) = match desc.view_type {
            ImageViewType::TexArray1D => (
                D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_DSV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            ImageViewType::Tex1D => (
                D3D12_DSV_DIMENSION_TEXTURE1D,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_DSV {
                        MipSlice: desc.sub_resources.base_mip_level,
                    },
                },
            ),
            ImageViewType::TexArray2D => (
                D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            ImageViewType::Tex2D => (
                D3D12_DSV_DIMENSION_TEXTURE2D,
                D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_DSV {
                        MipSlice: desc.sub_resources.base_mip_level,
                    },
                },
            ),
            // ImageViewType::Texture2D => (
            //     D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
            //     D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
            //         Texture2DMSArray: D3D12_TEX2DMS_ARRAY_DSV {
            //             FirstArraySlice: desc.sub_resources.base_array_slice,
            //             ArraySize: desc.sub_resources.num_array_slices,
            //         },
            //     },
            // ),
            // ImageViewType::Texture2D => (
            //     D3D12_DSV_DIMENSION_TEXTURE2DMS,
            //     D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
            //         Texture2DMS: D3D12_TEX2DMS_DSV {
            //             UnusedField_NothingToDefine: 0,
            //         },
            //     },
            // ),
            _ => unreachable!(),
        };

        D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: texture_format_to_dxgi(desc.format),
            ViewDimension: view_dimension,
            Flags: Default::default(),
            Anonymous: anonymous,
        }
    }

    pub fn make_srv_desc_for_view_desc(desc: &ImageViewDesc) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        let (dimension, anonymous) = match desc.view_type {
            ImageViewType::Tex1D => (
                D3D12_SRV_DIMENSION_TEXTURE1D,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::Tex2D => (
                D3D12_SRV_DIMENSION_TEXTURE2D,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        PlaneSlice: plane_layer_for_aspect_flag(
                            desc.format,
                            desc.sub_resources.aspect,
                        )
                        .unwrap(),
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::Tex3D => (
                D3D12_SRV_DIMENSION_TEXTURE3D,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture3D: D3D12_TEX3D_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::TexCube => (
                D3D12_SRV_DIMENSION_TEXTURECUBE,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    TextureCube: D3D12_TEXCUBE_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::TexArray1D => (
                D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::TexArray2D => (
                D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                        PlaneSlice: plane_layer_for_aspect_flag(
                            desc.format,
                            desc.sub_resources.aspect,
                        )
                        .unwrap(),
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
            ImageViewType::TexCubeArray => (
                D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
                D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                    TextureCubeArray: D3D12_TEXCUBE_ARRAY_SRV {
                        MostDetailedMip: desc.sub_resources.base_mip_level,
                        MipLevels: desc.sub_resources.num_mip_levels,
                        First2DArrayFace: desc.sub_resources.base_array_slice,
                        NumCubes: desc.sub_resources.num_array_slices / 6,
                        ResourceMinLODClamp: 0.0,
                    },
                },
            ),
        };

        D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: texture_format_to_dxgi(desc.format),
            ViewDimension: dimension,
            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: anonymous,
        }
    }

    pub fn make_uav_desc_for_view_desc(desc: &ImageViewDesc) -> D3D12_UNORDERED_ACCESS_VIEW_DESC {
        debug_assert!(desc.writable);

        let (dimension, anonymous) = match desc.view_type {
            ImageViewType::Tex1D => (
                D3D12_UAV_DIMENSION_TEXTURE1D,
                D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Texture1D: D3D12_TEX1D_UAV {
                        MipSlice: desc.sub_resources.base_mip_level,
                    },
                },
            ),
            ImageViewType::Tex2D => (
                D3D12_UAV_DIMENSION_TEXTURE2D,
                D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Texture2D: D3D12_TEX2D_UAV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        PlaneSlice: plane_layer_for_aspect_flag(
                            desc.format,
                            desc.sub_resources.aspect,
                        )
                        .unwrap(),
                    },
                },
            ),
            ImageViewType::Tex3D => (
                D3D12_UAV_DIMENSION_TEXTURE3D,
                D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Texture3D: D3D12_TEX3D_UAV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstWSlice: desc.sub_resources.base_array_slice,
                        WSize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            ImageViewType::TexArray1D => (
                D3D12_UAV_DIMENSION_TEXTURE1DARRAY,
                D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Texture1DArray: D3D12_TEX1D_ARRAY_UAV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                    },
                },
            ),
            ImageViewType::TexArray2D => (
                D3D12_UAV_DIMENSION_TEXTURE2DARRAY,
                D3D12_UNORDERED_ACCESS_VIEW_DESC_0 {
                    Texture2DArray: D3D12_TEX2D_ARRAY_UAV {
                        MipSlice: desc.sub_resources.base_mip_level,
                        FirstArraySlice: desc.sub_resources.base_array_slice,
                        ArraySize: desc.sub_resources.num_array_slices,
                        PlaneSlice: plane_layer_for_aspect_flag(
                            desc.format,
                            desc.sub_resources.aspect,
                        )
                        .unwrap(),
                    },
                },
            ),
            _ => {
                unimplemented!("Can't make UAVs for cube maps")
            }
        };

        D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: texture_format_to_dxgi(desc.format),
            ViewDimension: dimension,
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
        desc.name = self.name.as_deref();
        desc
    }

    fn get_view(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.views.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let view = self
                .device
                .descriptor_heaps
                .cpu_view_heap()
                .allocate()
                .ok_or(())?;

            if desc.writable {
                let desc = Self::make_uav_desc_for_view_desc(desc);
                unsafe {
                    self.device.device.CreateUnorderedAccessView(
                        &self.resource,
                        None,
                        Some(&desc),
                        view.into(),
                    );
                }
            } else {
                let desc = Self::make_srv_desc_for_view_desc(desc);
                unsafe {
                    self.device.device.CreateShaderResourceView(
                        &self.resource,
                        Some(&desc),
                        view.into(),
                    );
                }
            }

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.rtvs.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let view = self
                .device
                .descriptor_heaps
                .cpu_rtv_heap()
                .allocate()
                .ok_or(())?;

            unsafe {
                let desc = Self::make_rtv_desc_for_view_desc(desc);
                self.device
                    .device
                    .CreateRenderTargetView(&self.resource, Some(&desc), view.into());
            }

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
    }

    fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        let mut views = self.dsvs.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            let view = self
                .device
                .descriptor_heaps
                .cpu_dsv_heap()
                .allocate()
                .ok_or(())?;

            unsafe {
                let desc = Self::make_dsv_desc_for_view_desc(desc);
                self.device
                    .device
                    .CreateDepthStencilView(&self.resource, Some(&desc), view.into());
            }

            views.insert(desc.clone(), view);
            view
        };

        unsafe { Ok(std::mem::transmute::<_, ImageView>(view)) }
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
        for (_, view) in self.views.get_mut().drain() {
            self.device.descriptor_heaps.cpu_view_heap().free(view);
        }

        // Free all RTVs associated with this texture
        for (_, rtv) in self.rtvs.get_mut().drain() {
            self.device.descriptor_heaps.cpu_rtv_heap().free(rtv);
        }

        // Free all DSVs associated with this texture
        for (_, dsv) in self.dsvs.get_mut().drain() {
            self.device.descriptor_heaps.cpu_dsv_heap().free(dsv);
        }
    }
}
