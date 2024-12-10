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

use std::any::TypeId;
use std::collections::BTreeSet;

use interfaces::plugin::InitOrder;

use crate::interfaces::plugin::IPluginRegistrar;

pub struct PluginRegistrar {
    pub(crate) depends_on_list: BTreeSet<TypeId>,
    pub(crate) provided_interfaces: BTreeSet<TypeId>,
    pub(crate) init_after_list: BTreeSet<TypeId>,
}

impl IPluginRegistrar for PluginRegistrar {
    fn __requires(&mut self, dependency: TypeId, init: InitOrder) {
        self.depends_on_list.insert(dependency);
        match init {
            InitOrder::After => {
                self.init_after_list.insert(dependency);
            },
            InitOrder::DontCare => todo!(),
        }
    }

    fn __provides(&mut self, provides: TypeId) {
        self.provided_interfaces.insert(provides);
    }

    fn __uses(&mut self, uses: TypeId, init: InitOrder) {
        match init {
            InitOrder::After => {
                self.init_after_list.insert(uses);
            },
            InitOrder::DontCare => {
                // Intentionally blank as we don't track optional dependencies currently.
            },
        }
        
    }
}
