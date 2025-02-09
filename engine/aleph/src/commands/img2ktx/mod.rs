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

mod dynamic_buffer;
mod equirectangular_conversion;
mod sampled_image;
mod texture_type;
mod unorm_conversion;

pub use dynamic_buffer::*;
pub use equirectangular_conversion::*;
pub use sampled_image::*;
pub use unorm_conversion::*;

use std::io::BufWriter;

use aleph_ktx::{KtxDocumentDescription, VkFormat};
use aleph_math::UVec2;
use anyhow::anyhow;
use camino::Utf8Path;
use clap::parser::Values;
use clap::{Arg, ArgAction, ArgMatches, Command};
use image::imageops::FilterType;
use image::GenericImageView;

use crate::commands::img2ktx::texture_type::TextureType;
use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct Image2Ktx {}

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
        let gen_mips = Arg::new("gen-mips")
            .action(ArgAction::SetTrue)
            .short('g')
            .long("gen-mips")
            .help("Whether to generate a mip chain from the input image.")
            .long_help("Whether to generate a mip chain from the input image. Uses a bilinear filter by default.");
        let is_cube = Arg::new("is-cube")
            .action(ArgAction::SetTrue)
            .short('c')
            .long("is-cube")
            .help("Whether the input image set describes a cube map.")
            .long_help("Whether the input image set describes a cube map. Must provide six images, ordered by +X, -X, +Y, -Y, +Z, -Z");
        let equi_to_cube = Arg::new("equi-to-cube")
            .action(ArgAction::SetTrue)
            .short('e')
            .long("equi-to-cube")
            .help("Convert an equirectangular map to a cube map.")
            .long_help("Declares that the input is an equirectangular environment map, and we should convert it to a cube map.");
        let is_normal_map = Arg::new("is-normal-map")
            .action(ArgAction::SetTrue)
            .short('n')
            .long("is-normal-map")
            .help("Declares that the input image is a normal map.")
            .long_help("Declares that the input image is a normal map. This changes some things, like an additonal normalization operation when generating mips.");
        let to_half = Arg::new("to-half")
            .action(ArgAction::SetTrue)
            .long("to-half")
            .help("Declares that floating point input should be output in half-precision.")
            .long_help("Declares that floating point input should be output in half-precision. This only affects floating point input images like HDRIs.");
        let mip_filter = Arg::new("mip-filter")
            .short('f')
            .long("mip-filter")
            .help("The type of filter to use when downsampling mips.")
            .long_help("The type of filter to use when downsampling mips. Options: nearest, bilinear, cubic, gaussian, lanczos3")
            .default_value("bilinear")
            .required(false);
        Command::new(self.name())
            .about("Converts the given input image into the KTX2 format")
            .arg(input)
            .arg(output)
            .arg(gen_mips)
            .arg(mip_filter)
            .arg(is_normal_map)
            .arg(to_half)
            .arg(is_cube)
            .arg(equi_to_cube)
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

        let gen_mips = matches.get_flag("gen-mips");
        let is_normal_map = matches.get_flag("is-normal-map");
        let to_half = matches.get_flag("to-half");
        let is_cube = matches.get_flag("is-cube");
        let equi_to_cube = matches.get_flag("equi-to-cube");

        let mip_filter: String = matches.remove_one("mip-filter").unwrap();
        let mip_filter = mip_filter.to_lowercase();
        let mip_filter = parse_filter(&mip_filter)
            .ok_or_else(|| anyhow!("Unknown filter \"{}\"", &mip_filter))?;

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
            loaded_files.push(loaded);
        }

        // Validate that the input dimensions and color format match across all formats
        for this_image in loaded_files.iter() {
            let base_image = &loaded_files[0];

            if base_image.dimensions() != this_image.dimensions() {
                return Err(anyhow!("All input image dimensions must match!"));
            }
            if base_image.color() != this_image.color() {
                return Err(anyhow!("All input image pixel formats must match!"));
            }
        }

        let mut images = match (is_cube, is_array) {
            // cubemap array
            (true, true) => {
                log::info!("Input is Cube Array with '{}' images.", input_files.len());
                let dimensions = loaded_files[0].dimensions();
                let dimensions = UVec2::new(dimensions.0, dimensions.1);
                TextureType::CubeArray {
                    dimensions,
                    cube_num: (input_files.len() / 6) as u32,
                    level_num: 1,
                    images: loaded_files,
                }
            }
            // image array
            (false, true) => {
                log::info!("Input is Image Array with '{}' images.", input_files.len());
                let dimensions = loaded_files[0].dimensions();
                let dimensions = UVec2::new(dimensions.0, dimensions.1);
                TextureType::Array {
                    dimensions,
                    layer_num: input_files.len() as u32,
                    level_num: 1,
                    images: loaded_files,
                }
            }
            // single cubemap
            (true, false) => {
                log::info!("Input is Cube");
                let dimensions = loaded_files[0].dimensions();
                let dimensions = UVec2::new(dimensions.0, dimensions.1);
                TextureType::Cube {
                    dimensions,
                    level_num: 1,
                    images: loaded_files,
                }
            }
            // single image
            (false, false) => {
                log::info!("Input is Image");
                let dimensions = loaded_files[0].dimensions();
                let dimensions = UVec2::new(dimensions.0, dimensions.1);
                TextureType::Single {
                    dimensions,
                    level_num: 1,
                    images: loaded_files,
                }
            }
        };

        images.validate_image_count();
        images.validate_image_types();

        if equi_to_cube {
            let face_dimensions = UVec2::new(1024, 1024);
            images.equirectangular_to_cube_map(face_dimensions)?;
        }

        if gen_mips {
            images.generate_mips(mip_filter.into());
        }

        if is_normal_map {
            images.normalize()?;
        }

        // Swizzle 3 channel formats up to 4 channels as there are almost zero GPUs on the planet
        // that can sample from 3 channel formats
        match images.get_color_type() {
            image::ColorType::L8 => {}
            image::ColorType::La8 => {}
            image::ColorType::Rgb8 => images.swizzle_rgb_to_rgba()?,
            image::ColorType::Rgba8 => {}
            image::ColorType::L16 => {}
            image::ColorType::La16 => {}
            image::ColorType::Rgb16 => images.swizzle_rgb_to_rgba()?,
            image::ColorType::Rgba16 => {}
            image::ColorType::Rgb32F => images.swizzle_rgb_to_rgba()?,
            image::ColorType::Rgba32F => {}
            _ => unimplemented!(),
        }

        let final_color_type = images.get_color_type();

        let mut buffers = images.take_images_as_buffers();
        for buffer in buffers.iter_mut() {
            if to_half {
                buffer.to_half();
            }
            buffer.to_little_endian();
        }

        let mut image_references = Vec::new();
        for buffer in buffers.iter() {
            image_references.push(buffer.as_bytes());
        }

        // Setup mip state in common code to keep the match arms shorter
        let mut ktx = KtxDocumentDescription::new();

        match final_color_type {
            image::ColorType::L8 => {
                ktx.format(VkFormat::R8_UNORM);
            }
            image::ColorType::La8 => {
                ktx.format(VkFormat::R8G8_UNORM);
            }
            image::ColorType::Rgb8 => {
                ktx.format(VkFormat::R8G8B8A8_UNORM);
            }
            image::ColorType::Rgba8 => {
                ktx.format(VkFormat::R8G8B8A8_UNORM);
            }
            image::ColorType::L16 => {
                ktx.format(VkFormat::R16_UNORM);
            }
            image::ColorType::La16 => {
                ktx.format(VkFormat::R16G16_UNORM);
            }
            image::ColorType::Rgb16 => {
                ktx.format(VkFormat::R16G16B16A16_UNORM);
            }
            image::ColorType::Rgba16 => {
                ktx.format(VkFormat::R16G16B16A16_UNORM);
            }
            image::ColorType::Rgb32F => {
                if to_half {
                    ktx.format(VkFormat::R16G16B16A16_SFLOAT);
                } else {
                    ktx.format(VkFormat::R32G32B32A32_SFLOAT);
                }
            }
            image::ColorType::Rgba32F => {
                if to_half {
                    ktx.format(VkFormat::R16G16B16A16_SFLOAT);
                } else {
                    ktx.format(VkFormat::R32G32B32A32_SFLOAT);
                }
            }
            _ => unimplemented!(),
        }

        if equi_to_cube {
            ktx.format(VkFormat::R32G32B32A32_SFLOAT);
        }

        // If we've converted from an equirectangular map to a cube map then we need to change the
        // output resolution to the chosen cube face dimensions instead of the source image
        // dimensions
        match images {
            TextureType::Single {
                dimensions,
                level_num,
                ..
            } => {
                log::info!("Writing Image");
                ktx.image_2d(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureType::Array {
                dimensions,
                layer_num,
                level_num,
                ..
            } => {
                log::info!("Writing Image Array with '{layer_num}' images.");
                ktx.image_2d_array(
                    dimensions.x,
                    dimensions.y,
                    layer_num,
                    level_num,
                    &image_references,
                );
            }
            TextureType::Cube {
                dimensions,
                level_num,
                ..
            } => {
                log::info!("Writing Cube");
                ktx.cube(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureType::CubeArray {
                dimensions,
                cube_num,
                level_num,
                ..
            } => {
                log::info!("Writing Cube Array with '{}' images.", cube_num * 6);
                ktx.cube_array(
                    dimensions.x,
                    dimensions.y,
                    cube_num,
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

fn parse_filter(v: &str) -> Option<FilterType> {
    let v = match v {
        "nearest" => FilterType::Nearest,
        "bilinear" => FilterType::Triangle,
        "cubic" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        _ => return None,
    };
    Some(v)
}
