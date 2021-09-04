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

use crate::scheduler::{Label, Resource, ResourceId};
use crate::world::{Component, ComponentTypeId, World};
use std::any::{Any, TypeId};

///
/// The interface expected of an execution stage
///
pub trait Stage: Any + 'static {
    /// This will be called by a scheduler exactly once during an execution cycle.
    fn run(&mut self, world: &mut World);
}

impl dyn Stage {
    /// A vendored in version of [`Any::is`]
    #[inline]
    pub fn is<T: Stage>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// A vendored in version of [`Any::downcast_ref`]
    #[inline]
    pub fn downcast_ref<T: Stage>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn Stage as *const T)) }
        } else {
            None
        }
    }

    /// A vendored in version of [`Any::downcast_mut`]
    #[inline]
    pub fn downcast_mut<T: Stage>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn Stage as *mut T)) }
        } else {
            None
        }
    }
}

///
/// A generic interface expected of a type that describes the components and resources accessed by
/// something.
///
/// A task would use this interface to declare the resources it accesses and the scheduler will then
/// use the declared accesses to schedule tasks in parallel so their access conditions are met.
///
pub trait AccessDescriptor: 'static {
    /// Caller uses this to declare a shared/read access to the given component type
    fn reads_component_with_id(&mut self, component: ComponentTypeId);

    /// Caller uses this to declare a exclusive/write access to the given component type
    fn writes_component_with_id(&mut self, component: ComponentTypeId);

    /// Caller uses this to declare a shared/read access to the given resource
    fn reads_resource_with_id(&mut self, resource: ResourceId);

    /// Caller uses this to declare a exclusive/write access to the given resource
    fn writes_resource_with_id(&mut self, resource: ResourceId);

    /// Caller uses this to declare the label of another system that `self` should run before
    fn runs_before_label(&mut self, system: Box<dyn Label>);

    /// Caller uses this to declare the label of another system that `self` should run after
    fn runs_after_label(&mut self, system: Box<dyn Label>);
}

impl dyn AccessDescriptor {
    /// Generic wrapper around [`AccessDescriptor::reads_component_with_id`] that uses a generic
    /// parameter to get the ID.
    pub fn reads_component<T: Component>(&mut self) {
        self.reads_component_with_id(ComponentTypeId::of::<T>());
    }

    /// Generic wrapper around [`AccessDescriptor::writes_component_with_id`] that uses a generic
    /// parameter to get the ID.
    pub fn writes_component<T: Component>(&mut self) {
        self.writes_component_with_id(ComponentTypeId::of::<T>());
    }

    /// Generic wrapper around [`AccessDescriptor::reads_resource_with_id`] that uses a generic
    /// parameter to get the ID.
    pub fn reads_resource<T: Resource>(&mut self) {
        self.reads_resource_with_id(ResourceId::of::<T>());
    }

    /// Generic wrapper around [`AccessDescriptor::writes_resource_with_id`] that uses a generic
    /// parameter to get the ID.
    pub fn writes_resource<T: Resource>(&mut self) {
        self.writes_resource_with_id(ResourceId::of::<T>());
    }

    /// Generic wrapper around [`AccessDescriptor::runs_before_label`] that handles boxing the label
    pub fn runs_before(&mut self, system: impl Label) {
        self.runs_before_label(Box::new(system))
    }

    /// Generic wrapper around [`AccessDescriptor::runs_after_label`] that handles boxing the label
    pub fn runs_after(&mut self, system: impl Label) {
        self.runs_after_label(Box::new(system))
    }
}
