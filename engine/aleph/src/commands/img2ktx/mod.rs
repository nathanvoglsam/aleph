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

use aleph_image::{DynamicImageBuffer, IPixelStorage, ResizeFilter, TextureBuffer};
use aleph_ktx::{KtxDocumentDescription, VkFormat};
use aleph_math::UVec2;
use anyhow::anyhow;
use camino::Utf8Path;
use clap::parser::Values;
use clap::{Arg, ArgAction, ArgMatches, Command};

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
            .long("gen-mips")
            .help("Whether to generate a mip chain from the input image.")
            .long_help("Whether to generate a mip chain from the input image. Uses a bilinear filter by default.");
        let is_cube = Arg::new("is-cube")
            .action(ArgAction::SetTrue)
            .long("is-cube")
            .help("Whether the input image set describes a cube map.")
            .long_help("Whether the input image set describes a cube map. Must provide six images, ordered by +X, -X, +Y, -Y, +Z, -Z");
        let equi_to_cube = Arg::new("equi-to-cube")
            .action(ArgAction::SetTrue)
            .long("equi-to-cube")
            .help("Convert an equirectangular map to a cube map.")
            .long_help("Declares that the input is an equirectangular environment map, and we should convert it to a cube map.")
            .conflicts_with_all(["equi-to-oct", "oct-to-cube"]);
        let oct_to_cube = Arg::new("oct-to-cube")
            .action(ArgAction::SetTrue)
            .long("oct-to-cube")
            .help("Convert an octahedral map to a cube map.")
            .long_help("Declares that the input is an octahedral environment map, and we should convert it to a cube map.")
            .conflicts_with_all(["equi-to-oct", "equi-to-cube"]);
        let equi_to_oct = Arg::new("equi-to-oct")
            .action(ArgAction::SetTrue)
            .long("equi-to-oct")
            .help("Convert an equirectangular map to an octahedral map.")
            .long_help("Declares that the input is an equirectangular environment map, and we should convert it to an octahedral map.")
            .conflicts_with_all(["equi-to-cube", "oct-to-cube"]);
        let cube_size = Arg::new("cube-size")
            .long("cube-size")
            .help("The width/height of the cube faces to output when generating cube maps.")
            .long_help("The width/height, in texels, of the cube faces to output when generating cube maps. Use with --equi-to-cube, etc. This only applies when synthesizing a cube map from a non-cube input. Defaults to 512.")
            .value_parser(clap::value_parser!(u32))
            .default_value("512")
            .required(false);
        let oct_size = Arg::new("oct-size")
            .long("oct-size")
            .help("The width/height of the octahedral map to output when generating octahedral maps.")
            .long_help("The width/height, in texels, of the texture to output when generating octahedral maps. Use with --equi-to-oct, etc. This only applies when synthesizing an octahedral map from a non-cube input. Defaults to 512.")
            .value_parser(clap::value_parser!(u32))
            .default_value("512")
            .required(false);
        let is_normal_map = Arg::new("is-normal-map")
            .action(ArgAction::SetTrue)
            .long("is-normal-map")
            .help("Declares that the input image is a normal map.")
            .long_help("Declares that the input image is a normal map. This changes some things, like an additonal normalization operation when generating mips.");
        let to_half = Arg::new("to-half")
            .action(ArgAction::SetTrue)
            .long("to-half")
            .help("Declares that floating point input should be output in half-precision.")
            .long_help("Declares that floating point input should be output in half-precision. This only affects floating point input images like HDRIs.");
        let mip_filter = Arg::new("mip-filter")
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
            .arg(oct_to_cube)
            .arg(equi_to_oct)
            .arg(cube_size)
            .arg(oct_size)
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
        let oct_to_cube = matches.get_flag("oct-to-cube");
        let equi_to_oct = matches.get_flag("equi-to-oct");

        let conversions_enabled = [equi_to_cube, oct_to_cube, equi_to_oct]
            .into_iter()
            .filter(|&v| v)
            .count();
        if conversions_enabled > 1 {
            return Err(anyhow!(
                "Can't have more than one conversion operation enabled at once"
            ));
        }

        let mip_filter: String = matches.remove_one("mip-filter").unwrap();
        let mip_filter = mip_filter.to_lowercase();
        let mip_filter = parse_filter(&mip_filter)
            .ok_or_else(|| anyhow!("Unknown filter \"{}\"", &mip_filter))?;

        let cube_size: u32 = matches.remove_one("cube-size").unwrap();
        let oct_size: u32 = matches.remove_one("oct-size").unwrap();

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

        let mut images = match (is_cube, is_array) {
            // cubemap array
            (true, true) => {
                log::info!("Input is Cube Array with '{}' images.", input_files.len());
                let dimensions = loaded_files[0].dimensions();
                TextureBuffer::CubeArray {
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
                TextureBuffer::Array {
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
                TextureBuffer::Cube {
                    dimensions,
                    level_num: 1,
                    images: loaded_files,
                }
            }
            // single image
            (false, false) => {
                log::info!("Input is Image");
                let dimensions = loaded_files[0].dimensions();
                TextureBuffer::Single {
                    dimensions,
                    level_num: 1,
                    images: loaded_files,
                }
            }
        };

        images.validate_image_count();
        images.validate_image_types();

        if equi_to_cube {
            let face_dimensions = UVec2::new(cube_size, cube_size);
            images.equirectangular_to_cube_map(face_dimensions)?;
        }

        if oct_to_cube {
            let face_dimensions = UVec2::new(cube_size, cube_size);
            images.octahedral_to_cube_map(face_dimensions)?;
        }

        if equi_to_oct {
            let face_dimensions = UVec2::new(oct_size, oct_size);
            images.equirectangular_to_octahedral_map(face_dimensions)?;
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
            aleph_image::ColorType::RGB8Unorm => images.swizzle_rgb_to_rgba()?,
            aleph_image::ColorType::RGB16Unorm => images.swizzle_rgb_to_rgba()?,
            aleph_image::ColorType::RGB32Unorm => images.swizzle_rgb_to_rgba()?,
            _ => {}
        }

        for i in images.images_mut() {
            if to_half {
                i.to_half();
            }
            i.to_little_endian();
        }

        let final_color_type = images.get_color_type();
        let image_references = images.get_buffer_references();

        // Setup mip state in common code to keep the match arms shorter
        let mut ktx = KtxDocumentDescription::new();

        match final_color_type {
            aleph_image::ColorType::R8Unorm => ktx.format(VkFormat::R8_UNORM),
            aleph_image::ColorType::RG8Unorm => ktx.format(VkFormat::R8G8_UNORM),
            aleph_image::ColorType::RGB8Unorm => ktx.format(VkFormat::R8G8B8A8_UNORM),
            aleph_image::ColorType::RGBA8Unorm => ktx.format(VkFormat::R8G8B8A8_UNORM),
            aleph_image::ColorType::R16Unorm => ktx.format(VkFormat::R16_UNORM),
            aleph_image::ColorType::RG16Unorm => ktx.format(VkFormat::R16G16_UNORM),
            aleph_image::ColorType::RGB16Unorm => ktx.format(VkFormat::R16G16B16A16_UNORM),
            aleph_image::ColorType::RGBA16Unorm => ktx.format(VkFormat::R16G16B16A16_UNORM),
            aleph_image::ColorType::R32Unorm => unimplemented!(),
            aleph_image::ColorType::RG32Unorm => unimplemented!(),
            aleph_image::ColorType::RGB32Unorm => unimplemented!(),
            aleph_image::ColorType::RGBA32Unorm => unimplemented!(),
            aleph_image::ColorType::R16Float => ktx.format(VkFormat::R16_SFLOAT),
            aleph_image::ColorType::RG16Float => ktx.format(VkFormat::R16G16_SFLOAT),
            aleph_image::ColorType::RGB16Float => ktx.format(VkFormat::R16G16B16_SFLOAT),
            aleph_image::ColorType::RGBA16Float => ktx.format(VkFormat::R16G16B16A16_SFLOAT),
            aleph_image::ColorType::R32Float => ktx.format(VkFormat::R32_SFLOAT),
            aleph_image::ColorType::RG32Float => ktx.format(VkFormat::R32G32_SFLOAT),
            aleph_image::ColorType::RGB32Float => ktx.format(VkFormat::R32G32B32_SFLOAT),
            aleph_image::ColorType::RGBA32Float => ktx.format(VkFormat::R32G32B32A32_SFLOAT),
        };

        // If we've converted from an equirectangular map to a cube map then we need to change the
        // output resolution to the chosen cube face dimensions instead of the source image
        // dimensions
        match images {
            TextureBuffer::Single {
                dimensions,
                level_num,
                ..
            } => {
                log::info!("Writing Image");
                ktx.image_2d(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureBuffer::Array {
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
            TextureBuffer::Cube {
                dimensions,
                level_num,
                ..
            } => {
                log::info!("Writing Cube");
                ktx.cube(dimensions.x, dimensions.y, level_num, &image_references);
            }
            TextureBuffer::CubeArray {
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

fn parse_filter(v: &str) -> Option<ResizeFilter> {
    let v = match v {
        "nearest" => ResizeFilter::Nearest,
        "bilinear" => ResizeFilter::Linear,
        "cubic" => ResizeFilter::Cubic,
        "gaussian" => ResizeFilter::Gaussian,
        "lanczos3" => ResizeFilter::Lanczos3,
        _ => return None,
    };
    Some(v)
}
