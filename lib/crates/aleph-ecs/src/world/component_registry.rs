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

use crate::world::{Component, ComponentIdMap, ComponentTypeDescription, ComponentTypeId};

///
///
///
#[derive(Clone)]
pub struct ComponentRegistry {
    descriptions: ComponentIdMap<ComponentTypeDescription>,
}

impl ComponentRegistry {
    #[inline]
    pub fn new() -> ComponentRegistry {
        Self {
            descriptions: ComponentIdMap::with_hasher(Default::default()),
        }
    }

    #[inline]
    pub fn lookup(&self, id: ComponentTypeId) -> Option<&ComponentTypeDescription> {
        self.descriptions.get(&id)
    }

    #[inline]
    pub fn register<T: Component>(&mut self) -> ComponentTypeDescription {
        // Get the description, as generated by the component trait
        let description = T::get_type_description();

        // SAFETY: The type description provided by the component crate will always be valid as the
        //         description is generated by a blanket trait impl that auto fills the data using
        //         the compiler intrinsics (size_of, align_of, etc...)
        unsafe {
            if self.register_dynamic(&description) {
                description
            } else {
                panic!("Multiple components with the same id have been registered");
            }
        }
    }

    /// The function provides the raw implementation of adding to the component registry using an
    /// arbitrary `ComponentTypeDescription`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because there is no way to guarantee that the memory layout provided
    /// is valid for the provided ID. It is possible to provide the ID for a rust type but give an
    /// incorrect size and trigger UB.
    #[inline]
    pub unsafe fn register_dynamic(&mut self, description: &ComponentTypeDescription) -> bool {
        // If there is already a component registered with the provided ID we return false without
        // modifying the registry.
        if self.descriptions.get(&description.type_id).is_some() {
            false
        } else {
            // If we key isn't present we can insert the description
            self.descriptions
                .insert(description.type_id, description.clone());
            true
        }
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
