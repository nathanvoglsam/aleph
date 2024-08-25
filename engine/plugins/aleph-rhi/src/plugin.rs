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

use aleph_interfaces::any::{declare_interfaces, AnyArc, IAny};
use aleph_interfaces::make_plugin_description_for_crate;
use aleph_interfaces::platform::IWindowProvider;
use aleph_interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph_interfaces::rhi::IRhiProvider;
use aleph_rhi_api::{AdapterRequestOptions, BackendAPI};
use aleph_rhi_loader::{BackendConfigs, ContextOptions, RhiLoader};
use serde::Deserialize;

use crate::rhi_provider::RhiProvider;

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
            return Box::<Vec<(TypeId, AnyArc<dyn IAny>)>>::default();
        }

        let config = registry.config().unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        // Construct the context from the RHI loader with the final set of settings
        let context = self
            .rhi_loader
            .make_context(&ContextOptions {
                backend: config.api.into(),
                validation: config.validation,
                debug: config.debug,
                config: BackendConfigs {
                    vulkan: config.vulkan.map(|v| v.into()),
                    d3d12: config.d3d12.map(|v| v.into()),
                },
            })
            .unwrap();

        let window = registry
            .get_interface::<dyn IWindowProvider>()
            .and_then(|v| v.get_window());
        let surface = if let Some(window) = window {
            let surface = if cfg!(any(target_os = "ios", target_os = "macos")) {
                context.create_surface_for_metal_layer(window.metal_layer().unwrap())
            } else {
                context.create_surface(&window.as_ref(), &window.as_ref())
            };
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

#[derive(Copy, Clone, Debug, Deserialize)]
enum Backend {
    #[serde(rename = "d3d12")]
    D3D12,

    #[serde(rename = "vulkan")]
    Vulkan,
}

impl Into<BackendAPI> for Backend {
    fn into(self) -> BackendAPI {
        match self {
            Backend::D3D12 => BackendAPI::D3D12,
            Backend::Vulkan => BackendAPI::Vulkan,
        }
    }
}

#[derive(Deserialize)]
struct VulkanOptions {
    #[serde(rename = "denySync2")]
    pub deny_sync_2: bool,
}

impl VulkanOptions {
    pub fn log(&self) {
        log::info!("Config.vulkan.deny_sync_2 = {}", self.deny_sync_2);
    }
}

impl Into<aleph_rhi_loader::VulkanConfig> for VulkanOptions {
    fn into(self) -> aleph_rhi_loader::VulkanConfig {
        aleph_rhi_loader::VulkanConfig {
            deny_sync_2: self.deny_sync_2,
        }
    }
}

#[derive(Deserialize)]
struct D3D12Options {}

impl D3D12Options {
    pub fn log(&self) {}
}

impl Into<aleph_rhi_loader::D3D12Config> for D3D12Options {
    fn into(self) -> aleph_rhi_loader::D3D12Config {
        aleph_rhi_loader::D3D12Config {}
    }
}

#[derive(Deserialize)]
struct Config {
    pub api: Backend,

    #[serde(default)]
    pub vulkan: Option<VulkanOptions>,

    #[serde(default)]
    pub d3d12: Option<D3D12Options>,

    pub validation: bool,
    pub debug: bool,
}

impl Config {
    pub fn log(&self) {
        log::info!("Config.api = {:?}", self.api);

        self.vulkan.as_ref().inspect(|v| v.log());
        self.d3d12.as_ref().inspect(|v| v.log());

        log::info!("Config.validation = {}", self.validation);
        log::info!("Config.debug = {}", self.debug);
    }
}
