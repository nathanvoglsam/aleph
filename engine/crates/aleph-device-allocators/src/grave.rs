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

/// A utility type that wraps a `T` but will never make the inner `T` accessible (without unsafe).
///
/// # Why?
///
/// Because [`Grave`] never shares the inner `T` it means we can effectively store a `!Sync` type
/// in a `Sync` container by promising to never access the object it stores. This can be quite
/// useful when authoring other unsafe code. If you only need to store a `T` just for the
/// side-effects of it remaining alive but need to keep the `Sync` bound on the type then this type
/// might be fore you.
pub struct Grave<T>(T);

impl<T> Grave<T> {
    /// Constructs a new [`Grave`] closing over the given `T`
    pub const fn new(v: T) -> Self {
        Self(v)
    }

    /// Back! From the [`Grave`]!. Returns the inner object and destroys the grave.
    ///
    /// # How?
    ///
    /// The key constraint that allows [`Grave`] to impl `Sync` when `T` is `!Sync` is that we can
    /// never construct a `&T` to the inner object which would allow us to share the `T` across
    /// threads and cause UB. We _can_ make a `&Grave<T>` but because you can't get to the inner
    /// `T` we don't violate `T`'s `!Sync` bounds.
    ///
    /// No `&T` is made here, we just yield the inner object and destroy the grave. Making [`Grave`]
    /// a poor man's channel if you wrapped it in something like `AtomicCell<Box<Grave<T>>>`.
    ///
    /// Still useful if you need to resurrect the inner `T` instead of leaving it to die in the
    /// [`Grave`].
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Grave<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self(value)
    }
}

// Safety: We can never give out a shared reference to the inner type so it's safe to share this
//         type across threads as we can never get to the inner T that can't be.
unsafe impl<T: Send> Sync for Grave<T> {}
