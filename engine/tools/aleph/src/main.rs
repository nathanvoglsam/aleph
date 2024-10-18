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

use crate::commands::{Build, Bundle, GenConfigs, GenProj, GenVsCode, SubcommandSet, Uuid};

mod commands;
mod config_subproject;
mod crate_metadata;
mod project;
mod project_schema;
mod shader_system;
mod subproject;
mod templates;
mod utils;
mod vscode_settings;

// TODO: refactor the shader context stuff to use arenas and violently eject all the Cow crap from
//       the whole thing because it's fucking awful. Should heavily simplify sharing the shader
//       context around.
//
//       ideally we also end up with a framework for future project systems (haxe *cough*)

fn main() -> anyhow::Result<()> {
    let mut subcommands = SubcommandSet::new(env!("CARGO_PKG_NAME"));
    subcommands.register_subcommand(Uuid {});
    subcommands.register_subcommand(GenProj {});
    subcommands.register_subcommand(GenVsCode {});
    subcommands.register_subcommand(GenConfigs {});
    subcommands.register_subcommand(Build {});
    subcommands.register_subcommand(Bundle {});
    subcommands.register_subcommand(commands::shaders::make());
    subcommands.exec_as_root()
}
