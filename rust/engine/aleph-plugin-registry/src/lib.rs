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

pub extern crate aleph_interfaces as interfaces;
extern crate aleph_log as log;

pub use interfaces::any;

mod builder;
mod registrar;

pub use builder::PluginRegistryBuilder;

use crate::interfaces::any::{AnyArc, ISendSyncAny};
use crate::interfaces::plugin::{IPlugin, IRegistryAccessor};
use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};

///
pub struct PluginRegistry {
    /// This stores plugins that can only be invoked directly from the main thread
    plugins: Vec<Box<dyn IPlugin>>,

    /// Sharable storage for the set of all interfaces that have been provided by the registered
    /// plugins
    interfaces: HashMap<TypeId, AnyArc<dyn ISendSyncAny>>,

    /// The baked init execution sequence
    init_order: Vec<usize>,

    /// The baked update execution sequence
    update_order: Vec<usize>,

    /// The baked exit execution sequence
    exit_order: Vec<usize>,
}

impl PluginRegistry {
    /// Internal function that drives the initialization of all the plugins
    pub(crate) fn init_plugins(&mut self, mut provided_interfaces: Vec<HashSet<TypeId>>) {
        let mut plugins = std::mem::take(&mut self.plugins);
        let mut accessor = RegistryAccessor {
            interfaces: std::mem::take(&mut self.interfaces),
            should_quit: AtomicBool::new(false),
        };

        self.init_order.iter().cloned().for_each(|index| {
            // Take the set out of the list
            let mut provided = std::mem::take(&mut provided_interfaces[index]);
            let plugin = &mut plugins[index];

            // Log that we're initializing the plugin
            let description = plugin.get_description();
            log::info!(
                "Initializing Plugin [{} - {}.{}.{}]",
                description.name,
                description.major_version,
                description.minor_version,
                description.patch_version
            );

            let mut response = plugin.on_init(&accessor);
            response.interfaces().unwrap().for_each(|(id, object)| {
                if !provided.remove(&id) {
                    let description = plugin.get_description();
                    let message = format!(
                        "Plugin [{} - {}.{}.{}] tried to provide an interface it didn't declare",
                        description.name,
                        description.major_version,
                        description.minor_version,
                        description.patch_version
                    );
                    log::error!("{}", &message);
                    panic!("{}", &message);
                }

                if accessor.interfaces.insert(id, object).is_some() {
                    let description = plugin.get_description();
                    let message = format!(
                        "Plugin [{} - {}.{}.{}] provided an interface provided by another plugin",
                        description.name,
                        description.major_version,
                        description.minor_version,
                        description.patch_version
                    );
                    log::error!("{}", &message);
                    panic!("{}", &message);
                }
            });

            if !provided.is_empty() {
                let description = plugin.get_description();
                let message = format!(
                    "Plugin [{} - {}.{}.{}] failed to provide all the interfaces it declared",
                    description.name,
                    description.major_version,
                    description.minor_version,
                    description.patch_version
                );
                log::error!("{}", &message);
                panic!("{}", &message);
            }
        });

        self.plugins = plugins;
        self.interfaces = accessor.interfaces;
    }

    ///
    /// This function drives the main loop of the engine.
    ///
    /// This function will continuously loop, calling `on_update` for each plugin once per iteration
    /// of its internal loop, until any one of the plugins requests the loop to terminate.
    pub fn run(&mut self) {
        let mut plugins = std::mem::take(&mut self.plugins);
        let accessor = RegistryAccessor {
            interfaces: std::mem::take(&mut self.interfaces),
            should_quit: AtomicBool::new(false),
        };

        while !accessor.should_quit.load(Ordering::Relaxed) {
            self.update_order.iter().cloned().for_each(|v| {
                plugins[v].on_update(&accessor);
            });
        }

        self.plugins = plugins;
        self.interfaces = accessor.interfaces;
    }
}

impl Drop for PluginRegistry {
    fn drop(&mut self) {
        let mut plugins = std::mem::take(&mut self.plugins);
        let accessor = RegistryAccessor {
            interfaces: std::mem::take(&mut self.interfaces),
            should_quit: AtomicBool::new(false),
        };

        self.exit_order.iter().cloned().for_each(|v| {
            let plugin = &mut plugins[v];

            // Log that we're exiting the plugin
            let description = plugin.get_description();
            log::info!(
                "Un-initializing Plugin [{} - {}.{}.{}]",
                description.name,
                description.major_version,
                description.minor_version,
                description.patch_version
            );

            plugin.on_exit(&accessor);
        });

        self.plugins = plugins;
        self.interfaces = accessor.interfaces;
    }
}

struct RegistryAccessor {
    interfaces: HashMap<TypeId, AnyArc<dyn ISendSyncAny>>,
    should_quit: AtomicBool,
}

impl IRegistryAccessor for RegistryAccessor {
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn ISendSyncAny>> {
        self.interfaces.get(&interface).cloned()
    }

    fn request_quit(&self) {
        self.should_quit.store(true, Ordering::Relaxed);
    }
}
