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

use windows::Win32::Graphics::Direct3D12::*;

/// A value to be used with the `ComponentMapping` constructors.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct D3D12ComponentMappingValue(u32);

impl D3D12ComponentMappingValue {
    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should read from the 0th component in the underlying texture.
    pub const FROM_0: Self = Self(0);

    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should read from the 1st component in the underlying texture.
    pub const FROM_1: Self = Self(1);

    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should read from the 2nd component in the underlying texture.
    pub const FROM_2: Self = Self(2);

    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should read from the 3rd component in the underlying texture.
    pub const FROM_3: Self = Self(3);

    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should be hardwired to 0 (i.e will always return 0 when sampled)
    pub const FORCE_0: Self = Self(4);

    /// Value for use with `ComponentMapping` constructors. Means that the component being mapped
    /// should be hardwired to 1 (i.e will always return 0 when sampled)
    pub const FORCE_1: Self = Self(5);
}

/// This struct represents a wrapper around DirectX12's D3D12_SHADER_COMPONENT_MAPPING system.
///
/// The functionality is exposed as macros in the C/C++ headers so it needs to be hand
/// re-implemented in rust.
///
/// The `Default` implementation returns an identity mapping
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
#[repr(transparent)]
pub struct D3D12ComponentMapping(u32);

impl D3D12ComponentMapping {
    ///
    /// A `const fn` constructor that applies the given mappings to each component
    ///
    pub const fn new(
        component_0: D3D12ComponentMappingValue,
        component_1: D3D12ComponentMappingValue,
        component_2: D3D12ComponentMappingValue,
        component_3: D3D12ComponentMappingValue,
    ) -> Self {
        let r = component_0.0 & D3D12_SHADER_COMPONENT_MAPPING_MASK;
        let r = r;

        let g = component_1.0 & D3D12_SHADER_COMPONENT_MAPPING_MASK;
        let g = g << D3D12_SHADER_COMPONENT_MAPPING_SHIFT;

        let b = component_2.0 & D3D12_SHADER_COMPONENT_MAPPING_MASK;
        let b = b << (D3D12_SHADER_COMPONENT_MAPPING_SHIFT * 2);

        let a = component_3.0 & D3D12_SHADER_COMPONENT_MAPPING_MASK;
        let a = a << (D3D12_SHADER_COMPONENT_MAPPING_SHIFT * 3);

        let always_set = D3D12_SHADER_COMPONENT_MAPPING_ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES;

        Self(r | g | b | a | always_set)
    }

    ///
    /// Returns an identity mapping. That is:
    ///   - r -> r
    ///   - g -> g
    ///   - b -> b
    ///   - a -> a
    ///
    pub const fn identity() -> Self {
        Self::new(
            D3D12ComponentMappingValue::FROM_0,
            D3D12ComponentMappingValue::FROM_1,
            D3D12ComponentMappingValue::FROM_2,
            D3D12ComponentMappingValue::FROM_3,
        )
    }
}

impl Default for D3D12ComponentMapping {
    #[inline]
    fn default() -> Self {
        Self::identity()
    }
}

impl From<D3D12ComponentMapping> for u32 {
    #[inline]
    fn from(v: D3D12ComponentMapping) -> Self {
        v.0
    }
}
