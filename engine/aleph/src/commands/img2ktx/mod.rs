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

use std::io::BufWriter;

use aleph_image::{
    ColorType, DowncastImageBuffer, DynamicImageBuffer, DynamicTextureBuffer, IPixelStorage,
    ImageBuffer, PixelFormat, TextureBuffer, TextureType,
};
use aleph_ktx::{KtxDocumentDescription, VkFormat};
use anyhow::anyhow;
use camino::Utf8Path;
use clap::parser::Values;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct Image2Ktx;

impl ISubcommand for Image2Ktx {
    fn name(&self) -> &'static str {
        "img2ktx"
    }

    fn description(&mut self) -> Command {
        let input = Arg::new("input")
            .num_args(1..)
            .short('i')
            .long("input")
            .help("The input file.")
            .long_help("The input file. Supports png, jpg, bmp, jpeg, tga, tiff.")
            .required(true);
        let output = Arg::new("output")
            .short('o')
            .long("output")
            .help("The output file.")
            .long_help(
                "The output file. If unspecified the filename is derived from the input name.",
            )
            .required(false);
        let is_cube = Arg::new("is-cube")
            .action(ArgAction::SetTrue)
            .long("is-cube")
            .help("Whether the input image set describes a cube map.")
            .long_help("Whether the input image set describes a cube map. Must provide six images, ordered by +X, -X, +Y, -Y, +Z, -Z");
        Command::new(self.name())
            .about("Converts the given input image into the KTX2 format")
            .arg(input)
            .arg(output)
            .arg(is_cube)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let input_files: Values<String> = matches.remove_many("input").expect("input is required");
        let input_files = Vec::from_iter(input_files.map(|v| Utf8Path::new(&v).to_path_buf()));

        if input_files.is_empty() {
            return Err(anyhow!("No input files given!"));
        }

        let output_arg: Option<String> = matches.remove_one("output");
        let output = match output_arg {
            Some(v) => Utf8Path::new(&v).to_path_buf(),
            None => {
                // Take the name of the first input file
                input_files[0].with_extension("ktx2")
            }
        };

        let is_cube = matches.get_flag("is-cube");

        // Make sure we have enough input images to encode a cubemap(array)
        if is_cube {
            if input_files.len() % 6 != 0 {
                return Err(anyhow!(
                    "Image count must be a multiple of 6 for cube maps! Got '{}'",
                    input_files.len()
                ));
            }
        }

        // Check if we're trying to create a texture array
        let is_array = {
            if is_cube {
                // We've already checked the input set is a multiple of 6
                input_files.len() > 6
            } else {
                // We are _not_ making a cube so just check if we have multiple input images
                input_files.len() > 1
            }
        };

        let mut loaded_files = Vec::new();
        for input in input_files.iter() {
            let loaded = image::ImageReader::open(&input)?
                .with_guessed_format()?
                .decode()?;
            let loaded = DynamicImageBuffer::from_image(loaded);
            loaded_files.push(loaded);
        }

        // Validate that the input dimensions and color format match across all formats
        for this_image in loaded_files.iter() {
            let base_image = &loaded_files[0];

            if base_image.dimensions() != this_image.dimensions() {
                return Err(anyhow!("All input image dimensions must match!"));
            }
            if base_image.color_type() != this_image.color_type() {
                return Err(anyhow!("All input image pixel formats must match!"));
            }
        }

        let mut images = load_images_to_texture(loaded_files, is_cube, is_array);

        images.validate_image_count();
        images.to_little_endian()?;

        // Setup mip state in common code to keep the match arms shorter
        let mut ktx = KtxDocumentDescription::new();

        match images.get_color_type() {
            ColorType::R8Unorm => ktx.format(VkFormat::R8_UNORM),
            ColorType::RG8Unorm => ktx.format(VkFormat::R8G8_UNORM),
            ColorType::RGB8Unorm => ktx.format(VkFormat::R8G8B8_UNORM),
            ColorType::RGBA8Unorm => ktx.format(VkFormat::R8G8B8A8_UNORM),
            ColorType::R16Unorm => ktx.format(VkFormat::R16_UNORM),
            ColorType::RG16Unorm => ktx.format(VkFormat::R16G16_UNORM),
            ColorType::RGB16Unorm => ktx.format(VkFormat::R16G16B16_UNORM),
            ColorType::RGBA16Unorm => ktx.format(VkFormat::R16G16B16A16_UNORM),
            ColorType::R32Unorm => unimplemented!(),
            ColorType::RG32Unorm => unimplemented!(),
            ColorType::RGB32Unorm => unimplemented!(),
            ColorType::RGBA32Unorm => unimplemented!(),
            ColorType::R16Float => ktx.format(VkFormat::R16_SFLOAT),
            ColorType::RG16Float => ktx.format(VkFormat::R16G16_SFLOAT),
            ColorType::RGB16Float => ktx.format(VkFormat::R16G16B16_SFLOAT),
            ColorType::RGBA16Float => ktx.format(VkFormat::R16G16B16A16_SFLOAT),
            ColorType::R32Float => ktx.format(VkFormat::R32_SFLOAT),
            ColorType::RG32Float => ktx.format(VkFormat::R32G32_SFLOAT),
            ColorType::RGB32Float => ktx.format(VkFormat::R32G32B32_SFLOAT),
            ColorType::RGBA32Float => ktx.format(VkFormat::R32G32B32A32_SFLOAT),
        };

        // If we've converted from an equirectangular map to a cube map then we need to change the
        // output resolution to the chosen cube face dimensions instead of the source image
        // dimensions
        let image_references = images.get_buffer_references();
        match images.get_texture_type() {
            TextureType::Single => {
                let dimensions = images.dimensions();
                let level_num = images.level_num();

                log::info!("Writing Image");
                ktx.image_2d(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureType::Array => {
                let dimensions = images.dimensions();
                let layer_num = images.layer_num();
                let level_num = images.level_num();

                log::info!("Writing Image Array with '{layer_num}' images.");
                ktx.image_2d_array(
                    dimensions.x,
                    dimensions.y,
                    layer_num,
                    level_num,
                    &image_references,
                );
            }
            TextureType::Cube => {
                let dimensions = images.dimensions();
                let level_num = images.level_num();

                log::info!("Writing Cube");
                ktx.cube(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureType::CubeArray => {
                let dimensions = images.dimensions();
                let layer_num = images.layer_num();
                let level_num = images.level_num();

                log::info!("Writing Cube Array with '{}' images.", layer_num);
                ktx.cube_array(
                    dimensions.x,
                    dimensions.y,
                    layer_num / 6,
                    level_num,
                    &image_references,
                );
            }
        }

        let output_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output)?;

        let mut writer = BufWriter::new(output_file);
        ktx.write(&mut writer)?;

        Ok(())
    }

    fn dont_log(&self) -> bool {
        false
    }
}

fn load_images_to_texture(
    images: Vec<DynamicImageBuffer>,
    is_cube: bool,
    is_array: bool,
) -> DynamicTextureBuffer {
    match images[0].color_type() {
        ColorType::R8Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::R8Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RG8Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RG8Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGB8Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGB8Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGBA8Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGBA8Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::R16Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::R16Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RG16Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RG16Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGB16Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGB16Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGBA16Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGBA16Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::R32Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::R32Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RG32Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RG32Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGB32Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGB32Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGBA32Unorm => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGBA32Unorm(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::R16Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::R16Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RG16Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RG16Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGB16Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGB16Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGBA16Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGBA16Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::R32Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::R32Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RG32Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RG32Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGB32Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGB32Float(bundle_images_to_texture(images, is_cube, is_array))
        }
        ColorType::RGBA32Float => {
            let images = downcast_to_images(images);
            DynamicTextureBuffer::RGBA32Float(bundle_images_to_texture(images, is_cube, is_array))
        }
    }
}

fn bundle_images_to_texture<T: PixelFormat>(
    images: Vec<ImageBuffer<T>>,
    is_cube: bool,
    is_array: bool,
) -> TextureBuffer<T> {
    match (is_cube, is_array) {
        // cubemap array
        (true, true) => {
            log::info!("Input is Cube Array with '{}' images.", images.len());
            let dimensions = images[0].dimensions();
            TextureBuffer::CubeArray {
                dimensions,
                cube_num: (images.len() / 6) as u32,
                level_num: 1,
                images,
            }
        }
        // image array
        (false, true) => {
            log::info!("Input is Image Array with '{}' images.", images.len());
            let dimensions = images[0].dimensions();
            TextureBuffer::Array {
                dimensions,
                layer_num: images.len() as u32,
                level_num: 1,
                images,
            }
        }
        // single cubemap
        (true, false) => {
            log::info!("Input is Cube");
            let dimensions = images[0].dimensions();
            TextureBuffer::Cube {
                dimensions,
                level_num: 1,
                images,
            }
        }
        // single image
        (false, false) => {
            log::info!("Input is Image");
            let dimensions = images[0].dimensions();
            TextureBuffer::Single {
                dimensions,
                level_num: 1,
                images,
            }
        }
    }
}

fn downcast_to_images<T: PixelFormat>(images: Vec<DynamicImageBuffer>) -> Vec<ImageBuffer<T>>
where
    DynamicImageBuffer: DowncastImageBuffer<ImageBuffer<T>>,
{
    Vec::from_iter(
        images
            .into_iter()
            .map(|v| v.downcast_image_buffer().unwrap()),
    )
}
