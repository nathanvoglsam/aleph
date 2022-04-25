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

use interfaces::any::AnyArc;
use interfaces::gpu::{IBuffer, IDescriptorSet, ITexture};

/// Internal struct used to hold references to resources that need to have their lifetime extended
/// until some point in the future.
///
/// This is intended to be used with command lists. When recording all resources used by the command
/// list will have a reference added to a tracker instance which will keep the resources alive via
/// ref counting. The implementation will then keep track of the tracker and its associated command
/// list and will release the references once the command list has completed running on the GPU
/// timeline.
#[derive(Default)]
pub struct CommandListTracker {
    /// Any bare image resources we need to extend the lifetime of until the command list has been
    /// retired.
    pub images: Vec<AnyArc<dyn ITexture>>,

    /// Any bare buffer resources we need to extend the lifetime of until the command list has been
    /// retired
    pub buffers: Vec<AnyArc<dyn IBuffer>>,

    /// Any binding sets we need to extend the lifetime of until the command list has been retired
    pub binding_sets: Vec<AnyArc<dyn IDescriptorSet>>,
}

impl CommandListTracker {
    pub fn clear(&mut self) {
        self.images.clear();
        self.buffers.clear();
        self.binding_sets.clear();
    }
}
