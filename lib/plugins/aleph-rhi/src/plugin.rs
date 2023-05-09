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

use crate::rhi_provider::RhiProvider;
use aleph_interfaces::any::{declare_interfaces, AnyArc, IAny};
use aleph_interfaces::make_plugin_description_for_crate;
use aleph_interfaces::platform::IWindowProvider;
use aleph_interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph_interfaces::rhi::IRhiProvider;
use aleph_rhi_api::{AdapterRequestOptions, BackendAPI};
use aleph_rhi_loader::{ContextOptions, RhiLoader};
use std::any::TypeId;
use std::ops::Deref;

pub struct PluginRHI {
    rhi_loader: RhiLoader,
    rhi_provider: Option<AnyArc<RhiProvider>>,
}

impl PluginRHI {
    pub fn new() -> Self {
        Self {
            rhi_loader: RhiLoader::new(),
            rhi_provider: None,
        }
    }
}

impl Default for PluginRHI {
    fn default() -> Self {
        PluginRHI::new()
    }
}

impl IPlugin for PluginRHI {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.must_init_after::<dyn IWindowProvider>();

        if !self.rhi_loader.backends().is_empty() {
            registrar.provides_interface::<dyn IRhiProvider>();
        }
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        // If there are no GPU backends available we early exit and yield no provider as there's
        // nothing to provide
        if self.rhi_loader.backends().is_empty() {
            return Box::new(vec![]);
        }

        // Construct the context from the RHI loader with the final set of settings
        let context = self
            .rhi_loader
            .make_context(&ContextOptions {
                preferred_api: Some(BackendAPI::Vulkan),
                denied_backends: None,
                required_backend: None,
                validation: false,
                debug: false,
            })
            .unwrap();

        let window = registry
            .get_interface::<dyn IWindowProvider>()
            .and_then(|v| v.get_window());
        let surface = if let Some(window) = window {
            let surface = context.create_surface(&window.deref());
            match surface {
                Ok(v) => Some(v),
                Err(v) => {
                    log::warn!("Failed to create a surface ({})", v);
                    None
                }
            }
        } else {
            None
        };

        let adapter = context
            .request_adapter(&AdapterRequestOptions {
                surface: surface.as_ref().map(|v| v.as_ref()),
                power_class: Default::default(),
                type_preference: Default::default(),
                allow_software_adapters: true,
                deny_hardware_adapters: false,
            })
            .unwrap();

        let device = adapter.request_device().unwrap();

        self.rhi_provider = Some(AnyArc::new(RhiProvider {
            surface,
            adapter,
            device,
        }));

        let response = vec![(
            TypeId::of::<dyn IRhiProvider>(),
            AnyArc::map::<dyn IAny, _>(self.rhi_provider.clone().unwrap(), |v| v),
        )];
        Box::new(response)
    }
}

declare_interfaces!(PluginRHI, [IPlugin]);
