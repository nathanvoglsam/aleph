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

use std::ops::Deref;
use std::sync::Arc;

use aleph_rhi_api::*;
use aleph_rhi_loader::{
    BackendConfigs, ContextOptions, D3D12Config, MetalConfig, RhiLoader, VulkanConfig,
};
use api::platform::AWindow;
use api::plugin::{
    IPlugin, IPluginRegistrar, IRegistryAccessor, InitOrder, PluginDescription, Provides,
};
use api::rhi::{ARhiProvider, IRhiProvider};
use serde::Deserialize;

pub(crate) struct CoreRhi {
    rhi_loader: RhiLoader,
}

impl CoreRhi {
    pub fn new() -> Self {
        Self {
            rhi_loader: RhiLoader::new(),
        }
    }
}

impl IPlugin for CoreRhi {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "CoreRhi".to_string(),
            description: "Provides an rhi::IDevice".to_string(),
            major_version: 1,
            minor_version: 0,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.uses::<AWindow>(InitOrder::After);
        registrar.provides::<ARhiProvider>(Provides::Always);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let config = registry.config("rhi").unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        let window = registry.get_interface::<AWindow>();

        // If there are no GPU backends available we early exit and yield no provider as there's
        // nothing to provide
        if self.rhi_loader.backends().is_empty() {
            return;
        }

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
                    metal: config.metal.map(|v| v.into()),
                },
            })
            .unwrap();

        let surface = if let Some(window) = window {
            let surface = if cfg!(any(target_os = "ios", target_os = "macos")) {
                context.create_surface_for_metal_layer(window.metal_layer().unwrap())
            } else {
                context.create_surface(window.deref(), window.deref())
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
                power_class: AdapterPowerClass::HighPower,
                type_preference: Default::default(),
                allow_software_adapters: true,
                deny_hardware_adapters: false,
            })
            .unwrap();

        let device = adapter.request_device().unwrap();

        let provider = Arc::new(RhiProvider {
            surface,
            adapter,
            device,
        });

        registry.provide(ARhiProvider(provider));
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
enum Backend {
    #[serde(alias = "d3d12")]
    D3D12,

    #[serde(alias = "vulkan", alias = "VULKAN")]
    Vulkan,

    #[serde(alias = "metal", alias = "METAL")]
    Metal,
}

impl Into<BackendAPI> for Backend {
    fn into(self) -> BackendAPI {
        match self {
            Backend::D3D12 => BackendAPI::D3D12,
            Backend::Vulkan => BackendAPI::Vulkan,
            Backend::Metal => BackendAPI::Metal,
        }
    }
}

#[derive(Deserialize)]
struct Config {
    pub api: Backend,

    #[serde(default)]
    pub vulkan: Option<VulkanConfig>,

    #[serde(default)]
    pub d3d12: Option<D3D12Config>,

    #[serde(default)]
    pub metal: Option<MetalConfig>,

    pub validation: bool,
    pub debug: bool,
}

impl Config {
    pub fn log(&self) {
        log::info!("rhi.api = {:?}", self.api);

        // self.vulkan.as_ref().inspect(|v| {});
        // self.d3d12.as_ref().inspect(|v| {});

        log::info!("rhi.validation = {}", self.validation);
        log::info!("rhi.debug = {}", self.debug);
    }
}

struct RhiProvider {
    surface: Option<Arc<dyn ISurface>>,
    adapter: Arc<dyn IAdapter>,
    device: Arc<dyn IDevice>,
}

impl IRhiProvider for RhiProvider {
    fn surface(&self) -> Option<Arc<dyn ISurface>> {
        self.surface.clone()
    }

    fn adapter(&self) -> Arc<dyn IAdapter> {
        self.adapter.clone()
    }

    fn device(&self) -> Arc<dyn IDevice> {
        self.device.clone()
    }
}
