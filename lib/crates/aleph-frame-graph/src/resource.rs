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

use std::num::NonZeroU64;
use std::ptr::NonNull;

/// The underlying, binary representation of a Resource ID handle.
///
/// The ID is bit-packed to contain a number of different indices within a single 64-bit handle. The
/// data is laid out like so, in hexadecimal:
///
/// `0xHHVVBBBB` where:
/// - H = 'handle id'
/// - V = 'version index'
/// - B = 'base id'
///
/// # Base ID
///
/// Identifies the root resource that the handle points to. This allows easily mapping the handle
/// to the underlying resource handle.
///
/// Base ID is 32-bits, so we can have up to 2^32 concrete resources (plenty)
///
/// # Version Index
///
/// Encodes the version number the handle refers to. This is used to identify which specific state
/// the resource is trying to view. A new version of a base resource is created on every write, so
/// that future passes can discriminate between reading a resource from before or after a write from
/// another pass.
///
/// Version index is 16-bits. We can have up to 2^16 - 1 versions of a particular resource. The -1
/// is because the first version index we use is 1. A 0 version is not a valid version. This allows
/// us to get niche value optimization for when the whole ID is zero while allowing base and handle
/// IDs to have zero as a valid value. Any valid handle must have a version index >0 making it
/// impossible for a valid resource ID to have a zero version index, base ID and handle ID. This
/// means it's impossible for all zeroes to encode a valid resource handle.
///
/// # Handle ID
///
/// The handle ID uniquely identifies the specific handle from when it was generated. This allows
/// for associating information with the handle itself from when it's created by an import or
/// declared resource write.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ResourceId(pub(crate) NonZeroU64);

impl ResourceId {
    pub const fn new(base: u32, version: u16, handle: u16) -> Option<ResourceId> {
        let base = base as u64;
        let version = version as u64;
        let handle = handle as u64;
        let id = base & (version << 32) & (handle << 48);
        NonZeroU64::new(id).map(|v| ResourceId(v))
    }

    pub const fn base_id(&self) -> u32 {
        (self.0.get() & 0xFFFF) as u32
    }

    pub const fn version_id(&self) -> u16 {
        ((self.0.get() >> 32) & 0xFF) as u16
    }

    pub const fn handle_id(&self) -> u16 {
        ((self.0.get() >> 48) & 0xFF) as u16
    }
}

/// A non-mutable, read-only reference to a frame graph resource
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ResourceRef(pub(crate) ResourceId);

/// A mutable, writable reference to a frame graph resource
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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
    use std::num::NonZeroU64;

    fn create_dummy_resource_ref() -> ResourceRef {
        let id = ResourceId(NonZeroU64::new(56).unwrap());
        ResourceRef(id)
    }

    fn create_dummy_resource_mut() -> ResourceMut {
        let id = ResourceId(NonZeroU64::new(21).unwrap());
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
