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
pub use api::any;
use api::ecs::world::World;
use api::plugin::CoreRefs;
use api::schedule::WorldResource;
use api::scheduler::{Schedule, Stage, TypedTable};

mod builder;
mod quit_handle;
mod registrar;

use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

pub use builder::PluginRegistryBuilder;

use crate::api::any::{AnyArc, IAny};
use crate::api::plugin::{IPlugin, IQuitHandle, IRegistryAccessor};
use crate::platform::PlatformSDL3;
use crate::plugin_registry::quit_handle::QuitHandleImpl;
use crate::rhi::Rhi;

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
    resources: Option<Box<TypedTable>>,

    /// The core ECS world that constitures the 'game world'
    world: Option<Box<World>>,

    /// The SDL integration object for managing the connection to the SDL library
    platform: PlatformSDL3,

    /// The RHI integration object
    rhi: Rhi,
}

impl PluginRegistry {
    pub fn builder() -> PluginRegistryBuilder {
        PluginRegistryBuilder::new()
    }

    /// Internal function that drives the initialization of all the plugins
    pub(crate) fn init_plugins(
        &mut self,
        mut provided_interfaces: Vec<BTreeSet<TypeId>>,
        optional_provides: Vec<BTreeSet<TypeId>>,
    ) {
        let mut plugins = std::mem::take(&mut self.plugins);

        let configs = Self::load_configs();

        let quit_handle = AnyArc::map::<dyn IQuitHandle, _>(self.quit_handle.clone(), |v| v);
        let mut accessor = RegistryAccessor {
            quit_handle,
            configs,
            interfaces: self.interfaces.take().unwrap(),
            provides: Default::default(),
            schedule: self.schedule.take().unwrap(),
            resources: self.resources.take().unwrap(),
            world: self.world.take().unwrap(),
        };

        // Init the SDL2 integration
        self.platform.on_init(&mut accessor);

        // Init the RHI integration
        self.rhi.on_init(&mut accessor);

        self.init_order.iter().cloned().for_each(|index| {
            // Take the set out of the list
            let mut provided = std::mem::take(&mut provided_interfaces[index]);
            let optional = &optional_provides[index];

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

            plugin.v.on_init(&mut accessor);
            while let Some((id, object)) = accessor.provides.pop_first() {
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
            }

            // If provided contains interfaces not in the optional set then it means the plugin
            // failed to provide interfaces it promised to always provide. This is an error!
            if !provided.is_subset(optional) {
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
        });

        self.plugins = plugins;
        self.interfaces = Some(accessor.interfaces);
        self.schedule = Some(accessor.schedule);
        self.resources = Some(accessor.resources);
        self.world = Some(accessor.world);
    }

    fn load_configs() -> serde_json::Map<String, serde_json::Value> {
        log::info!("Initializing config JS runtime");
        let mut configs = ConfigRunner::new().expect("Failed to init config runner");

        match configs.run_all_configs() {
            Ok(_) => {}
            Err(v) => {
                if !matches!(v, RunConfigError::NoConfig) {
                    log::error!("Failed while running config script. Reason: {:?}", v);
                    // If the error is for anything other than a missing config then we panic
                    panic!("Failed while running config script. Reason: {:?}", v);
                }
            }
        }

        configs.finalize()
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
            aleph_objc::autoreleasepool(|| {
                aleph_profile::scope_named!("OnUpdate");

                schedule.run(&(), &mut resources);

                aleph_profile::finish_frame!();
            });
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

        self.platform.on_shutdown();

        // Destroy the plugins
        drop(plugins);
    }
}

pub(crate) struct RegistryAccessor {
    pub(crate) quit_handle: AnyArc<dyn IQuitHandle>,
    pub(crate) configs: serde_json::Map<String, serde_json::Value>,
    pub(crate) interfaces: BTreeMap<TypeId, AnyArc<dyn IAny>>,
    pub(crate) provides: BTreeMap<TypeId, AnyArc<dyn IAny>>,
    pub(crate) schedule: Box<Schedule>,
    pub(crate) resources: Box<TypedTable>,
    pub(crate) world: Box<World>,
}

impl IRegistryAccessor for RegistryAccessor {
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn IAny>> {
        self.interfaces.get(&interface).cloned()
    }

    fn __provide(&mut self, interface: TypeId, object: AnyArc<dyn IAny>) {
        assert!(
            object.__query_interface(interface).is_some(),
            "Attempting to provide an object that does not implement the specified interface!"
        );
        self.provides.insert(interface, object);
    }

    fn quit_handle(&self) -> AnyArc<dyn IQuitHandle> {
        self.quit_handle.clone()
    }

    fn config(&self, name: &str) -> Option<&serde_json::Value> {
        self.configs.get(name)
    }

    fn core(&mut self) -> CoreRefs<'_> {
        CoreRefs {
            resources: self.resources.as_mut(),
            schedule: self.schedule.as_mut(),
            world: self.world.as_mut(),
        }
    }
}

pub(crate) struct PluginEntry {
    pub(crate) v: Box<dyn IPlugin>,
    pub(crate) config: Option<serde_json::Value>,
}
