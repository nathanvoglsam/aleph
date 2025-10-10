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

use aleph_alloc::alloc::Global;
use aleph_alloc::vec::Vec as BVec;
use aleph_alloc::{Blink, BlinkAlloc};
use camino::Utf8Path;
use cargo_metadata::Package;

use crate::crate_metadata::{AlephCrateMetadata, ProjectCrateMetadata};
use crate::project::AlephProject;
use crate::subproject::{ISubproject, SubprojectCrateContext, SubprojectProjectContext};
use crate::utils::BumpExt;
use crate::utils::dunce_utf8::simplified;

#[derive(Debug)]
pub struct ConfigSubproject();

impl<'a> ISubproject<'a> for ConfigSubproject {
    type ProjectMeta = ConfigProjectMeta<'a>;

    type CrateMeta = ConfigCrateMeta<'a>;

    type ModuleMeta = ();

    fn load_project(arena: &'a Blink, ctx: &AlephProject) -> anyhow::Result<Self::ProjectMeta> {
        let configs_root = arena.alloc_utf8_path(ctx.configs_build_path());

        Ok(ConfigProjectMeta { configs_root })
    }

    fn retain_crate(_package: &Package, metadata: &AlephCrateMetadata) -> bool {
        // Skip any packages with no config script
        !metadata.configs.is_empty() || metadata.config_defs
    }

    fn load_crate(
        arena: &'a Blink,
        _ctx: &AlephProject,
        project_ctx: &SubprojectProjectContext<'a, Self>,
        package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<Self::CrateMeta> {
        let mut config_names = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());
        config_names.extend(metadata.configs.iter().map(|v| &*arena.copy_str(v)));
        let config_names = BVec::leak(config_names);

        let config_dir = package.manifest_path.parent().unwrap().join("config");
        let config_dir = arena.alloc_utf8_path(simplified(&config_dir));

        let iter = metadata.configs.iter().map(|v| {
            let dst = project_ctx.meta.configs_root.join(v).with_extension("js");
            let dst = arena.alloc_utf8_path(simplified(&dst));

            let src = config_dir.join(v).with_extension("js");
            let src = arena.alloc_utf8_path(simplified(&src));

            ConfigPair { src, dst }
        });
        let mut configs = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());
        configs.extend(iter);
        let configs = BVec::leak(configs);

        Ok(ConfigCrateMeta {
            config_dir,
            config_names,
            configs,
            defs_only: metadata.config_defs,
        })
    }

    fn get_module_names(
        arena: &'a Blink,
        _package: &Package,
        _metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<BVec<&'a str, &'a BlinkAlloc<Global>>> {
        Ok(BVec::new_in(arena.allocator()))
    }

    fn load_module(
        _arena: &'a Blink,
        _ctx: &AlephProject,
        _project_ctx: &SubprojectProjectContext<'a, Self>,
        _crate_ctx: &SubprojectCrateContext<'a, Self>,
        _package: &Package,
        _metadata: &AlephCrateMetadata,
        _module_name: &str,
    ) -> anyhow::Result<Self::ModuleMeta> {
        Ok(())
    }
}

impl ConfigSubproject {
    pub fn load<'a>(
        arena: &'a Blink,
        ctx: &AlephProject,
    ) -> anyhow::Result<ConfigProjectContext<'a>> {
        let metadata = ProjectCrateMetadata::load(ctx)?;
        let project_ctx = SubprojectProjectContext::load(&arena, ctx, &metadata)?;
        Ok(project_ctx)
    }

    pub fn ensure_build_directories(ctx: &SubprojectProjectContext<Self>) -> anyhow::Result<()> {
        ctx.meta.ensure_build_directories()?;
        Ok(())
    }
}

pub type ConfigProjectContext<'a> = SubprojectProjectContext<'a, ConfigSubproject>;

#[derive(Clone, Debug)]
pub struct ConfigProjectMeta<'a> {
    /// Path to '.aleph/configs'
    pub configs_root: &'a Utf8Path,
}

impl<'a> ConfigProjectMeta<'a> {
    pub fn ensure_build_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.configs_root)
    }
}

#[derive(Clone, Debug)]
pub struct ConfigCrateMeta<'a> {
    /// Path to the crate's config folder
    pub config_dir: &'a Utf8Path,

    /// The list of config names the crate exports. This is what [`Self::configs`] is derived from.
    pub config_names: &'a [&'a str],

    /// Pairs of src and dst files for each declared config.
    pub configs: &'a [ConfigPair<'a>],

    /// Whether this crate provides only type defs and no config script
    pub defs_only: bool,
}

#[derive(Clone, Debug)]
pub struct ConfigPair<'a> {
    /// Path to the 'config.js' file in the crate's 'config' folder.
    pub src: &'a Utf8Path,

    /// Path to '.aleph/configs/{config}.js'
    pub dst: &'a Utf8Path,
}
