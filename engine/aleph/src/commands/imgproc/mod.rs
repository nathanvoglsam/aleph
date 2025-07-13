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

mod gen_mips;
mod integrate_irradiance;
mod reproject_env;

use std::fs::File;
use std::io::{BufWriter, Read, Seek};
use std::path::Path;

use aleph_image::{
    ColorType, DynamicTextureBuffer, EnvironmentMapProjection, ImageBuffer, PixR, PixRG, PixRGB,
    PixRGBA, PixelChannelType, PixelFormat, ResizeFilter, SphericalMapping, TextureBuffer,
    TextureType, layer_and_level_from_set_index,
};
use aleph_ktx::{KtxDocument, KtxDocumentDescription, VkFormat};
use aleph_math::UVec2;
use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Arg, ArgAction, ArgMatches};
use half::f16;

use crate::commands::SubcommandSet;
use crate::commands::imgproc::gen_mips::GenMips;
use crate::commands::imgproc::integrate_irradiance::IntegrateIrradiance;
use crate::commands::imgproc::reproject_env::ReprojectEnv;

pub fn make() -> SubcommandSet {
    let mut subcommands =
        SubcommandSet::new("imgproc").about("Commands for processing images within");
    subcommands.register_subcommand(IntegrateIrradiance);
    subcommands.register_subcommand(ReprojectEnv);
    subcommands.register_subcommand(GenMips);
    subcommands
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

fn parse_spherical_type(v: &str) -> Option<SphericalMapping> {
    let v = match v {
        "equi" | "equirectangular" => SphericalMapping::Equirectangular,
        "oct" | "octahedral" => SphericalMapping::Octahedral,
        _ => return None,
    };
    Some(v)
}

fn parse_env_type(v: &str) -> Option<EnvironmentMapProjection> {
    let v = match v {
        "equi" | "equirectangular" => EnvironmentMapProjection::Equirectangular,
        "oct" | "octahedral" => EnvironmentMapProjection::Octahedral,
        "cube" => EnvironmentMapProjection::Cube,
        _ => return None,
    };
    Some(v)
}

fn load_ktx_document_to_texture<P: AsRef<Path>>(file: P) -> anyhow::Result<DynamicTextureBuffer> {
    let file = File::open(file)?;
    let mapped = unsafe { memmap2::Mmap::map(&file)? };
    let doc = KtxDocument::from_slice(&mapped)?;

    let tex = dynamic_texture_buffer_from_ktx(&mapped, &doc)?;

    tex.validate_image_count();

    Ok(tex)
}

fn dynamic_texture_buffer_from_ktx<R: Read + Seek>(
    data: &[u8],
    doc: &KtxDocument<R>,
) -> anyhow::Result<DynamicTextureBuffer> {
    let img = match doc.format() {
        VkFormat::R8_UNORM | VkFormat::R8_SRGB => {
            type C = u8;
            type P = PixR<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::R8Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R8G8_UNORM | VkFormat::R8G8_SRGB => {
            type C = u8;
            type P = PixRG<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RG8Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R8G8B8_UNORM | VkFormat::R8G8B8_SRGB => {
            type C = u8;
            type P = PixRGB<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGB8Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R8G8B8A8_UNORM | VkFormat::R8G8B8A8_SRGB => {
            type C = u8;
            type P = PixRGBA<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGBA8Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16_UNORM => {
            type C = u16;
            type P = PixR<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::R16Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16_UNORM => {
            type C = u16;
            type P = PixRG<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RG16Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16B16_UNORM => {
            type C = u16;
            type P = PixRGB<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGB16Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16B16A16_UNORM => {
            type C = u16;
            type P = PixRGBA<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGBA16Unorm(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16_SFLOAT => {
            type C = f16;
            type P = PixR<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::R16Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16_SFLOAT => {
            type C = f16;
            type P = PixRG<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RG16Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16B16_SFLOAT => {
            type C = f16;
            type P = PixRGB<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGB16Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R16G16B16A16_SFLOAT => {
            type C = f16;
            type P = PixRGBA<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGBA16Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R32_SFLOAT => {
            type C = f32;
            type P = PixR<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::R32Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R32G32_SFLOAT => {
            type C = f32;
            type P = PixRG<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RG32Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R32G32B32_SFLOAT => {
            type C = f32;
            type P = PixRGB<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGB32Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        VkFormat::R32G32B32A32_SFLOAT => {
            type C = f32;
            type P = PixRGBA<C>;
            let images = load_all_from_ktx::<R, C, P>(data, doc)?;
            DynamicTextureBuffer::RGBA32Float(bundle_all_from_ktx::<R, P>(doc, images)?)
        }
        _ => {
            log::error!("Unsupported format '{}'", doc.format());
            return Err(anyhow!("Unsupported format '{}'", doc.format()));
        }
    };

    img.validate_image_count();

    Ok(img)
}

fn bundle_all_from_ktx<R, P>(
    doc: &KtxDocument<R>,
    images: Vec<ImageBuffer<P>>,
) -> anyhow::Result<TextureBuffer<P>>
where
    R: Read + Seek,
    P: PixelFormat,
{
    let tex = match doc.document_type() {
        aleph_ktx::DocumentType::Image1D => unimplemented!(),
        aleph_ktx::DocumentType::Image3D => unimplemented!(),
        aleph_ktx::DocumentType::Array1D => unimplemented!(),
        aleph_ktx::DocumentType::Array3D => unimplemented!(),
        aleph_ktx::DocumentType::Image2D => {
            let dimensions = UVec2::new(doc.width(), doc.height());
            let level_num = doc.level_num();
            let tex = TextureBuffer::Single {
                dimensions,
                level_num,
                images,
            };
            tex
        }
        aleph_ktx::DocumentType::Array2D => {
            let dimensions = UVec2::new(doc.width(), doc.height());
            let level_num = doc.level_num();
            let layer_num = doc.layer_num();
            let tex = TextureBuffer::Array {
                dimensions,
                level_num,
                layer_num,
                images,
            };
            tex
        }
        aleph_ktx::DocumentType::Cube => {
            let dimensions = UVec2::new(doc.width(), doc.height());
            let level_num = doc.level_num();
            let tex = TextureBuffer::Cube {
                dimensions,
                level_num,
                images,
            };
            tex
        }
        aleph_ktx::DocumentType::CubeArray => {
            let dimensions = UVec2::new(doc.width(), doc.height());
            let level_num = doc.level_num();
            let cube_num = doc.layer_num() / doc.face_num();
            let tex = TextureBuffer::CubeArray {
                dimensions,
                level_num,
                cube_num,
                images,
            };
            tex
        }
    };

    tex.validate_image_count();

    Ok(tex)
}

fn load_all_from_ktx<R, C, P>(
    data: &[u8],
    doc: &KtxDocument<R>,
) -> anyhow::Result<Vec<ImageBuffer<P>>>
where
    R: Read + Seek,
    C: PixelChannelType,
    P: PixelFormat<Storage = C>,
{
    let layer_num = doc.layer_num() as usize;
    let level_num = doc.level_num() as usize;
    let image_count = layer_num * level_num;
    let mut images = Vec::new();
    for i in 0..image_count {
        let (layer, level) = layer_and_level_from_set_index(layer_num, level_num, i);
        let layer = u32::try_from(layer).unwrap();
        let level = u32::try_from(level).unwrap();

        let src = doc
            .get_level_info(level)
            .inspect_err(|e| log::error!("Failed to get level {level} in KTX doc: {e:?}"))?;
        let src = &data[src.to_slice_range()];

        let img = load_from_ktx::<R, C, P>(src, doc, layer);
        images.push(img);
    }
    Ok(images)
}

fn load_from_ktx<R, C, P>(level_data: &[u8], doc: &KtxDocument<R>, layer: u32) -> ImageBuffer<P>
where
    R: Read + Seek,
    C: PixelChannelType,
    P: PixelFormat<Storage = C>,
{
    // Number of elements we expect the texture to have. This is the count of individual 'C' values
    // we expect the data to contain. The actual byte size of the texture is...
    let elems = ImageBuffer::<P>::calculate_element_count(doc.width(), doc.height());

    // Calculated here. The size in bytes we expect the texture to be.
    let bytes = elems * std::mem::size_of::<C>();

    // This is the buffer we copy the texture into. This function assumes that the document is
    // mmap-ed, which is not suitable for efficiently processing.
    //
    // We also grab a view of the data as bytes.
    let mut data = Vec::<C>::new();
    data.resize(elems, C::default());
    let data_bytes = bytemuck::cast_slice_mut::<_, u8>(data.as_mut_slice());

    // Take a slice of exactly the number of bytes we expect the image to contain, so that we can...
    let b = bytes * layer as usize;
    let e = b + bytes;
    let src = &level_data[b..e];

    // Copy out of the mmapped document and into our destination image.
    data_bytes.copy_from_slice(src);

    // Finally we can construct the image
    ImageBuffer::<P>::from_data(doc.width(), doc.height(), data)
}

fn prepare_texture_for_gpu(tex: &mut DynamicTextureBuffer, to_half: bool) -> anyhow::Result<()> {
    // Swizzle 3 channel formats up to 4 channels as there are almost zero GPUs on the planet
    // that can sample from 3 channel formats
    match tex {
        DynamicTextureBuffer::RGB8Unorm(v) => {
            *tex = DynamicTextureBuffer::RGBA8Unorm(v.swizzle_rgb_to_rgba(1)?)
        }
        DynamicTextureBuffer::RGB16Unorm(v) => {
            *tex = DynamicTextureBuffer::RGBA16Unorm(v.swizzle_rgb_to_rgba(1)?)
        }
        DynamicTextureBuffer::RGB32Unorm(v) => {
            *tex = DynamicTextureBuffer::RGBA32Unorm(v.swizzle_rgb_to_rgba(1)?)
        }
        DynamicTextureBuffer::RGB16Float(v) => {
            *tex = DynamicTextureBuffer::RGBA16Float(v.swizzle_rgb_to_rgba(f16::from_f32(1.0))?)
        }
        DynamicTextureBuffer::RGB32Float(v) => {
            *tex = DynamicTextureBuffer::RGBA32Float(v.swizzle_rgb_to_rgba(1.0)?)
        }
        _ => {}
    }

    // Convert to half precision as the very final step before the le byteswap.
    if to_half {
        *tex = tex.to_half()?;
    }

    tex.to_little_endian()?;

    Ok(())
}

fn write_texture_to_ktx_file<P: AsRef<Path>>(
    tex: &DynamicTextureBuffer,
    dst: P,
) -> anyhow::Result<()> {
    tex.validate_image_count();

    let final_color_type = tex.get_color_type();
    let image_references = tex.get_buffer_references();

    // Setup mip state in common code to keep the match arms shorter
    let mut ktx = KtxDocumentDescription::new();

    match final_color_type {
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

    match tex.get_texture_type() {
        TextureType::Single => {
            let dimensions = tex.dimensions();
            let level_num = tex.level_num();

            log::info!("Writing Image");
            ktx.image_2d(dimensions.x, dimensions.y, level_num, &image_references);
        }
        TextureType::Array => {
            let dimensions = tex.dimensions();
            let layer_num = tex.layer_num();
            let level_num = tex.level_num();
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
            let dimensions = tex.dimensions();
            let level_num = tex.level_num();
            log::info!("Writing Cube");
            ktx.cube(dimensions.x, dimensions.y, level_num, &image_references);
        }
        TextureType::CubeArray => {
            let dimensions = tex.dimensions();
            let layer_num = tex.layer_num();
            let level_num = tex.level_num();
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
        .open(dst)?;

    let mut writer = BufWriter::new(output_file);
    ktx.write(&mut writer)?;

    Ok(())
}

fn input_arg() -> Arg {
    Arg::new("input")
        .short('i')
        .long("input")
        .help("The input file.")
        .long_help("The input file. Supports ktx2.")
        .required(true)
}

fn output_arg() -> Arg {
    Arg::new("output")
        .short('o')
        .long("output")
        .help("The output file.")
        .long_help("The output file. If unspecified the filename is derived from the input name.")
        .required(false)
}

fn gen_mips_arg() -> Arg {
    Arg::new("gen-mips")
        .action(ArgAction::SetTrue)
        .long("gen-mips")
        .help("Whether to generate a mip chain from the input image.")
        .long_help("Whether to generate a mip chain from the input image. Uses a bilinear filter by default.")
}

fn mip_filter_arg() -> Arg {
    Arg::new("mip-filter")
        .long("mip-filter")
        .help("The type of filter to use when downsampling mips.")
        .long_help("The type of filter to use when downsampling mips. Options: nearest, bilinear, cubic, gaussian, lanczos3")
        .default_value("bilinear")
        .required(false)
}

fn input_env_map_type_arg() -> Arg {
    Arg::new("in-proj")
        .long("in-proj")
        .help("The type of environment map projection the input texture contains.")
        .long_help("The type of environment map projection the input texture contains. Options: equi/equirectangular, oct/octahedral")
        .default_value("equirectangular")
        .required(false)
}

fn output_env_map_type_arg() -> Arg {
    Arg::new("out-proj")
        .long("out-proj")
        .help("The environment map projection to output to.")
        .long_help("The environment map projection to output to. Options: equi/equirectangular, oct/octahedral, cube")
        .default_value("cube")
        .required(false)
}

fn to_half_arg() -> Arg {
    Arg::new("to-half")
        .action(ArgAction::SetTrue)
        .long("to-half")
        .help("Declares that floating point input should be output in half-precision.")
        .long_help("Declares that floating point input should be output in half-precision. This only affects floating point input images like HDRIs.")
}

fn is_normal_map_arg() -> Arg {
    Arg::new("is-normal-map")
        .action(ArgAction::SetTrue)
        .long("is-normal-map")
        .help("Declares that the input image is a normal map.")
        .long_help("Declares that the input image is a normal map. This changes some things, like an additonal normalization operation when generating mips.")
}

fn get_input_match(matches: &mut ArgMatches) -> Utf8PathBuf {
    let input: String = matches.remove_one("input").expect("input is required");
    let input = Utf8Path::new(&input).to_path_buf();
    input
}

fn get_output_match(matches: &mut ArgMatches, input: &Utf8PathBuf) -> Utf8PathBuf {
    let output_arg: Option<String> = matches.remove_one("output");
    let output = match output_arg {
        Some(v) => Utf8Path::new(&v).to_path_buf(),
        None => {
            // Take the name of the first input file
            input.with_extension("ktx2")
        }
    };
    output
}

fn get_gen_mips_matches(matches: &mut ArgMatches) -> anyhow::Result<(bool, ResizeFilter)> {
    let gen_mips = matches.get_flag("gen-mips");

    let mip_filter = get_mip_filter_matches(matches)?;

    Ok((gen_mips, mip_filter))
}

fn get_mip_filter_matches(matches: &mut ArgMatches) -> anyhow::Result<ResizeFilter> {
    let mip_filter: String = matches.remove_one("mip-filter").unwrap();
    let mip_filter = mip_filter.to_lowercase();
    let mip_filter =
        parse_filter(&mip_filter).ok_or_else(|| anyhow!("Unknown filter \"{}\"", &mip_filter))?;
    Ok(mip_filter)
}

fn get_input_env_map_type_matches(matches: &mut ArgMatches) -> anyhow::Result<SphericalMapping> {
    let v: String = matches.remove_one("in-proj").unwrap();
    let v = v.to_lowercase();
    let v = parse_spherical_type(&v).ok_or_else(|| anyhow!("Unknown env map type \"{}\"", &v))?;
    Ok(v)
}

fn get_output_env_map_type_matches(
    matches: &mut ArgMatches,
) -> anyhow::Result<EnvironmentMapProjection> {
    let v: String = matches.remove_one("out-proj").unwrap();
    let v = v.to_lowercase();
    let v = parse_env_type(&v).ok_or_else(|| anyhow!("Unknown env map type \"{}\"", &v))?;
    Ok(v)
}

fn get_to_half_match(matches: &mut ArgMatches) -> bool {
    matches.get_flag("to-half")
}

fn get_is_normal_map_match(matches: &mut ArgMatches) -> bool {
    matches.get_flag("is-normal-map")
}
