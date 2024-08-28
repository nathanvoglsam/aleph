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

use bumpalo::Bump;
use serde::{Deserialize, Serialize};

use crate::haxe::HaxeSubproject;
use crate::project::AlephProject;
use crate::shader_system::ShaderSubproject;
use crate::utils::dunce_utf8;

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeWorkspace {
    pub folders: Vec<WorkspaceFolder>,
    pub settings: StandardSettings,
}

impl CodeWorkspace {
    pub fn from_project(project: &AlephProject) -> anyhow::Result<Self> {
        let path = project.project_root();
        let path = dunce_utf8::simplified(path).to_path_buf().into_string();
        let folders = vec![WorkspaceFolder { path }];

        let out = Self {
            folders,
            settings: StandardSettings::from_project(project)?,
        };
        Ok(out)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorkspaceFolder {
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StandardSettings {
    /// Tell the cmake plugin to shut up, we're a rust project (this will always be false)
    #[serde(rename = "cmake.configureOnOpen")]
    pub cmake_configure_on_open: bool,

    /// Path to slangd so the slang plugin has a language server that matches the compiler we use
    #[serde(rename = "slang.slangdLocation")]
    pub slang_slangd_location: String,

    /// List of include directories that slangd will search in as include roots. Always false
    #[serde(rename = "slang.additionalSearchPaths")]
    pub slang_additional_search_paths: Vec<String>,

    /// Tells the slang plugin _not_ to search in all directories so we get correct import
    /// resolutions
    #[serde(rename = "slang.searchInAllWorkspaceDirectories")]
    pub slang_search_in_all_workspace_directories: bool,

    /// Path to the 'haxe' executable that the haxe plugin needs for the language server
    #[serde(rename = "haxe.executable")]
    pub haxe_executable: String,

    /// List of (lists of?) haxe hxml configurations that the haxe plugin uses for context for
    /// autocomplete
    #[serde(rename = "haxe.configurations")]
    pub haxe_configurations: Vec<Vec<String>>,

    /// List of directories that is safe for haxe rename-symbol tools to edit within.
    #[serde(rename = "haxe.renameSourceFolders")]
    pub haxe_rename_source_folders: Vec<String>,

    /// Setting that forces rust-analyzer to use a separate target dir. Always set to true.
    #[serde(rename = "rust-analyzer.cargo.targetDir")]
    pub rust_analyzer_cargo_target_dir: bool,
}

impl StandardSettings {
    pub fn from_project(project: &AlephProject) -> anyhow::Result<Self> {
        let arena = Bump::new();
        let shaders_ctx = ShaderSubproject::load(&arena, project)?;
        let haxe_ctx = HaxeSubproject::load(&arena, project)?;

        let slangd_exe = project.slang_path().parent().unwrap().join("slangd");
        let slangd_exe = dunce_utf8::simplified(&slangd_exe)
            .to_path_buf()
            .into_string();

        let mut slang_search_paths = Vec::new();
        for shader_crate in shaders_ctx.crates {
            for shader_module in shader_crate.modules {
                let include_dir = dunce_utf8::simplified(shader_module.meta.include_dir)
                    .to_path_buf()
                    .into_string();
                slang_search_paths.push(include_dir);
            }
        }

        let mut haxe_rename_folders = Vec::new();
        for haxe_crate in haxe_ctx.crates {
            for haxe_module in haxe_crate.modules {
                let rename_dir = dunce_utf8::simplified(haxe_module.meta.source_dir)
                    .to_path_buf()
                    .into_string();
                haxe_rename_folders.push(rename_dir);
            }
        }

        let haxe_exe = project.haxe_path();
        let haxe_exe = dunce_utf8::simplified(&haxe_exe)
            .to_path_buf()
            .into_string();

        let mut haxe_configurations = Vec::new();
        haxe_configurations.push(vec!["build_hl.hxml".to_string()]);
        haxe_configurations.push(vec!["build_js.hxml".to_string()]);

        let out = Self {
            cmake_configure_on_open: false,
            slang_slangd_location: slangd_exe,
            slang_additional_search_paths: slang_search_paths,
            slang_search_in_all_workspace_directories: false,
            haxe_executable: haxe_exe,
            haxe_configurations,
            haxe_rename_source_folders: haxe_rename_folders,
            rust_analyzer_cargo_target_dir: true,
        };
        Ok(out)
    }
}
