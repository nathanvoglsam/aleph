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

use aleph_rhi_api::*;

use crate::TextureObjectDesc;

pub struct TextureObject {
    /// The texture object itself.
    texture: Option<TextureHandle>,

    /// The description the texture was created with.
    desc: TextureObjectDesc,

    /// The image view we want to view the texture through.
    default_view: Option<ImageView>,
}

impl TextureObject {
    pub fn new_for_desc(
        device: &dyn IDevice,
        desc: TextureObjectDesc,
    ) -> Result<Self, TextureCreateError> {
        let mut combined_usage = desc.usage;

        // We require copy dest so we can initialize the resource
        combined_usage |= ResourceUsageFlags::COPY_DEST;

        // We require shader resource and render target usage to be able to generate mip
        // maps into the texture.
        //
        // TODO: in the future we could use a compute based mip generator which would
        //       require unordered access.
        combined_usage |= ResourceUsageFlags::RENDER_TARGET;
        combined_usage |= ResourceUsageFlags::SHADER_RESOURCE;

        let api_desc = TextureDesc {
            width: desc.width.max(1),
            height: desc.height.max(1),
            depth: desc.depth.max(1),
            format: desc.format,
            dimension: TextureDimension::Texture2D, // TODO: need to propogate this
            clear_value: None,
            array_size: 1,
            mip_levels: desc.num_levels.get(),
            sample_count: 1,
            sample_quality: 0,
            usage: combined_usage,
            name: None,
        };
        let texture = device.create_texture(&api_desc)?;

        Ok(Self {
            texture: Some(texture),
            desc,
            default_view: None,
        })
    }

    pub fn update(
        &mut self,
        device: &dyn IDevice,
        texture: TextureHandle,
    ) -> Option<TextureHandle> {
        if let Some(old_texture) = &self.texture {
            let new_desc = device.get_texture_desc(&texture);
            let old_desc = device.get_texture_desc(old_texture);

            // It is illegal for any major property of the new texture to change from the old
            // texture.
            debug_assert_eq!(new_desc.format, old_desc.format);
            debug_assert_eq!(new_desc.dimension, old_desc.dimension);
            debug_assert_eq!(new_desc.clear_value, old_desc.clear_value);
            debug_assert_eq!(new_desc.sample_count, old_desc.sample_count);
            debug_assert_eq!(new_desc.sample_quality, old_desc.sample_quality);
            debug_assert_eq!(new_desc.usage, old_desc.usage);
        }

        // Swap the old texture for the new, taking the old texture to send it out to the caller
        let mut texture = Some(texture);
        std::mem::swap(&mut texture, &mut self.texture);

        // Query the appropriate view for the new texture
        self.recreate_default_view(device);

        // And give the old texture back out to the caller
        texture
    }

    /// Returns the texture object associated with the given [TextureHandle].
    ///
    /// May return [None] if either the handle is invalid (dead, wrong pool, etc) or if the pool
    /// doesn't have a texture for the requested handle yet. It's possible for a handle to have
    /// no texture, such as if the handle was reserved but hasn't been initialized with
    /// [TexturePool::update_texture] yet.
    pub const fn get(&self) -> Option<&TextureHandle> {
        self.texture.as_ref()
    }

    #[inline]
    pub fn get_owned(&self) -> Option<TextureHandle> {
        self.texture.clone()
    }

    /// The set of resource usage flags the texture was created to be used with.
    pub const fn desc(&self) -> &TextureObjectDesc {
        &self.desc
    }

    /// Returns an [ImageView], which is the default view for the requested handle.
    ///
    /// A default view is created when the texture is provided to the pool. The view will be a basic
    /// read-only SRV of the texture over all subresources as the texture's native format. The image
    /// array-ness and dimension (1D, 2D, 3D) is reflected and the most capable view will always
    /// be created. In general we will create:
    ///
    /// - [TextureDimension::Texture1D]: `TexArray1D` for array textures and `Tex1D` otherwise.
    /// - [TextureDimension::Texture2D]: `TexArray2D` for array textures and `Tex2D` otherwise.
    /// - [TextureDimension::Texture3D]: Always `Tex3D`, as there are no `TexArray3D` views.
    ///
    /// `TexCube` and `TexArrayCube` are handled elsewhere.
    ///
    /// May return [None] if either the handle is invalid (dead, wrong pool, etc) or if the pool
    /// doesn't have a texture for the requested handle yet. It's possible for a handle to have
    /// no texture, such as if the handle was reserved but hasn't been initialized with
    /// [TexturePool::update_texture] yet.
    pub const fn get_default_view(&self) -> Option<ImageView> {
        self.default_view
    }

    /// Queries an ImageView from the texture object, replacing the default view. If there's no
    /// texture object this does nothing.
    pub fn recreate_default_view(&mut self, device: &dyn IDevice) {
        if let Some(texture) = &self.texture {
            let desc = device.get_texture_desc(texture);

            let view_type = match desc.dimension {
                TextureDimension::Texture1D => {
                    if desc.array_size > 1 {
                        ImageViewType::TexArray1D
                    } else {
                        ImageViewType::Tex1D
                    }
                }
                TextureDimension::Texture2D => {
                    if desc.array_size > 1 {
                        ImageViewType::TexArray2D
                    } else {
                        ImageViewType::Tex2D
                    }
                }
                TextureDimension::Texture3D => {
                    // Can't make Tex3D arrays so the default view will always be
                    ImageViewType::Tex3D
                }
            };

            let sub_resources = TextureSubResourceSet::with_color()
                .with_mips(0, desc.mip_levels)
                .with_levels(0, desc.array_size);

            let view = device
                .get_texture_view(
                    texture,
                    &ImageViewDesc {
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
