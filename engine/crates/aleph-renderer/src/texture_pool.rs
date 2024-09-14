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

use std::num::NonZeroU8;

use aleph_any::AnyArc;
use aleph_rhi_api::*;

use crate::{ObjectPool, TextureHandle};

pub struct TexturePool {
    pool: ObjectPool<TextureObject>,
}

impl TexturePool {
    /// Constructs a new [TexturePool] with the given pool id tag.
    ///
    /// The `id` tag is stored inside any [TextureHandle] objects this pool allocates so that it
    /// can do some cheap (though not bulletproof) verification that the handle came from this
    /// pool.
    ///
    /// It's up to the caller to not reuse a pool id, at the very least at the same time.
    pub fn new(id: NonZeroU8) -> Self {
        Self {
            pool: ObjectPool::new(id),
        }
    }

    /// Creates a new texture handle with no texture data stored inside.
    ///
    /// The texture object inside will be missing until the data is populated using
    /// [TexturePool::update_texture].
    pub fn reserve_handle(&mut self) -> TextureHandle {
        self.alloc(TextureObject {
            texture: None,
            default_view: None,
        })
    }

    /// Creates a new texture object from the given texture and returns a [TextureHandle] that can
    /// be used to retreive the texture from the pool in the future.
    pub fn create_texture(&mut self, texture: Option<AnyArc<dyn ITexture>>) -> TextureHandle {
        let mut object = TextureObject {
            texture,
            default_view: None,
        };
        object.create_default_view();

        self.alloc(object)
    }

    /// Updates an existing texture object, identified by the given [TextureHandle] with a new RHI
    /// texture. Returns the old texture if one was present for the given handle.
    pub fn update_texture(
        &mut self,
        handle: TextureHandle,
        texture: AnyArc<dyn ITexture>,
    ) -> Option<AnyArc<dyn ITexture>> {
        if let Some(object) = self.get_mut(handle) {
            if let Some(old_texture) = &object.texture {
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
            std::mem::swap(&mut texture, &mut object.texture);

            // Query the appropriate view for the new texture
            object.create_default_view();

            // And give the old texture back out to the caller
            texture
        } else {
            None
        }
    }

    /// Returns the texture object associated with the given [TextureHandle].
    ///
    /// May return [None] if either the handle is invalid (dead, wrong pool, etc) or if the pool
    /// doesn't have a texture for the requested handle yet. It's possible for a handle to have
    /// no texture, such as if the handle was reserved but hasn't been initialized with
    /// [TexturePool::update_texture] yet.
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&dyn ITexture> {
        if let Some(object) = self.get_ref(handle) {
            object.texture.as_deref()
        } else {
            None
        }
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
    pub fn get_default_view(&self, handle: TextureHandle) -> Option<ImageView> {
        if let Some(object) = self.get_ref(handle) {
            object.default_view
        } else {
            None
        }
    }

    /// Removes the given texture from the pool, returning the [ITexture] object it was storing if
    /// one exists.
    ///
    /// The nested options represent two levels of optional values. The outer option return directly
    /// by this function signals whether a texture object was found and removed from the given
    /// handle. The inner option reflects that the referenced texture may not have an [ITexture]
    /// to return. Flattening the options would make a missing [ITexture] look like an invalid
    /// handle.
    pub fn destroy_texture(
        &mut self,
        handle: TextureHandle,
    ) -> Option<Option<AnyArc<dyn ITexture>>> {
        self.free(handle).map(|v| v.texture)
    }
}

impl TexturePool {
    pub(crate) fn alloc(&mut self, data: TextureObject) -> TextureHandle {
        let handle = self.pool.alloc(data);

        // Safety: uuuh, this is a texture pool, of course we give out _texture handles_
        unsafe { TextureHandle::from_handle(handle) }
    }

    pub(crate) fn get_ref(&self, handle: TextureHandle) -> Option<&TextureObject> {
        self.pool.get_ref(handle.to_handle())
    }

    pub(crate) fn get_mut(&mut self, handle: TextureHandle) -> Option<&mut TextureObject> {
        self.pool.get_mut(handle.to_handle())
    }

    pub(crate) fn free(&mut self, handle: TextureHandle) -> Option<TextureObject> {
        self.pool.free(handle.to_handle())
    }
}

pub struct TextureObject {
    /// The texture object itself
    texture: Option<AnyArc<dyn ITexture>>,

    /// The image view we want to view the texture through
    default_view: Option<ImageView>,
}

impl TextureObject {
    /// Queries an ImageView from the texture object, replacing the default view. If there's no
    /// texture object this does nothing.
    pub fn create_default_view(&mut self) {
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
