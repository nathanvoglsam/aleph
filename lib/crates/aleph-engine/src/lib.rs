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

// =================================================================================================
// Crate Imports
// =================================================================================================

// Re-export useful crates
extern crate aleph_core;
pub extern crate aleph_egui as egui;
pub extern crate aleph_interfaces as interfaces;
pub extern crate aleph_target as target;

pub mod plugin_registry;

pub mod core {
    pub use aleph_core::*;
}

pub mod any {
    pub use interfaces::any::*;
}

// =================================================================================================
// Modules
// =================================================================================================

use crate::interfaces::plugin::IPlugin;
use crate::plugin_registry::{PluginRegistry, PluginRegistryBuilder};

pub struct EngineBuilder {
    registry: PluginRegistryBuilder,
}

impl EngineBuilder {
    pub fn new() -> Self {
        // Initialize COM with MTA
        #[cfg(target_os = "windows")]
        unsafe {
            use aleph_windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
            CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED).unwrap();
        }

        #[cfg(target_os = "windows")]
        unsafe {
            aleph_windows::name_current_thread(&utf16_lit::utf16_null!("MainThread")).unwrap();
        }

        Self {
            registry: PluginRegistry::builder(),
        }
    }

    pub fn default_plugins(&mut self, headless: bool) -> &mut Self {
        self.plugin(core::PluginCore::new());

        if !headless {
            self.plugin(aleph_gpu_vulkan::PluginGpuVulkan::new());

            // #[cfg(target_os = "windows")]
            // self.plugin(aleph_gpu_dx12::PluginGpuDX12::new());
        }

        self.platform(headless);

        // This only makes sense to load on platforms we have a renderer for, and only if we're not
        // trying to run headless
        if !headless {
            self.plugin(egui::PluginEgui::new());
        }

        // This only makes sense to load on windows and not headless
        if !headless {
            self.plugin(aleph_render::PluginRender::new());
        }

        self
    }

    pub fn plugin(&mut self, plugin: impl IPlugin) -> &mut Self {
        self.registry.plugin(plugin);
        self
    }

    pub fn build(self) -> Engine {
        // Print engine info to the log so we know what engine version we're running on
        // First thing we do is initialize the log backend so everything can log from now on
        log::info!("");
        log::info!("Aleph Engine Starting");
        log::info!("");
        Engine::log_engine_info();
        log::info!("");

        // Print some system info to the log so we know what we were running on
        log::info!("");
        Engine::log_cpu_info();
        log::info!("");

        Engine {
            registry: self.registry.build(),
        }
    }

    #[cfg(feature = "aleph-sdl2")]
    fn platform(&mut self, headless: bool) {
        if headless {
            self.plugin(aleph_headless::PluginPlatformHeadless::new());
        } else {
            self.plugin(aleph_sdl2::PluginPlatformSDL2::new());
        }
    }

    #[cfg(not(feature = "aleph-sdl2"))]
    fn platform(&mut self, headless: bool) {
        if !headless {
            panic!("Requesting a non headless platform plugin when none is available");
        }
        self.plugin(aleph_headless::PluginPlatformHeadless::new())
    }
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Engine {
    registry: PluginRegistry,
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::new()
    }

    pub fn run(mut self) {
        self.registry.run()
    }

    ///
    /// Internal function for logging info about the engine
    ///
    fn log_engine_info() {
        let engine_name = "AlephEngine";
        let engine_version = env!("CARGO_PKG_VERSION");
        let arch = aleph_target::build::target_architecture().name();
        let os = aleph_target::build::target_platform().pretty_name();
        let build = aleph_target::build::target_build_type().pretty_name();
        let optimized = aleph_target::build::target_build_config().is_optimized();
        let debug = aleph_target::build::target_build_config().is_debug();
        log::info!("=== Engine Info ===");
        log::info!("Name       : {}", engine_name);
        log::info!("Version    : {}", engine_version);
        log::info!("Arch       : {}", arch);
        log::info!("OS         : {}", os);
        log::info!("Build Type : {}", build);
        log::info!("Optimized  : {}", optimized);
        log::info!("Debug      : {}", debug);
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_cpu_info() {
        let cpu_vendor = aleph_sys_info::cpu_vendor();
        let cpu_brand = aleph_sys_info::cpu_brand();
        let physical_cpus = aleph_sys_info::physical_core_count();
        let logical_cpus = aleph_sys_info::logical_core_count();
        let (system_ram_mib, system_ram_gib) = aleph_sys_info::installed_memory()
            .map(|v| {
                let v = v.get();
                let mib = (v / (1024 * 1024)).to_string();
                let gib = (v / (1024 * 1024 * 1024)).to_string();
                (mib, gib)
            })
            .unwrap_or(("Unknown".to_owned(), "Unknown".to_owned()));

        log::info!("=== CPU INFO ===");
        log::info!("CPU Vendor    : {}", cpu_vendor);
        log::info!("CPU Brand     : {}", cpu_brand);
        log::info!("Physical CPUs : {}", physical_cpus);
        log::info!("Logical CPUs  : {}", logical_cpus);
        log::info!(
            "System RAM    : {} MiB ({} GiB)",
            system_ram_mib,
            system_ram_gib
        );
    }
}
