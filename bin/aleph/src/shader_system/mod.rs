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

mod subproject;

use camino::Utf8PathBuf;
pub use subproject::{
    ShaderCrateContext, ShaderModuleContext, ShaderProjectContext, ShaderSubproject,
};

use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::anyhow;
use camino::Utf8Path;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

impl From<ShaderType> for aleph_shader_db::ShaderType {
    fn from(val: ShaderType) -> Self {
        match val {
            ShaderType::Compute => aleph_shader_db::ShaderType::Compute,
            ShaderType::Vertex => aleph_shader_db::ShaderType::Vertex,
            ShaderType::Hull => aleph_shader_db::ShaderType::Hull,
            ShaderType::Domain => aleph_shader_db::ShaderType::Domain,
            ShaderType::Geometry => aleph_shader_db::ShaderType::Geometry,
            ShaderType::Fragment => aleph_shader_db::ShaderType::Fragment,
            ShaderType::Amplification => aleph_shader_db::ShaderType::Amplification,
            ShaderType::Mesh => aleph_shader_db::ShaderType::Mesh,
        }
    }
}

impl ShaderType {
    pub fn from_ext(v: &str) -> Option<Self> {
        match v {
            "comp" | "compute" | "cs" => Some(Self::Compute),
            "vert" | "vertex" | "vs" => Some(Self::Vertex),
            "hull" | "hs" => Some(Self::Hull),
            "domain" | "ds" => Some(Self::Domain),
            "geom" | "geometry" | "gs" => Some(Self::Geometry),
            "frag" | "fragment" | "pix" | "pixel" | "ps" | "fg" => Some(Self::Fragment),
            "amp" | "as" => Some(Self::Amplification),
            "mesh" | "ms" => Some(Self::Mesh),
            _ => None,
        }
    }

    pub fn to_ninja_rule(self) -> &'static str {
        match self {
            ShaderType::Compute => "compute_shader",
            ShaderType::Vertex => "vertex_shader",
            ShaderType::Hull => "hull_shader",
            ShaderType::Domain => "domain_shader",
            ShaderType::Geometry => "geometry_shader",
            ShaderType::Fragment => "fragment_shader",
            ShaderType::Amplification => "amplification_shader",
            ShaderType::Mesh => "mesh_shader",
        }
    }

    pub fn shader_db_name_type(&self) -> &'static str {
        match self {
            ShaderType::Compute => "ShaderName<'static, Compute>",
            ShaderType::Vertex => "ShaderName<'static, Vertex>",
            ShaderType::Hull => "ShaderName<'static, Hull>",
            ShaderType::Domain => "ShaderName<'static, Domain>",
            ShaderType::Geometry => "ShaderName<'static, Geometry>",
            ShaderType::Fragment => "ShaderName<'static, Fragment>",
            ShaderType::Amplification => "ShaderName<'static, Amplification>",
            ShaderType::Mesh => "ShaderName<'static, Mesh>",
        }
    }

    pub fn shader_db_name_constructor(&self) -> &'static str {
        match self {
            ShaderType::Compute => "ShaderName::<Compute>::new",
            ShaderType::Vertex => "ShaderName::<Vertex>::new",
            ShaderType::Hull => "ShaderName::<'static, Hull>::new",
            ShaderType::Domain => "ShaderName::<'static, Domain>::new",
            ShaderType::Fragment => "ShaderName::<Fragment>::new",
            ShaderType::Geometry => "ShaderName::<Geometry>::new",
            ShaderType::Amplification => "ShaderName::<'static, Amplification>::new",
            ShaderType::Mesh => "ShaderName::<'static, Mesh>::new",
        }
    }
}

/// Enumeration of all shader related file formats. This includes source code and byte code file
/// formats.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShaderFileFormat {
    Hlsl,
    Slang,
    Dxil,
    Spirv,
}

impl ShaderFileFormat {
    pub fn from_file_ext(v: &str) -> Option<Self> {
        match v {
            "hlsl" => Some(ShaderFileFormat::Hlsl),
            "slang" => Some(ShaderFileFormat::Slang),
            _ => None,
        }
    }

    pub fn from_binary_file_ext(v: &str) -> Option<Self> {
        match v {
            "dxil" => Some(ShaderFileFormat::Dxil),
            "spirv" => Some(ShaderFileFormat::Spirv),
            _ => None,
        }
    }
}

/// Enumeration of all shader bytecode targets. This does not include variants for _source_ files
/// like hlsl or slang. See [ShaderFileFormat] for source formats.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ShaderTargetLanguage {
    /// The DXIL format for consumption by D3D12
    Dxil,

    /// The SPIR-V format for consumpation by Vulkan
    Spirv,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderDefinition<'a> {
    pub permutations: Vec<ShaderPermutationParameter<'a>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderModuleDefinitionFile<'a> {
    pub module: ShaderModuleDefinition<'a>,
}

impl<'a> ShaderModuleDefinitionFile<'a> {
    pub fn from_str(v: &str) -> anyhow::Result<Self> {
        let v = toml::from_str(v)?;
        Ok(v)
    }
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

pub struct ShaderFile<'a> {
    /// The path to the shader file
    pub path: &'a Utf8Path,

    /// The full file name of the shader file
    pub file_name: &'a str,

    /// The extracted file extension of the shader file.
    pub file_ext: ShaderFileFormat,

    /// The extracted shader type of the shader file.
    pub shader_type: ShaderType,

    /// The name of the shader, with the shader type and file extension stripped.
    pub name: &'a str,

    /// The name of the shader, including the shader type.
    pub name_with_type: &'a str,
}

impl<'a> ShaderFile<'a> {
    pub fn new(path: &'a Utf8Path) -> Option<Self> {
        Self::new_internal(path, ShaderFileFormat::from_file_ext)
    }

    pub fn new_binary(path: &'a Utf8Path) -> Option<Self> {
        Self::new_internal(path, ShaderFileFormat::from_binary_file_ext)
    }

    fn new_internal(
        path: &'a Utf8Path,
        ext_mapper: fn(&str) -> Option<ShaderFileFormat>,
    ) -> Option<Self> {
        let file_name = path.file_name()?;

        // Split out the last two dot segments of the file name. For something like
        // shader.frag.hlsl we should get a file_ext = hlsl and shader_type = frag with
        // name_segment = shader.
        //
        // We need to know part of the rest of the name so we can reject files like
        // 'frag.hlsl' as it is effectively a nameless shader.
        let mut dot_segments = file_name.split('.').rev();
        let file_ext_str = dot_segments.next()?;
        let shader_type_str = dot_segments.next()?;
        let _name_segment = dot_segments.next()?;

        let shader_type = ShaderType::from_ext(shader_type_str)?;
        let file_ext = ext_mapper(file_ext_str)?;

        // This _can't_ fail as we've already proven that these are the last segments of the file
        // name above.
        let file_name_no_ext = file_name.strip_suffix(file_ext_str).unwrap();
        let file_name_no_ext = file_name_no_ext.strip_suffix('.').unwrap();
        let file_name_no_s_type = file_name_no_ext.strip_suffix(shader_type_str).unwrap();
        let file_name_no_s_type = file_name_no_s_type.strip_suffix('.').unwrap();

        Some(Self {
            path,
            file_name,
            file_ext,
            shader_type,
            name: file_name_no_s_type,
            name_with_type: file_name_no_ext,
        })
    }

    pub fn ninja_rule(&self) -> &'static str {
        self.shader_type.to_ninja_rule()
    }
}

pub struct ShaderCompilationParams {
    pub disabled_warnings: String,
    pub defines: String,
    pub module_include: Utf8PathBuf,
}

impl ShaderCompilationParams {
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

        let module_include = dunce::simplified(ctx.meta.include_dir.as_std_path());
        let module_include = Utf8PathBuf::try_from(module_include.to_path_buf())?;

        Ok(ShaderCompilationParams {
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
