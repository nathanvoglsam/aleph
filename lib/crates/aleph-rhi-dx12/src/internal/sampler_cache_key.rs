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

use std::hash::{Hash, Hasher};
use std::mem::transmute;

use aleph_rhi_api::SamplerDesc;

#[repr(transparent)]
pub struct SamplerCacheKey<'a>(SamplerDesc<'a>);

impl<'a> SamplerCacheKey<'a> {
    pub fn new(desc: SamplerDesc<'a>) -> SamplerCacheKey<'a> {
        Self(desc)
    }

    pub unsafe fn from_desc<'b>(desc: &'b SamplerDesc<'a>) -> &'b SamplerCacheKey<'a> {
        let info = desc as *const SamplerDesc<'a> as *const SamplerCacheKey<'a>;
        // Safety: This is safe because both types have the same layout (repr transparent) and the
        //         lifetime is correctly passed across. This is just a slightly different view of
        //         the same type
        &*info
    }
}

impl<'a> From<SamplerCacheKey<'a>> for SamplerDesc<'a> {
    fn from(val: SamplerCacheKey<'a>) -> Self {
        val.0
    }
}

impl<'a> Hash for SamplerCacheKey<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.min_filter.hash(state);
        self.0.mag_filter.hash(state);
        self.0.mip_filter.hash(state);
        self.0.address_mode_u.hash(state);
        self.0.address_mode_v.hash(state);
        self.0.address_mode_w.hash(state);
        hash_f32(self.0.lod_bias, state);
        hash_f32(self.0.min_lod, state);
        hash_f32(self.0.max_lod, state);
        self.0.enable_anisotropy.hash(state);
        self.0.max_anisotropy.hash(state);
        self.0.compare_op.hash(state);
        self.0.border_color.hash(state);
    }
}

impl<'a> PartialEq for SamplerCacheKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        let compare = |l: &Self, r: &Self| {
            cmp(&l.0.min_filter, &r.0.min_filter)?;
            cmp(&l.0.mag_filter, &r.0.mag_filter)?;
            cmp(&l.0.mip_filter, &r.0.mip_filter)?;
            cmp(&l.0.address_mode_u, &r.0.address_mode_u)?;
            cmp(&l.0.address_mode_v, &r.0.address_mode_v)?;
            cmp(&l.0.address_mode_w, &r.0.address_mode_w)?;
            cmp_f32(&l.0.lod_bias, &r.0.lod_bias)?;
            cmp_f32(&l.0.min_lod, &r.0.min_lod)?;
            cmp_f32(&l.0.max_lod, &r.0.max_lod)?;
            cmp(&l.0.enable_anisotropy, &r.0.enable_anisotropy)?;
            cmp(&l.0.max_anisotropy, &r.0.max_anisotropy)?;
            cmp(&l.0.compare_op, &r.0.compare_op)?;
            cmp(&l.0.border_color, &r.0.border_color)?;
            Some(())
        };

        compare(self, other).is_some()
    }
}

impl<'a> Eq for SamplerCacheKey<'a> {}

fn cmp<T: PartialEq + Eq>(l: &T, r: &T) -> Option<()> {
    if l.eq(r) {
        Some(())
    } else {
        None
    }
}

fn cmp_f32(l: &f32, r: &f32) -> Option<()> {
    debug_assert!(l.is_finite());
    debug_assert!(r.is_finite());
    let l: u32 = unsafe { transmute(*l) };
    let r: u32 = unsafe { transmute(*r) };
    if l.eq(&r) {
        Some(())
    } else {
        None
    }
}

fn hash_f32<H: Hasher>(v: f32, state: &mut H) {
    debug_assert!(v.is_finite());
    let v: u32 = unsafe { transmute(v) };
    v.hash(state);
}
