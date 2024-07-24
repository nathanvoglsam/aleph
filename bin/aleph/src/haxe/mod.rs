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

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
pub use subproject::{
    HaxeCrateContext, HaxeCrateMeta, HaxeModuleContext, HaxeModuleMeta, HaxeProjectContext,
    HaxeProjectMeta, HaxeSubproject,
};

#[derive(Default, Serialize, Deserialize)]
pub struct HaxeModuleDefinitionFile<'a> {
    /// Top level module definition
    pub module: HaxeModuleDefinition<'a>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HaxeModuleDefinition<'a> {
    /// The name of the top-level package in this haxe module.
    pub name: Cow<'a, str>,

    /// Description of the haxe module for the 'lua' target.
    #[serde(default)]
    pub lua: HaxeLuaDefinition,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HaxeLuaDefinition {
    /// Flags whether this haxe module should be compiled to output a lua module. This will output
    /// a lua file that can be used with 'require' and the generated code will be callable by plain
    /// lua code.
    #[serde(default)]
    pub package: bool,

    /// Flags whether this haxe module should be added to the general class path for other haxe
    /// modules so that it can be consumed as a haxe library.
    #[serde(default)]
    pub library: bool,
}
