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

use std::num::{NonZeroU16, NonZeroU32};

use aleph_rhi_api::*;
use thiserror::Error;

/// Enumeration of all supported vertex stream identifiers.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum VertexStream {
    /// The position channel. Encodes the most fundamental vertex data, vertex positions. Can be
    /// 2D or 3D depending on the usage context.
    Position = 0,

    /// The normal channel. Contains vertex normals, needed in conjunction with the tangents to
    /// encode the reference frame for the tangent-space normal maps.
    Normal = 1,

    /// The tangent channel. Contains vertex tangents, needed in conjunction with the normals to
    /// encode the reference frame for tangent-space normal maps.
    Tangent = 2,

    /// The bitangent channel. Typically unused as this can be derived as the cross product of
    /// 'normal' and 'tangent'.
    Bitangent = 3,

    /// A stream of UV coordinates with a tag to allow multiple UV streams.
    UV(u8) = 4,

    /// An arbitrary vertex stream which can contain data unique to whatever situation it is used
    /// in. Contains a tag to allow multiple custom streams. Interpretation is unique to the
    /// pipeline the channel is used in, it's up to the user to ensure the data is used in the
    /// correct place.
    Custom(u8) = 5,
}

/// Describes a mesh layout, which represents a unique set of vertex streams with thier in-memory
/// formats.
///
/// [`MeshLayoutDesc`] requires that [`MeshLayoutDesc::streams`] is sorted based on the
/// [`VertexStream`] key and that there are no duplicate keys. This prevents two different layouts
/// with the same set of streams and formats comparing as not-equal because the streams are in a
/// different order.
///
/// # Why?
///
/// Why not use a hash set? Simplicity. A hash set needs allocation overhead to avoid collisions and
/// you pay the cost of hashing the keys. 99.99% of mesh layouts will contain 4 streams so the fixed
/// cost of a hash set will swallow all the O(1) benefits. Much simpler to just push into an array
/// and sort it. You could even just push in sorted order! Validating is O(n). With an average n of
/// 4 it's dirt cheap, and so much simpler.
///
/// The ultimate benefit is that we can then guarantee that two [`MeshLayoutDesc`] objects
/// constructed independently with the same set of streams must be exactly equal to eachother with
/// no chance of false-negatives from streams being in the wrong order. We _could_ allow unsorted
/// lists but then comparison becomes O(n^2). The goal is that we can hash-n-cache the descriptions
/// and associate them with some RHI state. We want all identical descriptions to have a single
/// identity so we can use them as a (cheap) key.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MeshLayoutDesc<'a>(&'a [(VertexStream, Format)]);

impl<'a> MeshLayoutDesc<'a> {
    /// A pre-made description for a basic static mesh using some simple defaults. Not the most
    /// optimal layout, but it is simple and flexible.
    pub const STANDARD_STATIC: MeshLayoutDesc<'static> = MeshLayoutDesc(&[
        (VertexStream::Position, Format::Rgb32Float),
        (VertexStream::Normal, Format::Rgb32Float),
        (VertexStream::Tangent, Format::Rgb32Float),
        (VertexStream::UV(0), Format::Rg32Float),
    ]);

    pub fn new(
        streams: &'a [(VertexStream, Format)],
    ) -> Result<MeshLayoutDesc<'a>, MeshLayoutDescError> {
        // We require that the set of streams is sorted.
        let is_sorted = streams.windows(2).all(|w| w[0].0 <= w[1].0);
        if !is_sorted {
            return Err(MeshLayoutDescError::NotSorted);
        }

        // We also require there are no duplicate streams.
        let no_duplicates = streams.windows(2).all(|w| w[0].0 != w[1].0);
        if !no_duplicates {
            return Err(MeshLayoutDescError::ContainsDuplicate);
        }

        Ok(Self(streams))
    }

    /// Get access to the inner list of vertex streams.
    pub fn as_inner(&self) -> &'a [(VertexStream, Format)] {
        &self.0
    }

    /// Will sort the given list of streams by the [`VertexStream`] key.
    ///
    /// You can use this before calling [`MeshLayoutDesc::new`] to ensure the layout is sorted.
    pub fn sort(streams: &mut [(VertexStream, Format)]) {
        streams.sort_unstable_by_key(|v| v.0);
    }
}

/// Set of errors that can be encountered when constructing a [`MeshLayoutDesc`].
#[derive(Error, Debug)]
pub enum MeshLayoutDescError {
    #[error("The layout description is not sorted.")]
    NotSorted,

    #[error("The layout contains a duplicate entry.")]
    ContainsDuplicate,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MeshLayoutId(NonZeroU32);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct MeshLayoutIdFields {
    /// A 'nonce' value owned by the renderer instance this layout was created from that is used
    /// to check that the layout is used with the renderer that created it.
    pub render_id: u16,

    /// The actual numerical id of the mesh layout.
    pub layout_id: NonZeroU16,
}

impl MeshLayoutIdFields {
    pub const fn from_id(v: MeshLayoutId) -> Self {
        let v = v.0.get();

        let render_id = (v >> 16) as u16;
        // Safety: The low half of MeshLayoutId is guaranteed to be non zero by construction.
        let layout_id = unsafe { NonZeroU16::new_unchecked((v & 0xFFFF) as u16) };
        Self {
            render_id,
            layout_id,
        }
    }

    /// # Safety
    ///
    /// The implementation of this function is safe, but creating an arbitrary [`MeshLayoutId`] is
    /// not as we use [`MeshLayoutIdFields::render_id`] as part of our safety checks. The user could
    /// break invariants if this was considered safe.
    pub const unsafe fn to_id(self) -> MeshLayoutId {
        let low = self.layout_id.get() as u32;
        let high = (self.render_id as u32) << 16;
        let word = low | high;
        // Safety: The low half is guaranteed to be non zero so this is safe
        let word = NonZeroU32::new_unchecked(word);
        MeshLayoutId(word)
    }
}

#[cfg(test)]
mod tests {
    use crate::MeshLayoutDesc;

    #[test]
    pub fn test_standard_static_layout_valid() {
        //! This test asserts that the [`MeshLayoutDesc::STANDARD_STATIC`] is valid, as we can't
        //! validate it at compile time but we assume it's valid.
        //!
        //! This protects against mistakes made when updating it.
        let _ = MeshLayoutDesc::new(MeshLayoutDesc::STANDARD_STATIC.as_inner()).unwrap();
    }
}
