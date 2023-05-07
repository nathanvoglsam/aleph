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

pub extern crate aleph_vulkan_alloc as vulkan_alloc;

extern crate cstr;

mod adapter;
mod buffer;
mod command_list;
mod context;
mod context_provider;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
mod encoder;
mod fence;
mod internal;
mod pipeline;
mod pipeline_layout;
mod queue;
mod sampler;
mod semaphore;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use plugin::PluginGpuVulkan;

mod plugin {
    use crate::context_provider::ContextProvider;
    use aleph_any::{declare_interfaces, AnyArc, IAny};
    use aleph_rhi_api::*;
    use interfaces::make_plugin_description_for_crate;
    use interfaces::plugin::{
        IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
    };
    use std::any::TypeId;

    pub struct PluginGpuVulkan();

    impl PluginGpuVulkan {
        pub fn new() -> Self {
            Self()
        }
    }

    impl IPlugin for PluginGpuVulkan {
        fn get_description(&self) -> PluginDescription {
            make_plugin_description_for_crate!()
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

    impl Default for PluginGpuVulkan {
        fn default() -> Self {
            Self::new()
        }
    }

    declare_interfaces!(PluginGpuVulkan, [IPlugin]);
}
