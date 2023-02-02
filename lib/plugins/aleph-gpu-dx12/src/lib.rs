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

#![cfg(target_os = "windows")]

pub extern crate aleph_dx12_alloc as dx12_alloc;
pub extern crate aleph_pix as pix;
pub extern crate aleph_windows as windows;

extern crate aleph_interfaces as interfaces;
extern crate aleph_log as log;
extern crate cstr;

mod adapter;
mod buffer;
mod command_list;
mod command_pool;
mod context;
mod context_provider;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
mod encoder;
mod internal;
mod pipeline;
mod pipeline_layout;
mod queue;
mod sampler;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use shader::ShaderData;
pub use plugin::PluginGpuDX12;

mod plugin {
    use crate::context_provider::ContextProvider;
    use interfaces::any::{declare_interfaces, AnyArc, IAny};
    use interfaces::gpu::IContextProvider;
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
            registrar.provides_interface::<dyn IContextProvider>();
        }

        fn on_init(&mut self, _registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
            let context_provider = AnyArc::new(ContextProvider::new());

            let response = vec![(
                TypeId::of::<dyn IContextProvider>(),
                AnyArc::map::<dyn IAny, _>(context_provider, |v| v),
            )];
            Box::new(response)
        }
    }

    impl Default for PluginGpuDX12 {
        fn default() -> Self {
            Self::new()
        }
    }

    declare_interfaces!(PluginGpuDX12, [IPlugin]);
}
