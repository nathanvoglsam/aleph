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

use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::anyhow;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use cargo_metadata::Package;
use serde::Deserialize;
use serde::Serialize;

use crate::project::AlephProject;
use crate::utils::BuildPlatform;

#[derive(Default, Serialize, Deserialize)]
pub struct AlephCrateMetadata<'a> {
    /// Shader metadata description
    pub shaders: ShaderCrateMetadata<'a>,
}

impl<'a> AlephCrateMetadata<'a> {
    pub fn value_for_package(package: &Package) -> Option<&serde_json::Value> {
        package
            .metadata
            .as_object()
            .map(|v| v.get("aleph"))
            .flatten()
    }

    pub fn load_for_package(
        package: &'a Package,
    ) -> anyhow::Result<Option<AlephCrateMetadata<'a>>> {
        if let Some(v) = Self::value_for_package(package) {
            Ok(Some(serde_json::from_value(v.clone())?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderCrateMetadata<'a> {
    /// Named set of shader modules that the crate contains within the shaders folder
    #[serde(default)]
    pub modules: Vec<Cow<'a, str>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderModuleDefinitionFile<'a> {
    pub module: ShaderModuleDefinition<'a>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderModuleDefinition<'a> {
    /// Pretty name for the shader module
    pub name: Cow<'a, str>,

    /// A list of disabled warning IDs to be used for the whole module
    #[serde(default)]
    pub disabled_warnings: Vec<i32>,

    /// A table of #defines to provide to all shaders in the module
    #[serde(default)]
    pub defines: HashMap<Cow<'a, str>, toml::Value>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderDefinition<'a> {
    pub permutations: Vec<ShaderPermutationParameter<'a>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderPermutationParameter<'a> {
    /// The name for the parameter
    pub name: Cow<'a, str>,

    /// The name of the #define that this parameter controls
    pub define: Cow<'a, str>,

    /// The number of bits to consume from the parameter bitset space. We allow a maximum of 64 bits
    /// so pack these well.
    pub bits: u8,

    /// A table of strings, keyed from the permutation value and used to map a permutation number to
    /// a string value to define
    pub define_map: Option<HashMap<u32, Cow<'a, str>>>,

    /// A range, defining a contiguous, inclusive range of values for this permutation parameter
    pub range: Option<[u32; 2]>,

    /// A list of individual allowed values for the permutation
    pub valid: Option<Vec<u32>>,
}

pub struct CompilationParams {
    pub disabled_warnings: String,
    pub defines: String,
    pub module_include: Utf8PathBuf,
}

impl CompilationParams {
    pub fn new(ctx: &ShaderModuleContext, def: &ShaderModuleDefinition) -> anyhow::Result<Self> {
        let mut disabled_warnings = String::new();
        for &disabled_warning in &def.disabled_warnings {
            use std::fmt::Write;
            write!(&mut disabled_warnings, "-Wno-{disabled_warning} ")?;
        }

        let mut defines = String::new();
        for (name, value) in &def.defines {
            use std::fmt::Write;

            match value {
                toml::Value::String(v) => {
                    write!(&mut defines, "-D{name}=\"{v}\" ")?;
                }
                toml::Value::Integer(v) => {
                    write!(&mut defines, "-D{name}={v} ")?;
                }
                toml::Value::Float(v) => {
                    write!(&mut defines, "-D{name}={v} ")?;
                }
                toml::Value::Boolean(v) => {
                    let v = if *v { 1 } else { 0 };
                    write!(&mut defines, "-D{name}={v} ")?;
                }
                toml::Value::Datetime(_) => {
                    return Err(anyhow!(
                        "Define '{}' for shader module '{}' has invalid type 'Datetime'",
                        name,
                        &ctx.module_name,
                    ))
                }
                toml::Value::Array(_) => {
                    return Err(anyhow!(
                        "Define '{}' for shader module '{}' has invalid type 'Array'",
                        name,
                        &ctx.module_name,
                    ))
                }
                toml::Value::Table(_) => {
                    return Err(anyhow!(
                        "Define '{}' for shader module '{}' has invalid type 'Table'",
                        name,
                        &ctx.module_name,
                    ))
                }
            }
        }

        let module_include = dunce::simplified(ctx.module_include_dir.as_std_path());
        let module_include = Utf8PathBuf::try_from(module_include.to_path_buf())?;

        Ok(CompilationParams {
            disabled_warnings,
            defines,
            module_include,
        })
    }

    pub fn write_ninja_overrides(&self, v: &mut impl std::io::Write) -> std::io::Result<()> {
        if !self.defines.is_empty() {
            writeln!(v, "    definitions = {}", self.defines)?;
        }
        if !self.disabled_warnings.is_empty() {
            writeln!(v, "    disabled_warnings = {}", self.disabled_warnings)?;
        }
        writeln!(v, "    module_include = {}", self.module_include)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct ProjectShaderContext<'a> {
    /// Path to '.aleph/shaders'
    pub shaders_output_root_dir: Cow<'a, Utf8Path>,

    /// Path to the shader build system's root ninja file
    pub root_ninja_file: Cow<'a, Utf8Path>,

    /// Path to the stamped out rules template used by the build system's ninja files
    pub root_rules_file: Cow<'a, Utf8Path>,

    /// The target platform we're cooking for
    pub platform: BuildPlatform,
}

impl<'a> ProjectShaderContext<'a> {
    pub fn new(project: &'a AlephProject, platform: BuildPlatform) -> anyhow::Result<Self> {
        let shaders_output_root_dir = project.shader_build_path();
        let shaders_output_root_dir = Utf8Path::from_path(shaders_output_root_dir)
            .ok_or(anyhow!("Shader dir has non-utf8 path"))?;

        let root_ninja_file = shaders_output_root_dir.join("build.ninja");
        let root_rules_file = shaders_output_root_dir.join("rules.ninja");

        Ok(Self {
            shaders_output_root_dir: Cow::Borrowed(shaders_output_root_dir),
            root_ninja_file: Cow::Owned(root_ninja_file),
            root_rules_file: Cow::Owned(root_rules_file),
            platform,
        })
    }

    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.shaders_output_root_dir.as_ref())
    }

    pub fn ensure_build_files(&self) -> std::io::Result<()> {
        std::fs::write(
            self.root_rules_file.as_ref(),
            crate::templates::SHADER_NINJA_RULES,
        )
    }

    pub fn get_borrowed(&'a self) -> Self {
        Self {
            shaders_output_root_dir: Cow::Borrowed(self.shaders_output_root_dir.as_ref()),
            root_ninja_file: Cow::Borrowed(self.root_ninja_file.as_ref()),
            root_rules_file: Cow::Borrowed(self.root_rules_file.as_ref()),
            platform: self.platform,
        }
    }
}

#[derive(Clone)]
pub struct ShaderCrateContext<'a> {
    pub project_ctx: ProjectShaderContext<'a>,
    pub crate_output_name: Cow<'a, str>,
    pub crate_output_dir: Cow<'a, Utf8Path>,
    pub crate_ninja_file: Cow<'a, Utf8Path>,
    pub crate_shader_dir: Cow<'a, Utf8Path>,
}

impl<'a> ShaderCrateContext<'a> {
    pub fn new_with_project_ctx(project_ctx: ProjectShaderContext<'a>, package: &Package) -> Self {
        let crate_output_name = format!("{}-{}", &package.name, &package.version);
        let crate_output_dir = project_ctx.shaders_output_root_dir.join(&crate_output_name);

        let crate_ninja_file = crate_output_dir.join("build.ninja");

        let crate_shader_dir = package.manifest_path.parent().unwrap();
        let crate_shader_dir = crate_shader_dir.join("shaders");

        Self {
            project_ctx,
            crate_output_name: Cow::Owned(crate_output_name),
            crate_output_dir: Cow::Owned(crate_output_dir),
            crate_ninja_file: Cow::Owned(crate_ninja_file),
            crate_shader_dir: Cow::Owned(crate_shader_dir),
        }
    }

    pub fn ensure_build_directories_no_parents(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.crate_output_dir.as_ref())
    }

    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        self.project_ctx.ensure_build_directories()?;
        self.ensure_build_directories_no_parents()
    }

    pub fn get_borrowed(&'a self) -> Self {
        Self {
            project_ctx: self.project_ctx.get_borrowed(),
            crate_output_name: Cow::Borrowed(self.crate_output_name.as_ref()),
            crate_output_dir: Cow::Borrowed(self.crate_output_dir.as_ref()),
            crate_ninja_file: Cow::Borrowed(self.crate_ninja_file.as_ref()),
            crate_shader_dir: Cow::Borrowed(self.crate_shader_dir.as_ref()),
        }
    }

    pub const fn platform(&self) -> BuildPlatform {
        self.project_ctx.platform
    }
}

#[derive(Clone)]
pub struct ShaderModuleContext<'a> {
    pub crate_ctx: ShaderCrateContext<'a>,
    pub module_name: Cow<'a, str>,
    pub module_output_dir: Cow<'a, Utf8Path>,
    pub module_ninja_file: Cow<'a, Utf8Path>,
    pub module_shader_dir: Cow<'a, Utf8Path>,
    pub module_toml_file: Cow<'a, Utf8Path>,
    pub module_source_dir: Cow<'a, Utf8Path>,
    pub module_include_dir: Cow<'a, Utf8Path>,
}

impl<'a> ShaderModuleContext<'a> {
    pub fn new_with_crate_ctx(
        crate_ctx: ShaderCrateContext<'a>,
        module_name: Cow<'a, str>,
    ) -> Self {
        let module_output_dir = crate_ctx.crate_output_dir.join(module_name.as_ref());
        let module_ninja_file = module_output_dir.join("build.ninja");

        let module_shader_dir = crate_ctx.crate_shader_dir.join(module_name.as_ref());
        let module_toml_file = module_shader_dir.join("Module.toml");
        let module_source_dir = module_shader_dir.join("source");
        let module_include_dir = module_shader_dir.join("include");

        Self {
            crate_ctx,
            module_name,
            module_output_dir: Cow::Owned(module_output_dir),
            module_ninja_file: Cow::Owned(module_ninja_file),
            module_shader_dir: Cow::Owned(module_shader_dir),
            module_toml_file: Cow::Owned(module_toml_file),
            module_source_dir: Cow::Owned(module_source_dir),
            module_include_dir: Cow::Owned(module_include_dir),
        }
    }

    pub fn ensure_build_directories_no_parents(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.module_output_dir.as_ref())
    }

    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        self.crate_ctx.ensure_build_directories()?;
        self.ensure_build_directories_no_parents()
    }

    // pub fn get_borrowed(&'a self) -> Self {
    //     Self {
    //         crate_ctx: self.crate_ctx.get_borrowed(),
    //         module_name: Cow::Borrowed(self.module_name.as_ref()),
    //         module_output_dir: Cow::Borrowed(self.module_output_dir.as_ref()),
    //         module_ninja_file: Cow::Borrowed(self.module_ninja_file.as_ref()),
    //         module_shader_dir: Cow::Borrowed(self.module_shader_dir.as_ref()),
    //         module_toml_file: Cow::Borrowed(self.module_toml_file.as_ref()),
    //         module_source_dir: Cow::Borrowed(self.module_source_dir.as_ref()),
    //         module_include_dir: Cow::Borrowed(self.module_include_dir.as_ref()),
    //     }
    // }

    pub const fn platform(&self) -> BuildPlatform {
        self.crate_ctx.platform()
    }
}
