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

use crate::interfaces::plugin::{IPluginRegistrar, UpdateStage};
use std::any::TypeId;
use std::collections::HashSet;

pub struct PluginRegistrar {
    pub depends_on_list: HashSet<TypeId>,
    pub provided_interfaces: HashSet<TypeId>,
    pub init_after_list: Vec<HashSet<TypeId>>,
    pub update_stage_dependencies: Vec<HashSet<TypeId>>,
    pub update_stages: HashSet<usize>,
}

impl IPluginRegistrar for PluginRegistrar {
    fn __depends_on(&mut self, dependency: TypeId) {
        self.depends_on_list.insert(dependency);
    }

    fn __provides_interface(&mut self, provides: TypeId) {
        self.provided_interfaces.insert(provides);
    }

    fn __must_init_after(&mut self, requires: TypeId) {
        self.init_after_list[0].insert(requires);
    }

    fn __must_update_after(&mut self, stage: UpdateStage, requires: TypeId) {
        if !self.update_stages.contains(&(stage as usize)) {
            panic!("Declared execution dependency for stage plugin not declared to execute in");
        }
        self.update_stage_dependencies[stage as usize].insert(requires);
    }

    fn update_stage(&mut self, stage: UpdateStage) {
        self.update_stages.insert(stage as usize);
    }
}
