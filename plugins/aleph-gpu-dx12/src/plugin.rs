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

use crate::context_provider::ContextProvider;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::gpu::IGpuContextProvider;
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use std::any::TypeId;

pub struct PluginGpuDX12();

impl PluginGpuDX12 {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginGpuDX12 {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginGpuDX12".to_string(),
            description: "The aleph-engine dx12 RHI backend".to_string(),
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch_version: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.provides_interface::<dyn IGpuContextProvider>();
    }

    fn on_init(&mut self, _registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let context_provider = ContextProvider::new();

        let response = vec![(
            TypeId::of::<dyn IGpuContextProvider>(),
            AnyArc::into_any(AnyArc::new(context_provider)),
        )];
        Box::new(response)
    }
}

declare_interfaces!(PluginGpuDX12, [IPlugin]);
