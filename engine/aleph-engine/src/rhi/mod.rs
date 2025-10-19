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

use aleph_interfaces::any::{AnyArc, declare_interfaces};
use aleph_interfaces::platform::IWindow;
use aleph_interfaces::rhi::IRhiProvider;
use aleph_rhi_api::*;
use aleph_rhi_loader::{
    BackendConfigs, ContextOptions, D3D12Config, MetalConfig, RhiLoader, VulkanConfig,
};
use interfaces::any::IAny;
use serde::Deserialize;

use crate::plugin_registry::RegistryAccessor;

pub(crate) fn rhi_interfaces() -> [TypeId; 1] {
    [TypeId::of::<dyn IRhiProvider>()]
}

pub(crate) struct Rhi {
    rhi_loader: RhiLoader,
}

impl Rhi {
    pub fn new() -> Self {
        Self {
            rhi_loader: RhiLoader::new(),
        }
    }
}

impl Rhi {
    pub(crate) fn on_init(&mut self, registry: &mut RegistryAccessor) {
        // If there are no GPU backends available we early exit and yield no provider as there's
        // nothing to provide
        if self.rhi_loader.backends().is_empty() {
            return;
        }

        let config = registry.configs.get("rhi").unwrap();
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
                    metal: config.metal.map(|v| v.into()),
                },
            })
            .unwrap();

        let window = registry
            .interfaces
            .get(&TypeId::of::<dyn IWindow>())
            .and_then(|v| v.query_interface::<dyn IWindow>());
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
                power_class: AdapterPowerClass::HighPower,
                type_preference: Default::default(),
                allow_software_adapters: true,
                deny_hardware_adapters: false,
            })
            .unwrap();

        let device = adapter.request_device().unwrap();

        let provider = AnyArc::new(RhiProvider {
            surface,
            adapter,
            device,
        });

        registry.interfaces.insert(
            TypeId::of::<dyn IRhiProvider>(),
            AnyArc::map::<dyn IAny, _>(provider, |v| v),
        );
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

pub(crate) struct RhiProvider {
    pub(crate) surface: Option<AnyArc<dyn ISurface>>,
    pub(crate) adapter: AnyArc<dyn IAdapter>,
    pub(crate) device: AnyArc<dyn IDevice>,
}

impl IRhiProvider for RhiProvider {
    fn surface(&self) -> Option<AnyArc<dyn ISurface>> {
        self.surface.clone()
    }

    fn adapter(&self) -> AnyArc<dyn IAdapter> {
        self.adapter.clone()
    }

    fn device(&self) -> AnyArc<dyn IDevice> {
        self.device.clone()
    }
}

declare_interfaces!(RhiProvider, [IRhiProvider]);
