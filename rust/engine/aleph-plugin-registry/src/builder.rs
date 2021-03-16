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

use crate::interfaces::plugin::stages::UpdateStage;
use crate::interfaces::plugin::IPlugin;
use crate::registrar::PluginRegistrar;
use crate::PluginRegistry;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

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
        // Construct our registrar with empty sets
        let mut registrar = PluginRegistrar {
            depends_on_list: Default::default(),
            provided_interfaces: Default::default(),
            init_after_list: Default::default(),
            update_stage_dependencies: vec![HashSet::default(); UpdateStage::STAGE_COUNT],
            update_stages: HashSet::new(),
        };

        // SoA storage for the plugin's execution dependencies for each execution stage
        let mut dependencies: Vec<HashSet<TypeId>> = Vec::new();
        let mut init_dependencies: Vec<HashSet<TypeId>> = Vec::new();
        let mut update_dependencies: Vec<Vec<HashSet<TypeId>>> = Vec::new();
        let mut provided_interfaces: Vec<HashSet<TypeId>> = Vec::new();
        let mut update_stages: Vec<HashSet<usize>> = Vec::new();

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
            update_stages.push(std::mem::take(&mut registrar.update_stages));
        });

        // Build a hash set populated with the number 0..n where n is the number of plugins.
        let unscheduled: HashSet<usize> = (0..self.plugins.len()).collect();

        // Build the init execution order
        //
        // Clone the unscheduled set as we need it multiple times and the mem copy is faster than
        // constructing the set multiple times.
        let init_order = build_execution_order(
            unscheduled.clone(),
            &self.plugins,
            &init_dependencies,
            &provided_interfaces,
        );

        let update_orders = build_update_exec_orders(
            &self.plugins,
            &update_dependencies,
            &provided_interfaces,
            &update_stages,
        );

        // Build the exit execution order
        //
        // The exit order is defined as the reverse of init order, so just reverse the init order.
        let exit_order = {
            let mut order = init_order.clone();
            order.reverse();
            order
        };

        // Package up the final registry with the computed execution orders
        let mut registry = PluginRegistry {
            plugins: self.plugins,
            interfaces: HashMap::new(),
            init_order,
            update_orders,
            exit_order,
        };

        // Initialize the plugins
        registry.init_plugins(provided_interfaces);

        // Return the registry
        registry
    }
}

fn build_update_exec_orders(
    plugins: &[Box<dyn IPlugin>],
    dependencies: &[Vec<HashSet<TypeId>>],
    provided_implementations: &[HashSet<TypeId>],
    update_stages: &[HashSet<usize>],
) -> Vec<Vec<usize>> {
    let mut output = Vec::new();

    (0..UpdateStage::STAGE_COUNT).into_iter().for_each(|stage| {
        let unscheduled: HashSet<usize> = plugins
            .iter()
            .enumerate()
            .filter(|(index, _)| update_stages[*index].contains(&stage))
            .map(|v| v.0)
            .collect();
        let order = build_execution_order(
            unscheduled,
            plugins,
            &dependencies[stage],
            provided_implementations,
        );
        output.push(order);
    });

    output
}

fn build_execution_order(
    mut unscheduled: HashSet<usize>,
    plugins: &[Box<dyn IPlugin>],
    dependencies: &[HashSet<TypeId>],
    provided_implementations: &[HashSet<TypeId>],
) -> Vec<usize> {
    // Output list we build the order into
    let mut order = Vec::new();

    // Set to keep track of what has been executed
    let mut executed = HashSet::new();

    // Set to keep track of what was executed over the course of a single scheduler iteration
    let mut newly_executed = HashSet::new();

    while !unscheduled.is_empty() {
        // Store how many plugins have been scheduled before attempting the next scheduling round
        let already_scheduled_count = order.len();

        unscheduled.retain(|v| {
            // If all of the the plugin's dependencies are in the executed set then we can execute
            // this plugin.
            //
            // If we can execute the plugin we add its index to the order and add it to the executed
            // set. We can then mark it to be removed from the unscheduled set.
            let dependencies_satisfied = executed.is_superset(&dependencies[*v]);
            if dependencies_satisfied {
                // Insert the plugin's concrete type id into the executed set
                newly_executed.insert(plugins[*v].type_id());

                // Insert the type id for all interfaces that the plugin implements
                newly_executed.extend(provided_implementations[*v].iter());

                // Schedule the plugin
                order.push(*v);
                false
            } else {
                true
            }
        });

        // Merge the newly_executed set into executed ready to be used next iteration
        executed.extend(newly_executed.iter());

        // Clear the newly executed set to ready for next iteration
        newly_executed.clear();

        // If the `already_scheduled_count` does not change over the course of an iteration it means
        // there are execution dependencies that can not be satisfied. This is an error condition
        // and so we panic if we detect when this has happened.
        if already_scheduled_count == order.len() {
            log::error!("A plugin has been registered with unsatisfiable execution dependencies");
            log::error!("Error was caused by one of the following plugins: ");
            for plugin in unscheduled {
                let description = plugins[plugin].get_description();
                log::error!(
                    "  {} - {}.{}.{}",
                    description.name,
                    description.major_version,
                    description.minor_version,
                    description.patch_version
                );
            }
            panic!("A plugin has been registered with unsatisfiable execution dependencies");
        }
    }

    order
}
