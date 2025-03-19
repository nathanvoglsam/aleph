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

mod dynamic_texture_buffer;

pub use dynamic_texture_buffer::DynamicTextureBuffer;

use aleph_math::{UVec2, Vec3};
use half::f16;

use crate::{
    image_to_equi, image_to_octahedral, image_to_whole_cube, integrate_irradiance_to_equi,
    integrate_irradiance_to_octahedral, integrate_irradiance_to_whole_cube, CubeSampler,
    EnvironmentMapProjection, EquirectangularDirectionalSampler, IPixelAccess, IPixelStorage,
    IResizeImage, ImageBuffer, OctahderalDirectionalSampler, PixR, PixRG, PixRGB, PixRGBA,
    PixelChannelType, PixelFormat, ResizeFilter, SphericalMapping, TextureOpError, TextureOpResult,
};

/// Type that closes over the types of textures we support working with. This includes the types
/// that we can import as well as export.
///
/// The purpose of this type is to track the type of an image dynamically as it flows through the
/// image conditioning pipeline. We have a number of filters and processes we can run on textures
/// in order to
#[derive(Clone)]
pub enum TextureBuffer<T: PixelFormat> {
    /// A single image. This may contain multiple images which define a mip chain.
    Single {
        /// The size of mip 0 of the image.
        dimensions: UVec2,

        /// The number of mip levels the texture should have.
        level_num: u32,

        /// The list of images that form the 'single' image variant. Length must be equal to
        /// 'level_num', and not zero.
        images: Vec<ImageBuffer<T>>,
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
        images: Vec<ImageBuffer<T>>,
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
        images: Vec<ImageBuffer<T>>,
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
        images: Vec<ImageBuffer<T>>,
    },
}

impl<T: PixelFormat> TextureBuffer<T> {
    /// Returns the dimensions of the underlying texture. This encodes the size of mip 0. For cubes
    /// this encodes the size of mip 0 of each face.
    pub const fn dimensions(&self) -> UVec2 {
        match self {
            TextureBuffer::Single { dimensions, .. } => *dimensions,
            TextureBuffer::Array { dimensions, .. } => *dimensions,
            TextureBuffer::Cube { dimensions, .. } => *dimensions,
            TextureBuffer::CubeArray { dimensions, .. } => *dimensions,
        }
    }

    pub const fn layer_num(&self) -> u32 {
        match self {
            TextureBuffer::Single { .. } => 1,
            TextureBuffer::Array { layer_num, .. } => *layer_num,
            TextureBuffer::Cube { .. } => 6,
            TextureBuffer::CubeArray { cube_num, .. } => *cube_num * 6,
        }
    }

    pub const fn level_num(&self) -> u32 {
        match self {
            TextureBuffer::Single { level_num, .. } => *level_num,
            TextureBuffer::Array { level_num, .. } => *level_num,
            TextureBuffer::Cube { level_num, .. } => *level_num,
            TextureBuffer::CubeArray { level_num, .. } => *level_num,
        }
    }

    pub fn images_ref(&self) -> &[ImageBuffer<T>] {
        match self {
            TextureBuffer::Single { images, .. } => images.as_slice(),
            TextureBuffer::Array { images, .. } => images.as_slice(),
            TextureBuffer::Cube { images, .. } => images.as_slice(),
            TextureBuffer::CubeArray { images, .. } => images.as_slice(),
        }
    }

    pub fn images_mut(&mut self) -> &mut [ImageBuffer<T>] {
        match self {
            TextureBuffer::Single { images, .. } => images.as_mut_slice(),
            TextureBuffer::Array { images, .. } => images.as_mut_slice(),
            TextureBuffer::Cube { images, .. } => images.as_mut_slice(),
            TextureBuffer::CubeArray { images, .. } => images.as_mut_slice(),
        }
    }

    pub const fn get_texture_type(&self) -> TextureType {
        match self {
            TextureBuffer::Single { .. } => TextureType::Single,
            TextureBuffer::Array { .. } => TextureType::Array,
            TextureBuffer::Cube { .. } => TextureType::Cube,
            TextureBuffer::CubeArray { .. } => TextureType::CubeArray,
        }
    }

    pub fn validate_image_count(&self) {
        assert_ne!(self.level_num(), 0);
        assert_ne!(self.layer_num(), 0);
        let expected_count = self.image_count_with_levels(self.level_num());
        assert_eq!(self.images_ref().len(), expected_count as usize);
    }

    pub fn get_buffer_references(&self) -> Vec<&[u8]> {
        self.validate_image_count();

        let buffers = Vec::from_iter(
            self.images_ref()
                .iter()
                .map(|v| bytemuck::cast_slice::<_, u8>(v.data())),
        );
        buffers
    }

    const fn image_count_with_levels(&self, level_num: u32) -> u32 {
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

    fn take_images(&mut self) -> Vec<ImageBuffer<T>> {
        match self {
            TextureBuffer::Single { images, .. } => std::mem::take(images),
            TextureBuffer::Array { images, .. } => std::mem::take(images),
            TextureBuffer::Cube { images, .. } => std::mem::take(images),
            TextureBuffer::CubeArray { images, .. } => std::mem::take(images),
        }
    }
}

impl<T> TextureBuffer<T>
where
    T: PixelFormat,
    ImageBuffer<T>: IResizeImage,
{
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

        let mut new_images = vec![ImageBuffer::new(0, 0); new_image_num as usize];

        for (i, input_image) in old_images.into_iter().enumerate() {
            let i = set_index_for_layer_and_level(layer_num, new_level_num as usize, i, 0);
            new_images[i] = input_image;
        }

        for layer in 0..layer_num {
            for level in 1..new_level_num as usize {
                let i = set_index_for_layer_and_level(
                    layer_num,
                    new_level_num as usize,
                    layer,
                    level - 1,
                );
                let last = &new_images[i];

                let new_width = (last.width() / 2).max(1);
                let new_height = (last.height() / 2).max(1);
                let next = last.resize(new_width, new_height, filter);

                let i =
                    set_index_for_layer_and_level(layer_num, new_level_num as usize, layer, level);
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
}

impl<C> TextureBuffer<PixRGB<C>>
where
    C: PixelChannelType,
{
    /// Assuming that the input image is a 3-channel UNORM packed normal map, this will perform
    /// an in-place renormalization step to ensure the stored normals are of unit length.
    ///
    /// This is important if mip maps of an input normal map were generated as the downsample filter
    /// will not maintain a normalized result.
    pub fn normalize(&mut self) {
        for image in self.images_mut() {
            image.filter_pixels_mut(|_pos, p| {
                let n = p.as_vec4().truncated();

                // The image we've just loaded is compressed into the [0,1] range, we need to unpack
                // it back into the [-1,1] range
                let n = n * Vec3::broadcast(2.0);
                let n = n - Vec3::broadcast(1.0);

                // Perform the renormalization and pack back into the [0,1] range
                let n = n.normalized();
                let n = n + Vec3::broadcast(1.0);
                let n = n / Vec3::broadcast(2.0);

                // Pack back into a pixel (unorm conversion)
                let x = C::from_float(n.x);
                let y = C::from_float(n.y);
                let z = C::from_float(n.z);
                PixRGB([x, y, z])
            });
        }
    }
}

impl<C, P> TextureBuffer<P>
where
    C: PixelChannelType,
    P: PixelFormat<Storage = C>,
{
    pub fn swizzle_rgb_to_rgba(&mut self, fill: C) -> TextureOpResult<TextureBuffer<PixRGBA<C>>> {
        let mut new_images = Vec::new();
        for image in self.images_ref() {
            let mut new = ImageBuffer::<PixRGBA<C>>::new(self.dimensions().x, self.dimensions().y);
            new.filter_pixels_mut(|pos, _| {
                let p: P = image.load(pos.x, pos.y);

                let mut new = [fill, fill, fill, fill];
                p.write_at(&mut new);

                PixRGBA(new)
            });
            new_images.push(new);
        }

        let out = match self {
            TextureBuffer::Single {
                dimensions,
                level_num,
                ..
            } => TextureBuffer::Single {
                dimensions: *dimensions,
                level_num: *level_num,
                images: new_images,
            },
            TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                ..
            } => TextureBuffer::Array {
                dimensions: *dimensions,
                level_num: *level_num,
                layer_num: *layer_num,
                images: new_images,
            },
            TextureBuffer::Cube {
                dimensions,
                level_num,
                ..
            } => TextureBuffer::Cube {
                dimensions: *dimensions,
                level_num: *level_num,
                images: new_images,
            },
            TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                ..
            } => TextureBuffer::CubeArray {
                dimensions: *dimensions,
                level_num: *level_num,
                cube_num: *cube_num,
                images: new_images,
            },
        };

        Ok(out)
    }
}

impl<T: PixelFormat> TextureBuffer<T> {
    pub fn integrate_irradiance(
        &mut self,
        src_mapping_2d: SphericalMapping,
        dst_mapping: EnvironmentMapProjection,
        face_dimensions: UVec2,
        samples: u32,
    ) -> TextureOpResult<TextureBuffer<T>> {
        fn integrate_2d_src<TT: PixelFormat>(
            dst: &mut Vec<ImageBuffer<TT>>,
            images: &[ImageBuffer<TT>],
            src_mapping: EnvironmentMapProjection,
            dst_mapping: EnvironmentMapProjection,
            face_dimensions: UVec2,
            samples: u32,
        ) {
            match dst_mapping {
                EnvironmentMapProjection::Equirectangular => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            let m = integrate_irradiance_to_equi(&src, face_dimensions, samples);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            let m = integrate_irradiance_to_equi(&src, face_dimensions, samples);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        let m = integrate_irradiance_to_equi(&src, face_dimensions, samples);
                        dst.push(m);
                    }
                },
                EnvironmentMapProjection::Octahedral => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            let m =
                                integrate_irradiance_to_octahedral(&src, face_dimensions, samples);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            let m =
                                integrate_irradiance_to_octahedral(&src, face_dimensions, samples);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        let m = integrate_irradiance_to_octahedral(&src, face_dimensions, samples);
                        dst.push(m);
                    }
                },
                EnvironmentMapProjection::Cube => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            integrate_irradiance_to_whole_cube(dst, &src, face_dimensions, samples);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            integrate_irradiance_to_whole_cube(dst, &src, face_dimensions, samples);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        integrate_irradiance_to_whole_cube(dst, &src, face_dimensions, samples);
                    }
                },
            }
        }
        let new_self = match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                integrate_2d_src(
                    &mut new_images,
                    images,
                    src_mapping_2d.into(),
                    dst_mapping,
                    face_dimensions,
                    samples,
                );

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::Cube {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
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

                for bundle in images.chunks_exact(1) {
                    integrate_2d_src(
                        &mut new_images,
                        bundle,
                        src_mapping_2d.into(),
                        dst_mapping,
                        face_dimensions,
                        samples,
                    );
                }

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Array {
                        dimensions: face_dimensions,
                        layer_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Array {
                        dimensions: face_dimensions,
                        layer_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::CubeArray {
                        dimensions: face_dimensions,
                        cube_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                }
            }
            TextureBuffer::Cube {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();

                for bundle in images.chunks_exact(6) {
                    integrate_2d_src(
                        &mut new_images,
                        bundle,
                        EnvironmentMapProjection::Cube,
                        dst_mapping,
                        face_dimensions,
                        samples,
                    );
                }

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::Cube {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                }
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();

        Ok(new_self)
    }

    pub fn reproject_environment_map(
        &mut self,
        src_mapping_2d: SphericalMapping,
        dst_mapping: EnvironmentMapProjection,
        face_dimensions: UVec2,
    ) -> TextureOpResult<TextureBuffer<T>> {
        fn reproject_2d_src<TT: PixelFormat>(
            dst: &mut Vec<ImageBuffer<TT>>,
            images: &[ImageBuffer<TT>],
            src_mapping: EnvironmentMapProjection,
            dst_mapping: EnvironmentMapProjection,
            face_dimensions: UVec2,
        ) {
            match dst_mapping {
                EnvironmentMapProjection::Equirectangular => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            let m = image_to_equi(&src, face_dimensions);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            let m = image_to_equi(&src, face_dimensions);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        let m = image_to_equi(&src, face_dimensions);
                        dst.push(m);
                    }
                },
                EnvironmentMapProjection::Octahedral => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            let m = image_to_octahedral(&src, face_dimensions);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            let m = image_to_octahedral(&src, face_dimensions);
                            dst.push(m);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        let m = image_to_octahedral(&src, face_dimensions);
                        dst.push(m);
                    }
                },
                EnvironmentMapProjection::Cube => match src_mapping {
                    EnvironmentMapProjection::Equirectangular => {
                        for image in images {
                            let src = EquirectangularDirectionalSampler(image);
                            image_to_whole_cube(dst, &src, face_dimensions);
                        }
                    }
                    EnvironmentMapProjection::Octahedral => {
                        for image in images {
                            let src = OctahderalDirectionalSampler(image);
                            image_to_whole_cube(dst, &src, face_dimensions);
                        }
                    }
                    EnvironmentMapProjection::Cube => {
                        let src = CubeSampler::new_from_slice(images);
                        image_to_whole_cube(dst, &src, face_dimensions);
                    }
                },
            }
        }
        let new_self = match self {
            TextureBuffer::Single {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();
                reproject_2d_src(
                    &mut new_images,
                    images,
                    src_mapping_2d.into(),
                    dst_mapping,
                    face_dimensions,
                );

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::Cube {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
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

                for bundle in images.chunks_exact(1) {
                    reproject_2d_src(
                        &mut new_images,
                        bundle,
                        src_mapping_2d.into(),
                        dst_mapping,
                        face_dimensions,
                    );
                }

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Array {
                        dimensions: face_dimensions,
                        layer_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Array {
                        dimensions: face_dimensions,
                        layer_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::CubeArray {
                        dimensions: face_dimensions,
                        cube_num: *layer_num,
                        level_num: 1,
                        images: new_images,
                    },
                }
            }
            TextureBuffer::Cube {
                level_num, images, ..
            } => {
                if *level_num > 1 {
                    return Err(TextureOpError::InvalidSrcType);
                }

                let mut new_images = Vec::new();

                for bundle in images.chunks_exact(6) {
                    reproject_2d_src(
                        &mut new_images,
                        bundle,
                        EnvironmentMapProjection::Cube,
                        dst_mapping,
                        face_dimensions,
                    );
                }

                match dst_mapping {
                    EnvironmentMapProjection::Equirectangular => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Octahedral => TextureBuffer::Single {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                    EnvironmentMapProjection::Cube => TextureBuffer::Cube {
                        dimensions: face_dimensions,
                        level_num: 1,
                        images: new_images,
                    },
                }
            }
            TextureBuffer::CubeArray { .. } => {
                return Err(TextureOpError::InvalidSrcType);
            }
        };

        new_self.validate_image_count();

        Ok(new_self)
    }

    pub fn to_little_endian(&mut self) {
        match self {
            TextureBuffer::Single { images, .. } => {
                images.iter_mut().for_each(|v| v.to_little_endian());
            }
            TextureBuffer::Array { images, .. } => {
                images.iter_mut().for_each(|v| v.to_little_endian());
            }
            TextureBuffer::Cube { images, .. } => {
                images.iter_mut().for_each(|v| v.to_little_endian());
            }
            TextureBuffer::CubeArray { images, .. } => {
                images.iter_mut().for_each(|v| v.to_little_endian());
            }
        }
    }
}

impl<P: PixelChannelType> TextureBuffer<PixR<P>> {
    pub fn to_half(&self) -> TextureBuffer<PixR<f16>> {
        match self {
            TextureBuffer::Single {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Single {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Array {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    layer_num: *layer_num,
                    images: new,
                }
            }
            TextureBuffer::Cube {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Cube {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::CubeArray {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    cube_num: *cube_num,
                    images: new,
                }
            }
        }
    }
}

impl<P: PixelChannelType> TextureBuffer<PixRG<P>> {
    pub fn to_half(&self) -> TextureBuffer<PixRG<f16>> {
        match self {
            TextureBuffer::Single {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Single {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Array {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    layer_num: *layer_num,
                    images: new,
                }
            }
            TextureBuffer::Cube {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Cube {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::CubeArray {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    cube_num: *cube_num,
                    images: new,
                }
            }
        }
    }
}

impl<P: PixelChannelType> TextureBuffer<PixRGB<P>> {
    pub fn to_half(&self) -> TextureBuffer<PixRGB<f16>> {
        match self {
            TextureBuffer::Single {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Single {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Array {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    layer_num: *layer_num,
                    images: new,
                }
            }
            TextureBuffer::Cube {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Cube {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::CubeArray {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    cube_num: *cube_num,
                    images: new,
                }
            }
        }
    }
}

impl<P: PixelChannelType> TextureBuffer<PixRGBA<P>> {
    pub fn to_half(&self) -> TextureBuffer<PixRGBA<f16>> {
        match self {
            TextureBuffer::Single {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Single {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Array {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    layer_num: *layer_num,
                    images: new,
                }
            }
            TextureBuffer::Cube {
                dimensions,
                level_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::Cube {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    images: new,
                }
            }
            TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                images,
            } => {
                let new = Vec::from_iter(images.iter().map(|v| v.to_half()));
                TextureBuffer::CubeArray {
                    dimensions: *dimensions,
                    level_num: *level_num,
                    cube_num: *cube_num,
                    images: new,
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TextureType {
    Single,
    Array,
    Cube,
    CubeArray,
}

/// Calculates the index for an image, assuming some image set with 'layer_num' and 'level_num'
/// images.
pub const fn set_index_for_layer_and_level(
    layer_num: usize,
    level_num: usize,
    layer: usize,
    level: usize,
) -> usize {
    assert!(layer < layer_num);
    assert!(level < level_num);

    let i = layer * level_num;
    let i = i + level;
    i
}

/// Calculates the layer and level index of an image, assuming some image set with 'layer_num' and
/// 'level_num' images. This is the inverse of [`calculate_set_index`].
///
/// Returns (layer, level)
pub const fn layer_and_level_from_set_index(
    layer_num: usize,
    level_num: usize,
    i: usize,
) -> (usize, usize) {
    let max_images = layer_num * level_num;
    assert!(i < max_images);

    let layer = i / level_num;
    let level = i % level_num;
    (layer, level)
}

#[cfg(test)]
mod tests {
    use crate::{layer_and_level_from_set_index, set_index_for_layer_and_level};

    #[test]
    fn set_mapper_associates() {
        fn test_for(layer_num: usize, level_num: usize) {
            for layer in 0..layer_num {
                for level in 0..level_num {
                    let i = set_index_for_layer_and_level(layer_num, level_num, layer, level);
                    let (i_layer, i_level) =
                        layer_and_level_from_set_index(layer_num, level_num, i);
                    assert_eq!(layer, i_layer);
                    assert_eq!(level, i_level);
                }
            }
        }

        for layer_num in 1..=128 {
            for level_num in 1..=128 {
                test_for(layer_num, level_num);
            }
        }
    }
}
