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

use crate::egui::EguiPlugin;
use crate::plugin_registry::interfaces::plugin::IPlugin;
use crate::plugin_registry::{PluginRegistry, PluginRegistryBuilder};
use aleph_sdl2::PlatformSDL2;
use aleph_windows_raw::{initialize_mta, name_current_thread};
use utf16_lit::utf16_null;

pub struct EngineBuilder {
    registry: PluginRegistryBuilder,
}

impl EngineBuilder {
    pub fn new() -> Self {
        // Initialize COM with MTA
        #[cfg(target_os = "windows")]
        initialize_mta().unwrap();

        #[cfg(target_os = "windows")]
        unsafe {
            name_current_thread(&utf16_null!("MainThread"));
        }

        // First thing we do is initialize the log backend so everything can log from now on
        aleph_logger::init();
        aleph_log::info!("");
        aleph_log::info!("Aleph Engine Starting");

        // Print engine info to the log so we know what engine version we're running on
        aleph_log::info!("");
        Engine::log_engine_info();
        aleph_log::info!("");

        // Print some system info to the log so we know what we were running on
        aleph_log::info!("");
        Engine::log_cpu_info();
        aleph_log::info!("");

        Self {
            registry: PluginRegistry::builder(),
        }
    }

    pub fn default_plugins(&mut self) -> &mut Self {
        self.plugin(PlatformSDL2::new());
        self.plugin(EguiPlugin::new());

        #[cfg(target_os = "windows")]
        self.plugin(aleph_render::RenderPlugin::new());

        self
    }

    pub fn plugin(&mut self, plugin: impl IPlugin) -> &mut Self {
        self.registry.plugin(plugin);
        self
    }

    pub fn build(self) -> Engine {
        Engine {
            registry: self.registry.build(),
        }
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
        let engine_version = "0.1.0";
        let arch = aleph_target::build::target_architecture().name();
        let os = aleph_target::build::target_platform().pretty_name();
        let build = aleph_target::build::target_build_type().pretty_name();
        aleph_log::info!("=== Engine Info ===");
        aleph_log::info!("Name    : {}", engine_name);
        aleph_log::info!("Version : {}", engine_version);
        aleph_log::info!("Arch    : {}", arch);
        aleph_log::info!("OS      : {}", os);
        aleph_log::info!("Build   : {}", build);
    }

    ///
    /// Internal function for logging info about the CPU that is being used
    ///
    fn log_cpu_info() {
        let cpu_vendor = aleph_cpu_info::cpu_vendor();
        let cpu_brand = aleph_cpu_info::cpu_brand();
        let physical_cpus = aleph_cpu_info::physical_core_count();
        let logical_cpus = aleph_cpu_info::logical_core_count();
        //let system_ram = Platform::system_ram(); // TODO: Replacement for this
        aleph_log::info!("=== CPU INFO ===");
        aleph_log::info!("CPU Vendor    : {}", cpu_vendor);
        aleph_log::info!("CPU Brand     : {}", cpu_brand);
        aleph_log::info!("Physical CPUs : {}", physical_cpus);
        aleph_log::info!("Logical CPUs  : {}", logical_cpus);
        //aleph_log::info!("System RAM    : {}MB", system_ram);
    }
}
