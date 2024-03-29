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

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet};

use crate::interfaces::plugin::IPlugin;
use crate::plugin_registry::quit_handle::QuitHandleImpl;
use crate::plugin_registry::registrar::PluginRegistrar;
use crate::plugin_registry::PluginRegistry;

///
///
///
pub struct PluginRegistryBuilder {
    plugins: Vec<Box<dyn IPlugin>>,
}

impl PluginRegistryBuilder {
    /// Construct a new builder
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Add a new plugin
    pub fn plugin(&mut self, plugin: impl IPlugin) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Construct the final plugin registry, baking in execution orders
    pub fn build(mut self) -> PluginRegistry {
        // First we handle registering the plugins. Here we call `IPlugin::register` for each
        // plugin and collect their responses so we can schedule their execution phases.
        let SchedulerState {
            dependencies,
            mut init_dependencies,
            mut update_dependencies,
            mut provided_interfaces,
            mut should_update,
        } = self.handle_plugin_registration();

        // Then we need a resolution phase
        self.resolve_dependencies(
            &dependencies,
            &mut init_dependencies,
            &mut update_dependencies,
            &provided_interfaces,
        );

        let (init_order, update_order, exit_order) = self.schedule_plugin_execution(
            &mut init_dependencies,
            &mut update_dependencies,
            &mut provided_interfaces,
            &mut should_update,
        );

        // Package up the final registry with the computed execution orders
        let mut registry = PluginRegistry {
            plugins: self.plugins,
            quit_handle: QuitHandleImpl::new(),
            interfaces: BTreeMap::new(),
            init_order,
            update_order,
            exit_order,
        };

        // Initialize the plugins
        registry.init_plugins(provided_interfaces);

        // Return the registry
        registry
    }

    fn handle_plugin_registration(&mut self) -> SchedulerState {
        // Construct our registrar with empty sets
        let mut registrar = PluginRegistrar {
            depends_on_list: Default::default(),
            provided_interfaces: Default::default(),
            init_after_list: BTreeSet::default(),
            update_stage_dependencies: BTreeSet::default(),
            should_update: false,
        };

        // SoA storage for the plugin's execution dependencies for each execution stage
        let mut dependencies: Vec<BTreeSet<TypeId>> = Vec::new();
        let mut init_dependencies: Vec<BTreeSet<TypeId>> = Vec::new();
        let mut update_dependencies: Vec<BTreeSet<TypeId>> = Vec::new();
        let mut provided_interfaces: Vec<BTreeSet<TypeId>> = Vec::new();
        let mut should_update: Vec<bool> = Vec::new();

        // Iterate over each plugin and record its registration info
        self.plugins.iter_mut().for_each(|plugin| {
            // Collect the plugin's registration info
            plugin.register(&mut registrar);

            // Take the dependency lists from the registrar and add them to the lists we're
            // building
            dependencies.push(std::mem::take(&mut registrar.depends_on_list));
            init_dependencies.push(std::mem::take(&mut registrar.init_after_list));
            update_dependencies.push(std::mem::take(&mut registrar.update_stage_dependencies));
            provided_interfaces.push(std::mem::take(&mut registrar.provided_interfaces));
            should_update.push(registrar.should_update);
        });
        SchedulerState {
            dependencies,
            init_dependencies,
            update_dependencies,
            provided_interfaces,
            should_update,
        }
    }

    fn resolve_dependencies(
        &self,
        dependencies: &[BTreeSet<TypeId>],
        init_dependencies: &mut [BTreeSet<TypeId>],
        update_dependencies: &mut [BTreeSet<TypeId>],
        provided_interfaces: &[BTreeSet<TypeId>],
    ) {
        // Collect a flattened set of all interfaces that are mandatory for the full set of plugins
        // to work
        let mandatory_interfaces: BTreeSet<TypeId> = dependencies
            .iter()
            .flat_map(|v| v.iter().cloned())
            .collect();

        // Collect a flattened set of all interfaces that have been provided by the set of plugins
        let all_interfaces: BTreeSet<TypeId> = provided_interfaces
            .iter()
            .flat_map(|v| v.iter().cloned())
            .chain(self.plugins.iter().map(|v| v.type_id()))
            .collect();

        // If all interfaces contain the entirety of mandatory interfaces then all plugins provided
        // satisfy everyone's requirements.
        if !all_interfaces.is_superset(&mandatory_interfaces) {
            log::error!("Plugins have been registered with unsatisfied dependencies");
            log::error!("Error was caused by the following plugins: ");

            // Get the set of mandatory interfaces that have not been provided
            let unprovided_interfaces: BTreeSet<TypeId> = mandatory_interfaces
                .difference(&all_interfaces)
                .copied()
                .collect();

            // List of the name of all plugins with unsatisfied dependencies
            self.plugins
                .iter()
                .enumerate()
                .filter(|(i, _v)| !dependencies[*i].is_disjoint(&unprovided_interfaces))
                .for_each(|(_i, v)| {
                    let description = v.get_description();
                    log::error!(
                        "  {} - {}.{}.{}",
                        description.name,
                        description.major_version,
                        description.minor_version,
                        description.patch_version
                    );
                });

            panic!("Plugins have been registered with unsatisfied dependencies")
        }

        // Remove non existent, but non mandatory interfaces from the execution dependencies so we
        // can handle optional dependencies
        let resolver_fn = |v: &mut BTreeSet<TypeId>| {
            let set = std::mem::take(v);
            let set: BTreeSet<TypeId> = set
                .into_iter()
                .filter(|v| all_interfaces.contains(v))
                .collect();
            *v = set;
        };
        init_dependencies.iter_mut().for_each(&resolver_fn);
        update_dependencies.iter_mut().for_each(&resolver_fn);
    }

    fn schedule_plugin_execution(
        &self,
        init_dependencies: &mut [BTreeSet<TypeId>],
        update_dependencies: &mut [BTreeSet<TypeId>],
        provided_interfaces: &mut [BTreeSet<TypeId>],
        should_update: &mut [bool],
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        // Build a hash set populated with the number 0..n where n is the number of plugins.
        let unscheduled: BTreeSet<usize> = (0..self.plugins.len()).collect();

        // Build the init execution order
        //
        // Clone the unscheduled set as we need it multiple times and the mem copy is faster than
        // constructing the set multiple times.
        let init_order =
            self.build_execution_order(unscheduled, init_dependencies, provided_interfaces);

        let update_orders =
            self.build_update_exec_orders(update_dependencies, provided_interfaces, should_update);

        // Build the exit execution order
        //
        // The exit order is defined as the reverse of init order, so just reverse the init order.
        let exit_order = {
            let mut order = init_order.clone();
            order.reverse();
            order
        };
        (init_order, update_orders, exit_order)
    }

    fn build_update_exec_orders(
        &self,
        execution_dependencies: &[BTreeSet<TypeId>],
        provided_implementations: &[BTreeSet<TypeId>],
        should_update: &[bool],
    ) -> Vec<usize> {
        let unscheduled: BTreeSet<usize> = self
            .plugins
            .iter()
            .enumerate()
            .filter(|(index, _)| should_update[*index])
            .map(|v| v.0)
            .collect();
        self.build_execution_order(
            unscheduled,
            execution_dependencies,
            provided_implementations,
        )
    }

    fn build_execution_order(
        &self,
        mut unscheduled: BTreeSet<usize>,
        execution_dependencies: &[BTreeSet<TypeId>],
        provided_implementations: &[BTreeSet<TypeId>],
    ) -> Vec<usize> {
        // Output list we build the order into
        let mut order = Vec::new();

        // Set to keep track of what has been executed
        let mut executed = BTreeSet::new();

        // Set to keep track of what was executed over the course of a single scheduler iteration
        let mut newly_executed = BTreeSet::new();

        while !unscheduled.is_empty() {
            // Store how many plugins have been scheduled before attempting the next scheduling round
            let already_scheduled_count = order.len();

            let map = std::mem::take(&mut unscheduled);
            let map: BTreeSet<usize> = map
                .into_iter()
                .filter(|v| {
                    // If all of the the plugin's dependencies are in the executed set then we can execute
                    // this plugin.
                    //
                    // If we can execute the plugin we add its index to the order and add it to the executed
                    // set. We can then mark it to be removed from the unscheduled set.
                    let dependencies_satisfied = executed.is_superset(&execution_dependencies[*v]);
                    if dependencies_satisfied {
                        // Insert the plugin's concrete type id into the executed set
                        newly_executed.insert(self.plugins[*v].type_id());

                        // Insert the type id for all interfaces that the plugin implements
                        newly_executed.extend(provided_implementations[*v].iter());

                        // Schedule the plugin
                        order.push(*v);
                        false
                    } else {
                        true
                    }
                })
                .collect();
            unscheduled = map;

            // Merge the newly_executed set into executed ready to be used next iteration
            executed.extend(newly_executed.iter());

            // Clear the newly executed set to ready for next iteration
            newly_executed.clear();

            // If the `already_scheduled_count` does not change over the course of an iteration it means
            // there are execution dependencies that can not be satisfied. This is an error condition
            // and so we panic if we detect when this has happened.
            assert_ne!(already_scheduled_count, order.len());
        }

        order
    }
}

struct SchedulerState {
    dependencies: Vec<BTreeSet<TypeId>>,
    init_dependencies: Vec<BTreeSet<TypeId>>,
    update_dependencies: Vec<BTreeSet<TypeId>>,
    provided_interfaces: Vec<BTreeSet<TypeId>>,
    should_update: Vec<bool>,
}

impl Default for PluginRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
