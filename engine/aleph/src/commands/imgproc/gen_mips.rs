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

use aleph_image::TextureBuffer;
use anyhow::anyhow;
use clap::{ArgMatches, Command};

use crate::commands::imgproc::{
    get_input_match, get_is_normal_map_match, get_mip_filter_matches, get_output_match,
    get_to_half_match, input_arg, is_normal_map_arg, load_ktx_document_to_texture, mip_filter_arg,
    output_arg, prepare_texture_for_gpu, to_half_arg, write_texture_to_ktx_file,
};
use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct GenMips;

impl ISubcommand for GenMips {
    fn name(&self) -> &'static str {
        "gen_mips"
    }

    fn description(&mut self) -> Command {
        let input = input_arg();
        let output = output_arg();
        let mip_filter = mip_filter_arg();
        let to_half = to_half_arg();
        let is_normal_map = is_normal_map_arg();
        Command::new(self.name())
            .about("Converts the given input image with no mip levels, generate those mips from the base level using the requested filter")
            .arg(input)
            .arg(output)
            .arg(mip_filter)
            .arg(to_half)
            .arg(is_normal_map)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        // HANDLE INPUT ARGS
        let input = get_input_match(&mut matches);
        let output = get_output_match(&mut matches, &input);
        let mip_filter = get_mip_filter_matches(&mut matches)?;
        let to_half = get_to_half_match(&mut matches);
        let is_normal_map = get_is_normal_map_match(&mut matches);

        // LOAD TEXTURES AND VALIDATE INPUT IS COMPATIBLE WITH THE PROCESS
        let mut images = load_ktx_document_to_texture(&input)?;

        match &images {
            TextureBuffer::Single { level_num, .. }
            | TextureBuffer::Array { level_num, .. }
            | TextureBuffer::Cube { level_num, .. }
            | TextureBuffer::CubeArray { level_num, .. } => {
                if *level_num > 1 {
                    log::error!("Can't run 'gen_mips' on a texture with mip maps!");
                    return Err(anyhow!("Can't run 'gen_mips' on a texture with mip maps!"));
                }
            }
        }

        // PERFORM THE TEXTURE PROCESSING
        images.generate_mips(mip_filter.into());

        if is_normal_map {
            images.normalize()?;
        }

        // OUTPUT MAPPING AND TYPE CONVERSIONS
        prepare_texture_for_gpu(&mut images, to_half)?;

        // WRITE THE RESULTS TO DISK
        write_texture_to_ktx_file(&images, &output)?;

        Ok(())
    }

    fn dont_log(&self) -> bool {
        false
    }
}
