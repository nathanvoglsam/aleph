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

mod crate_context;
mod module_context;
mod project_context;

use std::fmt::Debug;

use bumpalo::collections::Vec as BVec;
use bumpalo::Bump;
use cargo_metadata::Package;
pub use crate_context::SubprojectCrateContext;
pub use module_context::SubprojectModuleContext;
pub use project_context::SubprojectProjectContext;

use crate::crate_metadata::AlephCrateMetadata;
use crate::project::AlephProject;

pub trait ISubproject<'a>: Sized {
    type ProjectMeta: 'a + Sized + Debug;
    type CrateMeta: 'a + Sized + Debug;
    type ModuleMeta: 'a + Sized + Debug;

    fn load_project(arena: &'a Bump, ctx: &AlephProject) -> anyhow::Result<Self::ProjectMeta>;

    fn retain_crate(package: &Package, metadata: &AlephCrateMetadata) -> bool;

    fn load_crate(
        arena: &'a Bump,
        ctx: &AlephProject,
        project_ctx: &SubprojectProjectContext<'a, Self>,
        package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<Self::CrateMeta>;

    fn get_module_names(
        arena: &'a Bump,
        package: &Package,
        metadata: &AlephCrateMetadata,
    ) -> anyhow::Result<BVec<'a, &'a str>>;

    fn load_module(
        arena: &'a Bump,
        ctx: &AlephProject,
        project_ctx: &SubprojectProjectContext<'a, Self>,
        crate_ctx: &SubprojectCrateContext<'a, Self>,
        package: &Package,
        metadata: &AlephCrateMetadata,
        module_name: &str,
    ) -> anyhow::Result<Self::ModuleMeta>;
}
