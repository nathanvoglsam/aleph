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

use aleph_math::UVec2;
use clap::{Arg, ArgMatches, Command};

use crate::commands::imgproc::{
    get_input_env_map_type_matches, get_input_match, get_output_env_map_type_matches,
    get_output_match, get_to_half_match, input_arg, input_env_map_type_arg,
    load_ktx_document_to_texture, output_arg, output_env_map_type_arg, prepare_texture_for_gpu,
    to_half_arg, write_texture_to_ktx_file,
};
use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct IntegrateIrradiance;

impl ISubcommand for IntegrateIrradiance {
    fn name(&self) -> &'static str {
        "integrate_irradiance"
    }

    fn description(&mut self) -> Command {
        let input = input_arg();
        let output = output_arg();
        let in_proj = input_env_map_type_arg();
        let out_proj = output_env_map_type_arg();
        let size = Arg::new("size")
            .long("size")
            .help("The width/height of a cube map face to output when generating cube maps.")
            .long_help("The width/height of a cube map face to output when generating cube maps. Defaults to 512.")
            .value_parser(clap::value_parser!(u32))
            .default_value("512")
            .required(false);
        let samples = Arg::new("samples")
            .long("samples")
            .help("The number of samples to take per pixel in the output image.")
            .long_help("The number of samples to take per pixel in the output image.")
            .value_parser(clap::value_parser!(u32))
            .default_value("2048")
            .required(false);
        let to_half = to_half_arg();
        Command::new(self.name())
            .about("Convolves the given environment map into a diffuse irradiance map.")
            .arg(input)
            .arg(output)
            .arg(in_proj)
            .arg(out_proj)
            .arg(to_half)
            .arg(size)
            .arg(samples)
    }

    fn exec(&mut self, _project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        // HANDLE INPUT ARGS
        let input = get_input_match(&mut matches);
        let output = get_output_match(&mut matches, &input);
        let to_half = get_to_half_match(&mut matches);
        let in_proj = get_input_env_map_type_matches(&mut matches).unwrap();
        let out_proj = get_output_env_map_type_matches(&mut matches).unwrap();

        let size: u32 = matches.remove_one("size").unwrap();
        let samples: u32 = matches.remove_one("samples").unwrap();

        // LOAD TEXTURES AND VALIDATE INPUT IS COMPATIBLE WITH THE PROCESS
        let mut images = load_ktx_document_to_texture(&input)?;

        // PERFORM THE TEXTURE PROCESSING
        let face_dimensions = UVec2::new(size, size);
        images =
            images.integrate_irradiance(in_proj, out_proj, face_dimensions, samples as usize)?;

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
