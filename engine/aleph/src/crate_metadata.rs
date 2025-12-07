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

use cargo_metadata::Package;
use serde::{Deserialize, Serialize};

use crate::project::AlephProject;

/// Description of all the elements expected to be found in the 'package.metadata.aleph' key in
/// a crate's Cargo.toml.
///
/// This specifies a bunch of things like what sub-projects are colocated with the Cargo project
/// like shader modules or script modules.
#[derive(Default, Serialize, Deserialize)]
pub struct AlephCrateMetadata<'a> {
    /// Shader metadata description
    pub shaders: Option<ShaderCrateMetadata<'a>>,

    /// An optional list of config object names that the crate wishes to export into the config
    /// system.
    ///
    /// Each name in this list will map to a '.js' and '.d.ts' file inside the 'configs' directory
    /// of the crate. Each config will be mapped into the game's config space and will be loaded
    /// at launch. Each config can be queried by name from the config system. Only a single crate
    /// can export a config by a given name. If multiple crates attempt to do so then the config
    /// cook will fail at build time.
    #[serde(default)]
    pub configs: Vec<String>,
}

impl<'a> AlephCrateMetadata<'a> {
    pub fn load_for_package(package: &'a Package) -> anyhow::Result<AlephCrateMetadata<'a>> {
        let v = match Self::value_for_package(package) {
            Some(v) => serde_json::from_value(v.clone())?,
            None => AlephCrateMetadata::default(),
        };
        Ok(v)
    }

    pub fn is_empty(&self) -> bool {
        self.shaders.is_none() && self.configs.is_empty()
    }

    fn value_for_package(package: &Package) -> Option<&serde_json::Value> {
        package.metadata.as_object().and_then(|v| v.get("aleph"))
    }
}

/// A list of packages paired with their 'package.metadata.aleph' data deserialized.
#[derive(Default)]
pub struct ProjectCrateMetadata<'a> {
    /// A list of packages paired with the parsed [`AlephCrateMetadata`] from the packages metadata.
    pub metadata: Vec<(&'a Package, AlephCrateMetadata<'a>)>,
}

impl<'a> ProjectCrateMetadata<'a> {
    pub fn load(project: &'a AlephProject) -> anyhow::Result<Self> {
        // Load the cargo metadata from the project
        let metadata = project.get_cargo_metadata()?;

        // Load all the aleph metadata package metadata objects and parse them into our schema.
        //
        // If the object is empty we don't create an entry in the ProjectCrateMetadata.
        let mut filtered = Vec::with_capacity(metadata.packages.len());
        for p in metadata.packages.iter() {
            let metadata = AlephCrateMetadata::load_for_package(p)?;

            if !metadata.is_empty() {
                filtered.push((p, metadata));
            }
        }

        Ok(Self { metadata: filtered })
    }
}

/// The description of the 'package.metadata.aleph.shaders' key in a package's metadata.
#[derive(Default, Serialize, Deserialize)]
pub struct ShaderCrateMetadata<'a> {
    /// Named set of shader modules that the crate contains within the shaders folder
    #[serde(default)]
    pub modules: Vec<Cow<'a, str>>,
}
