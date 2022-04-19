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

use crate::swap_chain::SwapChain;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::gpu::{IAcquiredTexture, ITexture};
use std::ops::Deref;
use std::sync::atomic::Ordering;

pub struct AcquiredTexture {
    pub(crate) swap_chain: AnyArc<SwapChain>,
    pub(crate) image: AnyArc<dyn ITexture>,
}

declare_interfaces!(AcquiredTexture, [IAcquiredTexture]);

unsafe impl Send for AcquiredTexture {}

impl Drop for AcquiredTexture {
    fn drop(&mut self) {
        let result = self
            .swap_chain
            .acquired
            .compare_exchange(true, false, Ordering::Release, Ordering::Relaxed)
            .is_ok();
        assert!(
            result,
            "Attempted to release image acquisition but it had already been released"
        );
    }
}

impl IAcquiredTexture for AcquiredTexture {
    fn image(&self) -> &dyn ITexture {
        self.image.deref()
    }
}
