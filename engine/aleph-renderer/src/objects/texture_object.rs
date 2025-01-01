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

use aleph_any::AnyArc;
use aleph_rhi_api::*;

pub struct TextureObject {
    /// The texture object itself
    texture: Option<AnyArc<dyn ITexture>>,

    /// The image view we want to view the texture through
    default_view: Option<ImageView>,
}

impl TextureObject {
    pub const fn new() -> Self {
        Self {
            texture: None,
            default_view: None,
        }
    }

    pub const fn new_with(texture: AnyArc<dyn ITexture>) -> Self {
        Self {
            texture: Some(texture),
            default_view: None,
        }
    }

    pub const fn new_with_opt(texture: Option<AnyArc<dyn ITexture>>) -> Self {
        Self {
            texture,
            default_view: None,
        }
    }

    pub fn update(&mut self, texture: AnyArc<dyn ITexture>) -> Option<AnyArc<dyn ITexture>> {
        if let Some(old_texture) = &self.texture {
            let new_desc = texture.desc_ref();
            let old_desc = old_texture.desc_ref();

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
        self.recreate_default_view();

        // And give the old texture back out to the caller
        texture
    }

    /// Returns the texture object associated with the given [TextureHandle].
    ///
    /// May return [None] if either the handle is invalid (dead, wrong pool, etc) or if the pool
    /// doesn't have a texture for the requested handle yet. It's possible for a handle to have
    /// no texture, such as if the handle was reserved but hasn't been initialized with
    /// [TexturePool::update_texture] yet.
    pub fn get_texture(&self) -> Option<&dyn ITexture> {
        self.texture.as_ref().map(|v| v.as_ref())
    }

    pub fn get_owned(&self) -> Option<AnyArc<dyn ITexture>> {
        self.texture.clone()
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
    pub fn recreate_default_view(&mut self) {
        if let Some(texture) = &self.texture {
            let desc = texture.desc_ref();

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

            let view = texture
                .get_view(&ImageViewDesc {
                    format: desc.format,
                    view_type,
                    sub_resources,
                    writable: false,
                })
                .unwrap();

            self.default_view = Some(view);
        }
    }
}
