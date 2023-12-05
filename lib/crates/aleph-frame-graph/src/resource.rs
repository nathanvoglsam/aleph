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

use crate::internal::VersionIndex;
use std::num::NonZeroU16;
use std::ptr::NonNull;

/// The underlying, binary representation of a Resource ID handle.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ResourceId {
    /// A special niche value used to enable Option<ResourceId> to have the same size as ResourceId
    pub niche_value: NonZeroU16,

    /// The index of the root resource this handle refers to
    pub root: u16,

    /// The index of the resource version this handle refers to
    pub version: u32,
}

impl ResourceId {
    pub fn new(root: u16, version: u32) -> ResourceId {
        debug_assert_ne!(version, VersionIndex::INVALID.0);

        // We assert this here as it's the most likely code to execute to catch this case. The
        // condition is constant so it will compile to nothing as long as we get the expected
        // output.
        assert_eq!(
            std::mem::size_of::<ResourceId>(),
            std::mem::size_of::<Option<ResourceId>>()
        );

        Self {
            niche_value: NonZeroU16::new(0xFF).unwrap(),
            root,
            version,
        }
    }

    pub const fn root_id(&self) -> u16 {
        self.root
    }

    pub const fn version_id(&self) -> u32 {
        self.version
    }
}

/// A non-mutable, read-only reference to a frame graph resource
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ResourceRef(pub(crate) ResourceId);

/// A mutable, writable reference to a frame graph resource
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ResourceMut(pub(crate) ResourceId);

// Allow using a mutable resource as an immutable one
impl AsRef<ResourceRef> for ResourceMut {
    fn as_ref(&self) -> &ResourceRef {
        assert_eq!(
            std::mem::size_of::<ResourceMut>(),
            std::mem::size_of::<ResourceRef>()
        );
        assert_eq!(
            std::mem::align_of::<ResourceMut>(),
            std::mem::align_of::<ResourceRef>()
        );

        let ptr = NonNull::from(self).cast::<ResourceRef>();

        // Safety: Types are guaranteed to have same layout, the const generic is purely a compile
        //         time utility.
        unsafe { ptr.as_ref() }
    }
}

// Allow using a mutable resource as an immutable one
impl Into<ResourceRef> for ResourceMut {
    fn into(self) -> ResourceRef {
        ResourceRef(self.0)
    }
}

// Dummy identity AsRef implementation
impl AsRef<ResourceRef> for ResourceRef {
    fn as_ref(&self) -> &ResourceRef {
        self
    }
}

// Dummy identity AsRef implementation
impl AsRef<ResourceMut> for ResourceMut {
    fn as_ref(&self) -> &ResourceMut {
        self
    }
}

// Allow comparing a mutable resource handle to an immutable resource handle
impl PartialEq<ResourceMut> for ResourceRef {
    fn eq(&self, other: &ResourceMut) -> bool {
        self.0.eq(&other.0)
    }
}

// Allow comparing an immutable resource handle to a mutable resource handle
impl PartialEq<ResourceRef> for ResourceMut {
    fn eq(&self, other: &ResourceRef) -> bool {
        self.0.eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::ResourceId;
    use crate::{ResourceMut, ResourceRef};

    fn create_dummy_resource_ref() -> ResourceRef {
        let id = ResourceId::new(1, 2);
        ResourceRef(id)
    }

    fn create_dummy_resource_mut() -> ResourceMut {
        let id = ResourceId::new(3, 2);
        ResourceMut(id)
    }

    fn test_into_resource<T: Into<ResourceRef>>(v: T) -> ResourceRef {
        v.into()
    }

    fn test_as_ref_resource<T: AsRef<ResourceRef>>(v: T) -> ResourceRef {
        *v.as_ref()
    }

    #[test]
    pub fn test_size() {
        assert_eq!(
            std::mem::size_of::<ResourceMut>(),
            std::mem::size_of::<ResourceRef>()
        );
        assert_eq!(
            std::mem::align_of::<ResourceMut>(),
            std::mem::align_of::<ResourceRef>()
        );
    }

    #[test]
    pub fn test_into_resource_exec() {
        let dummy_mut = create_dummy_resource_mut();
        let dummy_mut_after = test_into_resource(dummy_mut);

        let dummy_ref = create_dummy_resource_ref();
        let dummy_ref_after = test_into_resource(dummy_ref);

        assert_eq!(dummy_mut, dummy_mut_after);
        assert_eq!(dummy_ref, dummy_ref_after);
    }

    #[test]
    pub fn test_as_ref_exec() {
        let dummy_mut = create_dummy_resource_mut();
        let dummy_mut_after = test_as_ref_resource(dummy_mut);

        let dummy_ref = create_dummy_resource_ref();
        let dummy_ref_after = test_as_ref_resource(dummy_ref);

        assert_eq!(dummy_mut, dummy_mut_after);
        assert_eq!(dummy_ref, dummy_ref_after);
        assert_ne!(dummy_mut, dummy_ref);
        assert_ne!(dummy_mut_after, dummy_ref_after);
    }
}
