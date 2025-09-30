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

use std::num::NonZeroU64;
use std::ptr::NonNull;

use aleph_alloc::BHashMap;
use aleph_any::AnyArc;
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::owned_desc::OwnedTextureDesc;
use blink_alloc::{Blink, BlinkAlloc};
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::ProtocolObject;
use objc2_foundation::{NSRange, NSString};
use objc2_metal::*;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::conv;
use crate::internal::image_view::ImageViewObject;

pub struct Texture {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: TextureObjects,
    pub(crate) views: Mutex<BHashMap<ImageViewDesc, ImageView, RhiSystem>>,
    pub(crate) rtvs: Mutex<BHashMap<ImageViewDesc, ImageView, RhiSystem>>,
    pub(crate) dsvs: Mutex<BHashMap<ImageViewDesc, ImageView, RhiSystem>>,
    pub(crate) image_views: Mutex<Blink<BlinkAlloc<RhiSystem>>>,
    pub(crate) desc: OwnedTextureDesc,
}

unsafe_impl_iobject!(Texture, "01980753-5c4f-7ae3-be3b-974d0f86022a");

impl Texture {
    pub(crate) fn create(
        device: &Device,
        desc: &TextureDesc,
    ) -> Result<TextureHandle, TextureCreateError> {
        let mtl_desc = unsafe { MTLTextureDescriptor::new() };
        unsafe {
            let (array_len, texture_type) = match (desc.array_size, desc.dimension) {
                (v, TextureDimension::Texture1D) if v > 1 => (v, MTLTextureType::Type1DArray),
                (v, TextureDimension::Texture2D) if v > 1 => (v, MTLTextureType::Type2DArray),
                (v, TextureDimension::Texture3D) if v > 1 => unimplemented!(),
                (_, TextureDimension::Texture1D) => (1, MTLTextureType::Type1D),
                (_, TextureDimension::Texture2D) => (1, MTLTextureType::Type2D),
                (_, TextureDimension::Texture3D) => (1, MTLTextureType::Type3D),
            };
            mtl_desc.setTextureType(texture_type);
            mtl_desc.setPixelFormat(conv::format_to_pixel_mtl(desc.format));
            mtl_desc.setWidth(desc.width as usize);
            mtl_desc.setHeight(desc.height as usize);
            mtl_desc.setDepth(desc.depth as usize);
            mtl_desc.setMipmapLevelCount(desc.mip_levels as usize);
            mtl_desc.setArrayLength(array_len as usize);
            mtl_desc.setUsage(conv::resource_usage_to_texture_usage_mtl(desc.usage));
            mtl_desc.setStorageMode(MTLStorageMode::Private);
            mtl_desc.setHazardTrackingMode(MTLHazardTrackingMode::Tracked);
            mtl_desc.setAllowGPUOptimizedContents(true);
            mtl_desc.setSampleCount(desc.sample_count as usize);
        }

        let texture = match device.device.newTextureWithDescriptor(&mtl_desc) {
            Some(v) => v,
            None => return Err(TextureCreateError::Platform),
        };

        if let Some(name) = desc.name
            && device.context.debug
        {
            let label = NSString::from_str(name);
            texture.setLabel(Some(&label));
        }

        let out = Texture {
            _device: device.this.upgrade().unwrap(),
            id: device.object_counter.next_texture(),
            views: Default::default(),
            objects: TextureObjects { texture },
            rtvs: Default::default(),
            dsvs: Default::default(),
            image_views: Mutex::new(Blink::new_in(BlinkAlloc::new_in(RhiSystem::default()))),
            desc: OwnedTextureDesc::new(desc.clone()),
        };
        let out = Object::new_arc_opaque(out);
        unsafe { Ok(TextureHandle::new(out)) }
    }

    pub(crate) fn get(v: &TextureHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown Texture implementation!")
    }

    pub(crate) fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    pub(crate) const fn desc(&self) -> &TextureDesc<'_> {
        self.desc.get()
    }

    pub(crate) fn get_view(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_mtl_texture_view(&self.views, desc)
    }

    pub(crate) fn get_rtv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_mtl_texture_view(&self.rtvs, desc)
    }

    pub(crate) fn get_dsv(&self, desc: &ImageViewDesc) -> Result<ImageView, ()> {
        self.get_mtl_texture_view(&self.dsvs, desc)
    }

    /// The code-path for 'get_rtv' and 'get_dsv' is almost identical, except for the target hash
    /// map.
    pub(crate) fn get_mtl_texture_view(
        &self,
        map: &Mutex<BHashMap<ImageViewDesc, ImageView, RhiSystem>>,
        desc: &ImageViewDesc,
    ) -> Result<ImageView, ()> {
        let mut views = map.lock();

        let view = if let Some(view) = views.get(desc) {
            *view
        } else {
            // Check if we were asked to make a view of the same texture type as the base texture
            // resource.
            let is_resource_an_array = self.desc().array_size > 1;
            let view_type_matches_resource = match (is_resource_an_array, self.desc().dimension) {
                (false, TextureDimension::Texture1D) => desc.view_type == ImageViewType::Tex1D,
                (false, TextureDimension::Texture2D) => desc.view_type == ImageViewType::Tex2D,
                (false, TextureDimension::Texture3D) => desc.view_type == ImageViewType::Tex3D,
                (true, TextureDimension::Texture1D) => desc.view_type == ImageViewType::TexArray1D,
                (true, TextureDimension::Texture2D) => desc.view_type == ImageViewType::TexArray2D,
                (true, TextureDimension::Texture3D) => unreachable!(),
            };

            // Determine if the requested view covers the _entire_ resource.
            let all_mips_and_slices = desc.sub_resources.base_mip_level == 0
                && desc.sub_resources.num_mip_levels == self.desc().mip_levels
                && desc.sub_resources.base_array_slice == 0
                && desc.sub_resources.num_array_slices == self.desc().array_size;

            // Determine if the requested view is asking for the same format as the texture resource
            // itself
            let view_format_matches_texture = desc.format == self.desc().format;

            let view = match (
                all_mips_and_slices,
                view_type_matches_resource,
                view_format_matches_texture,
            ) {
                (true, true, true) => self.objects.texture.clone(),
                _ => {
                    let texture_type = conv::image_view_type_to_mtl(desc.view_type);
                    let level_range = NSRange::new(
                        desc.sub_resources.base_mip_level as usize,
                        desc.sub_resources.num_mip_levels as usize,
                    );
                    let slice_range = NSRange::new(
                        desc.sub_resources.base_array_slice as usize,
                        desc.sub_resources.num_array_slices as usize,
                    );
                    let view = autoreleasepool(|_| unsafe {
                        self.objects
                            .texture
                            .newTextureViewWithPixelFormat_textureType_levels_slices(
                                conv::format_to_pixel_mtl(desc.format),
                                texture_type,
                                level_range,
                                slice_range,
                            )
                            .expect("Failed to construct MTLTexture image view")
                    });
                    if self._device.context.debug {
                        autoreleasepool(|_| view.setLabel(self.objects.texture.label().as_deref()));
                    }
                    view
                }
            };

            let view = unsafe {
                // Allocate the view into the internal bump allocator
                let alloc = self.image_views.lock();
                let view = NonNull::from(alloc.put(ImageViewObject { texture: view }));
                ImageView::from_raw(view.cast())
            };

            views.insert(desc.clone(), view);
            view
        };

        Ok(view)
    }
}

/// Wrapper to scope our 'unsafe impl Send+Sync'
pub struct TextureObjects {
    pub texture: Retained<ProtocolObject<dyn MTLTexture>>,
}

// Safety: Needed because of 'MTLTexture'
unsafe impl Send for TextureObjects {}
unsafe impl Sync for TextureObjects {}
