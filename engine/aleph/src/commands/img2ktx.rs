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

use aleph_ktx::{calculate_set_index, KtxDocumentDescription, VkFormat};
use aleph_math::Vec3;
use anyhow::anyhow;
use camino::Utf8Path;
use clap::parser::Values;
use clap::{Arg, ArgAction, ArgMatches, Command};
use half::f16;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use num_traits::ops::bytes::ToBytes;

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

        // Once we've proven dimensions and color are uniform we can extract the common data to be
        // used later
        let color_type = loaded_files[0].color();
        let base_width = loaded_files[0].width();
        let base_height = loaded_files[0].height();
        let max_mip_levels = u32::max(base_width, base_height) as f32;
        let max_mip_levels = max_mip_levels.log2().floor() + 1.0;
        let max_mip_levels = max_mip_levels as u32;

        // If the user doesn't ask us to generate mip maps then we have to clamp to 1 level as we
        // don't have any other mips to populate.
        let level_num = if gen_mips { max_mip_levels as usize } else { 1 };

        // Create the image array in the layout expected by the encoder to simplify our future
        // processing code.
        let layer_num = input_files.len();
        let image_num = layer_num * level_num;
        let mut images = vec![DynamicImage::ImageLuma8(ImageBuffer::new(0, 0)); image_num];
        for (i, input_image) in loaded_files.into_iter().enumerate() {
            let i = calculate_set_index(layer_num, level_num, i, 0);
            images[i] = input_image;
        }

        // Generate mipmaps if requested
        if gen_mips {
            for layer in 0..layer_num {
                for level in 1..level_num {
                    let i = calculate_set_index(layer_num, level_num, layer, level - 1);
                    let last = &images[i];

                    let new_width = (last.width() / 2).max(1);
                    let new_height = (last.height() / 2).max(1);
                    let mut next = last.resize_exact(new_width, new_height, mip_filter.into());

                    if is_normal_map {
                        normalize_normal_map(&mut next)?;
                    }

                    let i = calculate_set_index(layer_num, level_num, layer, level);
                    images[i] = next;
                }
            }
        }

        // Setup mip state in common code to keep the match arms shorter
        let mut ktx = KtxDocumentDescription::new();

        // Swizzle 3 channel formats up to 4 channels as there are almost zero GPUs on the planet
        // that can sample from 3 channel formats
        match color_type {
            image::ColorType::L8 => {
                // Intentional no-op
            }
            image::ColorType::La8 => {
                // Intentional no-op
            }
            image::ColorType::Rgb8 => {
                for layer in 0..layer_num {
                    for level in 0..level_num {
                        let i = calculate_set_index(layer_num, level_num, layer, level);
                        let level = &mut images[i];

                        let rgb = level.as_rgb8().unwrap();
                        let swizzled = swizzle_rgb_to_rgba(
                            rgb.as_raw().as_slice(),
                            rgb.width(),
                            rgb.height(),
                            0xFF,
                        );
                        let swizzled =
                            RgbaImage::from_vec(rgb.width(), rgb.height(), swizzled).unwrap();
                        *level = DynamicImage::ImageRgba8(swizzled);
                    }
                }
            }
            image::ColorType::Rgba8 => {
                // Intentional no-op
            }
            image::ColorType::L16 => {
                // intentional no-op
            }
            image::ColorType::La16 => {
                // intentional no-op
            }
            image::ColorType::Rgb16 => {
                for layer in 0..layer_num {
                    for level in 0..level_num {
                        let i = calculate_set_index(layer_num, level_num, layer, level);
                        let level = &mut images[i];

                        let rgb = level.as_rgb16().unwrap();

                        let swizzled = swizzle_rgb_to_rgba(
                            rgb.as_raw().as_slice(),
                            rgb.width(),
                            rgb.height(),
                            0xFF,
                        );
                        let swizzled: ImageBuffer<Rgba<u16>, Vec<u16>> =
                            ImageBuffer::from_vec(rgb.width(), rgb.height(), swizzled).unwrap();
                        *level = DynamicImage::ImageRgba16(swizzled);
                    }
                }
            }
            image::ColorType::Rgba16 => {
                // intentional no-op
            }
            image::ColorType::Rgb32F => {
                for layer in 0..layer_num {
                    for level in 0..level_num {
                        let i = calculate_set_index(layer_num, level_num, layer, level);
                        let level = &mut images[i];

                        let rgb = level.as_rgb32f().unwrap();

                        let swizzled = swizzle_rgb_to_rgba(
                            rgb.as_raw().as_slice(),
                            rgb.width(),
                            rgb.height(),
                            1.0f32,
                        );

                        let swizzled: ImageBuffer<Rgba<f32>, Vec<f32>> =
                            ImageBuffer::from_vec(rgb.width(), rgb.height(), swizzled).unwrap();
                        *level = DynamicImage::ImageRgba32F(swizzled);
                    }
                }
            }
            image::ColorType::Rgba32F => {
                // intentional no-op
            }
            _ => unimplemented!(),
        }

        // Now that our image aware conversions are done we drop down to raw buffer level.
        let mut buffers = Vec::from_iter(images.into_iter().map(|v| match v {
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
            _ => todo!(),
        }));

        // Perform any endian swap or per-channel conversions like dropping f32->f16
        for buffer in buffers.iter_mut() {
            match buffer {
                DynamicBuffer::U8(_) => {
                    // intentional no-op
                }
                DynamicBuffer::U16(v) => {
                    for p in v.iter_mut() {
                        *p = bytemuck::cast::<_, u16>(p.to_le_bytes());
                    }
                }
                DynamicBuffer::F16(v) => {
                    for p in v.iter_mut() {
                        *p = bytemuck::cast::<_, f16>(p.to_le_bytes());
                    }
                }
                v @ DynamicBuffer::F32(_) => {
                    if let DynamicBuffer::F32(b) = v {
                        if to_half {
                            let converted = Vec::from_iter(b.iter().map(|v| {
                                let v = f16::from_f32(*v);
                                let v = bytemuck::cast::<_, f16>(v.to_le_bytes());
                                v
                            }));
                            *v = DynamicBuffer::F16(converted);
                        } else {
                            let converted = Vec::from_iter(b.iter().map(|v| {
                                let v = bytemuck::cast::<_, f32>(v.to_le_bytes());
                                v
                            }));
                            *v = DynamicBuffer::F32(converted);
                        }
                    };
                }
            }
        }

        let mut image_references = Vec::new();
        for buffer in buffers.iter() {
            image_references.push(buffer.as_bytes());
        }

        match color_type {
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
                ktx.format(VkFormat::R16G16B16A16_SFLOAT);
            }
            image::ColorType::Rgba32F => {
                ktx.format(VkFormat::R16G16B16A16_SFLOAT);
            }
            _ => todo!(),
        }

        let output_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output)?;

        match (is_cube, is_array) {
            // cubemap array
            (true, true) => {
                log::info!("Writing Cube Array with '{layer_num}' images.");
                ktx.cube_array(base_width, base_height, layer_num as u32 / 6, level_num as u32, &image_references);
            },
            // image array
            (false, true) => {
                log::info!("Writing Image Array with '{layer_num}' images.");
                ktx.image_2d_array(base_width, base_height, layer_num as u32, level_num as u32, &image_references);
            },
            // single cubemap
            (true, false) => {
                log::info!("Writing Cube");
                ktx.cube(base_width, base_height, level_num as u32, &image_references);
            }
            // single image
            (false, false) => {
                log::info!("Writing Image");
                ktx.image_2d(base_width, base_height, level_num as u32, &image_references);
            }
        }

        let mut writer = BufWriter::new(output_file);
        ktx.write(&mut writer)?;

        Ok(())
    }

    fn dont_log(&self) -> bool {
        false
    }
}

enum DynamicBuffer {
    U8(Vec<u8>),
    U16(Vec<u16>),
    F16(Vec<f16>),
    F32(Vec<f32>),
}

impl DynamicBuffer {
    fn as_bytes(&self) -> &[u8] {
        match self {
            DynamicBuffer::U8(items) => bytemuck::cast_slice::<u8, u8>(items.as_slice()),
            DynamicBuffer::U16(items) => bytemuck::cast_slice::<u16, u8>(items.as_slice()),
            DynamicBuffer::F16(items) => bytemuck::cast_slice::<f16, u8>(items.as_slice()),
            DynamicBuffer::F32(items) => bytemuck::cast_slice::<f32, u8>(items.as_slice()),
        }
    }
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

fn unorm_u8_to_f32(v: u8) -> f32 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (255.0 * 3.0);
    return (v as f32 * K0) * K1;
}

fn unorm_u16_to_f32(v: u16) -> f32 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (65535.0 * 3.0);
    return (v as f32 * K0) * K1;
}

fn f32_to_unorm_u8(v: f32) -> u8 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (255.0 * 3.0);
    return ((v / K1) / K0).round() as u8;
}

fn f32_to_unorm_u16(v: f32) -> u16 {
    const K0: f32 = 3.0;
    const K1: f32 = 1.0 / (65535.0 * 3.0);
    return ((v / K1) / K0).round() as u16;
}

#[cfg(test)]
mod tests {
    use crate::commands::img2ktx::{
        f32_to_unorm_u16, f32_to_unorm_u8, unorm_u16_to_f32, unorm_u8_to_f32,
    };

    #[test]
    fn unorm_u8_to_f32_associates() {
        for i in 0..256i32 {
            let i: u8 = i.try_into().unwrap();

            let f = unorm_u8_to_f32(i);
            let u = f32_to_unorm_u8(f);
            assert_eq!(i, u);
        }
    }

    #[test]
    fn unorm_u16_to_f32_associates() {
        for i in 0..65536i32 {
            let i: u16 = i.try_into().unwrap();

            let f = unorm_u16_to_f32(i);
            let u = f32_to_unorm_u16(f);
            assert_eq!(i, u);
        }
    }
}

// TODO: Implement CPU Equirectangular -> Cube conversion using Rust scalar code and Intel ISPC
//       code. We can base our sampling logic in the D3D11 spec: https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#7.18%20Texture%20Sampling
//
// # Important Part for Linear Sampling
//
// 7.18.8 Linear Sample Addressing
//
// Similar to the previous section, set aside how sampler state is configured and how mipmap LOD is chosen for now, and consider simply the task of linear sampling an Element from a particular miplevel of a Texture1D, given a scalar floating point texture coordinate in normalized space. Linear sampling in 1D selects the nearest two texels to the sample location and weights the texels based on the proximity of the sample location to them.
//
//     Given a 1D texture coordinate in normalized space U, assumed to be any float32 value.
//     U is scaled by the Texture1D size, and 0.5f is subtracted. Call this scaledU.
//     scaledU is converted to at least 16.8 Fixed Point(3.2.4.1). Call this fxpScaledU.
//     The integer part of fxpScaledU is the chosen left texel. Call this tFloorU. Note that the conversion to Fixed Point(3.2.4.1) basically accomplished: tFloorU = floor(scaledU).
//     The right texel, tCeilU is simply tFloorU + 1.
//     The weight value wCeilU is assigned the fractional part of fxpScaledU, converted to float(3.2.4.2) (although using less than full float32 precision for computing and processing wCeilU and wFloorU is permitted).
//     The weight value wFloorU is 1.0f - wCeilU.
//     If tFloorU or tCeilU are out of range of the texture, D3D11_SAMPLER_STATE's AddressU mode is applied(7.18.9) to each individually.
//     Since more than one texel is chosen, the single sample result is computed as:
//
//     texelFetch(tFloorU) * wFloorU +
//     texelFetch( tCeilU) *  wCeilU
//
// The procedure described above applies to linear sampling of a given miplevel of a Texture2D as well:
//
//     Peform the texel selection to both U and V directions independently, producing 2 U texel locations and 2 V texel locations. Combined, these select 4 texels: (tFloorU,tFloorV), (tFloorU,tCeilV), (tCeilU,tFloorV), (tCeilU,tCeilV).
//     There are also 4 weight values produced: wFloorU, wCeilU, wFloorV, wCeilV.
//     The linear sample result is:
//
//     texelFetch(tFloorU,tFloorV) * wFloorU * wFloorV +
//     texelFetch(tFloorU, tCeilV) * wFloorU *  wCeilV +
//     texelFetch( tCeilU,tFloorV) *  wCeilU * wFloorV +
//     texelFetch( tCeilU, tCeilV) *  wCeilU *  wCeilV
//
// Performing linear sampling of a miplevel of a Texture3D Resource extends the concepts described above to fetching of 8 texels.
//
