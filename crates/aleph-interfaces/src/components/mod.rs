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

mod transform {
    ///
    /// This component stores an object's transform in world space as position, rotation and scale
    ///
    #[derive(Clone, PartialEq, Debug, Default)]
    #[repr(C)]
    pub struct Transform {
        /// This stores the position of the entity in world space using double-precision floating
        /// point. This makes position "just work" for planet scale worlds
        pub position: ultraviolet::DVec3,

        /// This stores the rotation of the entity in world space using single-precision floating
        /// point. Rotation will not need more precision than an f32.
        pub rotation: ultraviolet::Rotor3,

        /// This stores the scaling component using single-precision floating point. Double
        /// precision would be wasted here so f32 is chosen instead.
        pub scale: ultraviolet::Vec3,
    }
}

pub use transform::Transform;
