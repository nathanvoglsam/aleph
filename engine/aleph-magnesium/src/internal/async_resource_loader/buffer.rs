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

pub struct BufferLoadState<C> {
    /// The buffer object we're uploading into on the GPU.
    pub buffer: rhi::BufferHandle,

    /// Number of bytes allocated from the loader's pool associated with this buffer load.
    pub bytes_allocated: u64,

    /// Number of bytes submitted to the upload queue associated with this buffer load.
    pub bytes_submitted: u64,

    /// Total number of bytes needed for the resource to be considered completely uploaded.
    pub bytes_needed: u64,

    /// Caller provided cookie that will be sent alongside messages on the loader's notification
    /// channel to identify the associated request.
    pub cookie: C,
}

impl<C> BufferLoadState<C> {
    pub const fn remaining_bytes(&self) -> u64 {
        self.bytes_needed - self.bytes_submitted
    }

    pub const fn has_outstanding_range(&self) -> bool {
        self.bytes_allocated > self.bytes_submitted
    }

    pub const fn is_complete(&self) -> bool {
        self.bytes_submitted == self.bytes_needed
    }
}
