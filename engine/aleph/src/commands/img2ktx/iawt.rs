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

use aleph_ktx::calculate_set_index;
use aleph_math::{UVec2, Vec3};
use anyhow::anyhow;
use image::{imageops, ColorType, DynamicImage, ImageBuffer, Rgba, RgbaImage};

use crate::commands::img2ktx::{
    equi_to_cube_dyn, f32_to_unorm_u16, f32_to_unorm_u8, unorm_u16_to_f32, unorm_u8_to_f32,
    DynamicBuffer, NegX, NegY, NegZ, PosX, PosY, PosZ,
};

/// Type that closes over the types of textures we support working with. This includes the types
/// that we can import as well as export.
///
/// The purpose of this type is to track the type of an image dynamically as it flows through the
/// image conditioning pipeline. We have a number of filters and processes we can run on textures
/// in order to
pub enum TextureVariant {
    /// A single image. This may contain multiple images which define a mip chain.
    Single {
        /// The size of mip 0 of the image.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The list of images that form the 'single' image variant. Length must be equal to
        /// 'level_num', and not zero.
        images: Vec<DynamicImage>,
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
        images: Vec<DynamicImage>,
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
        images: Vec<DynamicImage>,
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
        images: Vec<DynamicImage>,
    },
}

impl TextureVariant {
    /// Returns the dimensions of the underlying texture. This encodes the size of mip 0. For cubes
    /// this encodes the size of mip 0 of each face.
    pub fn dimensions(&self) -> UVec2 {
        match self {
            TextureVariant::Single { dimensions, .. } => *dimensions,
            TextureVariant::Array { dimensions, .. } => *dimensions,
            TextureVariant::Cube { dimensions, .. } => *dimensions,
            TextureVariant::CubeArray { dimensions, .. } => *dimensions,
        }
    }

    pub fn layer_num(&self) -> u32 {
        match self {
            TextureVariant::Single { .. } => 1,
            TextureVariant::Array { layer_num, .. } => *layer_num,
            TextureVariant::Cube { .. } => 6,
            TextureVariant::CubeArray { cube_num, .. } => *cube_num * 6,
        }
    }

    pub fn level_num(&self) -> u32 {
        match self {
            TextureVariant::Single { level_num, .. } => *level_num,
            TextureVariant::Array { level_num, .. } => *level_num,
            TextureVariant::Cube { level_num, .. } => *level_num,
            TextureVariant::CubeArray { level_num, .. } => *level_num,
        }
    }

    pub fn images_ref(&self) -> &[DynamicImage] {
        match self {
            TextureVariant::Single { images, .. } => images.as_slice(),
            TextureVariant::Array { images, .. } => images.as_slice(),
            TextureVariant::Cube { images, .. } => images.as_slice(),
            TextureVariant::CubeArray { images, .. } => images.as_slice(),
        }
    }

    pub fn images_mut(&mut self) -> &mut [DynamicImage] {
        match self {
            TextureVariant::Single { images, .. } => images.as_mut_slice(),
            TextureVariant::Array { images, .. } => images.as_mut_slice(),
            TextureVariant::Cube { images, .. } => images.as_mut_slice(),
            TextureVariant::CubeArray { images, .. } => images.as_mut_slice(),
        }
    }

    pub fn get_color_type(&self) -> ColorType {
        self.validate_image_types();
        let images = self.images_ref();
        images[0].color()
    }

    pub fn validate_image_types(&self) {
        let images = self.images_ref();

        let base = images[0].color();
        let all_images_same_color_type = images.iter().all(|v| v.color() == base);
        assert!(all_images_same_color_type);
    }

    pub fn validate_image_count(&self) {
        assert_ne!(self.level_num(), 0);
        assert_ne!(self.layer_num(), 0);
        let expected_count = self.image_count_with_levels(self.level_num());
        assert_eq!(self.images_ref().len(), expected_count as usize);
    }

    pub fn take_images_as_buffers(&mut self) -> Vec<DynamicBuffer> {
        self.validate_image_count();
        self.validate_image_types();

        let buffers = Vec::from_iter(self.take_images().into_iter().map(|v| match v {
            DynamicImage::ImageLuma8(v) => DynamicBuffer::U8(v.into_raw()),
            DynamicImage::ImageLumaA8(v) => DynamicBuffer::U8(v.into_raw()),
            DynamicImage::ImageRgb8(v) => DynamicBuffer::U8(v.into_raw()),
            DynamicImage::ImageRgba8(v) => DynamicBuffer::U8(v.into_raw()),
            DynamicImage::ImageLuma16(v) => DynamicBuffer::U16(v.into_raw()),
            DynamicImage::ImageLumaA16(v) => DynamicBuffer::U16(v.into_raw()),
            DynamicImage::ImageRgb16(v) => DynamicBuffer::U16(v.into_raw()),
            DynamicImage::ImageRgba16(v) => DynamicBuffer::U16(v.into_raw()),
            DynamicImage::ImageRgb32F(v) => DynamicBuffer::F32(v.into_raw()),
            DynamicImage::ImageRgba32F(v) => DynamicBuffer::F32(v.into_raw()),
            _ => unimplemented!(),
        }));
        buffers
    }

    fn image_count_with_levels(&self, level_num: u32) -> u32 {
        match self {
            TextureVariant::Single { .. } => level_num,
            TextureVariant::Array { layer_num, .. } => *layer_num * level_num,
            TextureVariant::Cube { .. } => level_num * 6,
            TextureVariant::CubeArray {
                level_num,
                cube_num,
                ..
            } => *cube_num * *level_num * 6,
        }
    }

    fn take_images(&mut self) -> Vec<DynamicImage> {
        match self {
            TextureVariant::Single { images, .. } => std::mem::take(images),
            TextureVariant::Array { images, .. } => std::mem::take(images),
            TextureVariant::Cube { images, .. } => std::mem::take(images),
            TextureVariant::CubeArray { images, .. } => std::mem::take(images),
        }
    }
}

impl TextureVariant {
    pub fn generate_mips(&mut self, filter: imageops::FilterType) {
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
            vec![DynamicImage::ImageLuma8(ImageBuffer::new(0, 0)); new_image_num as usize];

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
                let next = last.resize_exact(new_width, new_height, filter);

                let i = calculate_set_index(layer_num, new_level_num as usize, layer, level);
                new_images[i] = next;
            }
        }

        match self {
            TextureVariant::Single {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureVariant::Array {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureVariant::Cube {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
            TextureVariant::CubeArray {
                level_num, images, ..
            } => {
                *images = new_images;
                *level_num = new_level_num;
            }
        }
    }

    pub fn normalize(&mut self) -> anyhow::Result<()> {
        for image in self.images_mut() {
            normalize_normal_map(image)?;
        }
        Ok(())
    }

    pub fn swizzle_rgb_to_rgba(&mut self) -> anyhow::Result<()> {
        let color_type = self.get_color_type();

        match color_type {
            ColorType::L8 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::La8 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::Rgb8 => {}
            ColorType::Rgba8 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::L16 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::La16 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::Rgb16 => {}
            ColorType::Rgba16 => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            ColorType::Rgb32F => {}
            ColorType::Rgba32F => return Err(anyhow!("'{:?}' is not an RGB format!", color_type)),
            _ => unimplemented!(),
        }

        for image in self.images_mut() {
            let new_image = match image {
                DynamicImage::ImageRgb8(image) => {
                    let swizzled = swizzle_rgb_to_rgba(
                        image.as_raw().as_slice(),
                        image.width(),
                        image.height(),
                        0xFF,
                    );
                    let swizzled =
                        RgbaImage::from_vec(image.width(), image.height(), swizzled).unwrap();
                    DynamicImage::ImageRgba8(swizzled)
                }
                DynamicImage::ImageRgb16(image) => {
                    let swizzled = swizzle_rgb_to_rgba(
                        image.as_raw().as_slice(),
                        image.width(),
                        image.height(),
                        0xFF,
                    );
                    let swizzled: ImageBuffer<Rgba<u16>, Vec<u16>> =
                        ImageBuffer::from_vec(image.width(), image.height(), swizzled).unwrap();
                    DynamicImage::ImageRgba16(swizzled)
                }
                DynamicImage::ImageRgb32F(image) => {
                    let swizzled = swizzle_rgb_to_rgba(
                        image.as_raw().as_slice(),
                        image.width(),
                        image.height(),
                        1.0f32,
                    );

                    let swizzled: ImageBuffer<Rgba<f32>, Vec<f32>> =
                        ImageBuffer::from_vec(image.width(), image.height(), swizzled).unwrap();
                    DynamicImage::ImageRgba32F(swizzled)
                }
                _ => unimplemented!(),
            };
            *image = new_image;
        }

        Ok(())
    }

    pub fn equirectangular_to_cube_map(&mut self, face_dimensions: UVec2) -> anyhow::Result<()> {
        let new_self = match self {
            TextureVariant::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(anyhow!(
                        "Can't perform equirectangular conversion on an image with mips!"
                    ));
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = equi_to_cube_dyn::<PosX>(&image, face_dimensions);
                    let nx = equi_to_cube_dyn::<NegX>(&image, face_dimensions);
                    let py = equi_to_cube_dyn::<PosY>(&image, face_dimensions);
                    let ny = equi_to_cube_dyn::<NegY>(&image, face_dimensions);
                    let pz = equi_to_cube_dyn::<PosZ>(&image, face_dimensions);
                    let nz = equi_to_cube_dyn::<NegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureVariant::Cube {
                    dimensions: face_dimensions,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureVariant::Array {
                level_num,
                layer_num,
                images,
                ..
            } => {
                if *level_num > 1 {
                    return Err(anyhow!(
                        "Can't perform equirectangular conversion on an image with mips!"
                    ));
                }

                let mut new_images = Vec::new();
                for image in images.drain(..) {
                    let px = equi_to_cube_dyn::<PosX>(&image, face_dimensions);
                    let nx = equi_to_cube_dyn::<NegX>(&image, face_dimensions);
                    let py = equi_to_cube_dyn::<PosY>(&image, face_dimensions);
                    let ny = equi_to_cube_dyn::<NegY>(&image, face_dimensions);
                    let pz = equi_to_cube_dyn::<PosZ>(&image, face_dimensions);
                    let nz = equi_to_cube_dyn::<NegZ>(&image, face_dimensions);
                    new_images.push(px);
                    new_images.push(nx);
                    new_images.push(py);
                    new_images.push(ny);
                    new_images.push(pz);
                    new_images.push(nz);
                }

                TextureVariant::CubeArray {
                    dimensions: face_dimensions,
                    cube_num: *layer_num,
                    level_num: 1,
                    images: new_images,
                }
            }
            TextureVariant::Cube { .. } => {
                return Err(anyhow!(
                    "Can't perform equirectangular conversion on a cubemap input!"
                ));
            }
            TextureVariant::CubeArray { .. } => {
                return Err(anyhow!(
                    "Can't perform equirectangular conversion on a cubemap input!"
                ));
            }
        };

        new_self.validate_image_count();
        new_self.validate_image_types();

        *self = new_self;

        Ok(())
    }
}

fn normalize_normal_map(i: &mut DynamicImage) -> anyhow::Result<()> {
    match i {
        DynamicImage::ImageLuma8(_i) => {
            return Err(anyhow!("Need at least 3 channels for a normal map. Got 1"))
        }
        DynamicImage::ImageLumaA8(_i) => {
            return Err(anyhow!("Need at least 3 channels for a normal map. Got 2"))
        }
        DynamicImage::ImageRgb8(i) => {
            for p in i.pixels_mut() {
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
        DynamicImage::ImageRgba8(i) => {
            for p in i.pixels_mut() {
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
        DynamicImage::ImageLuma16(_i) => {
            return Err(anyhow!("Need at least 3 channels for a normal map. Got 1"))
        }
        DynamicImage::ImageLumaA16(_i) => {
            return Err(anyhow!("Need at least 3 channels for a normal map. Got 2"))
        }
        DynamicImage::ImageRgb16(i) => {
            for p in i.pixels_mut() {
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
        DynamicImage::ImageRgba16(i) => {
            for p in i.pixels_mut() {
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
        DynamicImage::ImageRgb32F(i) => {
            for p in i.pixels_mut() {
                let n = Vec3::new(p[0], p[1], p[2]);

                // The actual normalization we want to do
                let n = n.normalized();

                p[0] = n.x;
                p[1] = n.y;
                p[2] = n.z;
            }
        }
        DynamicImage::ImageRgba32F(i) => {
            for p in i.pixels_mut() {
                let n = Vec3::new(p[0], p[1], p[2]);

                // The actual normalization we want to do
                let n = n.normalized();

                p[0] = n.x;
                p[1] = n.y;
                p[2] = n.z;
            }
        }
        _ => return Err(anyhow!("Unknown Pixel Format")),
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
