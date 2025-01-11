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

use aleph_rhi_api::*;

#[repr(transparent)]
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct OwnedBufferDesc {
    desc: BufferDesc<'static>,
}

impl OwnedBufferDesc {
    /// Constructs a new [`OwnedBufferDesc`] wrapper that upgrades the 'name' entry to be heap
    /// allocated while still masquerading as a 'str'.
    ///
    /// Useful for storing in place inside object implementations for handing out to API callers in
    /// desc getter functions.
    pub fn new<'a>(desc: BufferDesc<'a>) -> Self {
        // Allocate the name on the heap and update the desc with the new allocated name ptr
        let name: Option<&str> = match desc.name {
            Some(name) => {
                let v: Box<str> = Box::from(name);
                let v = Box::leak(v);
                Some(v)
            }
            None => None,
        };

        let mut desc = desc.strip_name();
        desc.name = name;

        Self { desc }
    }

    /// Gets the inner desc
    pub const fn get(&self) -> &BufferDesc {
        &self.desc
    }
}

impl Clone for OwnedBufferDesc {
    #[inline]
    fn clone(&self) -> Self {
        OwnedBufferDesc::new(self.get().clone())
    }
}

impl Drop for OwnedBufferDesc {
    #[inline]
    fn drop(&mut self) {
        match self.desc.name {
            Some(v) => {
                // Safety: we control the construction of this type, and we guarantee that the name
                //         is really a Box<str> so we reconstitute the box and drop it here.
                unsafe {
                    let v = v as *const str as *mut str;
                    let v: Box<str> = Box::from_raw(v);
                    drop(v);
                }
            }
            None => {}
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Debug)]
pub struct OwnedTextureDesc {
    desc: TextureDesc<'static>,
}

impl OwnedTextureDesc {
    /// Constructs a new [`OwnedTextureDesc`] wrapper that upgrades the 'name' entry to be heap
    /// allocated while still masquerading as a 'str'.
    ///
    /// Useful for storing in place inside object implementations for handing out to API callers in
    /// desc getter functions.
    pub fn new<'a>(desc: TextureDesc<'a>) -> Self {
        // Allocate the name on the heap and update the desc with the new allocated name ptr
        let name: Option<&str> = match desc.name {
            Some(name) => {
                let v: Box<str> = Box::from(name);
                let v = Box::leak(v);
                Some(v)
            }
            None => None,
        };

        let mut desc = desc.strip_name();
        desc.name = name;

        Self { desc }
    }

    /// Gets the inner desc
    pub const fn get(&self) -> &TextureDesc {
        &self.desc
    }
}

impl Clone for OwnedTextureDesc {
    #[inline]
    fn clone(&self) -> Self {
        OwnedTextureDesc::new(self.get().clone())
    }
}

impl Drop for OwnedTextureDesc {
    fn drop(&mut self) {
        match self.desc.name {
            Some(v) => {
                // Safety: we control the construction of this type, and we guarantee that the name
                //         is really a Box<str> so we reconstitute the box and drop it here.
                unsafe {
                    let v = v as *const str as *mut str;
                    let v: Box<str> = Box::from_raw(v);
                    drop(v);
                }
            }
            None => {}
        }
    }
}

#[repr(transparent)]
#[derive(PartialEq, Debug)]
pub struct OwnedSamplerDesc {
    desc: SamplerDesc<'static>,
}

impl OwnedSamplerDesc {
    /// Constructs a new [`OwnedSamplerDesc`] wrapper that upgrades the 'name' entry to be heap
    /// allocated while still masquerading as a 'str'.
    ///
    /// Useful for storing in place inside object implementations for handing out to API callers in
    /// desc getter functions.
    pub fn new<'a>(desc: SamplerDesc<'a>) -> Self {
        // Allocate the name on the heap and update the desc with the new allocated name ptr
        let name: Option<&str> = match desc.name {
            Some(name) => {
                let v: Box<str> = Box::from(name);
                let v = Box::leak(v);
                Some(v)
            }
            None => None,
        };

        let mut desc = desc.strip_name();
        desc.name = name;

        Self { desc }
    }

    /// Gets the inner desc
    pub const fn get(&self) -> &SamplerDesc {
        &self.desc
    }
}

impl Clone for OwnedSamplerDesc {
    #[inline]
    fn clone(&self) -> Self {
        OwnedSamplerDesc::new(self.get().clone())
    }
}

impl Drop for OwnedSamplerDesc {
    fn drop(&mut self) {
        match self.desc.name {
            Some(v) => {
                // Safety: we control the construction of this type, and we guarantee that the name
                //         is really a Box<str> so we reconstitute the box and drop it here.
                unsafe {
                    let v = v as *const str as *mut str;
                    let v: Box<str> = Box::from_raw(v);
                    drop(v);
                }
            }
            None => {}
        }
    }
}
