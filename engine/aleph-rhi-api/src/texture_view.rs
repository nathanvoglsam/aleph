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

use std::ptr::NonNull;

use crate::*;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ImageView(NonNull<()>);

impl ImageView {
    #[inline]
    pub fn get_rtv_for(device: &dyn IDevice, texture: &TextureHandle) -> Result<Self, ()> {
        device.get_texture_rtv(texture, &ImageViewDesc::rtv_for_texture(device, texture))
    }

    #[inline]
    pub fn get_dsv_for(device: &dyn IDevice, texture: &TextureHandle) -> Result<Self, ()> {
        device.get_texture_dsv(texture, &ImageViewDesc::dsv_for_texture(device, texture))
    }

    #[inline]
    pub fn get_srv_for(device: &dyn IDevice, texture: &TextureHandle) -> Result<Self, ()> {
        device.get_texture_view(texture, &ImageViewDesc::srv_for_texture(device, texture))
    }

    #[inline]
    pub fn get_uav_for(device: &dyn IDevice, texture: &TextureHandle) -> Result<Self, ()> {
        device.get_texture_view(texture, &ImageViewDesc::uav_for_texture(device, texture))
    }

    pub const fn descriptor_write(self, image_layout: ImageLayout) -> ImageDescriptorWrite {
        ImageDescriptorWrite::new(self, image_layout)
    }

    pub const fn srv_write(self) -> ImageDescriptorWrite {
        ImageDescriptorWrite::srv(self)
    }

    pub const fn uav_write(self) -> ImageDescriptorWrite {
        ImageDescriptorWrite::uav(self)
    }
}

unsafe impl Send for ImageView {}
unsafe impl Sync for ImageView {}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ImageViewDesc {
    /// The format that the texture will be viewed as through this descriptor
    pub format: Format,

    /// The type of view of the given image to create.
    pub view_type: ImageViewType,

    /// The set of sub resources that will be accessed through this descriptor
    pub sub_resources: TextureSubResourceSet,

    /// Whether the image can be written to through this descriptor.
    pub writable: bool,
}

impl ImageViewDesc {
    /// Returns the [`ImageViewDesc`] as-is, except with [`ImageViewDesc::format`] set to the given
    /// format.
    pub const fn with_format(mut self, format: Format) -> ImageViewDesc {
        self.format = format;
        self
    }

    #[inline]
    pub fn srv_for_texture(device: &dyn IDevice, texture: &TextureHandle) -> ImageViewDesc {
        let desc = device.get_texture_desc(texture);
        Self::srv_for_desc(desc)
    }

    #[inline]
    pub fn srv_for_desc(desc: &TextureDesc) -> ImageViewDesc {
        debug_assert!(desc.usage.contains(ResourceUsageFlags::SHADER_RESOURCE));
        let view_type = match desc.dimension {
            TextureDimension::Texture1D => ImageViewType::Tex1D,
            TextureDimension::Texture2D => ImageViewType::Tex2D,
            TextureDimension::Texture3D => ImageViewType::Tex3D,
        };
        let aspect = desc.format.aspect_mask();
        ImageViewDesc {
            format: desc.format,
            view_type,
            sub_resources: TextureSubResourceSet {
                aspect,
                base_mip_level: 0,
                num_mip_levels: desc.mip_levels,
                base_array_slice: 0,
                num_array_slices: desc.array_size,
            },
            writable: false,
        }
    }

    #[inline]
    pub fn uav_for_texture(device: &dyn IDevice, texture: &TextureHandle) -> ImageViewDesc {
        let desc = device.get_texture_desc(texture);
        Self::uav_for_desc(desc)
    }

    #[inline]
    pub fn uav_for_desc(desc: &TextureDesc) -> ImageViewDesc {
        debug_assert!(desc.usage.contains(ResourceUsageFlags::UNORDERED_ACCESS));
        let view_type = match desc.dimension {
            TextureDimension::Texture1D => ImageViewType::Tex1D,
            TextureDimension::Texture2D => ImageViewType::Tex2D,
            TextureDimension::Texture3D => ImageViewType::Tex3D,
        };
        let aspect = desc.format.aspect_mask();
        ImageViewDesc {
            format: desc.format,
            view_type,
            sub_resources: TextureSubResourceSet {
                aspect,
                base_mip_level: 0,
                num_mip_levels: desc.mip_levels,
                base_array_slice: 0,
                num_array_slices: desc.array_size,
            },
            writable: true,
        }
    }

    #[inline]
    pub fn rtv_for_texture(device: &dyn IDevice, texture: &TextureHandle) -> ImageViewDesc {
        let desc = device.get_texture_desc(texture);
        Self::rtv_for_desc(desc)
    }

    #[inline]
    pub fn rtv_for_desc(desc: &TextureDesc) -> ImageViewDesc {
        debug_assert!(desc.usage.contains(ResourceUsageFlags::RENDER_TARGET));
        let view_type = match desc.dimension {
            TextureDimension::Texture1D => ImageViewType::Tex1D,
            TextureDimension::Texture2D => ImageViewType::Tex2D,
            TextureDimension::Texture3D => ImageViewType::Tex3D,
        };
        let aspect = desc.format.aspect_mask();
        ImageViewDesc {
            format: desc.format,
            view_type,
            sub_resources: TextureSubResourceSet {
                aspect,
                base_mip_level: 0,
                num_mip_levels: desc.mip_levels,
                base_array_slice: 0,
                num_array_slices: desc.array_size,
            },
            writable: false,
        }
    }

    #[inline]
    pub fn dsv_for_texture(device: &dyn IDevice, texture: &TextureHandle) -> ImageViewDesc {
        let desc = device.get_texture_desc(texture);
        Self::rtv_for_desc(desc)
    }

    #[inline]
    pub fn dsv_for_desc(desc: &TextureDesc) -> ImageViewDesc {
        debug_assert!(desc.usage.contains(ResourceUsageFlags::RENDER_TARGET));
        let view_type = match desc.dimension {
            TextureDimension::Texture1D => ImageViewType::Tex1D,
            TextureDimension::Texture2D => ImageViewType::Tex2D,
            TextureDimension::Texture3D => ImageViewType::Tex3D,
        };
        let aspect = desc.format.aspect_mask();
        ImageViewDesc {
            format: desc.format,
            view_type,
            sub_resources: TextureSubResourceSet {
                aspect,
                base_mip_level: 0,
                num_mip_levels: desc.mip_levels,
                base_array_slice: 0,
                num_array_slices: desc.array_size,
            },
            writable: false,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ImageViewType {
    Tex1D,
    Tex2D,
    Tex3D,
    TexCube,
    TexArray1D,
    TexArray2D,
    TexCubeArray,
}

impl std::fmt::Display for ImageViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageViewType::Tex1D => f.write_str("Tex1D"),
            ImageViewType::Tex2D => f.write_str("Tex2D"),
            ImageViewType::Tex3D => f.write_str("Tex3D"),
            ImageViewType::TexCube => f.write_str("TexCube"),
            ImageViewType::TexArray1D => f.write_str("TexArray1D"),
            ImageViewType::TexArray2D => f.write_str("TexArray2D"),
            ImageViewType::TexCubeArray => f.write_str("TexCubeArray"),
        }
    }
}
