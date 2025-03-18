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

use aleph_image::{SphericalMapping, TextureType};
use aleph_math::UVec2;
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};

use crate::commands::imgproc::{
    gen_mips_arg, get_gen_mips_matches, get_input_match, get_output_match, get_to_half_match,
    input_arg, load_ktx_document_to_texture, mip_filter_arg, output_arg, prepare_texture_for_gpu,
    to_half_arg, write_texture_to_ktx_file,
};
use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct EquiToCube;

impl ISubcommand for EquiToCube {
    fn name(&self) -> &'static str {
        "equi_to_cube"
    }

    fn description(&mut self) -> Command {
        let input = input_arg();
        let output = output_arg();
        let gen_mips = gen_mips_arg();
        let mip_filter = mip_filter_arg();
        let size = Arg::new("size")
            .long("size")
            .help("The width/height of a cube map face to output when generating cube maps.")
            .long_help("The width/height of a cube map face to output when generating cube maps. Defaults to 512.")
            .value_parser(clap::value_parser!(u32))
            .default_value("512")
            .required(false);
        let to_half = to_half_arg();
        Command::new(self.name())
            .about("Converts the given input image, assuming it is a 2D equirectangular map, into an equivalent cube map")
            .arg(input)
            .arg(output)
            .arg(gen_mips)
            .arg(mip_filter)
            .arg(to_half)
            .arg(size)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        // HANDLE INPUT ARGS
        let input = get_input_match(&mut matches);
        let output = get_output_match(&mut matches, &input);
        let (gen_mips, mip_filter) = get_gen_mips_matches(&mut matches)?;
        let to_half = get_to_half_match(&mut matches);

        let size: u32 = matches.remove_one("size").unwrap();

        // LOAD TEXTURES AND VALIDATE INPUT IS COMPATIBLE WITH THE PROCESS
        let mut images = load_ktx_document_to_texture(&input)?;

        match images.get_texture_type() {
            TextureType::Single | TextureType::Array => {
                let level_num = images.level_num();
                if level_num > 1 {
                    log::error!("Can't run 'equi_to_cube' on a texture with mip maps!");
                    return Err(anyhow!(
                        "Can't run 'equi_to_cube' on a texture with mip maps!"
                    ));
                }
            }
            TextureType::Cube | TextureType::CubeArray => {
                log::error!("Can't run 'equi_to_cube' on a cubemap input!");
                return Err(anyhow!("Can't run 'equi_to_cube' on a cubemap input!"));
            }
        }

        // PERFORM THE TEXTURE PROCESSING
        let face_dimensions = UVec2::new(size, size);
        images.spherical_map_to_cube_map(SphericalMapping::Equirectangular, face_dimensions)?;

        if gen_mips {
            images.generate_mips(mip_filter.into())?;
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
