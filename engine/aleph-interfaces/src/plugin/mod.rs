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

use std::any::{TypeId, type_name};

use any::IAny;
use ecs::world::World;
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
    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {}

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
/// An abstract interface over any potential concrete implementation of an accessor into the plugin
/// registry. This can be used to retrieve interface implementations, request the main loop exit,
/// etc.
///
pub trait IRegistryAccessor {
    /// Object safe implementation of `get_interface`. See wrapper for more info.
    #[doc(hidden)]
    fn __get_interface(&self, interface: TypeId) -> Option<AnyArc<dyn IAny>>;

    /// Object safe implementation of `provide`. See wrapper for more info.
    #[doc(hidden)]
    fn __provide(&mut self, interface: TypeId, object: AnyArc<dyn IAny>);

    /// Registry quit handle which can be freely sent to other threads. The object is used to
    /// request the engine/plugin registry to exit.
    fn quit_handle(&self) -> AnyArc<dyn IQuitHandle>;

    /// Fetch the config object for the given root config name.
    ///
    /// # Example
    ///
    /// Each crate can export a number of named config objects. The aleph-engine crate (at the time
    /// of writing) exports 'core' and 'rhi' config objects. To access these objects simply call:
    /// ```ignore
    /// let accessor: &dyn IRegistryAccessor = something;
    /// let rhi = accessor.config_by_name("rhi");
    /// let core = accessor.config_by_name("core");
    /// ```
    ///
    /// Only a single crate can export a config for a given name.
    fn config(&self, name: &str) -> Option<&serde_json::Value>;

    /// Access to a core set of resources provided by the engine, wrapped in a [`CoreRefs`].
    fn core(&mut self) -> CoreRefs<'_>;
}

impl<'a> dyn IRegistryAccessor + 'a {
    /// Get a reference counted handle to the interface with the type given by the `T` type
    /// parameter.
    pub fn get_interface<T: IAny + ?Sized>(&self) -> Option<AnyArc<T>> {
        self.__get_interface(TypeId::of::<T>())
            .map(|v| v.query_interface::<T>().unwrap())
    }

    pub fn provide<I: IAny + ?Sized, T: IAny>(&mut self, object: AnyArc<T>) {
        // We check this in the __provide layer too but we can generate much better panic messages
        // if we do it here where we can get the type names of all objects involved.
        let interface_name = type_name::<I>();
        let got_name = type_name::<T>();
        assert!(
            object.query_interface::<I>().is_some(),
            "Attempting to provide an object '{}' that does not implement the '{}' interface!",
            got_name,
            interface_name
        );
        self.__provide(TypeId::of::<I>(), AnyArc::map::<dyn IAny, _>(object, |v| v));
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
/// implementation, or an abstract interface like `IWindow`. This way it is possible for a
/// plugin to depend on both specific plugins (i.e `WindowSDL2`) or they can declare a
/// dependency that is generic over arbitrary plugins that provide an abstract interface
/// (i.e `IWindow`) implementation.
///
pub trait IPluginRegistrar {
    /// Object safe implementation of `requires`. See wrapper for more info.
    #[doc(hidden)]
    fn __requires(&mut self, requires: TypeId, init: InitOrder);

    /// Object safe implementation of `provides_interface`. See wrapper for more info.
    #[doc(hidden)]
    fn __provides(&mut self, provides: TypeId, availability: Provides);

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
    pub fn provides<T: IAny + ?Sized>(&mut self, availability: Provides) {
        self.__provides(TypeId::of::<T>(), availability)
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Provides {
    Always,
    Optional,
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
