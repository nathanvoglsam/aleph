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

use blink_alloc::Blink;

use crate::crate_metadata::ProjectCrateMetadata;
use crate::project::AlephProject;
use crate::subproject::{ISubproject, SubprojectCrateContext, SubprojectModuleContext};

#[derive(Clone, Debug)]
pub struct SubprojectProjectContext<'a, T: ISubproject<'a>> {
    /// The list of crates a part of this subproject set
    pub crates: &'a [SubprojectCrateContext<'a, T>],

    /// Any subproject type specific metadata
    pub meta: T::ProjectMeta,
}

impl<'a, T: ISubproject<'a>> SubprojectProjectContext<'a, T> {
    pub fn load(
        arena: &'a Blink,
        ctx: &AlephProject,
        metadata: &ProjectCrateMetadata,
    ) -> anyhow::Result<Self> {
        let meta = T::load_project(arena, ctx)?;

        let partial_project_ctx = Self { crates: &[], meta };

        let mut crate_ctxs = Vec::with_capacity(metadata.metadata.len());
        for (p, m) in metadata.metadata.iter() {
            if T::retain_crate(p, m) {
                let meta = T::load_crate(arena, ctx, &partial_project_ctx, p, m)?;

                let partial_crate_ctx = SubprojectCrateContext { modules: &[], meta };

                let module_names = T::get_module_names(arena, p, m)?;

                let mut module_ctxs = Vec::with_capacity(module_names.len());
                for module_name in module_names {
                    let meta = T::load_module(
                        arena,
                        ctx,
                        &partial_project_ctx,
                        &partial_crate_ctx,
                        p,
                        m,
                        module_name,
                    )?;

                    let module_ctx = SubprojectModuleContext { module_name, meta };
                    module_ctxs.push(module_ctx);
                }

                let final_crate_ctx = SubprojectCrateContext {
                    modules: &*module_ctxs.leak(),
                    meta: partial_crate_ctx.meta,
                };

                crate_ctxs.push(final_crate_ctx);
            }
        }

        let final_project_ctx = Self {
            crates: &*crate_ctxs.leak(),
            meta: partial_project_ctx.meta,
        };

        Ok(final_project_ctx)
    }
}
