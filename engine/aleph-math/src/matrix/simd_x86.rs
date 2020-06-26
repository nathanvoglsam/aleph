//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

use core::mem::MaybeUninit;

use crate::matrix::TMat4x4;
use crate::traits::Real;
use crate::vector::TVec4;

///
/// SIMD implementation of mat4 -> vec4 multiplication
///
#[inline]
pub fn simd_f32_apply<T: Real>(lhs: &TMat4x4<T>, vec: TVec4<T>) -> TVec4<T> {
    unsafe {
        let x = _mm_set1_ps(vec[0].as_f32());
        let y = _mm_set1_ps(vec[1].as_f32());
        let z = _mm_set1_ps(vec[2].as_f32());
        let w = _mm_set1_ps(vec[3].as_f32());

        let col0 = _mm_load_ps(&lhs.data[0] as *const T as *const f32);
        let col1 = _mm_load_ps(&lhs.data[4] as *const T as *const f32);
        let col2 = _mm_load_ps(&lhs.data[8] as *const T as *const f32);
        let col3 = _mm_load_ps(&lhs.data[12] as *const T as *const f32);

        let p0 = _mm_mul_ps(x, col0);
        let p1 = _mm_mul_ps(y, col1);
        let p2 = _mm_mul_ps(z, col2);
        let p3 = _mm_mul_ps(w, col3);

        let out = _mm_add_ps(_mm_add_ps(p0, p1), _mm_add_ps(p2, p3));

        let mut out_vec: MaybeUninit<TVec4<T>> = MaybeUninit::uninit();
        _mm_store_ps(out_vec.as_mut_ptr() as *mut f32, out);
        out_vec.assume_init()
    }
}

///
/// SIMD implementation of mat4 -> mat4 multiplication
///
#[inline]
pub fn simd_f32_mul_assign<T: Real>(lhs: &mut TMat4x4<T>, rhs: &TMat4x4<T>) {
    let (row0, row1, row2, row3) = unsafe {
        let row0 = &lhs[0] as *const T as *const f32;
        let row0 = _mm_load_ps(row0);

        let row1 = &lhs[4] as *const T as *const f32;
        let row1 = _mm_load_ps(row1);

        let row2 = &lhs[8] as *const T as *const f32;
        let row2 = _mm_load_ps(row2);

        let row3 = &lhs[12] as *const T as *const f32;
        let row3 = _mm_load_ps(row3);
        (row0, row1, row2, row3)
    };
    for i in 0..4 {
        let (brod0, brod1, brod2, brod3) = unsafe {
            let brod0 = rhs[(i * 4)].as_f32();
            let brod0 = _mm_set1_ps(brod0);

            let brod1 = rhs[(i * 4) + 1].as_f32();
            let brod1 = _mm_set1_ps(brod1);

            let brod2 = rhs[(i * 4) + 2].as_f32();
            let brod2 = _mm_set1_ps(brod2);

            let brod3 = rhs[(i * 4) + 3].as_f32();
            let brod3 = _mm_set1_ps(brod3);
            (brod0, brod1, brod2, brod3)
        };
        let row = unsafe {
            _mm_add_ps(
                _mm_add_ps(_mm_mul_ps(brod0, row0), _mm_mul_ps(brod1, row1)),
                _mm_add_ps(_mm_mul_ps(brod2, row2), _mm_mul_ps(brod3, row3)),
            )
        };
        unsafe {
            let store = &mut lhs[4 * i] as *mut T as *mut f32;
            _mm_store_ps(store, row);
        }
    }
}
