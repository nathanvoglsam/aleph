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
use camino::Utf8Path;
use clap::{Arg, ArgMatches, Command};
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
            .long_help("The input file. Supports png, jpg, bmp, jpeg, tga, tiff")
            .required(true);
        let output = Arg::new("output")
            .short('o')
            .long("output")
            .help("The output file.")
            .long_help("The output file. Supports ktx2")
            .required(false);
        Command::new(self.name())
            .about("Converts the given input image into the KTX2 format")
            .arg(input)
            .arg(output)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let input_arg: String = matches.remove_one("input").expect("platform is required");
        let input = Utf8Path::new(&input_arg).to_path_buf();

        let output_arg: Option<String> = matches.remove_one("output");
        let output = match output_arg {
            Some(v) => Utf8Path::new(&v).to_path_buf(),
            None => input.with_extension("ktx2"),
        };

        let loaded = image::ImageReader::open(&input)?
            .with_guessed_format()?
            .decode()?;

        match loaded {
            DynamicImage::ImageLuma8(i) => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                let mut ktx = KtxDocumentDescription::new();
                ktx.format(VkFormat::R8_UNORM);
                ktx.mip_generate();

                let levels = [i.as_raw().as_slice()];

                ktx.image_2d(i.width(), i.height(), &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            DynamicImage::ImageLumaA8(_i) => unimplemented!("ImageLumaA8"),
            DynamicImage::ImageRgb8(i) => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                let mut ktx = KtxDocumentDescription::new();
                ktx.format(VkFormat::R8G8B8A8_UNORM);
                ktx.mip_generate();

                let mut swizzled = Vec::new();

                let bytes = i.width() as usize * i.height() as usize * 4;
                swizzled.resize(bytes, 0xFF);

                let src_row_width = i.width() as usize * 3;
                let dst_row_width = i.width() as usize * 4;
                let src = i.as_raw().as_slice();

                for row in 0..i.height() as usize {
                    let dst_row_start = row * dst_row_width;
                    let dst_row_end = dst_row_start + dst_row_width;
                    let dst = &mut swizzled[dst_row_start..dst_row_end];

                    let src_row_start = row * src_row_width;
                    let src_row_end = src_row_start + src_row_width;
                    let src = &src[src_row_start..src_row_end];

                    for col in 0..i.width() as usize {
                        let dst_base = col as usize * 4;
                        let dst = &mut dst[dst_base..dst_base + 3];

                        let src_base = col as usize * 3;
                        let src = &src[src_base..src_base + 3];

                        dst.copy_from_slice(src);
                    }
                }

                let levels = [swizzled.as_slice()];

                ktx.image_2d(i.width(), i.height(), &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            DynamicImage::ImageRgba8(i) => {
                let output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&output)?;

                let mut ktx = KtxDocumentDescription::new();
                ktx.format(VkFormat::R8G8B8A8_UNORM);
                ktx.mip_generate();

                let levels = [i.as_raw().as_slice()];

                ktx.image_2d(i.width(), i.height(), &levels);

                let mut writer = BufWriter::new(output_file);
                ktx.write(&mut writer)?;
            }
            DynamicImage::ImageLuma16(_i) => unimplemented!("ImageLuma16"),
            DynamicImage::ImageLumaA16(_i) => unimplemented!("ImageLumaA16"),
            DynamicImage::ImageRgb16(_i) => unimplemented!("ImageRgb16"),
            DynamicImage::ImageRgba16(_i) => unimplemented!("ImageRgba16"),
            DynamicImage::ImageRgb32F(_i) => unimplemented!("ImageRgb32F"),
            DynamicImage::ImageRgba32F(_i) => unimplemented!("ImageRgba32F"),
            _ => todo!(),
        };

        Ok(())
    }

    fn dont_log(&self) -> bool {
        true
    }
}
