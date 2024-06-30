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

use crate::render::{ObjectPool, TextureHandle};
use aleph_rhi_api::*;
use interfaces::any::AnyArc;
use std::num::NonZeroU8;

pub struct TexturePool {
    pool: ObjectPool<TextureObject>,
}

impl TexturePool {
    pub fn new(id: NonZeroU8) -> Self {
        Self {
            pool: ObjectPool::new(id),
        }
    }

    /// Creates a new texture object from the given texture and returns a [TextureHandle] that can
    /// be used to retreive the texture from the pool in the future.
    pub fn create_texture(&mut self, texture: AnyArc<dyn ITexture>) -> TextureHandle {
        let mut object = TextureObject {
            texture: Some(texture),
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

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&dyn ITexture> {
        if let Some(object) = self.get_ref(handle) {
            object.texture.as_deref()
        } else {
            None
        }
    }

    pub fn get_default_view(&self, handle: TextureHandle) -> Option<ImageView> {
        if let Some(object) = self.get_ref(handle) {
            object.default_view
        } else {
            None
        }
    }

    pub fn destroy_texture(&mut self, handle: TextureHandle) -> Option<()> {
        self.free(handle).map(|_| ())
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
    /// Queries an ImageView from the texture object, replacing the default view. If there's n
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
