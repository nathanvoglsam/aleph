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

use crate::internal::Magnesium;
use crate::internal::object_pool::ObjectPool;
use crate::resource::texture::physical::PhysicalTextureDesc;
use crate::resource::texture::simple::SimpleTextureDesc;
use crate::resource::texture::single::SingleTextureDesc;
use crate::resource::texture::{TextureHandle, TexturePoolAccessor};
use crate::resource_loader::mip_upload::{LevelOffsets, MipUploadDesc};

/// Manager that owns the texture object pool, and any other resources directly associated with
/// our pooled texture resources.
pub struct TextureObjectStore {
    pub pool: ObjectPool<TextureObject, TextureHandle, MgTexSystem>,
}

impl TextureObjectStore {
    /// Constructs a new, empty object store
    pub fn new() -> Self {
        Self {
            pool: ObjectPool::new_in(),
        }
    }

    /// Create a read-only [`TexturePoolAccessor`]. See the accessor type for more information.
    pub const fn accessor(&self) -> TexturePoolAccessor<'_> {
        TexturePoolAccessor(self)
    }

    /// Cleanup code invoked when the renderer object is dropped.
    pub fn clean_up(&mut self) {
        self.pool.clear();
    }
}

/// The object we store _inside_ the texture pool managed by a [`TextureObjectStore`]
pub struct TextureObject {
    /// The RHI texture object we manage inside the pool.
    pub object: Option<rhi::TextureHandle>,

    /// Default view based on the expected use for the resource.
    pub default_view: Option<rhi::ImageView>,

    /// A subresource set that covers the entire resource. Cached so we don't need to look up the
    /// desc every time we need it. Saves a virtual call.
    pub subresource_all: rhi::TextureSubResourceSet,

    /// The format of the texture. Cached to save looking up the desc every time we need it.
    pub format: rhi::Format,
}

impl TextureObject {
    /// Queries an ImageView from the texture object, replacing the default view. If there's no
    /// texture object this does nothing.
    pub fn recreate_default_view(&mut self, device: &dyn rhi::IDevice) {
        if let Some(texture) = &self.object {
            let desc = device.get_texture_desc(texture);

            let view_type = match desc.dimension {
                rhi::TextureDimension::Texture1D => {
                    if desc.array_size > 1 {
                        rhi::ImageViewType::TexArray1D
                    } else {
                        rhi::ImageViewType::Tex1D
                    }
                }
                rhi::TextureDimension::Texture2D => {
                    if desc.array_size > 1 {
                        rhi::ImageViewType::TexArray2D
                    } else {
                        rhi::ImageViewType::Tex2D
                    }
                }
                rhi::TextureDimension::Texture3D => {
                    // Can't make Tex3D arrays so the default view will always be
                    rhi::ImageViewType::Tex3D
                }
            };

            let sub_resources = rhi::TextureSubResourceSet::with_color()
                .with_mips(0, desc.mip_levels)
                .with_levels(0, desc.array_size);

            let view = device
                .get_texture_view(
                    texture,
                    &rhi::ImageViewDesc {
                        format: desc.format,
                        view_type,
                        sub_resources,
                        writable: false,
                    },
                )
                .unwrap();

            self.default_view = Some(view);
        }
    }
}

pub fn make_standard_texture_desc<T: SimpleTextureDesc>(
    desc: &T,
) -> Option<rhi::TextureDesc<'static>> {
    // Standard usage valid for access as an SRV + uploads. We need RT for mip generation.
    const USAGE: rhi::ResourceUsageFlags = rhi::ResourceUsageFlags::COPY_DEST
        .union(rhi::ResourceUsageFlags::SHADER_RESOURCE)
        .union(rhi::ResourceUsageFlags::RENDER_TARGET);
    Some(rhi::TextureDesc {
        width: desc.storage_width(),
        height: desc.storage_height(),
        depth: desc.storage_depth(),
        format: desc.format(),
        dimension: desc.texture_dimension()?,
        clear_value: None,
        array_size: 1,
        mip_levels: desc.num_levels().get(),
        sample_count: 1,
        sample_quality: 0,
        usage: USAGE,
        name: None,
    })
}

/// Allocator category for the texture object store
pub struct Texture;
aleph_alloc::new_child_alloc_category!(Magnesium, Texture, "019b2eff-71d9-7c03-a3a8-9ab3efa9b04e");

pub type MgTexSystem = aleph_alloc::instrumentation::Instrumented<Texture>;

pub fn validate_mip_data_for_immediate_upload<T: SimpleTextureDesc>(
    desc: &T,
    data: &MipUploadDesc,
) -> Option<()> {
    // Check that the offsets are correctly aligned for each mip level
    validate_offsets_for(desc, &data.data)?;

    // Check that data for all mip levels is available
    let num_levels = desc.num_levels().get();
    let level_range = data.data.level_range();
    if level_range.start != 0 {
        return None;
    }
    if level_range.end != num_levels {
        return None;
    }

    // Check that the included buffer is large enough for all mip level's data.
    //
    // Finds the highest offset byte that won't be read.
    let mut max_mip_end = usize::MIN;
    for (i, &offset) in data.data.level_offsets.iter().enumerate() {
        let level = data.data.base_level as u32 + i as u32;
        let bytes = desc
            .as_level(level)
            .with_aligned_stride(data.stride_align)
            .upload_bytes();

        // Keep the larger of the existing highest offset or our new offset if it is larger.
        max_mip_end = usize::max(max_mip_end, offset + bytes);
    }
    if data.buffer.bytes().len() < max_mip_end {
        // One of the levels would need data that is out of bounds of the included upload buffer
        // range.
        return None;
    }

    Some(())
}

/// Validates that the data description is compatible with the given [`SimpleTextureDesc`].
pub fn validate_offsets_for<T: SimpleTextureDesc>(desc: &T, offsets: &LevelOffsets) -> Option<()> {
    for offset in offsets.level_offsets.iter() {
        // Offsets must be 512 byte aligned
        if *offset % 512 != 0 {
            return None;
        }
    }

    let levels = offsets.level_range();

    // Mip chain addresses an out of bounds mip level
    if levels.start >= desc.num_levels().get() {
        return None;
    }

    // Mip chain addresses an out of bounds mip level
    if levels.end > desc.num_levels().get() {
        return None;
    }

    Some(())
}
