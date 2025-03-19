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

/// The set of supported 2D environment map projection schemes.
///
/// This only includes 2D projections which map all the points on the surface of a sphere onto a
/// single 2D texture. Other schemes like [`EnvironmentMapProjection::Cube`] are not enumerated here
/// as they use multiple textures.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SphericalMapping {
    /// A 2D projection using an equirectangular projection function.
    Equirectangular,

    /// A 2D projection using an octahedral projection function.
    Octahedral,
}

impl Into<EnvironmentMapProjection> for SphericalMapping {
    #[inline(always)]
    fn into(self) -> EnvironmentMapProjection {
        match self {
            SphericalMapping::Equirectangular => EnvironmentMapProjection::Equirectangular,
            SphericalMapping::Octahedral => EnvironmentMapProjection::Octahedral,
        }
    }
}

/// The set of supported environment map projection schemes
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EnvironmentMapProjection {
    /// A 2D projection using an equirectangular projection function.
    Equirectangular,

    /// A 2D projection using an octahedral projection function.
    Octahedral,

    /// A 2.5D projection that uses 6 images arranged into a cube to define the environment map.
    ///
    /// AKA: A cube map.
    Cube,
}
