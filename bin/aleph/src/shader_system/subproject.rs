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

use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use camino::Utf8Path;
use cargo_metadata::Package;

use crate::crate_metadata::AlephCrateMetadata;
use crate::crate_metadata::ProjectCrateMetadata;
use crate::project::AlephProject;
use crate::subproject::ISubproject;
use crate::subproject::SubprojectCrateContext;
use crate::subproject::SubprojectModuleContext;
use crate::subproject::SubprojectProjectContext;
use crate::utils::BumpExt;

pub struct ShaderSubproject();

impl<'a> ISubproject<'a> for ShaderSubproject {
    type ProjectMeta = ShaderProjectMeta<'a>;

    type CrateMeta = ShaderCrateMeta<'a>;

    type ModuleMeta = ShaderModuleMeta<'a>;

    fn load_project(arena: &'a Bump, ctx: &AlephProject) -> anyhow::Result<Self::ProjectMeta> {
        let output_root = arena.alloc_utf8_path(ctx.shader_build_path());

        let root_ninja_file = arena.alloc_utf8_path(&output_root.join("build.ninja"));
        let root_rules_file = arena.alloc_utf8_path(&output_root.join("rules.ninja"));

        Ok(ShaderProjectMeta {
            output_root,
            root_ninja_file,
            root_rules_file,
        })
    }

    fn retain_crate(_package: &Package, metadata: &AlephCrateMetadata) -> bool {
        // Skip any packages with no shader metadata or no shader modules if the metadata was
        // defined with no modules specified.
        if let Some(shaders) = &metadata.shaders {
            !shaders.modules.is_empty()
        } else {
            false
        }
    }

    fn load_crate(
        arena: &'a Bump,
        _ctx: &AlephProject,
        project_ctx: &SubprojectProjectContext<'a, Self>,
        package: &Package,
        _metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<Self::CrateMeta> {
        let output_name = format!("{}-{}", &package.name, &package.version);
        let output_name = arena.alloc_str(&output_name);

        let output_dir = project_ctx.meta.output_root.join(&output_name);
        let output_dir = arena.alloc_utf8_path(&output_dir);

        let shader_dir = package.manifest_path.parent().unwrap().join("shaders");
        let shader_dir = arena.alloc_utf8_path(&shader_dir);

        Ok(ShaderCrateMeta {
            output_name,
            output_dir,
            shader_dir,
        })
    }

    fn get_module_names(
        arena: &'a Bump,
        _package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<BVec<'a, &'a str>> {
        let out = if let Some(shaders) = &metadata.shaders {
            let iter = shaders
                .modules
                .iter()
                .map(|v| &*arena.alloc_str(v.as_ref()));
            BVec::from_iter_in(iter, arena)
        } else {
            BVec::new_in(arena)
        };
        Ok(out)
    }

    fn load_module(
        arena: &'a Bump,
        _ctx: &AlephProject,
        _project_ctx: &SubprojectProjectContext<'a, Self>,
        crate_ctx: &SubprojectCrateContext<'a, Self>,
        _package: &Package,
        _metadata: &AlephCrateMetadata,
        module_name: &str,
    ) -> anyhow::Result<Self::ModuleMeta> {
        let output_dir = crate_ctx.meta.output_dir.join(module_name);
        let output_dir = arena.alloc_utf8_path(&output_dir);

        let ninja_file = output_dir.join("build.ninja");
        let ninja_file = arena.alloc_utf8_path(&ninja_file);

        let shader_dir = crate_ctx.meta.shader_dir.join(module_name);

        let toml_file = shader_dir.join("Module.toml");
        let toml_file = arena.alloc_utf8_path(&toml_file);

        let source_dir = shader_dir.join("source");
        let source_dir = arena.alloc_utf8_path(&source_dir);

        let include_dir = shader_dir.join("include");
        let include_dir = arena.alloc_utf8_path(&include_dir);

        Ok(ShaderModuleMeta {
            output_dir,
            ninja_file,
            toml_file,
            source_dir,
            include_dir,
        })
    }
}

impl ShaderSubproject {
    pub fn load<'a>(
        arena: &'a Bump,
        ctx: &AlephProject,
    ) -> anyhow::Result<ShaderProjectContext<'a>> {
        let metadata = ProjectCrateMetadata::load(ctx)?;
        let project_ctx = SubprojectProjectContext::load(&arena, ctx, &metadata)?;
        Ok(project_ctx)
    }

    pub fn ensure_build_directories(ctx: &SubprojectProjectContext<Self>) -> anyhow::Result<()> {
        ctx.meta.ensure_build_directories()?;
        for crate_ctx in ctx.crates.iter() {
            crate_ctx.meta.ensure_build_directories()?;
            for module_ctx in crate_ctx.modules.iter() {
                module_ctx.meta.ensure_build_directories()?;
            }
        }
        Ok(())
    }

    pub fn ensure_build_files(ctx: &SubprojectProjectContext<Self>) -> anyhow::Result<()> {
        ctx.meta.ensure_build_files()?;
        Ok(())
    }
}

pub type ShaderProjectContext<'a> = SubprojectProjectContext<'a, ShaderSubproject>;

#[derive(Clone)]
pub struct ShaderProjectMeta<'a> {
    /// Path to '.aleph/shaders'
    pub output_root: &'a Utf8Path,

    /// Path to the shader build system's root ninja file
    pub root_ninja_file: &'a Utf8Path,

    /// Path to the stamped out rules template used by the build system's ninja files
    pub root_rules_file: &'a Utf8Path,
}

impl<'a> ShaderProjectMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_root)
    }

    pub fn ensure_build_files(&self) -> std::io::Result<()> {
        std::fs::write(self.root_rules_file, crate::templates::SHADER_NINJA_RULES)
    }
}

pub type ShaderCrateContext<'a> = SubprojectCrateContext<'a, ShaderSubproject>;

#[derive(Clone)]
pub struct ShaderCrateMeta<'a> {
    pub output_name: &'a str,
    pub output_dir: &'a Utf8Path,
    pub shader_dir: &'a Utf8Path,
}

impl<'a> ShaderCrateMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_dir)
    }
}

pub type ShaderModuleContext<'a> = SubprojectModuleContext<'a, ShaderSubproject>;

#[derive(Clone)]
pub struct ShaderModuleMeta<'a> {
    pub output_dir: &'a Utf8Path,
    pub ninja_file: &'a Utf8Path,
    pub toml_file: &'a Utf8Path,
    pub source_dir: &'a Utf8Path,
    pub include_dir: &'a Utf8Path,
}

impl<'a> ShaderModuleMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_dir)
    }
}
