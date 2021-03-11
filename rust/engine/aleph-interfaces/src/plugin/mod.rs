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

use crate::any::AnyArc;
use any::IAny;
use std::any::TypeId;

///
/// The interface that must be implemented by any engine plugin.
///
/// A plugin represents the primary interface for injecting code into the engine's lifecycle. Most
/// functionality *should* be implemented as a plugin that interacts with other plugins.
///
/// An `IPlugin` has the following lifecycle:
///
/// - A concrete `IPlugin` implementation is constructed by someone and given to the plugin registry
///   to take ownership of.
///
/// - The plugin registry will, at some point during initialization, call `IPlugin::register`
///   exactly once so a plugin can declare its execution dependencies and to declare which abstract
///   interfaces the plugin provides.
///
/// - The plugin registry will call `IPlugin::get_implementation` exactly once so that it can
///   collect the list of implementation objects that the plugin provides.
///
/// - The plugin registry will then use the dependencies declared from each plugin to compute a
///   final execution order for each execution stage.
///
/// - `IPlugin::on_init` will be called exactly once, respecting the dependencies declared in the
///   registration phase.
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
    /// Called by the plugin registry exactly once so that a plugin can register its execution
    /// dependencies
    fn register(&mut self, registrar: &mut dyn IPluginRegistrar);

    /// Called by the engine runtime exactly once during the init phase so a plugin can initialize
    /// itself in regards to other plugins
    fn on_init(&mut self);

    /// Called by the engine runtime exactly once *per iteration* of the main loop
    fn on_update(&mut self);

    /// Called by the engine runtime exactly once during the shutdown phase of the engine
    fn on_exit(&mut self);

    /// Will be called by the plugin registry to retrieve the list of all implemented
    fn get_implementations<'a>(&'a self) -> Box<dyn IImplementationList + 'a>;
}

///
/// A trait used by `IPlugin::get_implementations` that is used to abstract
///
pub trait IImplementationList: Iterator<Item = (TypeId, AnyArc<dyn IAny>)> {}

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
    /// Object safe implementation of `depends_on`. See wrapper for more info.
    fn __depends_on(&mut self, dependency: TypeId);

    /// Object safe implementation of `must_init_after`. See wrapper for more info.
    fn __must_init_after(&mut self, requires: TypeId);

    /// Object safe implementation of `must_update_after`. See wrapper for more info.
    fn __must_update_after(&mut self, requires: TypeId);
}

impl dyn IPluginRegistrar {
    /// Declares that the plugin depends on the existence of another plugin given by the type
    /// parameter.
    pub fn depends_on<T: IAny>(&mut self) {
        self.__depends_on(TypeId::of::<T>())
    }

    /// Declares that the plugin's init function can only execute *after* the given plugin has had
    /// its own init function execute.
    pub fn must_init_after<T: IAny>(&mut self) {
        self.__must_init_after(TypeId::of::<T>())
    }

    /// Declares that the plugin's update function can only execute *after* the given plugin has had
    /// its own update function execute.
    pub fn must_update_after<T: IAny>(&mut self) {
        self.__must_update_after(TypeId::of::<T>())
    }
}
