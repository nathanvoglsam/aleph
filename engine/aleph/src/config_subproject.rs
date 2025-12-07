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
        !metadata.configs.is_empty()
    }

    fn load_crate(
        arena: &'a Blink,
        _ctx: &AlephProject,
        _project_ctx: &SubprojectProjectContext<'a, Self>,
        package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<Self::CrateMeta> {
        let config_dir = package.manifest_path.parent().unwrap().join("config");

        let mut config_names = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());
        let mut configs = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());
        let mut override_names = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());
        let mut overrides = BVec::with_capacity_in(metadata.configs.len(), arena.allocator());

        for name in metadata.configs.iter() {
            let name = &*arena.copy_str(name);
            let src = config_dir.join(name).with_extension("ts");
            let src = arena.alloc_utf8_path(simplified(&src));

            if name.starts_with('@') {
                // Override config
                override_names.push(name);
                overrides.push(src);
            } else {
                // Base config
                config_names.push(name);
                configs.push(src);
            }
        }

        let config_names: &'a [&'a str] = BVec::leak(config_names);
        let configs: &'a [&'a Utf8Path] = BVec::leak(configs);
        let override_names: &'a [&'a str] = BVec::leak(override_names);
        let overrides: &'a [&'a Utf8Path] = BVec::leak(overrides);

        Ok(ConfigCrateMeta {
            config_names,
            configs,
            override_names,
            overrides,
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
    /// The list of config names the crate exports. This is what [`Self::configs`] is derived from.
    pub config_names: &'a [&'a str],

    /// Path to the '{config}.ts' file in the crate's 'config' folder.
    pub configs: &'a [&'a Utf8Path],

    /// The list of override names the crate exports. This is what [`Self::overrides`] is derived
    /// from. Overrides are marked with a name starting with '@'.
    pub override_names: &'a [&'a str],

    /// Path to the '{override}.ts' file in the crate's 'config' folder.
    pub overrides: &'a [&'a Utf8Path],
}
