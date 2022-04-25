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

use bitflags::bitflags;

bitflags! {
    pub struct ResourceStates: u32 {
        /// The state of the resource is unknown or undefined.
        const UNKNOWN                   = 0;

        /// General purpose state. Resources in this state can be used as anything.
        ///
        /// # Warning
        ///
        /// This has significant performance implications, especially for textures. Only use this
        /// when absolutely necessary.
        const COMMON                    = 0x00000001;

        /// State allowing use as a constant/uniform buffer.
        const CONSTANT_BUFFER           = 0x00000002;

        /// State allowing use as a vertex buffer.
        const VERTEX_BUFFER             = 0x00000004;

        /// State allowing use as an index buffer.
        const INDEX_BUFFER              = 0x00000008;

        const INDIRECT_ARGUMENT         = 0x00000010;

        const SHADER_RESOURCE           = 0x00000020;

        const UNORDERED_ACCESS          = 0x00000040;

        /// State allowing use as a render target.
        const RENDER_TARGET             = 0x00000080;

        const DEPTH_WRITE               = 0x00000100;

        const DEPTH_READ                = 0x00000200;

        const STREAM_OUT                = 0x00000400;

        /// State allowing use as a copy destination.
        const COPY_DEST                 = 0x00000800;

        /// State allowing use as a copy source.
        const COPY_SOURCE               = 0x00001000;

        const RESOLVE_DEST              = 0x00002000;

        const RESOLVE_SOURCE            = 0x00004000;

        /// State allowing use as a presentation surface. A resource must be in this state on the
        /// GPU timeline for a resource to be used to present from.
        const PRESENT                   = 0x00008000;

        const ACCEL_STRUCT_READ         = 0x00010000;

        const ACCEL_STRUCT_WRITE        = 0x00020000;

        const ACCEL_STRUCT_BUILD_INPUT  = 0x00040000;

        const ACCEL_STRUCT_BUILD_BLAS   = 0x00080000;

        const SHADING_RATE_SURFACE      = 0x00100000;
    }
}

impl Default for ResourceStates {
    #[inline]
    fn default() -> Self {
        ResourceStates::UNKNOWN
    }
}

/// Enumeration of all CPU access modes for resources
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CpuAccessMode {
    /// Resource can not be accessed by the CPU at all (device local)
    None,

    /// Resource can be read by the CPU (read back)
    Read,

    /// Resource can be written by the CPU (upload)
    Write,
}
