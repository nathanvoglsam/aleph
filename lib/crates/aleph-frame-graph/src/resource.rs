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

/// The underlying, binary representation of a Resource ID handle
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ResourceId(pub(crate) NonZeroU64);

impl ResourceId {
    pub const fn version_id(&self) -> u16 {
        todo!()
    }

    pub const fn resource_id(&self) -> u32 {
        todo!()
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
