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
        pub position: aleph_math::DVec3,

        /// This stores the rotation of the entity in world space using single-precision floating
        /// point. Rotation will not need more precision than an f32.
        pub rotation: aleph_math::Rotor3,

        /// This stores the scaling component using single-precision floating point. Double
        /// precision would be wasted here so f32 is chosen instead.
        pub scale: aleph_math::Vec3,
    }
    ecs::register_component!(Transform);
}

mod transform_history {
    use crate::components::Transform;

    ///
    /// This component stores an object's world space position in the previous frame.
    ///
    /// # Background
    ///
    /// This is typically used for TAA motion vectors, and is optional. Objects without this
    /// component are assumed to be static and will not generate motion vectors.
    ///
    #[derive(Clone, PartialEq, Debug, Default)]
    #[repr(C)]
    pub struct TransformHistory {
        pub previous: Transform,
    }
    ecs::register_component!(TransformHistory);
}

mod camera {
    ///
    /// This component attaches camera behavior to an entity. The fields provide the configuration
    /// for the renderer's projection matrix (except for the aspect ratio, which is derived by the
    /// renderer from screen resolution).
    ///
    /// A [`Camera`] entity requires a [`crate::components::Transform`] component to provide a world
    /// transform for the camera to render from.
    ///
    #[derive(Clone, PartialEq, Debug, Default)]
    #[repr(C)]
    pub struct Camera {
        /// The vertical field-of-view of the perspective projection, in _degrees_.
        pub vertical_fov: f32,

        /// The near plane distance from the camera. We always use a reversed-z infinite perspective
        /// projection so there is no far plane to specify.
        pub z_near: f32,
    }
    ecs::register_component!(Camera);
}

mod lights {
    ///
    ///
    ///
    #[derive(Clone, PartialEq, Debug, Default)]
    #[repr(C)]
    pub struct PointLight {
        /// The intensity of the light in lumens
        pub intensity: f32,
    }
    ecs::register_component!(PointLight);
}

mod static_mesh {
    use mg::material_instance::MaterialInstanceHandle;
    use mg::resource::buffer::BufferHandle;

    ///
    ///
    ///
    #[derive(Clone, PartialEq, Debug)]
    #[repr(C)]
    pub struct StaticMesh {
        pub vtx: BufferHandle,
        pub idx: BufferHandle,
        pub material_instance: MaterialInstanceHandle,
    }
    ecs::register_component!(StaticMesh);
}

pub use camera::Camera;
pub use lights::PointLight;
pub use static_mesh::StaticMesh;
pub use transform::Transform;
pub use transform_history::TransformHistory;
