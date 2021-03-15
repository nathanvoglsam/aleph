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

pub mod stages;

use crate::any::AnyArc;
use crate::plugin::stages::{InitStage, UpdateStage};
use any::{IAny, ISendSyncAny};
use std::any::TypeId;

///
/// The interface that must be implemented by any engine plugin.
///
/// A plugin represents the primary interface for injecting code into the engine's lifecycle. Most
/// functionality *should* be implemented as a plugin that interacts with other plugins.
///
/// An `IPlugin` has the following lifecycle:
///
/// - A concrete `IPlugin` implementation is constructed by someone. This must be done before the
///   plugin registry itself is created as the full set of plugins to register needs to be finalized
///   before a plugin registry can be constructed.
///
/// - The plugin registry will, at some point during initialization, call `IPlugin::register`
///   exactly once so a plugin can declare its execution dependencies and to declare which abstract
///   interfaces the plugin provides.
///
/// - The plugin registry will then use the dependencies declared from each plugin to compute a
///   final execution order for each execution stage.
///
/// - `IPlugin::on_init` will be called exactly once, respecting the dependencies declared in the
///   registration phase. The `on_init` function will return the list of provided interfaces paired
///   with the object that actually implements the interface.
///
/// - Directly after `IPlugin::on_init` is called, and before another plugin's `on_init` function is
///   called, `IPlugin::get_interfaces` will be called. The `get_interfaces` function provides
///
///   This introduces some repetition but allows for a plugin to have completed initialization
///   before handing out its implementations.
///
/// - The engine now moves into the main loop. `IPlugin::on_update` will be called exactly once
///   *per iteration* of the main loop. Once again, any declared execution dependencies will be
///   respected.
///
/// - The engine will eventually exit the main loop. `IPlugin::on_exit` will be called exactly once
///   so that the plugin can destroy any resources that may require ordering against other plugins.
///
///   The execution order for `IPlugin::on_exit` is defined as the inverse of the initialization
///   execution order.
///
/// - Eventually, at some unspecified time, the plugins will be dropped when the plugin registry is
///   destroyed.
///
pub trait IPlugin: IAny {
    /// This function can be called at any time to retrieve a description of the plugin. This will
    /// be used for logging and debug info
    fn get_description(&self) -> PluginDescription;

    /// Called by the plugin registry exactly once so that a plugin can register its execution
    /// dependencies
    fn register(&mut self, registrar: &mut dyn IPluginRegistrar);

    /// Called by the engine runtime exactly once during the init phase so a plugin can initialize
    /// itself in regards to other plugins
    #[allow(unused_variables)]
    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        Box::new(Vec::new())
    }

    ///
    #[allow(unused_variables)]
    fn on_input_collection(&mut self, registry: &dyn IRegistryAccessor) {}

    ///
    #[allow(unused_variables)]
    fn on_pre_update(&mut self, registry: &dyn IRegistryAccessor) {}

    /// Called by the engine runtime exactly once *per iteration* of the main loop
    #[allow(unused_variables)]
    fn on_update(&mut self, registry: &dyn IRegistryAccessor) {}

    ///
    #[allow(unused_variables)]
    fn on_post_update(&mut self, registry: &dyn IRegistryAccessor) {}

    ///
    #[allow(unused_variables)]
    fn on_render(&mut self, registry: &dyn IRegistryAccessor) {}

    /// Called by the engine runtime exactly once during the shutdown phase of the engine
    #[allow(unused_variables)]
    fn on_exit(&mut self, registry: &dyn IRegistryAccessor) {}
}

///
/// A plugin description
///
#[derive(Clone, Debug)]
pub struct PluginDescription {
    pub name: String,
    pub description: String,
    pub major_version: u32,
    pub minor_version: u32,
    pub patch_version: u32,
}

///
/// A generic wrapper over the response expected from a plugin for the `on_init` function.
///
/// Rather than using a concrete type for the response we use an interface to allow for updating
/// the response format in the future without changing the plugin interface.
///
pub trait IInitResponse {
    /// Take the interfaces iterator from the init response.
    ///
    /// This function must yield a non `None` value *at least* once. It may continue to return a
    /// non `None` value after the first call, but such behavior is not required and *should not*
    /// be relied on.
    fn interfaces(&mut self) -> Option<Box<dyn IInterfaceIterator>>;
}

///
/// A helper implementation that can save manually implementing `IInitResponse`
///
impl IInitResponse for Vec<(TypeId, AnyArc<dyn ISendSyncAny>)> {
    fn interfaces(&mut self) -> Option<Box<dyn IInterfaceIterator>> {
        let take = std::mem::take(self);
        if take.is_empty() {
            None
        } else {
            let iter = take.into_iter();
            Some(Box::new(iter))
        }
    }
}

///
/// A generic iterator interface that is used by the plugin initialization process to get the
/// provided interfaces from a plugin
///
pub trait IInterfaceIterator: Iterator<Item = (TypeId, AnyArc<dyn ISendSyncAny>)> {}

impl<T: Iterator<Item = (TypeId, AnyArc<dyn ISendSyncAny>)>> IInterfaceIterator for T {}

///
/// An abstract interface over any potential concrete implementation of an accessor into the plugin
/// registry. This can be used to retrieve interface implementations, request the main loop exit,
/// etc.
///
pub trait IRegistryAccessor: 'static {
    /// Object safe implementation of `get_interface`. See wrapper for more info.
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn ISendSyncAny>>;

    /// Used by a plugin to tell the registry it should exit the main loop and quit.
    ///
    /// This function has no effect during the `on_init` or `on_exit` functions.
    fn request_quit(&self);
}

impl dyn IRegistryAccessor {
    /// Get a reference counted handle to the interface with the type given by the `T` type
    /// parameter.
    pub fn get_interface<T: ISendSyncAny + ?Sized>(&self) -> Option<AnyArc<T>> {
        self.__get_interface(TypeId::of::<T>())
            .map(|v| v.query_interface::<T>().unwrap())
    }
}

///
/// The interface used by plugins to manipulate their initialization and execution order.
///
/// The methods declared directly on this trait are not meant to be used directly. There are wrapper
/// functions declared that make them easier to use. This level of indirection is required to make
/// this trait object safe.
///
/// The methods on this trait are wrapped with generic functions that ask for generic type
/// parameters instead of the raw `TypeId`. Example wrapper:
///
/// ```ignore
/// use std::any::TypeId;
/// use aleph_interfaces::any::IAny;
///
/// pub trait IPluginRegistrar {
///    /// Object safe implementation
///    fn __depends_on(&mut self, dependency: TypeId);
/// }
/// impl dyn IPluginRegistrar {
///     /// Generic wrapper
///     pub fn depends_on<T: IAny>(&mut self) {
///         self.__depends_on(TypeId::of::<T>())
///     }
/// }
/// ```
///
/// The `TypeId`/type parameter can either be a concrete type, such as a specific plugin
/// implementation, or an abstract interface like `IWindowProvider`. This way it is possible for a
/// plugin to depend on both specific plugins (i.e `WindowProviderSDL2`) or they can declare a
/// dependency that is generic over arbitrary plugins that provide an abstract interface
/// (i.e `IWindowProvider`) implementation.
///
pub trait IPluginRegistrar: 'static {
    /// Object safe implementation of `depends_on`. See wrapper for more info.
    fn __depends_on(&mut self, dependency: TypeId);

    /// Object safe implementation of `provides_interface`. See wrapper for more info.
    fn __provides_interface(&mut self, provides: TypeId);

    /// Object safe implementation of `must_init_after`. See wrapper for more info.
    fn __must_init_after(&mut self, requires: TypeId);

    /// Object safe implementation of `must_update_after`. See wrapper for more info.
    fn __must_update_after(&mut self, requires: TypeId);

    /// Register the execution stage this plugin's `on_init` should be called in.
    ///
    /// Default init stage is `InitStage::Main`
    fn init_stage(&mut self, stage: InitStage);

    /// Register the execution stage this plugin's `on_update` should be called in.
    ///
    /// Default update stage is `UpdateStage::Update`
    fn update_stage(&mut self, stage: UpdateStage);
}

impl dyn IPluginRegistrar {
    /// Declares that the plugin depends on the existence of another plugin given by the type
    /// parameter. This can be used to declare that one plugin requires another plugin, or another
    /// interface to exist without specifying any execution dependencies.
    pub fn depends_on<T: IAny + ?Sized>(&mut self) {
        self.__depends_on(TypeId::of::<T>())
    }

    /// Declares that the plugin will provide an object that implements the interface given by the
    /// `T` type parameter.
    pub fn provides_interface<T: IAny + ?Sized>(&mut self) {
        self.__provides_interface(TypeId::of::<T>())
    }

    /// Declares that the plugin's init function can only execute *after* the given plugin has had
    /// its own init function execute.
    pub fn must_init_after<T: IAny + ?Sized>(&mut self) {
        self.__must_init_after(TypeId::of::<T>())
    }

    /// Declares that the plugin's update function can only execute *after* the given plugin has had
    /// its own update function execute.
    pub fn must_update_after<T: IAny + ?Sized>(&mut self) {
        self.__must_update_after(TypeId::of::<T>())
    }
}
