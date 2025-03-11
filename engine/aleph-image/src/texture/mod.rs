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

use aleph_math::{UVec2, Vec3};
use half::f16;

use crate::utils::{f32_to_unorm_u16, f32_to_unorm_u8, unorm_u16_to_f32, unorm_u8_to_f32};
use crate::{
    cube_to_equi, cube_to_octahedral, equi_to_cube_dyn, equi_to_octahedral_dyn,
    octahedral_to_cube_dyn, ColorType, CubeSampler, DowncastImageBuffer, DynamicImageBuffer,
    FaceNegX, FaceNegY, FaceNegZ, FacePosX, FacePosY, FacePosZ, IPixelStorage, IResizeImage,
    ImageBuffer, PixRGBA, PixelFormat, ResizeFilter, TextureOpError, TextureOpResult,
};

/// Type that closes over the types of textures we support working with. This includes the types
/// that we can import as well as export.
///
/// The purpose of this type is to track the type of an image dynamically as it flows through the
/// image conditioning pipeline. We have a number of filters and processes we can run on textures
/// in order to
pub enum TextureBuffer {
    /// A single image. This may contain multiple images which define a mip chain.
    Single {
        /// The size of mip 0 of the image.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The list of images that form the 'single' image variant. Length must be equal to
        /// 'level_num', and not zero.
        images: Vec<DynamicImageBuffer>,
    },
    /// An array of single images, potentially including a mip chain for each.
    Array {
        /// The size of mip 0 of each image in the array.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The number of images in the image array
        layer_num: u32,

        /// The list of images that form the array. Length must be equal to
        /// 'level_num * layer_num', and not zero.
        images: Vec<DynamicImageBuffer>,
    },
    /// A singular cube map image. This is similar to [`TextureVariant::Array`] that assumes
    /// 'layer_num = 6'.
    Cube {
        /// The size of mip 0 of each cube face.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The list of images that form the 'cube' image variant. Length must be equal to
        /// 'level_num * 6', and not zero.
        images: Vec<DynamicImageBuffer>,
    },
    /// A special case of [`TextureVariant::Array`] that encodes an array of cube maps. Instead we
    /// have 'cube_num' and derive the total image number as 'cube_num * 6'.
    CubeArray {
        /// The size of mip 0 of each cube face in the array.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The number of cube maps in the cube array.
        cube_num: u32,

        /// list of images that form the 'cube array' image variant. Length must be equal to
        /// 'level_num * cube_num * 6', and not zero.
        images: Vec<DynamicImageBuffer>,
    },
}

impl TextureBuffer {
    /// Returns the dimensions of the underlying texture. This encodes the size of mip 0. For cubes
    /// this encodes the size of mip 0 of each face.
    pub fn dimensions(&self) -> UVec2 {
        match self {
            TextureBuffer::Single { dimensions, .. } => *dimensions,
            TextureBuffer::Array { dimensions, .. } => *dimensions,
            TextureBuffer::Cube { dimensions, .. } => *dimensions,
            TextureBuffer::CubeArray { dimensions, .. } => *dimensions,
        }
    }

    pub fn layer_num(&self) -> u32 {
        match self {
            TextureBuffer::Single { .. } => 1,
            TextureBuffer::Array { layer_num, .. } => *layer_num,
            TextureBuffer::Cube { .. } => 6,
            TextureBuffer::CubeArray { cube_num, .. } => *cube_num * 6,
        }
    }

    pub fn level_num(&self) -> u32 {
        match self {
            TextureBuffer::Single { level_num, .. } => *level_num,
            TextureBuffer::Array { level_num, .. } => *level_num,
            TextureBuffer::Cube { level_num, .. } => *level_num,
            TextureBuffer::CubeArray { level_num, .. } => *level_num,
        }
    }

    pub fn images_ref(&self) -> &[DynamicImageBuffer] {
        match self {
            TextureBuffer::Single { images, .. } => images.as_slice(),
            TextureBuffer::Array { images, .. } => images.as_slice(),
            TextureBuffer::Cube { images, .. } => images.as_slice(),
            TextureBuffer::CubeArray { images, .. } => images.as_slice(),
        }
    }

    pub fn images_mut(&mut self) -> &mut [DynamicImageBuffer] {
        match self {
            TextureBuffer::Single { images, .. } => images.as_mut_slice(),
            TextureBuffer::Array { images, .. } => images.as_mut_slice(),
            TextureBuffer::Cube { images, .. } => images.as_mut_slice(),
            TextureBuffer::CubeArray { images, .. } => images.as_mut_slice(),
        }
    }

    pub fn get_color_type(&self) -> ColorType {
        self.validate_image_types();
        let images = self.images_ref();
        images[0].color_type()
    }

    pub fn validate_image_types(&self) {
        let images = self.images_ref();

        let base = images[0].color_type();
        let all_images_same_color_type = images.iter().all(|v| v.color_type() == base);
        assert!(all_images_same_color_type);
    }

    pub fn validate_image_count(&self) {
        assert_ne!(self.level_num(), 0);
        assert_ne!(self.layer_num(), 0);
        let expected_count = self.image_count_with_levels(self.level_num());
        assert_eq!(self.images_ref().len(), expected_count as usize);
    }

    pub fn get_buffer_references(&self) -> Vec<&[u8]> {
        self.validate_image_count();
        self.validate_image_types();

        let buffers = Vec::from_iter(self.images_ref().iter().map(|v| match v {
            DynamicImageBuffer::R8Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RG8Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGB8Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGBA8Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::R16Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RG16Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGB16Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGBA16Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::R32Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RG32Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGB32Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGBA32Unorm(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::R16Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RG16Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGB16Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGBA16Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::R32Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RG32Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGB32Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
            DynamicImageBuffer::RGBA32Float(i) => bytemuck::cast_slice::<_, u8>(i.data()),
        }));
        buffers
    }

    fn image_count_with_levels(&self, level_num: u32) -> u32 {
        match self {
            TextureBuffer::Single { .. } => level_num,
            TextureBuffer::Array { layer_num, .. } => *layer_num * level_num,
            TextureBuffer::Cube { .. } => level_num * 6,
            TextureBuffer::CubeArray {
                level_num,
                cube_num,
                ..
            } => *cube_num * *level_num * 6,
        }
    }

    fn take_images(&mut self) -> Vec<DynamicImageBuffer> {
        match self {
            TextureBuffer::Single { images, .. } => std::mem::take(images),
            TextureBuffer::Array { images, .. } => std::mem::take(images),
            TextureBuffer::Cube { images, .. } => std::mem::take(images),
            TextureBuffer::CubeArray { images, .. } => std::mem::take(images),
        }
    }
}

impl TextureBuffer {
    pub fn generate_mips(&mut self, filter: ResizeFilter) {
        assert_eq!(self.level_num(), 1);
        self.validate_image_count();

        let old_images = self.take_images();
        let layer_num = self.layer_num() as usize;

        let dims = self.dimensions();
        let new_level_num = u32::max(dims.x, dims.y) as f32;
        let new_level_num = new_level_num.log2().floor() + 1.0;
        let new_level_num = new_level_num as u32;
        let new_image_num = self.image_count_with_levels(new_level_num);

        let mut new_images =
            vec![DynamicImageBuffer::R8Unorm(ImageBuffer::new(0, 0)); new_image_num as usize];

        for (i, input_image) in old_images.into_iter().enumerate() {
            let i = calculate_set_index(layer_num, new_level_num as usize, i, 0);
            new_images[i] = input_image;
        }

        for layer in 0..layer_num {
            for level in 1..new_level_num as usize {
                let i = calculate_set_index(layer_num, new_level_num as usize, layer, level - 1);
                let last = &new_images[i];

                let new_width = (last.width() / 2).max(1);
                let new_height = (last.height() / 2).max(1);
                let next = last.resize(new_width, new_height, filter);

                let i = calculate_set_index(layer_num, new_level_num as usize, layer, level);
                new_images[i] = next;
            }
        }

        match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureBuffer::Array {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureBuffer::Cube {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureBuffer::CubeArray {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
        }
    }

    pub fn normalize(&mut self) -> TextureOpResult<()> {
        for image in self.images_mut() {
            normalize_normal_map(image)?;
        }
        Ok(())
    }

    pub fn swizzle_rgb_to_rgba(&mut self) -> TextureOpResult<()> {
        let color_type = self.get_color_type();

        match color_type {
            ColorType::RGB8Unorm => {}
            ColorType::RGB16Unorm => {}
            ColorType::RGB32Unorm => {}
            ColorType::RGB16Float => {}
            ColorType::RGB32Float => {}
            _ => return Err(TextureOpError::InvalidSrcFormat),
        }

        for image in self.images_mut() {
            let new_image = match image {
                DynamicImageBuffer::RGB8Unorm(image) => {
                    let swizzled =
                        swizzle_rgb_to_rgba(image.data(), image.width(), image.height(), 0xFF);
                    let swizzled = ImageBuffer::<PixRGBA<_>>::from_data(
                        image.width(),
                        image.height(),
                        swizzled,
                    );
                    DynamicImageBuffer::RGBA8Unorm(swizzled)
                }
                DynamicImageBuffer::RGB16Unorm(image) => {
                    let swizzled =
                        swizzle_rgb_to_rgba(image.data(), image.width(), image.height(), 0xFFFF);
                    let swizzled = ImageBuffer::<PixRGBA<_>>::from_data(
                        image.width(),
                        image.height(),
                        swizzled,
                    );
                    DynamicImageBuffer::RGBA16Unorm(swizzled)
                }
                DynamicImageBuffer::RGB32Unorm(image) => {
                    let swizzled = swizzle_rgb_to_rgba(
                        image.data(),
                        image.width(),
                        image.height(),
                        0xFFFFFFFF,
                    );
                    let swizzled = ImageBuffer::<PixRGBA<_>>::from_data(
                        image.width(),
                        image.height(),
                        swizzled,
                    );
                    DynamicImageBuffer::RGBA32Unorm(swizzled)
                }
                DynamicImageBuffer::RGB16Float(image) => {
                    let swizzled =
                        swizzle_rgb_to_rgba(image.data(), image.width(), image.height(), f16::ONE);
                    let swizzled = ImageBuffer::<PixRGBA<_>>::from_data(
                        image.width(),
                        image.height(),
                        swizzled,
                    );
                    DynamicImageBuffer::RGBA16Float(swizzled)
                }
                DynamicImageBuffer::RGB32Float(image) => {
                    let swizzled =
                        swizzle_rgb_to_rgba(image.data(), image.width(), image.height(), 1.0);
                    let swizzled = ImageBuffer::<PixRGBA<_>>::from_data(
                        image.width(),
                        image.height(),
                        swizzled,
                    );
                    DynamicImageBuffer::RGBA32Float(swizzled)
                }
                _ => unreachable!(),
            };
            *image = new_image;
        }

        Ok(())
    }

    pub fn equirectangular_to_cube_map(&mut self, face_dimensions: UVec2) -> TextureOpResult<()> {
        let new_self = match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = equi_to_cube_dyn::<FacePosX>(&image, face_dimensions);
                    let nx = equi_to_cube_dyn::<FaceNegX>(&image, face_dimensions);
                    let py = equi_to_cube_dyn::<FacePosY>(&image, face_dimensions);
                    let ny = equi_to_cube_dyn::<FaceNegY>(&image, face_dimensions);
                    let pz = equi_to_cube_dyn::<FacePosZ>(&image, face_dimensions);
                    let nz = equi_to_cube_dyn::<FaceNegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureBuffer::Cube {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Array {
                level_num,
                layer_num,
                images,
                ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = equi_to_cube_dyn::<FacePosX>(&image, face_dimensions);
                    let nx = equi_to_cube_dyn::<FaceNegX>(&image, face_dimensions);
                    let py = equi_to_cube_dyn::<FacePosY>(&image, face_dimensions);
                    let ny = equi_to_cube_dyn::<FaceNegY>(&image, face_dimensions);
                    let pz = equi_to_cube_dyn::<FacePosZ>(&image, face_dimensions);
                    let nz = equi_to_cube_dyn::<FaceNegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureBuffer::CubeArray {
                    dimensions: face_dimensions,
                    cube_num: *layer_num,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Cube { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }

    pub fn cube_map_to_octahedral_map(&mut self, face_dimensions: UVec2) -> TextureOpResult<()> {
        let new_self = match self {
            TextureBuffer::Single { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::Array { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::Cube {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut accessor = CubeToOctAccess {
                    dim: face_dimensions,
                    out: None,
                };
                cube_sampler_access(images, &mut accessor);

                let new_images = vec![accessor.out.unwrap()];
                TextureBuffer::Single {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }

    pub fn cube_map_to_equirectangular_map(
        &mut self,
        face_dimensions: UVec2,
    ) -> TextureOpResult<()> {
        let new_self = match self {
            TextureBuffer::Single { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::Array { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::Cube {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut accessor = CubeToEquiAccess {
                    dim: face_dimensions,
                    out: None,
                };
                cube_sampler_access(images, &mut accessor);

                let new_images = vec![accessor.out.unwrap()];
                TextureBuffer::Single {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }

    pub fn equirectangular_to_octahedral_map(
        &mut self,
        face_dimensions: UVec2,
    ) -> TextureOpResult<()> {
        let new_self = match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let i = equi_to_octahedral_dyn(&image, face_dimensions);
                    new_images.push(i);
                }

                TextureBuffer::Single {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Array {
                level_num,
                layer_num,
                images,
                ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let i = equi_to_octahedral_dyn(&image, face_dimensions);
                    new_images.push(i);
                }

                TextureBuffer::Array {
                    dimensions: face_dimensions,
                    layer_num: *layer_num,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Cube { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }

    pub fn octahedral_to_cube_map(&mut self, face_dimensions: UVec2) -> TextureOpResult<()> {
        let new_self = match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = octahedral_to_cube_dyn::<FacePosX>(&image, face_dimensions);
                    let nx = octahedral_to_cube_dyn::<FaceNegX>(&image, face_dimensions);
                    let py = octahedral_to_cube_dyn::<FacePosY>(&image, face_dimensions);
                    let ny = octahedral_to_cube_dyn::<FaceNegY>(&image, face_dimensions);
                    let pz = octahedral_to_cube_dyn::<FacePosZ>(&image, face_dimensions);
                    let nz = octahedral_to_cube_dyn::<FaceNegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureBuffer::Cube {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Array {
                level_num,
                layer_num,
                images,
                ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = octahedral_to_cube_dyn::<FacePosX>(&image, face_dimensions);
                    let nx = octahedral_to_cube_dyn::<FaceNegX>(&image, face_dimensions);
                    let py = octahedral_to_cube_dyn::<FacePosY>(&image, face_dimensions);
                    let ny = octahedral_to_cube_dyn::<FaceNegY>(&image, face_dimensions);
                    let pz = octahedral_to_cube_dyn::<FacePosZ>(&image, face_dimensions);
                    let nz = octahedral_to_cube_dyn::<FaceNegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureBuffer::CubeArray {
                    dimensions: face_dimensions,
                    cube_num: *layer_num,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureBuffer::Cube { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }
}

fn normalize_normal_map(i: &mut DynamicImageBuffer) -> TextureOpResult<()> {
    match i {
        DynamicImageBuffer::RGB8Unorm(i) => {
            for p in i.data_mut().chunks_exact_mut(3) {
                let x = unorm_u8_to_f32(p[0]);
                let y = unorm_u8_to_f32(p[1]);
                let z = unorm_u8_to_f32(p[2]);

                // Expand the UNORM into the full encoded normal range
                let n = Vec3::new(x, y, z);
                let n = n * Vec3::broadcast(2.0);
                let n = n - Vec3::broadcast(1.0);

                // The actual normalization we want to do
                let n = n.normalized();

                // Map back into packed representation
                let x = (n.x + 1.0) / 2.0;
                let y = (n.y + 1.0) / 2.0;
                let z = (n.z + 1.0) / 2.0;

                p[0] = f32_to_unorm_u8(x);
                p[1] = f32_to_unorm_u8(y);
                p[2] = f32_to_unorm_u8(z);
            }
        }
        DynamicImageBuffer::RGBA8Unorm(i) => {
            for p in i.data_mut().chunks_exact_mut(4) {
                let x = unorm_u8_to_f32(p[0]);
                let y = unorm_u8_to_f32(p[1]);
                let z = unorm_u8_to_f32(p[2]);

                // Expand the UNORM into the full encoded normal range
                let n = Vec3::new(x, y, z);
                let n = n * Vec3::broadcast(2.0);
                let n = n - Vec3::broadcast(1.0);

                // The actual normalization we want to do
                let n = n.normalized();

                // Map back into packed representation
                let x = (n.x + 1.0) / 2.0;
                let y = (n.y + 1.0) / 2.0;
                let z = (n.z + 1.0) / 2.0;

                p[0] = f32_to_unorm_u8(x);
                p[1] = f32_to_unorm_u8(y);
                p[2] = f32_to_unorm_u8(z);
            }
        }
        DynamicImageBuffer::RGB16Unorm(i) => {
            for p in i.data_mut().chunks_exact_mut(3) {
                let x = unorm_u16_to_f32(p[0]);
                let y = unorm_u16_to_f32(p[1]);
                let z = unorm_u16_to_f32(p[2]);

                // Expand the UNORM into the full encoded normal range
                let n = Vec3::new(x, y, z);
                let n = n * Vec3::broadcast(2.0);
                let n = n - Vec3::broadcast(1.0);

                // The actual normalization we want to do
                let n = n.normalized();

                // Map back into UNORM
                let x = (n.x + 1.0) / 2.0;
                let y = (n.y + 1.0) / 2.0;
                let z = (n.z + 1.0) / 2.0;

                p[0] = f32_to_unorm_u16(x);
                p[1] = f32_to_unorm_u16(y);
                p[2] = f32_to_unorm_u16(z);
            }
        }
        DynamicImageBuffer::RGBA16Unorm(i) => {
            for p in i.data_mut().chunks_exact_mut(4) {
                let x = unorm_u16_to_f32(p[0]);
                let y = unorm_u16_to_f32(p[1]);
                let z = unorm_u16_to_f32(p[2]);

                // Expand the UNORM into the full encoded normal range
                let n = Vec3::new(x, y, z);
                let n = n * Vec3::broadcast(2.0);
                let n = n - Vec3::broadcast(1.0);

                // The actual normalization we want to do
                let n = n.normalized();

                // Map back into UNORM
                let x = (n.x + 1.0) / 2.0;
                let y = (n.y + 1.0) / 2.0;
                let z = (n.z + 1.0) / 2.0;

                p[0] = f32_to_unorm_u16(x);
                p[1] = f32_to_unorm_u16(y);
                p[2] = f32_to_unorm_u16(z);
            }
        }
        DynamicImageBuffer::RGB32Float(i) => {
            for p in i.data_mut().chunks_exact_mut(3) {
                let n = Vec3::new(p[0], p[1], p[2]);

                // The actual normalization we want to do
                let n = n.normalized();

                p[0] = n.x;
                p[1] = n.y;
                p[2] = n.z;
            }
        }
        DynamicImageBuffer::RGBA32Float(i) => {
            for p in i.data_mut().chunks_exact_mut(4) {
                let n = Vec3::new(p[0], p[1], p[2]);

                // The actual normalization we want to do
                let n = n.normalized();

                p[0] = n.x;
                p[1] = n.y;
                p[2] = n.z;
            }
        }
        _ => return Err(TextureOpError::InvalidSrcFormat),
    }
    Ok(())
}

fn swizzle_rgb_to_rgba<T: Copy + Clone>(
    src: &[T],
    width: u32,
    height: u32,
    default_value: T,
) -> Vec<T> {
    let mut level: Vec<T> = Vec::new();
    let bytes = width as usize * height as usize * 4;
    level.resize(bytes, default_value);

    let src_row_width = width as usize * 3;
    let dst_row_width = width as usize * 4;

    for row in 0..height as usize {
        let dst_row_start = row * dst_row_width;
        let dst_row_end = dst_row_start + dst_row_width;
        let dst = &mut level[dst_row_start..dst_row_end];

        let src_row_start = row * src_row_width;
        let src_row_end = src_row_start + src_row_width;
        let src = &src[src_row_start..src_row_end];

        for col in 0..width as usize {
            let dst_base = col as usize * 4;
            let dst = &mut dst[dst_base..dst_base + 3];

            let src_base = col as usize * 3;
            let src = &src[src_base..src_base + 3];

            dst.copy_from_slice(src);
        }
    }

    level
}

fn calculate_set_index(layer_num: usize, level_num: usize, layer: usize, level: usize) -> usize {
    assert!(layer < layer_num);
    assert!(level < level_num);

    let i = layer * level_num;
    let i = i + level;
    i
}

trait ICubeSamplerAccess {
    fn access<T: PixelFormat>(&mut self, sampler: &CubeSampler<T>)
    where
        DynamicImageBuffer: From<ImageBuffer<T>>;
}

struct CubeToOctAccess {
    dim: UVec2,
    out: Option<DynamicImageBuffer>,
}

impl ICubeSamplerAccess for CubeToOctAccess {
    fn access<T: PixelFormat>(&mut self, sampler: &CubeSampler<T>)
    where
        DynamicImageBuffer: From<ImageBuffer<T>>,
    {
        let out = cube_to_octahedral(sampler, self.dim);
        self.out = Some(DynamicImageBuffer::from(out));
    }
}

struct CubeToEquiAccess {
    dim: UVec2,
    out: Option<DynamicImageBuffer>,
}

impl ICubeSamplerAccess for CubeToEquiAccess {
    fn access<T: PixelFormat>(&mut self, sampler: &CubeSampler<T>)
    where
        DynamicImageBuffer: From<ImageBuffer<T>>,
    {
        let out = cube_to_equi(sampler, self.dim);
        self.out = Some(DynamicImageBuffer::from(out));
    }
}

fn cube_sampler_access<A: ICubeSamplerAccess>(images: &[DynamicImageBuffer], a: &mut A) {
    assert!(
        images.len() >= 6,
        "At least 6 images are needed to form a texture cube"
    );

    let first = &images[0];

    match first {
        DynamicImageBuffer::R8Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RG8Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGB8Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGBA8Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::R16Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RG16Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGB16Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGBA16Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::R32Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RG32Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGB32Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGBA32Unorm(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::R16Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RG16Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGB16Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGBA16Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::R32Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RG32Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGB32Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
        DynamicImageBuffer::RGBA32Float(i) => {
            let sampler = downcast_rest_to_cube_sampler(i, &images[1..]);
            a.access(&sampler);
        }
    }
}

/// Kind of awful thing we use to construct a statically typed CubeSampler from our array of
/// dynamically typed cube images. This is part of [`cube_sampler_access`], where we use a match
/// to get a concrete type in a dynamic context and then rely on type coercion to deduce what T
/// should be without naming it (as that would suck to type out).
///
/// We _could_ just name the type in [`cube_sampler_access`], but this works too.
fn downcast_rest_to_cube_sampler<'a, T: PixelFormat>(
    first: &'a ImageBuffer<T>,
    images: &'a [DynamicImageBuffer],
) -> CubeSampler<'a, T>
where
    DynamicImageBuffer: DowncastImageBuffer<ImageBuffer<T>>,
{
    CubeSampler::new(
        first,
        images[0].downcast_image_buffer().unwrap(),
        images[1].downcast_image_buffer().unwrap(),
        images[2].downcast_image_buffer().unwrap(),
        images[3].downcast_image_buffer().unwrap(),
        images[4].downcast_image_buffer().unwrap(),
    )
}
