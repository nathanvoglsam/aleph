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

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSError;
use objc2_metal::*;

use crate::device::Device;

/// A simple wrapper over a [`MTLResidencySet`].
///
/// Handles adding/removing allocations and uses an internal dirty flag to skip calling 'commit' if
/// no changes have been made to the set since the last call.
pub struct ResidencySet {
    pub set: Retained<ProtocolObject<dyn MTLResidencySet>>,
    dirty: bool,
}

impl ResidencySet {
    /// Constructs a new residency set.
    pub fn new(device: &Device) -> Result<Self, Retained<NSError>> {
        unsafe {
            let descriptor = MTLResidencySetDescriptor::new();
            descriptor.setInitialCapacity(32);
            let set = device
                .device
                .newResidencySetWithDescriptor_error(&descriptor)?;

            let out = Self { set, dirty: true };

            Ok(out)
        }
    }

    /// Adds the given allocation to the residency set. Also marks the set as dirty.
    pub fn add_allocation(&mut self, allocation: &ProtocolObject<dyn MTLAllocation>) {
        self.set.addAllocation(allocation);
        self.dirty = true;
    }

    /// Removes the given allocation from the residency set. Also marks the set as dirty.
    pub fn remove_allocation(&mut self, allocation: &ProtocolObject<dyn MTLAllocation>) {
        self.set.removeAllocation(allocation);
        self.dirty = true;
    }

    /// Commits any outstanding changes to the set, if it is marked as dirty. This will skip calling
    /// the Metal 'commit' API if we think there are no changes.
    pub fn commit_if_dirty(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.set.commit();
        }
    }
}
