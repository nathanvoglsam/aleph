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

use camino::Utf8Path;
use serde::{Deserialize, Serialize};
pub use subproject::{HaxeCrateContext, HaxeModuleContext, HaxeProjectContext, HaxeSubproject};

#[derive(Default, Serialize, Deserialize)]
pub struct HaxeModuleDefinitionFile {
    /// Description of the haxe module for the 'hl' target.
    #[serde(default)]
    pub hl: HaxeHlDefinition,

    /// Description of the haxe module for the 'js' target.
    #[serde(default)]
    pub js: HaxeJsDefinition,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HaxeHlDefinition {
    /// Flags whether this haxe module should be compiled to output a hl module. This will output
    /// a hl file that can be used with hashlink.
    #[serde(default)]
    pub package: bool,

    /// Flags whether this haxe module should be added to the general class path for other haxe
    /// modules so that it can be consumed as a haxe library.
    #[serde(default)]
    pub library: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HaxeJsDefinition {
    /// Flags whether this haxe module should be added as config script. A compiled JS module will
    /// be output for this package and it will be included in the config script bundle under the
    /// name of the crate that owns this haxe module.
    #[serde(default)]
    pub config_script: bool,

    /// Flags whether this haxe module should be added to the general class path for other haxe
    /// modules so that it can be consumed as a haxe library.
    #[serde(default)]
    pub library: bool,
}

#[derive(Default, Debug)]
pub struct ClasspathBundle<'a> {
    pub hl: Vec<&'a Utf8Path>,
    pub js: Vec<&'a Utf8Path>,
}
