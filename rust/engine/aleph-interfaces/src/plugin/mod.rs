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

use any::IAny;
use std::any::TypeId;

///
/// The interface that must be implemented by any engine plugin.
///
pub trait IPlugin: IAny {
    fn register(&mut self, registrar: &mut dyn IPluginRegistrar);

    fn on_init(&mut self);

    fn on_update(&mut self);

    fn on_exit(&mut self);
}

///
/// The interface used by plugins to manipulate their initialization and execution order
///
pub trait IPluginRegistrar {
    /// Object safe implementation of `requires_for_init`
    fn __must_init_after(&mut self, requires: TypeId);

    /// Object safe implementation of `must_update_after`
    fn __must_update_after(&mut self, requires: TypeId);

    /// Object safe implementation of `must_exit_after`
    fn __must_exit_after(&mut self, requires: TypeId);

    /// Object safe implementation of `provides_implementation`
    fn __provides_implementation(&mut self, implements: TypeId);
}

impl dyn IPluginRegistrar {
    pub fn must_init_after<T: IAny>(&mut self) {
        self.__must_init_after(TypeId::of::<T>())
    }

    pub fn must_update_after<T: IAny>(&mut self) {
        self.__must_update_after(TypeId::of::<T>())
    }

    pub fn must_exit_after<T: IAny>(&mut self) {
        self.__must_exit_after(TypeId::of::<T>())
    }

    pub fn provides_implementation<T: IAny>(&mut self) {
        self.__provides_implementation(TypeId::of::<T>())
    }
}
