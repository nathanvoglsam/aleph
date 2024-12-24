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

mod build;
mod genproj;

use crate::commands::shaders::build::BuildShaderProj;
use crate::commands::shaders::genproj::GenShaderProj;
use crate::commands::SubcommandSet;
use crate::shader_system::{ShaderFile, ShaderModuleContext};

pub fn make() -> SubcommandSet {
    let mut subcommands = SubcommandSet::new("shaders")
        .about("Commands for handling shaders within an aleph-engine project");
    subcommands.register_subcommand(GenShaderProj {});
    subcommands.register_subcommand(BuildShaderProj {});
    subcommands
}

fn shader_name_for_file_in_module<const IS_SOURCE_FILE: bool>(
    module: &ShaderModuleContext,
    shader_file: &ShaderFile,
) -> anyhow::Result<String> {
    use std::fmt::Write;

    // if we have a source file or a binary file we need to use a different prefix to get our
    // stripped path
    let prefix = if IS_SOURCE_FILE {
        module.meta.source_dir
    } else {
        module.meta.output_dir
    };

    let module_name = module.meta.output_dir.file_name().unwrap();
    let entry_path_tail = shader_file.path.strip_prefix(prefix)?;

    let mut shader_name = format!("{module_name}/");
    for component in entry_path_tail.parent().unwrap().components() {
        write!(&mut shader_name, "{component}/")?;
    }
    shader_name.push_str(shader_file.name_with_type);

    Ok(shader_name)
}

fn shader_name_for_src_file_in_module(
    module: &ShaderModuleContext,
    shader_file: &ShaderFile,
) -> anyhow::Result<String> {
    shader_name_for_file_in_module::<true>(module, shader_file)
}

fn shader_name_for_bin_file_in_module(
    module: &ShaderModuleContext<'_>,
    shader_file: &ShaderFile<'_>,
) -> anyhow::Result<String> {
    shader_name_for_file_in_module::<false>(module, shader_file)
}
