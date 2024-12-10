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
pub extern crate aleph_egui as egui;
pub extern crate aleph_interfaces as interfaces;
pub extern crate aleph_target as target;

pub mod plugin_registry;

pub mod any {
    pub use interfaces::any::*;
}

// =================================================================================================
// Modules
// =================================================================================================

use std::env::current_dir;
use std::path::Path;

use log::LevelFilter;

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
            CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        }

        #[cfg(target_os = "windows")]
        unsafe {
            aleph_windows::name_current_thread(&utf16_lit::utf16_null!("MainThread")).unwrap();
        }

        rayon::ThreadPoolBuilder::new()
            .thread_name(|v| format!("Rayon Worker {v}"))
            .start_handler(|i| {
                // Initialize COM with MTA
                #[cfg(target_os = "windows")]
                unsafe {
                    use aleph_windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
                    CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
                }
                name_for_pool_thread_i(i);
            })
            .build_global()
            .unwrap();

        #[cfg(not(target_os = "android"))]
        fn create_logger() -> env_logger::Logger {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Trace)
                .build()
        }

        #[cfg(target_os = "android")]
        fn create_logger() -> android_logger::AndroidLogger {
            let config = android_logger::Config::default().with_max_level(log::LevelFilter::Trace);
            android_logger::AndroidLogger::new(config)
        }

        // This will be one of the earliest pieces of code to run in aleph engine so initialize the
        // logger here. By initializing it here then this plugin remains optional (technically)
        let logger = create_logger();
        log::set_boxed_logger(Box::new(logger)).expect("Attempting to install logger");
        log::set_max_level(LevelFilter::Trace);

        // Android won't log panics properly afaik? We re-route to log so we can see it in logcat.
        if cfg!(target_os = "android") {
            std::panic::set_hook(Box::new(|v| {
                log::error!("{}", v);
            }));
        }

        Self {
            registry: PluginRegistry::builder(),
        }
    }

    pub fn default_plugins(&mut self) -> &mut Self {
        self.plugin(aleph_rhi::PluginRHI::new());
        self.plugin(egui::PluginEgui::new());
        self.plugin(aleph_render::PluginRender::new());

        self
    }

    pub fn plugin(&mut self, plugin: impl IPlugin) -> &mut Self {
        self.registry.plugin(plugin);
        self
    }

    pub fn build(mut self, cont: impl FnOnce(Engine)) {
        aleph_sdl2::PluginPlatformSDL2::setup(move |v| {
            self.plugin(v);
            let engine = self.init();
            cont(engine)
        });
    }

    fn init(self) -> Engine {
        // Print engine info to the log so we know what engine version we're running on
        // First thing we do is initialize the log backend so everything can log from now on
        log::info!("Aleph Engine Starting");
        Engine::log_engine_info();

        // Print some system info to the log so we know what we were running on
        Engine::log_cpu_info();

        // Print some general environment info so we can deduce things about how we're running
        Engine::log_env_info();

        Engine {
            registry: self.registry.build(),
        }
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
    /// Internal function for logging info about the engine
    ///
    fn log_env_info() {
        let current_dir = current_dir();
        let working_dir = match &current_dir {
            Ok(v) => v.display(),
            Err(v) => {
                log::warn!("Failed to get current working directory. Reason {v:?}");
                Path::new("Unknown").display()
            }
        };
        log::info!("=== Environment Info ===");
        log::info!("Working Dir : {}", working_dir);
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

/// This function is a crime against nature, but we need the strings for thread names to be constant
/// literals for some profilers. So we just make a monstorous table and map the thread index to a
/// static name. From time to time we may need to make this bigger.
///
/// As of writing this (10/Oct/2024) the biggest core counts we could see are from AMD's giant
/// epyc server chips with 128 cores, we may need to bump the size of the table up if we get bigger
/// chips.
fn name_for_pool_thread_i(i: usize) {
    match i {
        0 => aleph_profile::register_thread!("Rayon Worker 0"),
        1 => aleph_profile::register_thread!("Rayon Worker 1"),
        2 => aleph_profile::register_thread!("Rayon Worker 2"),
        3 => aleph_profile::register_thread!("Rayon Worker 3"),
        4 => aleph_profile::register_thread!("Rayon Worker 4"),
        5 => aleph_profile::register_thread!("Rayon Worker 5"),
        6 => aleph_profile::register_thread!("Rayon Worker 6"),
        7 => aleph_profile::register_thread!("Rayon Worker 7"),
        8 => aleph_profile::register_thread!("Rayon Worker 8"),
        9 => aleph_profile::register_thread!("Rayon Worker 9"),
        10 => aleph_profile::register_thread!("Rayon Worker 10"),
        11 => aleph_profile::register_thread!("Rayon Worker 11"),
        12 => aleph_profile::register_thread!("Rayon Worker 12"),
        13 => aleph_profile::register_thread!("Rayon Worker 13"),
        14 => aleph_profile::register_thread!("Rayon Worker 14"),
        15 => aleph_profile::register_thread!("Rayon Worker 15"),
        16 => aleph_profile::register_thread!("Rayon Worker 16"),
        17 => aleph_profile::register_thread!("Rayon Worker 17"),
        18 => aleph_profile::register_thread!("Rayon Worker 18"),
        19 => aleph_profile::register_thread!("Rayon Worker 19"),
        20 => aleph_profile::register_thread!("Rayon Worker 20"),
        21 => aleph_profile::register_thread!("Rayon Worker 21"),
        22 => aleph_profile::register_thread!("Rayon Worker 22"),
        23 => aleph_profile::register_thread!("Rayon Worker 23"),
        24 => aleph_profile::register_thread!("Rayon Worker 24"),
        25 => aleph_profile::register_thread!("Rayon Worker 25"),
        26 => aleph_profile::register_thread!("Rayon Worker 26"),
        27 => aleph_profile::register_thread!("Rayon Worker 27"),
        28 => aleph_profile::register_thread!("Rayon Worker 28"),
        29 => aleph_profile::register_thread!("Rayon Worker 29"),
        30 => aleph_profile::register_thread!("Rayon Worker 30"),
        31 => aleph_profile::register_thread!("Rayon Worker 31"),
        32 => aleph_profile::register_thread!("Rayon Worker 32"),
        33 => aleph_profile::register_thread!("Rayon Worker 33"),
        34 => aleph_profile::register_thread!("Rayon Worker 34"),
        35 => aleph_profile::register_thread!("Rayon Worker 35"),
        36 => aleph_profile::register_thread!("Rayon Worker 36"),
        37 => aleph_profile::register_thread!("Rayon Worker 37"),
        38 => aleph_profile::register_thread!("Rayon Worker 38"),
        39 => aleph_profile::register_thread!("Rayon Worker 39"),
        40 => aleph_profile::register_thread!("Rayon Worker 40"),
        41 => aleph_profile::register_thread!("Rayon Worker 41"),
        42 => aleph_profile::register_thread!("Rayon Worker 42"),
        43 => aleph_profile::register_thread!("Rayon Worker 43"),
        44 => aleph_profile::register_thread!("Rayon Worker 44"),
        45 => aleph_profile::register_thread!("Rayon Worker 45"),
        46 => aleph_profile::register_thread!("Rayon Worker 46"),
        47 => aleph_profile::register_thread!("Rayon Worker 47"),
        48 => aleph_profile::register_thread!("Rayon Worker 48"),
        49 => aleph_profile::register_thread!("Rayon Worker 49"),
        50 => aleph_profile::register_thread!("Rayon Worker 50"),
        51 => aleph_profile::register_thread!("Rayon Worker 51"),
        52 => aleph_profile::register_thread!("Rayon Worker 52"),
        53 => aleph_profile::register_thread!("Rayon Worker 53"),
        54 => aleph_profile::register_thread!("Rayon Worker 54"),
        55 => aleph_profile::register_thread!("Rayon Worker 55"),
        56 => aleph_profile::register_thread!("Rayon Worker 56"),
        57 => aleph_profile::register_thread!("Rayon Worker 57"),
        58 => aleph_profile::register_thread!("Rayon Worker 58"),
        59 => aleph_profile::register_thread!("Rayon Worker 59"),
        60 => aleph_profile::register_thread!("Rayon Worker 60"),
        61 => aleph_profile::register_thread!("Rayon Worker 61"),
        62 => aleph_profile::register_thread!("Rayon Worker 62"),
        63 => aleph_profile::register_thread!("Rayon Worker 63"),
        64 => aleph_profile::register_thread!("Rayon Worker 64"),
        65 => aleph_profile::register_thread!("Rayon Worker 65"),
        66 => aleph_profile::register_thread!("Rayon Worker 66"),
        67 => aleph_profile::register_thread!("Rayon Worker 67"),
        68 => aleph_profile::register_thread!("Rayon Worker 68"),
        69 => aleph_profile::register_thread!("Rayon Worker 69"),
        70 => aleph_profile::register_thread!("Rayon Worker 70"),
        71 => aleph_profile::register_thread!("Rayon Worker 71"),
        72 => aleph_profile::register_thread!("Rayon Worker 72"),
        73 => aleph_profile::register_thread!("Rayon Worker 73"),
        74 => aleph_profile::register_thread!("Rayon Worker 74"),
        75 => aleph_profile::register_thread!("Rayon Worker 75"),
        76 => aleph_profile::register_thread!("Rayon Worker 76"),
        77 => aleph_profile::register_thread!("Rayon Worker 77"),
        78 => aleph_profile::register_thread!("Rayon Worker 78"),
        79 => aleph_profile::register_thread!("Rayon Worker 79"),
        80 => aleph_profile::register_thread!("Rayon Worker 80"),
        81 => aleph_profile::register_thread!("Rayon Worker 81"),
        82 => aleph_profile::register_thread!("Rayon Worker 82"),
        83 => aleph_profile::register_thread!("Rayon Worker 83"),
        84 => aleph_profile::register_thread!("Rayon Worker 84"),
        85 => aleph_profile::register_thread!("Rayon Worker 85"),
        86 => aleph_profile::register_thread!("Rayon Worker 86"),
        87 => aleph_profile::register_thread!("Rayon Worker 87"),
        88 => aleph_profile::register_thread!("Rayon Worker 88"),
        89 => aleph_profile::register_thread!("Rayon Worker 89"),
        90 => aleph_profile::register_thread!("Rayon Worker 90"),
        91 => aleph_profile::register_thread!("Rayon Worker 91"),
        92 => aleph_profile::register_thread!("Rayon Worker 92"),
        93 => aleph_profile::register_thread!("Rayon Worker 93"),
        94 => aleph_profile::register_thread!("Rayon Worker 94"),
        95 => aleph_profile::register_thread!("Rayon Worker 95"),
        96 => aleph_profile::register_thread!("Rayon Worker 96"),
        97 => aleph_profile::register_thread!("Rayon Worker 97"),
        98 => aleph_profile::register_thread!("Rayon Worker 98"),
        99 => aleph_profile::register_thread!("Rayon Worker 99"),
        100 => aleph_profile::register_thread!("Rayon Worker 100"),
        101 => aleph_profile::register_thread!("Rayon Worker 101"),
        102 => aleph_profile::register_thread!("Rayon Worker 102"),
        103 => aleph_profile::register_thread!("Rayon Worker 103"),
        104 => aleph_profile::register_thread!("Rayon Worker 104"),
        105 => aleph_profile::register_thread!("Rayon Worker 105"),
        106 => aleph_profile::register_thread!("Rayon Worker 106"),
        107 => aleph_profile::register_thread!("Rayon Worker 107"),
        108 => aleph_profile::register_thread!("Rayon Worker 108"),
        109 => aleph_profile::register_thread!("Rayon Worker 109"),
        110 => aleph_profile::register_thread!("Rayon Worker 110"),
        111 => aleph_profile::register_thread!("Rayon Worker 111"),
        112 => aleph_profile::register_thread!("Rayon Worker 112"),
        113 => aleph_profile::register_thread!("Rayon Worker 113"),
        114 => aleph_profile::register_thread!("Rayon Worker 114"),
        115 => aleph_profile::register_thread!("Rayon Worker 115"),
        116 => aleph_profile::register_thread!("Rayon Worker 116"),
        117 => aleph_profile::register_thread!("Rayon Worker 117"),
        118 => aleph_profile::register_thread!("Rayon Worker 118"),
        119 => aleph_profile::register_thread!("Rayon Worker 119"),
        120 => aleph_profile::register_thread!("Rayon Worker 120"),
        121 => aleph_profile::register_thread!("Rayon Worker 121"),
        122 => aleph_profile::register_thread!("Rayon Worker 122"),
        123 => aleph_profile::register_thread!("Rayon Worker 123"),
        124 => aleph_profile::register_thread!("Rayon Worker 124"),
        125 => aleph_profile::register_thread!("Rayon Worker 125"),
        126 => aleph_profile::register_thread!("Rayon Worker 126"),
        127 => aleph_profile::register_thread!("Rayon Worker 127"),
        128 => aleph_profile::register_thread!("Rayon Worker 128"),
        129 => aleph_profile::register_thread!("Rayon Worker 129"),
        130 => aleph_profile::register_thread!("Rayon Worker 130"),
        131 => aleph_profile::register_thread!("Rayon Worker 131"),
        132 => aleph_profile::register_thread!("Rayon Worker 132"),
        133 => aleph_profile::register_thread!("Rayon Worker 133"),
        134 => aleph_profile::register_thread!("Rayon Worker 134"),
        135 => aleph_profile::register_thread!("Rayon Worker 135"),
        136 => aleph_profile::register_thread!("Rayon Worker 136"),
        137 => aleph_profile::register_thread!("Rayon Worker 137"),
        138 => aleph_profile::register_thread!("Rayon Worker 138"),
        139 => aleph_profile::register_thread!("Rayon Worker 139"),
        140 => aleph_profile::register_thread!("Rayon Worker 140"),
        141 => aleph_profile::register_thread!("Rayon Worker 141"),
        142 => aleph_profile::register_thread!("Rayon Worker 142"),
        143 => aleph_profile::register_thread!("Rayon Worker 143"),
        144 => aleph_profile::register_thread!("Rayon Worker 144"),
        145 => aleph_profile::register_thread!("Rayon Worker 145"),
        146 => aleph_profile::register_thread!("Rayon Worker 146"),
        147 => aleph_profile::register_thread!("Rayon Worker 147"),
        148 => aleph_profile::register_thread!("Rayon Worker 148"),
        149 => aleph_profile::register_thread!("Rayon Worker 149"),
        150 => aleph_profile::register_thread!("Rayon Worker 150"),
        151 => aleph_profile::register_thread!("Rayon Worker 151"),
        152 => aleph_profile::register_thread!("Rayon Worker 152"),
        153 => aleph_profile::register_thread!("Rayon Worker 153"),
        154 => aleph_profile::register_thread!("Rayon Worker 154"),
        155 => aleph_profile::register_thread!("Rayon Worker 155"),
        156 => aleph_profile::register_thread!("Rayon Worker 156"),
        157 => aleph_profile::register_thread!("Rayon Worker 157"),
        158 => aleph_profile::register_thread!("Rayon Worker 158"),
        159 => aleph_profile::register_thread!("Rayon Worker 159"),
        160 => aleph_profile::register_thread!("Rayon Worker 160"),
        161 => aleph_profile::register_thread!("Rayon Worker 161"),
        162 => aleph_profile::register_thread!("Rayon Worker 162"),
        163 => aleph_profile::register_thread!("Rayon Worker 163"),
        164 => aleph_profile::register_thread!("Rayon Worker 164"),
        165 => aleph_profile::register_thread!("Rayon Worker 165"),
        166 => aleph_profile::register_thread!("Rayon Worker 166"),
        167 => aleph_profile::register_thread!("Rayon Worker 167"),
        168 => aleph_profile::register_thread!("Rayon Worker 168"),
        169 => aleph_profile::register_thread!("Rayon Worker 169"),
        170 => aleph_profile::register_thread!("Rayon Worker 170"),
        171 => aleph_profile::register_thread!("Rayon Worker 171"),
        172 => aleph_profile::register_thread!("Rayon Worker 172"),
        173 => aleph_profile::register_thread!("Rayon Worker 173"),
        174 => aleph_profile::register_thread!("Rayon Worker 174"),
        175 => aleph_profile::register_thread!("Rayon Worker 175"),
        176 => aleph_profile::register_thread!("Rayon Worker 176"),
        177 => aleph_profile::register_thread!("Rayon Worker 177"),
        178 => aleph_profile::register_thread!("Rayon Worker 178"),
        179 => aleph_profile::register_thread!("Rayon Worker 179"),
        180 => aleph_profile::register_thread!("Rayon Worker 180"),
        181 => aleph_profile::register_thread!("Rayon Worker 181"),
        182 => aleph_profile::register_thread!("Rayon Worker 182"),
        183 => aleph_profile::register_thread!("Rayon Worker 183"),
        184 => aleph_profile::register_thread!("Rayon Worker 184"),
        185 => aleph_profile::register_thread!("Rayon Worker 185"),
        186 => aleph_profile::register_thread!("Rayon Worker 186"),
        187 => aleph_profile::register_thread!("Rayon Worker 187"),
        188 => aleph_profile::register_thread!("Rayon Worker 188"),
        189 => aleph_profile::register_thread!("Rayon Worker 189"),
        190 => aleph_profile::register_thread!("Rayon Worker 190"),
        191 => aleph_profile::register_thread!("Rayon Worker 191"),
        192 => aleph_profile::register_thread!("Rayon Worker 192"),
        193 => aleph_profile::register_thread!("Rayon Worker 193"),
        194 => aleph_profile::register_thread!("Rayon Worker 194"),
        195 => aleph_profile::register_thread!("Rayon Worker 195"),
        196 => aleph_profile::register_thread!("Rayon Worker 196"),
        197 => aleph_profile::register_thread!("Rayon Worker 197"),
        198 => aleph_profile::register_thread!("Rayon Worker 198"),
        199 => aleph_profile::register_thread!("Rayon Worker 199"),
        200 => aleph_profile::register_thread!("Rayon Worker 200"),
        201 => aleph_profile::register_thread!("Rayon Worker 201"),
        202 => aleph_profile::register_thread!("Rayon Worker 202"),
        203 => aleph_profile::register_thread!("Rayon Worker 203"),
        204 => aleph_profile::register_thread!("Rayon Worker 204"),
        205 => aleph_profile::register_thread!("Rayon Worker 205"),
        206 => aleph_profile::register_thread!("Rayon Worker 206"),
        207 => aleph_profile::register_thread!("Rayon Worker 207"),
        208 => aleph_profile::register_thread!("Rayon Worker 208"),
        209 => aleph_profile::register_thread!("Rayon Worker 209"),
        210 => aleph_profile::register_thread!("Rayon Worker 210"),
        211 => aleph_profile::register_thread!("Rayon Worker 211"),
        212 => aleph_profile::register_thread!("Rayon Worker 212"),
        213 => aleph_profile::register_thread!("Rayon Worker 213"),
        214 => aleph_profile::register_thread!("Rayon Worker 214"),
        215 => aleph_profile::register_thread!("Rayon Worker 215"),
        216 => aleph_profile::register_thread!("Rayon Worker 216"),
        217 => aleph_profile::register_thread!("Rayon Worker 217"),
        218 => aleph_profile::register_thread!("Rayon Worker 218"),
        219 => aleph_profile::register_thread!("Rayon Worker 219"),
        220 => aleph_profile::register_thread!("Rayon Worker 220"),
        221 => aleph_profile::register_thread!("Rayon Worker 221"),
        222 => aleph_profile::register_thread!("Rayon Worker 222"),
        223 => aleph_profile::register_thread!("Rayon Worker 223"),
        224 => aleph_profile::register_thread!("Rayon Worker 224"),
        225 => aleph_profile::register_thread!("Rayon Worker 225"),
        226 => aleph_profile::register_thread!("Rayon Worker 226"),
        227 => aleph_profile::register_thread!("Rayon Worker 227"),
        228 => aleph_profile::register_thread!("Rayon Worker 228"),
        229 => aleph_profile::register_thread!("Rayon Worker 229"),
        230 => aleph_profile::register_thread!("Rayon Worker 230"),
        231 => aleph_profile::register_thread!("Rayon Worker 231"),
        232 => aleph_profile::register_thread!("Rayon Worker 232"),
        233 => aleph_profile::register_thread!("Rayon Worker 233"),
        234 => aleph_profile::register_thread!("Rayon Worker 234"),
        235 => aleph_profile::register_thread!("Rayon Worker 235"),
        236 => aleph_profile::register_thread!("Rayon Worker 236"),
        237 => aleph_profile::register_thread!("Rayon Worker 237"),
        238 => aleph_profile::register_thread!("Rayon Worker 238"),
        239 => aleph_profile::register_thread!("Rayon Worker 239"),
        240 => aleph_profile::register_thread!("Rayon Worker 240"),
        241 => aleph_profile::register_thread!("Rayon Worker 241"),
        242 => aleph_profile::register_thread!("Rayon Worker 242"),
        243 => aleph_profile::register_thread!("Rayon Worker 243"),
        244 => aleph_profile::register_thread!("Rayon Worker 244"),
        245 => aleph_profile::register_thread!("Rayon Worker 245"),
        246 => aleph_profile::register_thread!("Rayon Worker 246"),
        247 => aleph_profile::register_thread!("Rayon Worker 247"),
        248 => aleph_profile::register_thread!("Rayon Worker 248"),
        249 => aleph_profile::register_thread!("Rayon Worker 249"),
        250 => aleph_profile::register_thread!("Rayon Worker 250"),
        251 => aleph_profile::register_thread!("Rayon Worker 251"),
        252 => aleph_profile::register_thread!("Rayon Worker 252"),
        253 => aleph_profile::register_thread!("Rayon Worker 253"),
        254 => aleph_profile::register_thread!("Rayon Worker 254"),
        255 => aleph_profile::register_thread!("Rayon Worker 255"),
        256 => aleph_profile::register_thread!("Rayon Worker 256"),
        257 => aleph_profile::register_thread!("Rayon Worker 257"),
        258 => aleph_profile::register_thread!("Rayon Worker 258"),
        259 => aleph_profile::register_thread!("Rayon Worker 259"),
        260 => aleph_profile::register_thread!("Rayon Worker 260"),
        261 => aleph_profile::register_thread!("Rayon Worker 261"),
        262 => aleph_profile::register_thread!("Rayon Worker 262"),
        263 => aleph_profile::register_thread!("Rayon Worker 263"),
        264 => aleph_profile::register_thread!("Rayon Worker 264"),
        265 => aleph_profile::register_thread!("Rayon Worker 265"),
        266 => aleph_profile::register_thread!("Rayon Worker 266"),
        267 => aleph_profile::register_thread!("Rayon Worker 267"),
        268 => aleph_profile::register_thread!("Rayon Worker 268"),
        269 => aleph_profile::register_thread!("Rayon Worker 269"),
        270 => aleph_profile::register_thread!("Rayon Worker 270"),
        271 => aleph_profile::register_thread!("Rayon Worker 271"),
        272 => aleph_profile::register_thread!("Rayon Worker 272"),
        273 => aleph_profile::register_thread!("Rayon Worker 273"),
        274 => aleph_profile::register_thread!("Rayon Worker 274"),
        275 => aleph_profile::register_thread!("Rayon Worker 275"),
        276 => aleph_profile::register_thread!("Rayon Worker 276"),
        277 => aleph_profile::register_thread!("Rayon Worker 277"),
        278 => aleph_profile::register_thread!("Rayon Worker 278"),
        279 => aleph_profile::register_thread!("Rayon Worker 279"),
        280 => aleph_profile::register_thread!("Rayon Worker 280"),
        281 => aleph_profile::register_thread!("Rayon Worker 281"),
        282 => aleph_profile::register_thread!("Rayon Worker 282"),
        283 => aleph_profile::register_thread!("Rayon Worker 283"),
        284 => aleph_profile::register_thread!("Rayon Worker 284"),
        285 => aleph_profile::register_thread!("Rayon Worker 285"),
        286 => aleph_profile::register_thread!("Rayon Worker 286"),
        287 => aleph_profile::register_thread!("Rayon Worker 287"),
        288 => aleph_profile::register_thread!("Rayon Worker 288"),
        289 => aleph_profile::register_thread!("Rayon Worker 289"),
        290 => aleph_profile::register_thread!("Rayon Worker 290"),
        291 => aleph_profile::register_thread!("Rayon Worker 291"),
        292 => aleph_profile::register_thread!("Rayon Worker 292"),
        293 => aleph_profile::register_thread!("Rayon Worker 293"),
        294 => aleph_profile::register_thread!("Rayon Worker 294"),
        295 => aleph_profile::register_thread!("Rayon Worker 295"),
        296 => aleph_profile::register_thread!("Rayon Worker 296"),
        297 => aleph_profile::register_thread!("Rayon Worker 297"),
        298 => aleph_profile::register_thread!("Rayon Worker 298"),
        299 => aleph_profile::register_thread!("Rayon Worker 299"),
        300 => aleph_profile::register_thread!("Rayon Worker 300"),
        301 => aleph_profile::register_thread!("Rayon Worker 301"),
        302 => aleph_profile::register_thread!("Rayon Worker 302"),
        303 => aleph_profile::register_thread!("Rayon Worker 303"),
        304 => aleph_profile::register_thread!("Rayon Worker 304"),
        305 => aleph_profile::register_thread!("Rayon Worker 305"),
        306 => aleph_profile::register_thread!("Rayon Worker 306"),
        307 => aleph_profile::register_thread!("Rayon Worker 307"),
        308 => aleph_profile::register_thread!("Rayon Worker 308"),
        309 => aleph_profile::register_thread!("Rayon Worker 309"),
        310 => aleph_profile::register_thread!("Rayon Worker 310"),
        311 => aleph_profile::register_thread!("Rayon Worker 311"),
        312 => aleph_profile::register_thread!("Rayon Worker 312"),
        313 => aleph_profile::register_thread!("Rayon Worker 313"),
        314 => aleph_profile::register_thread!("Rayon Worker 314"),
        315 => aleph_profile::register_thread!("Rayon Worker 315"),
        316 => aleph_profile::register_thread!("Rayon Worker 316"),
        317 => aleph_profile::register_thread!("Rayon Worker 317"),
        318 => aleph_profile::register_thread!("Rayon Worker 318"),
        319 => aleph_profile::register_thread!("Rayon Worker 319"),
        320 => aleph_profile::register_thread!("Rayon Worker 320"),
        321 => aleph_profile::register_thread!("Rayon Worker 321"),
        322 => aleph_profile::register_thread!("Rayon Worker 322"),
        323 => aleph_profile::register_thread!("Rayon Worker 323"),
        324 => aleph_profile::register_thread!("Rayon Worker 324"),
        325 => aleph_profile::register_thread!("Rayon Worker 325"),
        326 => aleph_profile::register_thread!("Rayon Worker 326"),
        327 => aleph_profile::register_thread!("Rayon Worker 327"),
        328 => aleph_profile::register_thread!("Rayon Worker 328"),
        329 => aleph_profile::register_thread!("Rayon Worker 329"),
        330 => aleph_profile::register_thread!("Rayon Worker 330"),
        331 => aleph_profile::register_thread!("Rayon Worker 331"),
        332 => aleph_profile::register_thread!("Rayon Worker 332"),
        333 => aleph_profile::register_thread!("Rayon Worker 333"),
        334 => aleph_profile::register_thread!("Rayon Worker 334"),
        335 => aleph_profile::register_thread!("Rayon Worker 335"),
        336 => aleph_profile::register_thread!("Rayon Worker 336"),
        337 => aleph_profile::register_thread!("Rayon Worker 337"),
        338 => aleph_profile::register_thread!("Rayon Worker 338"),
        339 => aleph_profile::register_thread!("Rayon Worker 339"),
        340 => aleph_profile::register_thread!("Rayon Worker 340"),
        341 => aleph_profile::register_thread!("Rayon Worker 341"),
        342 => aleph_profile::register_thread!("Rayon Worker 342"),
        343 => aleph_profile::register_thread!("Rayon Worker 343"),
        344 => aleph_profile::register_thread!("Rayon Worker 344"),
        345 => aleph_profile::register_thread!("Rayon Worker 345"),
        346 => aleph_profile::register_thread!("Rayon Worker 346"),
        347 => aleph_profile::register_thread!("Rayon Worker 347"),
        348 => aleph_profile::register_thread!("Rayon Worker 348"),
        349 => aleph_profile::register_thread!("Rayon Worker 349"),
        350 => aleph_profile::register_thread!("Rayon Worker 350"),
        351 => aleph_profile::register_thread!("Rayon Worker 351"),
        352 => aleph_profile::register_thread!("Rayon Worker 352"),
        353 => aleph_profile::register_thread!("Rayon Worker 353"),
        354 => aleph_profile::register_thread!("Rayon Worker 354"),
        355 => aleph_profile::register_thread!("Rayon Worker 355"),
        356 => aleph_profile::register_thread!("Rayon Worker 356"),
        357 => aleph_profile::register_thread!("Rayon Worker 357"),
        358 => aleph_profile::register_thread!("Rayon Worker 358"),
        359 => aleph_profile::register_thread!("Rayon Worker 359"),
        360 => aleph_profile::register_thread!("Rayon Worker 360"),
        361 => aleph_profile::register_thread!("Rayon Worker 361"),
        362 => aleph_profile::register_thread!("Rayon Worker 362"),
        363 => aleph_profile::register_thread!("Rayon Worker 363"),
        364 => aleph_profile::register_thread!("Rayon Worker 364"),
        365 => aleph_profile::register_thread!("Rayon Worker 365"),
        366 => aleph_profile::register_thread!("Rayon Worker 366"),
        367 => aleph_profile::register_thread!("Rayon Worker 367"),
        368 => aleph_profile::register_thread!("Rayon Worker 368"),
        369 => aleph_profile::register_thread!("Rayon Worker 369"),
        370 => aleph_profile::register_thread!("Rayon Worker 370"),
        371 => aleph_profile::register_thread!("Rayon Worker 371"),
        372 => aleph_profile::register_thread!("Rayon Worker 372"),
        373 => aleph_profile::register_thread!("Rayon Worker 373"),
        374 => aleph_profile::register_thread!("Rayon Worker 374"),
        375 => aleph_profile::register_thread!("Rayon Worker 375"),
        376 => aleph_profile::register_thread!("Rayon Worker 376"),
        377 => aleph_profile::register_thread!("Rayon Worker 377"),
        378 => aleph_profile::register_thread!("Rayon Worker 378"),
        379 => aleph_profile::register_thread!("Rayon Worker 379"),
        380 => aleph_profile::register_thread!("Rayon Worker 380"),
        381 => aleph_profile::register_thread!("Rayon Worker 381"),
        382 => aleph_profile::register_thread!("Rayon Worker 382"),
        383 => aleph_profile::register_thread!("Rayon Worker 383"),
        384 => aleph_profile::register_thread!("Rayon Worker 384"),
        385 => aleph_profile::register_thread!("Rayon Worker 385"),
        386 => aleph_profile::register_thread!("Rayon Worker 386"),
        387 => aleph_profile::register_thread!("Rayon Worker 387"),
        388 => aleph_profile::register_thread!("Rayon Worker 388"),
        389 => aleph_profile::register_thread!("Rayon Worker 389"),
        390 => aleph_profile::register_thread!("Rayon Worker 390"),
        391 => aleph_profile::register_thread!("Rayon Worker 391"),
        392 => aleph_profile::register_thread!("Rayon Worker 392"),
        393 => aleph_profile::register_thread!("Rayon Worker 393"),
        394 => aleph_profile::register_thread!("Rayon Worker 394"),
        395 => aleph_profile::register_thread!("Rayon Worker 395"),
        396 => aleph_profile::register_thread!("Rayon Worker 396"),
        397 => aleph_profile::register_thread!("Rayon Worker 397"),
        398 => aleph_profile::register_thread!("Rayon Worker 398"),
        399 => aleph_profile::register_thread!("Rayon Worker 399"),
        400 => aleph_profile::register_thread!("Rayon Worker 400"),
        401 => aleph_profile::register_thread!("Rayon Worker 401"),
        402 => aleph_profile::register_thread!("Rayon Worker 402"),
        403 => aleph_profile::register_thread!("Rayon Worker 403"),
        404 => aleph_profile::register_thread!("Rayon Worker 404"),
        405 => aleph_profile::register_thread!("Rayon Worker 405"),
        406 => aleph_profile::register_thread!("Rayon Worker 406"),
        407 => aleph_profile::register_thread!("Rayon Worker 407"),
        408 => aleph_profile::register_thread!("Rayon Worker 408"),
        409 => aleph_profile::register_thread!("Rayon Worker 409"),
        410 => aleph_profile::register_thread!("Rayon Worker 410"),
        411 => aleph_profile::register_thread!("Rayon Worker 411"),
        412 => aleph_profile::register_thread!("Rayon Worker 412"),
        413 => aleph_profile::register_thread!("Rayon Worker 413"),
        414 => aleph_profile::register_thread!("Rayon Worker 414"),
        415 => aleph_profile::register_thread!("Rayon Worker 415"),
        416 => aleph_profile::register_thread!("Rayon Worker 416"),
        417 => aleph_profile::register_thread!("Rayon Worker 417"),
        418 => aleph_profile::register_thread!("Rayon Worker 418"),
        419 => aleph_profile::register_thread!("Rayon Worker 419"),
        420 => aleph_profile::register_thread!("Rayon Worker 420"),
        421 => aleph_profile::register_thread!("Rayon Worker 421"),
        422 => aleph_profile::register_thread!("Rayon Worker 422"),
        423 => aleph_profile::register_thread!("Rayon Worker 423"),
        424 => aleph_profile::register_thread!("Rayon Worker 424"),
        425 => aleph_profile::register_thread!("Rayon Worker 425"),
        426 => aleph_profile::register_thread!("Rayon Worker 426"),
        427 => aleph_profile::register_thread!("Rayon Worker 427"),
        428 => aleph_profile::register_thread!("Rayon Worker 428"),
        429 => aleph_profile::register_thread!("Rayon Worker 429"),
        430 => aleph_profile::register_thread!("Rayon Worker 430"),
        431 => aleph_profile::register_thread!("Rayon Worker 431"),
        432 => aleph_profile::register_thread!("Rayon Worker 432"),
        433 => aleph_profile::register_thread!("Rayon Worker 433"),
        434 => aleph_profile::register_thread!("Rayon Worker 434"),
        435 => aleph_profile::register_thread!("Rayon Worker 435"),
        436 => aleph_profile::register_thread!("Rayon Worker 436"),
        437 => aleph_profile::register_thread!("Rayon Worker 437"),
        438 => aleph_profile::register_thread!("Rayon Worker 438"),
        439 => aleph_profile::register_thread!("Rayon Worker 439"),
        440 => aleph_profile::register_thread!("Rayon Worker 440"),
        441 => aleph_profile::register_thread!("Rayon Worker 441"),
        442 => aleph_profile::register_thread!("Rayon Worker 442"),
        443 => aleph_profile::register_thread!("Rayon Worker 443"),
        444 => aleph_profile::register_thread!("Rayon Worker 444"),
        445 => aleph_profile::register_thread!("Rayon Worker 445"),
        446 => aleph_profile::register_thread!("Rayon Worker 446"),
        447 => aleph_profile::register_thread!("Rayon Worker 447"),
        448 => aleph_profile::register_thread!("Rayon Worker 448"),
        449 => aleph_profile::register_thread!("Rayon Worker 449"),
        450 => aleph_profile::register_thread!("Rayon Worker 450"),
        451 => aleph_profile::register_thread!("Rayon Worker 451"),
        452 => aleph_profile::register_thread!("Rayon Worker 452"),
        453 => aleph_profile::register_thread!("Rayon Worker 453"),
        454 => aleph_profile::register_thread!("Rayon Worker 454"),
        455 => aleph_profile::register_thread!("Rayon Worker 455"),
        456 => aleph_profile::register_thread!("Rayon Worker 456"),
        457 => aleph_profile::register_thread!("Rayon Worker 457"),
        458 => aleph_profile::register_thread!("Rayon Worker 458"),
        459 => aleph_profile::register_thread!("Rayon Worker 459"),
        460 => aleph_profile::register_thread!("Rayon Worker 460"),
        461 => aleph_profile::register_thread!("Rayon Worker 461"),
        462 => aleph_profile::register_thread!("Rayon Worker 462"),
        463 => aleph_profile::register_thread!("Rayon Worker 463"),
        464 => aleph_profile::register_thread!("Rayon Worker 464"),
        465 => aleph_profile::register_thread!("Rayon Worker 465"),
        466 => aleph_profile::register_thread!("Rayon Worker 466"),
        467 => aleph_profile::register_thread!("Rayon Worker 467"),
        468 => aleph_profile::register_thread!("Rayon Worker 468"),
        469 => aleph_profile::register_thread!("Rayon Worker 469"),
        470 => aleph_profile::register_thread!("Rayon Worker 470"),
        471 => aleph_profile::register_thread!("Rayon Worker 471"),
        472 => aleph_profile::register_thread!("Rayon Worker 472"),
        473 => aleph_profile::register_thread!("Rayon Worker 473"),
        474 => aleph_profile::register_thread!("Rayon Worker 474"),
        475 => aleph_profile::register_thread!("Rayon Worker 475"),
        476 => aleph_profile::register_thread!("Rayon Worker 476"),
        477 => aleph_profile::register_thread!("Rayon Worker 477"),
        478 => aleph_profile::register_thread!("Rayon Worker 478"),
        479 => aleph_profile::register_thread!("Rayon Worker 479"),
        480 => aleph_profile::register_thread!("Rayon Worker 480"),
        481 => aleph_profile::register_thread!("Rayon Worker 481"),
        482 => aleph_profile::register_thread!("Rayon Worker 482"),
        483 => aleph_profile::register_thread!("Rayon Worker 483"),
        484 => aleph_profile::register_thread!("Rayon Worker 484"),
        485 => aleph_profile::register_thread!("Rayon Worker 485"),
        486 => aleph_profile::register_thread!("Rayon Worker 486"),
        487 => aleph_profile::register_thread!("Rayon Worker 487"),
        488 => aleph_profile::register_thread!("Rayon Worker 488"),
        489 => aleph_profile::register_thread!("Rayon Worker 489"),
        490 => aleph_profile::register_thread!("Rayon Worker 490"),
        491 => aleph_profile::register_thread!("Rayon Worker 491"),
        492 => aleph_profile::register_thread!("Rayon Worker 492"),
        493 => aleph_profile::register_thread!("Rayon Worker 493"),
        494 => aleph_profile::register_thread!("Rayon Worker 494"),
        495 => aleph_profile::register_thread!("Rayon Worker 495"),
        496 => aleph_profile::register_thread!("Rayon Worker 496"),
        497 => aleph_profile::register_thread!("Rayon Worker 497"),
        498 => aleph_profile::register_thread!("Rayon Worker 498"),
        499 => aleph_profile::register_thread!("Rayon Worker 499"),
        _ => aleph_profile::register_thread!("Rayon Worker n"),
    }
}
