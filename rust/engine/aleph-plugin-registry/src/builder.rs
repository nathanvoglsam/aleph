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

use crate::interfaces::plugin::stages;
use crate::interfaces::plugin::stages::{InitStage, UpdateStage_};
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
            plugins: stages::default_stages(),
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
            update_after_list: Default::default(),
            init_stage: InitStage::Main,
            update_stage: UpdateStage_::Update,
        };

        // SoA storage for the plugin's execution dependencies for each execution stage
        let mut dependencies: Vec<HashSet<TypeId>> = Vec::new();
        let mut init_dependencies: Vec<HashSet<TypeId>> = Vec::new();
        let mut update_dependencies: Vec<HashSet<TypeId>> = Vec::new();
        let mut provided_interfaces: Vec<HashSet<TypeId>> = Vec::new();
        let mut init_stages: Vec<usize> = Vec::new();
        let mut update_stages: Vec<usize> = Vec::new();

        // Iterate over each plugin and record its registration info
        self.plugins.iter_mut().for_each(|plugin| {
            // Collect the plugin's registration info
            plugin.register(&mut registrar);

            // Take the dependency lists from the registrar and add them to the lists we're
            // building
            dependencies.push(std::mem::take(&mut registrar.depends_on_list));
            init_dependencies.push(std::mem::take(&mut registrar.init_after_list));
            update_dependencies.push(std::mem::take(&mut registrar.update_after_list));
            provided_interfaces.push(std::mem::take(&mut registrar.provided_interfaces));
            init_stages.push(registrar.init_stage as usize);
            update_stages.push(registrar.update_stage as usize);
        });

        // Build a hash set populated with the number 0..n where n is the number of plugins.
        let unscheduled: HashSet<usize> = (0..self.plugins.len()).collect();

        // Arrays we use to count how many plugins of each stage have been scheduled
        let mut init_stage_counts = vec![0usize; InitStage::STAGE_COUNT];
        let mut update_stage_counts = vec![0usize; UpdateStage_::STAGE_COUNT];

        // Count the number of plugins in each stage
        let iterator = init_stages
            .iter()
            .cloned()
            .zip(update_stages.iter().cloned());
        for (init_stage, update_stage) in iterator {
            init_stage_counts[init_stage] += 1;
            update_stage_counts[update_stage] += 1;
        }

        // Build the init execution order
        //
        // Clone the unscheduled set as we need it multiple times and the mem copy is faster than
        // constructing the set multiple times.
        let init_order = build_execution_order(
            unscheduled.clone(),
            &self.plugins,
            &init_dependencies,
            &provided_interfaces,
            &init_stages,
            init_stage_counts,
        );

        // Build the update execution order
        //
        // Just pass the set in as we don't need it anymore.
        let update_order = build_execution_order(
            unscheduled,
            &self.plugins,
            &update_dependencies,
            &provided_interfaces,
            &update_stages,
            update_stage_counts,
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
            update_order,
            exit_order,
        };

        // Initialize the plugins
        registry.init_plugins(provided_interfaces);

        // Return the registry
        registry
    }
}

// TODO: Explicit execution stages system for more reliable execution ordering
fn build_execution_order(
    mut unscheduled: HashSet<usize>,
    plugins: &[Box<dyn IPlugin>],
    dependencies: &[HashSet<TypeId>],
    provided_implementations: &[HashSet<TypeId>],
    stages: &[usize],
    mut stage_counts: Vec<usize>,
) -> Vec<usize> {
    // Output list we build the order into
    let mut order = Vec::new();

    // Set to keep track of what has been executed
    let mut executed = HashSet::new();

    // Set to keep track of what was executed over the course of a single scheduler iteration
    let mut newly_executed = HashSet::new();

    // Context for the current stage we're scheduling
    let mut current_stage = 0;

    while !unscheduled.is_empty() {
        // Store how many plugins have been scheduled before attempting the next scheduling round
        let already_scheduled_count = order.len();

        // If we've scheduled all plugins in the current stage, move to the next stage
        if stage_counts[current_stage] == 0 {
            current_stage += 1;

            // If we've run out of stages then we have finished scheduling plugins and should exit
            // the loop.
            //
            // Otherwise we restart our loop iteration
            if current_stage >= stage_counts.len() {
                break;
            } else {
                continue;
            }
        }

        unscheduled.retain(|v| {
            // If all of the the plugin's dependencies are in the executed set then we can execute
            // this plugin.
            //
            // If we can execute the plugin we add its index to the order and add it to the executed
            // set. We can then mark it to be removed from the unscheduled set.
            let dependencies_satisfied = executed.is_superset(&dependencies[*v]);

            // We also need to check if we're in the correct execution stage to schedule this plugin
            let correct_stage = stages[*v] == current_stage;
            if dependencies_satisfied && correct_stage {
                // Insert the plugin's concrete type id into the executed set
                newly_executed.insert(plugins[*v].type_id());

                // Insert the type id for all interfaces that the plugin implements
                newly_executed.extend(provided_implementations[*v].iter());

                // Decrease the count of unscheduled plugins in the current stage
                stage_counts[current_stage] -= 1;

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
