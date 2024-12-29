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

use aleph_ktx::{KtxDocumentDescription, VkFormat};
use aleph_math::Vec3;
use anyhow::anyhow;
use camino::Utf8Path;
use clap::{Arg, ArgAction, ArgMatches, Command};
use half::f16;
use image::imageops::FilterType;
use image::DynamicImage;

use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct Image2Ktx {}

impl ISubcommand for Image2Ktx {
    fn name(&self) -> &'static str {
        "img2ktx"
    }

    fn description(&mut self) -> Command {
        let input = Arg::new("input")
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
        let is_normal_map = Arg::new("is-normal-map")
            .action(ArgAction::SetTrue)
            .short('n')
            .long("is-normal-map")
            .help("Declares that the input image is a normal map.")
            .long_help("Declares that the input image is a normal map. This changes some things, like an additonal normalization operation when generating mips.");
        let to_half = Arg::new("to-half")
            .action(ArgAction::SetTrue)
            .short('h')
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
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let input_arg: String = matches.remove_one("input").expect("platform is required");
        let input = Utf8Path::new(&input_arg).to_path_buf();

        let output_arg: Option<String> = matches.remove_one("output");
        let output = match output_arg {
            Some(v) => Utf8Path::new(&v).to_path_buf(),
            None => input.with_extension("ktx2"),
        };

        let gen_mips = matches.get_flag("gen-mips");
        let is_normal_map = matches.get_flag("is-normal-map");
        let to_half = matches.get_flag("to-half");

        let mip_filter: String = matches.remove_one("mip-filter").unwrap();
        let mip_filter = mip_filter.to_lowercase();
        let mip_filter = parse_filter(&mip_filter)
            .ok_or_else(|| anyhow!("Unknown filter \"{}\"", &mip_filter))?;

        let loaded = image::ImageReader::open(&input)?
            .with_guessed_format()?
            .decode()?;

        let color_type = loaded.color();
        let base_width = loaded.width();
        let base_height = loaded.height();

        // Generate mipmaps if requested
        let mut loaded = if gen_mips {
            let mip_levels = u32::max(loaded.width(), loaded.height()) as f32;
            let mip_levels = mip_levels.log2().floor() + 1.0;
            let mip_levels = mip_levels as u32;
            let mut chain = vec![loaded];

            if mip_levels > 1 {
                // The first mip is already filled by definition
                for _ in 1..mip_levels {
                    let last = chain.last().unwrap();
                    let new_width = (last.width() / 2).max(1);
                    let new_height = (last.height() / 2).max(1);
                    let mut next = last.resize_exact(new_width, new_height, mip_filter.into());

                    if is_normal_map {
                        normalize_normal_map(&mut next)?;
                    }

                    chain.push(next);
                }
            }

            chain
        } else {
            vec![loaded]
        };

        // Setup mip state in common code to keep the match arms shorter
        let mut ktx = KtxDocumentDescription::new();
        if gen_mips {
            ktx.mip_levels(loaded.len() as u32);
        } else {
            ktx.mip_generate();
        }

        match color_type {
            image::ColorType::L8 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R8_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter() {
                    let i = i.as_luma8().unwrap();
                    levels.push(i.as_raw().as_slice());
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::La8 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R8G8_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter() {
                    let i = i.as_luma_alpha8().unwrap();
                    levels.push(i.as_raw().as_slice());
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::Rgb8 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R8G8B8A8_UNORM);

                let mut swizzled_levels = Vec::new();
                for i in loaded.iter() {
                    let i = i.as_rgb8().unwrap();

                    let level =
                        swizzle_rgb_to_rgba(i.as_raw().as_slice(), i.width(), i.height(), 0xFF);

                    swizzled_levels.push(level);
                }

                let mut levels = Vec::new();
                for i in swizzled_levels.iter() {
                    levels.push(i.as_slice());
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::Rgba8 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R8G8B8A8_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter() {
                    let i = i.as_rgba8().unwrap();
                    levels.push(i.as_raw().as_slice());
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::L16 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R16_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter_mut() {
                    let i = i.as_mut_luma16().unwrap();

                    // byte swap to convert our BE to LE when needed
                    if cfg!(target_endian = "big") {
                        for p in i.pixels_mut() {
                            p.0[0] = p.0[0].to_le();
                        }
                    }

                    let level = bytemuck::cast_slice::<_, u8>(i.as_raw().as_slice());
                    levels.push(level);
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::La16 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R16G16_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter_mut() {
                    let i = i.as_mut_luma_alpha16().unwrap();

                    // byte swap to convert our BE to LE when needed
                    if cfg!(target_endian = "big") {
                        for p in i.pixels_mut() {
                            p.0[0] = p.0[0].to_le();
                            p.0[1] = p.0[1].to_le();
                        }
                    }

                    let level = bytemuck::cast_slice::<_, u8>(i.as_raw().as_slice());
                    levels.push(level);
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::Rgb16 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R16G16B16A16_UNORM);

                let mut swizzled_levels = Vec::new();
                for i in loaded.iter_mut() {
                    let i = i.as_mut_rgb16().unwrap();

                    // byte swap to convert our BE to LE when needed
                    if cfg!(target_endian = "big") {
                        for p in i.pixels_mut() {
                            p.0[0] = p.0[0].to_le();
                            p.0[1] = p.0[1].to_le();
                            p.0[2] = p.0[2].to_le();
                        }
                    }

                    let level =
                        swizzle_rgb_to_rgba(i.as_raw().as_slice(), i.width(), i.height(), 0xFFFF);

                    swizzled_levels.push(level);
                }

                let mut levels = Vec::new();
                for i in swizzled_levels.iter() {
                    let level = bytemuck::cast_slice::<_, u8>(i.as_slice());
                    levels.push(level);
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::Rgba16 => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                ktx.format(VkFormat::R16G16B16A16_UNORM);

                let mut levels = Vec::new();
                for i in loaded.iter_mut() {
                    let i = i.as_mut_rgba16().unwrap();

                    // byte swap to convert our BE to LE when needed
                    if cfg!(target_endian = "big") {
                        for p in i.pixels_mut() {
                            p.0[0] = p.0[0].to_le();
                            p.0[1] = p.0[1].to_le();
                            p.0[2] = p.0[2].to_le();
                            p.0[3] = p.0[3].to_le();
                        }
                    }

                    let level = bytemuck::cast_slice::<_, u8>(i.as_raw().as_slice());
                    levels.push(level);
                }

                ktx.image_2d(base_width, base_height, &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            image::ColorType::Rgb32F => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                if to_half {
                    ktx.format(VkFormat::R16G16B16A16_SFLOAT);

                    let mut swizzled_levels = Vec::new();
                    for i in loaded.iter_mut() {
                        let i = i.as_rgb32f().unwrap();

                        let halved = cast_f32_buffer_to_f16(i.as_raw());

                        // Default value is 1.0f with a little endian swap if we're on a big endian
                        // platform. All data must be in little endian order.
                        let default_value = {
                            let v = f16::from_f32(1.0);
                            let v = v.to_le_bytes();
                            bytemuck::cast::<_, f16>(v)
                        };

                        let level = swizzle_rgb_to_rgba(
                            halved.as_slice(),
                            i.width(),
                            i.height(),
                            default_value,
                        );

                        swizzled_levels.push(level);
                    }

                    let mut levels = Vec::new();
                    for i in swizzled_levels.iter() {
                        let level = bytemuck::cast_slice::<_, u8>(i.as_slice());
                        levels.push(level);
                    }

                    ktx.image_2d(base_width, base_height, &levels);

                    let mut writer = BufWriter::new(output_file);
                    ktx.write(&mut writer)?;
                } else {
                    ktx.format(VkFormat::R32G32B32A32_SFLOAT);

                    let mut swizzled_levels = Vec::new();
                    for i in loaded.iter_mut() {
                        let i = i.as_mut_rgb32f().unwrap();

                        // byte swap to convert our BE to LE when needed
                        if cfg!(target_endian = "big") {
                            for p in i.pixels_mut() {
                                p.0[0] = bytemuck::cast::<_, f32>(p.0[0].to_le_bytes());
                                p.0[1] = bytemuck::cast::<_, f32>(p.0[1].to_le_bytes());
                                p.0[2] = bytemuck::cast::<_, f32>(p.0[2].to_le_bytes());
                            }
                        }

                        // Default value is 1.0f with a little endian swap if we're on a big endian
                        // platform. All data must be in little endian order.
                        let default_value = {
                            let v = 1.0f32;
                            let v = v.to_le_bytes();
                            bytemuck::cast::<_, f32>(v)
                        };

                        let level = swizzle_rgb_to_rgba(
                            i.as_raw().as_slice(),
                            i.width(),
                            i.height(),
                            default_value,
                        );

                        swizzled_levels.push(level);
                    }

                    let mut levels = Vec::new();
                    for i in swizzled_levels.iter() {
                        let level = bytemuck::cast_slice::<_, u8>(i.as_slice());
                        levels.push(level);
                    }

                    ktx.image_2d(base_width, base_height, &levels);

                    let mut writer = BufWriter::new(output_file);
                    ktx.write(&mut writer)?;
                }
            }
            image::ColorType::Rgba32F => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                if to_half {
                    ktx.format(VkFormat::R16G16B16A16_SFLOAT);

                    let mut halved_levels = Vec::new();
                    for i in loaded.iter_mut() {
                        let i = i.as_mut_rgba32f().unwrap();

                        let halved = cast_f32_buffer_to_f16(i.as_raw());
                        halved_levels.push(halved);
                    }

                    let mut levels = Vec::new();
                    for halved in halved_levels.iter() {
                        let level = bytemuck::cast_slice::<_, u8>(halved.as_slice());
                        levels.push(level);
                    }

                    ktx.image_2d(base_width, base_height, &levels);

                    let mut writer = BufWriter::new(output_file);
                    ktx.write(&mut writer)?;
                } else {
                    ktx.format(VkFormat::R32G32B32A32_SFLOAT);

                    let mut levels = Vec::new();
                    for i in loaded.iter_mut() {
                        let i = i.as_mut_rgba32f().unwrap();

                        // byte swap to convert our BE to LE when needed
                        if cfg!(target_endian = "big") {
                            for p in i.pixels_mut() {
                                p.0[0] = bytemuck::cast::<_, f32>(p.0[0].to_le_bytes());
                                p.0[1] = bytemuck::cast::<_, f32>(p.0[1].to_le_bytes());
                                p.0[2] = bytemuck::cast::<_, f32>(p.0[2].to_le_bytes());
                                p.0[3] = bytemuck::cast::<_, f32>(p.0[3].to_le_bytes());
                            }
                        }

                        let level = bytemuck::cast_slice::<_, u8>(i.as_raw().as_slice());
                        levels.push(level);
                    }

                    ktx.image_2d(base_width, base_height, &levels);

                    let mut writer = BufWriter::new(output_file);
                    ktx.write(&mut writer)?;
                }
            }
            _ => unimplemented!("Unknown Pixel Format"),
        }

        Ok(())
    }

    fn dont_log(&self) -> bool {
        true
    }
}

fn cast_f32_buffer_to_f16(v: &[f32]) -> Vec<f16> {
    Vec::from_iter(v.iter().map(|v| {
        let v = f16::from_f32(*v);
        if cfg!(target_endian = "big") {
            bytemuck::cast::<_, f16>(v.to_le_bytes())
        } else {
            v
        }
    }))
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
