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

use std::any::TypeId;

use any::IAny;
use ecs::World;
use scheduler::{Schedule, TypedTable};

use crate::any::AnyArc;

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
/// - The plugin registry will, at some point during initialization, call [IPlugin::register]
///   exactly once so a plugin can declare its execution dependencies, which abstract interfaces the
///   plugin provides, if it would like to be scheduled in the main loop.
///
/// - The plugin registry will then use the dependencies declared from each plugin to compute a
///   final execution order for each execution stage.
///
/// - `IPlugin::on_init` will be called exactly once, respecting the dependencies declared in the
///   registration phase. The `on_init` function will return the list of provided interfaces paired
///   with the object that actually implements the interface.
///
/// - The engine now moves into the main loop. The plugin registry has computed an execution order
///   that respects the requirements specified in [IPlugin::register]. The registry will use this
///   order to call the [IPlugin::on_update] function *exactly once* on the main thread every
///   iteration of the main loop.
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
    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        Box::<Vec<(TypeId, AnyArc<dyn IAny>)>>::default()
    }

    /// Called by the engine runtime exactly once during the shutdown phase of the engine
    #[allow(unused_variables)]
    fn on_exit(&mut self) {}

    /// Called by the engine runtime exactly once during the shutdown phase of the engine, after
    /// 'on_exit' is called and _after_ most of the plugin system machinery has been destroyed.
    ///
    /// This includes:
    ///     - No ECS world
    ///     - Scheduler has been destroyed
    ///     - Resources and Interfaces have been destroyed
    ///
    /// # Why?
    ///
    /// It might be useful to have a hook after all those items have been freed. But we do also
    /// need a hook for _before_ they're destroyed so we can finalize any pipelined work (GPU).
    #[allow(unused_variables)]
    fn on_shutdown(&mut self) {}
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
    fn interfaces(&mut self) -> Box<dyn IInterfaceIterator>;
}

/// Utility function for a default, empty response object
pub fn default_response() -> Box<dyn IInitResponse> {
    #[allow(clippy::box_default)]
    Box::new(Vec::new())
}

///
/// A helper implementation that can save manually implementing `IInitResponse`
///
impl IInitResponse for Vec<(TypeId, AnyArc<dyn IAny>)> {
    fn interfaces(&mut self) -> Box<dyn IInterfaceIterator> {
        let take = std::mem::take(self);
        let iter = take.into_iter();
        Box::new(iter)
    }
}

///
/// A generic iterator interface that is used by the plugin initialization process to get the
/// provided interfaces from a plugin
///
pub trait IInterfaceIterator: Iterator<Item = (TypeId, AnyArc<dyn IAny>)> {}

impl<T: Iterator<Item = (TypeId, AnyArc<dyn IAny>)>> IInterfaceIterator for T {}

///
/// An abstract interface over any potential concrete implementation of an accessor into the plugin
/// registry. This can be used to retrieve interface implementations, request the main loop exit,
/// etc.
///
pub trait IRegistryAccessor {
    /// Object safe implementation of `get_interface`. See wrapper for more info.
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn IAny>>;

    /// Registry quit handle which can be freely sent to other threads. The object is used to
    /// request the engine/plugin registry to exit.
    fn quit_handle(&self) -> AnyArc<dyn IQuitHandle>;

    /// Get the plugin's config, if one was found.
    fn config(&self) -> Option<&serde_json::Value>;

    /// Access to a core set of resources provided by the engine, wrapped in a [`CoreRefs`].
    fn core(&mut self) -> CoreRefs;
}

impl<'a> dyn IRegistryAccessor + 'a {
    /// Get a reference counted handle to the interface with the type given by the `T` type
    /// parameter.
    pub fn get_interface<T: IAny + ?Sized>(&self) -> Option<AnyArc<T>> {
        self.__get_interface(TypeId::of::<T>())
            .map(|v| v.query_interface::<T>().unwrap())
    }
}

pub struct CoreRefs<'a> {
    pub resources: &'a mut TypedTable,
    pub schedule: &'a mut Schedule,
    pub world: &'a mut World,
}

///
/// Interface for accessing the registry's quit handle
///
pub trait IQuitHandle: IAny + Send + Sync + 'static {
    /// Requests that the registry exit the main loop, call each plugin's `on_exit` and exit.
    ///
    /// # Info
    ///
    /// Calling `quit` will not immediately exit the main loop. If called within a main loop
    /// iteration the iteration will continue to completion and no further iterations will occur.
    ///
    /// This way there can never be a partial main loop iteration.
    fn quit(&self);

    /// Returns whether a quit has been requested
    fn quit_requested(&self) -> bool;
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
pub trait IPluginRegistrar {
    /// Object safe implementation of `requires`. See wrapper for more info.
    #[doc(hidden)]
    fn __requires(&mut self, dependency: TypeId, init: InitOrder);

    /// Object safe implementation of `provides_interface`. See wrapper for more info.
    #[doc(hidden)]
    fn __provides(&mut self, provides: TypeId);

    /// Object safe implementation of `uses`. See wrapper for more info.
    #[doc(hidden)]
    fn __uses(&mut self, requires: TypeId, init: InitOrder);
}

impl<'a> dyn IPluginRegistrar + 'a {
    /// Declares that the plugin depends on the existence of another plugin given by the type
    /// parameter. This can be used to declare that one plugin requires another plugin, or another
    /// interface to exist.
    ///
    /// The 'init' parameter controls whether an execution dependency is also implied during the
    /// init phase.
    pub fn requires<T: IAny + ?Sized>(&mut self, init: InitOrder) {
        self.__requires(TypeId::of::<T>(), init);
    }

    /// Declares that the plugin will provide an object that implements the interface given by the
    /// `T` type parameter.
    pub fn provides<T: IAny + ?Sized>(&mut self) {
        self.__provides(TypeId::of::<T>())
    }

    /// Declares a soft dependency on the given interface. This is similar to [`Self::requires`] but
    /// does not cause a failure if the interface is not made available by another plugin (or the
    /// engine itself).
    ///
    /// The 'init' parameter controls whether an execution dependency is also implied during the
    /// init phase.
    pub fn uses<T: IAny + ?Sized>(&mut self, init: InitOrder) {
        self.__uses(TypeId::of::<T>(), init)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum InitOrder {
    After,
    DontCare,
}

///
/// This utility macro will generate an initializer for a [PluginDescription] that will capture and
/// initialize all the fields from the invoking crate's cargo metadata.
///
/// Specifically this will generate the following:
/// ```ignore
/// aleph_interfaces::plugin::PluginDescription {
///      name: env!("CARGO_PKG_NAME").to_string(),
///      description: env!("CARGO_PKG_DESCRIPTION").to_string(),
///      major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
///      minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
///      patch_version: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
///  }
/// ```
///
/// This has to be a macro as the 'env!' macros need to be resolved in the calling crate instead of
/// this crate as otherwise the cargo env vars will contain the wrong values (for this crate).
///
#[macro_export]
macro_rules! make_plugin_description_for_crate {
    () => {
        $crate::plugin::PluginDescription {
            name: env!("CARGO_PKG_NAME").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch_version: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    };
}
