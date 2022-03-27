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

/// Internal trait used on command list implementations.
///
/// This trait exposes functionality for a command list to return itself to the command pool that
/// allocated it so it can be reused.
pub trait ReturnToPool {
    /// Release any references to tracked resources and return the command list to the command pool
    /// so it can be reused.
    ///
    /// # Safety
    ///
    /// This function is safe as the implementation is safe, but calling this at the incorrect time
    /// will break invariants for other unsafe code.
    ///
    /// This will release the tracker references for the command list. If the command list is in
    /// flight when this is called then this de-allocate resources being used on the GPU timeline.
    fn return_to_pool(&mut self);
}

/// Dummy implementation to make developing implementations easier when not all queue/list types
/// have been implemented yet
impl ReturnToPool for () {
    fn return_to_pool(&mut self) {
        unimplemented!()
    }
}

/// Internal struct for pairing a submission index with a command list instance. This is used by
/// queues to track what lists are in flight.
pub struct InFlightCommandList<T: ReturnToPool> {
    pub index: u64,
    pub list: Box<T>,
}
