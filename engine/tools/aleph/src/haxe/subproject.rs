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

#[derive(Debug)]
pub struct HaxeSubproject();

impl<'a> ISubproject<'a> for HaxeSubproject {
    type ProjectMeta = HaxeProjectMeta<'a>;

    type CrateMeta = HaxeCrateMeta<'a>;

    type ModuleMeta = HaxeModuleMeta<'a>;

    fn load_project(arena: &'a Bump, ctx: &AlephProject) -> anyhow::Result<Self::ProjectMeta> {
        let output_root = arena.alloc_utf8_path(ctx.haxe_build_path());

        Ok(HaxeProjectMeta { output_root })
    }

    fn retain_crate(_package: &Package, metadata: &AlephCrateMetadata) -> bool {
        // Skip any packages with no shader metadata or no shader modules if the metadata was
        // defined with no modules specified.
        if let Some(haxe) = &metadata.haxe {
            !haxe.modules.is_empty()
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

        let haxe_dir = package.manifest_path.parent().unwrap().join("haxe");
        let haxe_dir = arena.alloc_utf8_path(&haxe_dir);

        Ok(HaxeCrateMeta {
            output_name,
            output_dir,
            haxe_dir,
        })
    }

    fn get_module_names(
        arena: &'a Bump,
        _package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<BVec<'a, &'a str>> {
        let out = if let Some(haxe) = &metadata.haxe {
            let iter = haxe.modules.iter().map(|v| &*arena.alloc_str(v.as_ref()));
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

        let build_lua_file = output_dir.join("build_lua.hxml");
        let build_lua_file = arena.alloc_utf8_path(&build_lua_file);

        let build_js_file = output_dir.join("build_js.hxml");
        let build_js_file = arena.alloc_utf8_path(&build_js_file);

        let haxe_dir = crate_ctx.meta.haxe_dir.join(module_name);

        let toml_file = haxe_dir.join("Module.toml");
        let toml_file = arena.alloc_utf8_path(&toml_file);

        let source_dir = haxe_dir.join("src");
        let source_dir = arena.alloc_utf8_path(&source_dir);

        Ok(HaxeModuleMeta {
            output_dir,
            build_lua_file,
            build_js_file,
            toml_file,
            source_dir,
        })
    }
}

impl HaxeSubproject {
    pub fn load<'a>(arena: &'a Bump, ctx: &AlephProject) -> anyhow::Result<HaxeProjectContext<'a>> {
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
}

pub type HaxeProjectContext<'a> = SubprojectProjectContext<'a, HaxeSubproject>;

#[derive(Clone, Debug)]
pub struct HaxeProjectMeta<'a> {
    /// Path to '.aleph/haxe'
    pub output_root: &'a Utf8Path,
}

impl<'a> HaxeProjectMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_root)
    }
}

pub type HaxeCrateContext<'a> = SubprojectCrateContext<'a, HaxeSubproject>;

#[derive(Clone, Debug)]
pub struct HaxeCrateMeta<'a> {
    /// The name of the output directory. The '{crate}' portion of '.aleph/haxe/{crate}'.
    pub output_name: &'a str,

    /// Path to '.aleph/haxe/{crate}'
    pub output_dir: &'a Utf8Path,

    /// Path to the 'haxe' dir in the crate's folder. Adjactent to Cargo.toml.
    pub haxe_dir: &'a Utf8Path,
}

impl<'a> HaxeCrateMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_dir)
    }
}

pub type HaxeModuleContext<'a> = SubprojectModuleContext<'a, HaxeSubproject>;

#[derive(Clone, Debug)]
pub struct HaxeModuleMeta<'a> {
    /// Path to '.aleph/haxe/{crate}/{module}'
    pub output_dir: &'a Utf8Path,

    /// Path to '.aleph/haxe/{crate}/{module}/build_lua.hxml'
    pub build_lua_file: &'a Utf8Path,

    /// Path to '.aleph/haxe/{crate}/{module}/build_js.hxml'
    pub build_js_file: &'a Utf8Path,

    /// Path to 'haxe/{module}/Module.toml' in the crate's folder.
    pub toml_file: &'a Utf8Path,

    /// Path to 'haxe/{module}/src' in the crate's folder.
    pub source_dir: &'a Utf8Path,
}

impl<'a> HaxeModuleMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.output_dir)
    }
}
