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

use aleph_config::{ConfigRunner, RunConfigError};
pub use interfaces::any;
use interfaces::ecs::World;
use interfaces::schedule::WorldResource;
use interfaces::scheduler::{Resources, Schedule, Stage};

mod builder;
mod quit_handle;
mod registrar;

use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

pub use builder::PluginRegistryBuilder;

use crate::interfaces::any::{AnyArc, IAny};
use crate::interfaces::plugin::{IPlugin, IQuitHandle, IRegistryAccessor};
use crate::plugin_registry::quit_handle::QuitHandleImpl;

///
pub struct PluginRegistry {
    /// This stores plugins that can only be invoked directly from the main thread
    plugins: Vec<PluginEntry>,

    quit_handle: AnyArc<QuitHandleImpl>,

    /// Sharable storage for the set of all interfaces that have been provided by the registered
    /// plugins
    interfaces: Option<BTreeMap<TypeId, AnyArc<dyn IAny>>>,

    /// The baked init execution sequence
    init_order: Vec<usize>,

    /// The baked exit execution sequence
    exit_order: Vec<usize>,

    /// The scheduler for our update loop run order
    schedule: Option<Box<Schedule>>,

    /// The resource store, accessed by the scheduler
    resources: Option<Box<Resources>>,

    /// The core ECS world that constitures the 'game world'
    world: Option<Box<World>>,
}

impl PluginRegistry {
    pub fn builder() -> PluginRegistryBuilder {
        PluginRegistryBuilder::new()
    }

    /// Internal function that drives the initialization of all the plugins
    pub(crate) fn init_plugins(&mut self, mut provided_interfaces: Vec<BTreeSet<TypeId>>) {
        let mut plugins = std::mem::take(&mut self.plugins);

        Self::load_configs(&mut plugins);

        let quit_handle = AnyArc::map::<dyn IQuitHandle, _>(self.quit_handle.clone(), |v| v);
        let mut accessor = RegistryAccessor {
            quit_handle,
            config: None,
            interfaces: self.interfaces.take().unwrap(),
            schedule: self.schedule.take().unwrap(),
            resources: self.resources.take().unwrap(),
            world: self.world.take().unwrap(),
        };

        self.init_order.iter().cloned().for_each(|index| {
            // Take the set out of the list
            let mut provided = std::mem::take(&mut provided_interfaces[index]);
            let plugin = &mut plugins[index];

            // Log that we're initializing the plugin
            let description = plugin.v.get_description();
            log::info!(
                "Initializing Plugin [{} - {}.{}.{}]",
                description.name,
                description.major_version,
                description.minor_version,
                description.patch_version
            );

            // Take the config value from the slot in the plugin entry so the plugin can query it
            // in its functions
            accessor.config = std::mem::take(&mut plugin.config);

            let mut response = plugin.v.on_init(&mut accessor);
            response.interfaces().for_each(|(id, object)| {
                if !provided.remove(&id) {
                    let description = plugin.v.get_description();
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
                    let description = plugin.v.get_description();
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
                let description = plugin.v.get_description();
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

            // Put the config back in its entry
            plugin.config = std::mem::take(&mut accessor.config);
        });

        self.plugins = plugins;
        self.interfaces = Some(accessor.interfaces);
        self.schedule = Some(accessor.schedule);
        self.resources = Some(accessor.resources);
        self.world = Some(accessor.world);
    }

    fn load_configs(plugins: &mut [PluginEntry]) {
        log::info!("Initializing config JS runtime");
        let mut configs = ConfigRunner::new().expect("Failed to init config runner");

        for plugin in plugins.iter() {
            let desc = plugin.v.get_description();
            log::info!(
                "Running config script for plugin [{} - {}.{}.{}]",
                &desc.name,
                desc.major_version,
                desc.minor_version,
                desc.patch_version
            );

            match configs.run_config_by_name(&desc.name) {
                Ok(_) => {}
                Err(v) => {
                    if !matches!(v, RunConfigError::NoConfig) {
                        log::error!("Failed while running config script. Reason: {:?}", v);
                        // If the error is for anything other than a missing config then we panic
                        panic!("Failed while running config script. Reason: {:?}", v);
                    } else {
                        log::warn!("No config found for plugin");
                    }
                }
            }
        }

        match configs.run_override_script() {
            Ok(_) => {}
            Err(v) => {
                log::error!("Failed while running @override script. Reason: {:?}", v);
                if !matches!(v, RunConfigError::NoConfig) {
                    // If the error is for anything other than a missing config then we panic
                    panic!("Failed while running @override script. Reason: {:?}", v);
                }
            }
        }

        let mut configs = configs.finalize();

        for plugin in plugins.iter_mut() {
            let desc = plugin.v.get_description();
            plugin.config = configs.remove(&desc.name);
        }
    }

    ///
    /// This function drives the main loop of the engine.
    ///
    /// This function will continuously loop, calling `on_update` for each plugin once per iteration
    /// of its internal loop, until any one of the plugins requests the loop to terminate.
    pub fn run(&mut self) {
        let mut schedule = self.schedule.take().unwrap();
        let mut resources = self.resources.take().unwrap();
        let world = self.world.take().unwrap();

        resources.insert::<WorldResource>(WorldResource(*world));

        while !self.quit_handle.quit_requested() {
            aleph_profile::scope!("aleph::OnUpdate");

            schedule.run(&(), &mut resources);

            aleph_profile::finish_frame!();
        }

        let world = resources.take::<WorldResource>().unwrap();
        let world = Box::new(world.0); // Re-box for compatibility. Perf isn't too important here

        // self.plugins = plugins;
        //self.interfaces = accessor.interfaces;
        self.schedule = Some(schedule);
        self.resources = Some(resources);
        self.world = Some(world);
    }
}

impl Drop for PluginRegistry {
    fn drop(&mut self) {
        let mut plugins = std::mem::take(&mut self.plugins);

        self.exit_order.iter().cloned().for_each(|v| {
            // If we panic in the plugin setup phase we can end up calling 'drop' on PluginRegistry
            // when not all of the plugins are actually stored in the plugins array yet.
            //
            // If we don't manually handle the OOB case then we'll trigger a double panic, which
            // prevents properly unwinding and adds noise to the output.
            if let Some(plugin) = plugins.get_mut(v) {
                // Take the config value from the slot in the plugin entry and immediately destroy
                // it.
                drop(plugin.config.take());

                // Log that we're exiting the plugin
                let description = plugin.v.get_description();
                log::info!(
                    "Un-initializing Plugin [{} - {}.{}.{}]",
                    description.name,
                    description.major_version,
                    description.minor_version,
                    description.patch_version
                );

                plugin.v.on_exit();
            }
        });

        // Manually destroy these so we can log when they happen
        log::debug!("Destroying PluginRegistry interfaces table");
        drop(self.interfaces.take());

        log::debug!("Destroying ECS world");
        drop(self.world.take());

        log::debug!("Destroying PluginRegistry scheduler");
        drop(self.schedule.take());

        log::debug!("Destroying PluginRegistry resource table");
        drop(self.resources.take());

        self.exit_order.iter().cloned().for_each(|v| {
            if let Some(plugin) = plugins.get_mut(v) {
                // Log that we're exiting the plugin
                let description = plugin.v.get_description();
                log::info!(
                    "Shutdown for Plugin [{} - {}.{}.{}]",
                    description.name,
                    description.major_version,
                    description.minor_version,
                    description.patch_version
                );

                plugin.v.on_shutdown();
            }
        });

        // Destroy the plugins
        drop(plugins);
    }
}

struct RegistryAccessor {
    quit_handle: AnyArc<dyn IQuitHandle>,
    config: Option<serde_json::Value>,
    interfaces: BTreeMap<TypeId, AnyArc<dyn IAny>>,
    schedule: Box<Schedule>,
    resources: Box<Resources>,
    world: Box<World>,
}

impl<'a> IRegistryAccessor<'a> for RegistryAccessor {
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn IAny>> {
        self.interfaces.get(&interface).cloned()
    }

    fn quit_handle(&self) -> AnyArc<dyn IQuitHandle> {
        self.quit_handle.clone()
    }

    fn config(&self) -> Option<&serde_json::Value> {
        self.config.as_ref()
    }

    fn resources(&mut self) -> &mut Resources {
        &mut self.resources
    }

    fn schedule(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    fn world(&mut self) -> &mut World {
        &mut self.world
    }
}

pub(crate) struct PluginEntry {
    pub(crate) v: Box<dyn IPlugin>,
    pub(crate) config: Option<serde_json::Value>,
}
