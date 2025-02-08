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

#[cfg(test)]
mod tests;

use std::io::Write;
use std::num::NonZero;

use aleph_vk2dfd::vk2dfd;
use aleph_vk_format::VkFormat;
use byteorder::{LittleEndian, WriteBytesExt};
use num_integer::{lcm, Integer};

use crate::document::FILE_IDENTIFIER;
use crate::{format_type_size, SuperCompressionScheme};

/// The name of the encoder, as will be written into the KTX key/value data
pub const ENCODER_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

pub struct KtxDocumentDescription<'a> {
    width: u32,
    height: u32,
    depth: u32,
    format: VkFormat,
    generate_mips: bool,
    level_num: u32,
    kvd: KeyValueEntries<'a>,
    doc_type: DocumentTypeDescription<'a>,
}

impl<'a> KtxDocumentDescription<'a> {
    /// Constructs a new, invalid [`DocumentDescription`] that needs to be filled out.
    pub fn new() -> Self {
        static DEFAULT_KVD: [KeyValueEntry; 1] = [KeyValueEntry::writer()];

        Self {
            width: 0,
            height: 0,
            depth: 0,
            format: VkFormat::UNDEFINED,
            generate_mips: false,
            level_num: 0,
            kvd: KeyValueEntries {
                entries: &DEFAULT_KVD,
            },
            doc_type: DocumentTypeDescription::Image1D { images: &[] },
        }
    }

    pub fn key_value_data(&mut self, kvd: KeyValueEntries<'a>) -> &mut Self {
        self.kvd = kvd;
        self
    }

    /// Specify the format of the texture
    pub fn format(&mut self, format: VkFormat) -> &mut Self {
        assert_ne!(format, VkFormat::UNDEFINED);
        assert!(format.is_known());
        self.format = format;
        self
    }

    /// Declares that the file should only store the single, highest detail mip and flag to readers
    /// that they should generate the remainder of the mip chain themselves.
    pub fn generate_mips(&mut self, yes: bool) -> &mut Self {
        self.generate_mips = yes;
        self
    }

    pub fn image_1d(&mut self, width: u32, level_num: u32, images: ImageSet<'a>) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(level_num, 0);
        assert_eq!(level_num as usize, images.len());

        self.level_num = level_num;
        self.width = width;
        self.doc_type = DocumentTypeDescription::Image1D { images };
        self
    }

    pub fn image_1d_array(
        &mut self,
        width: u32,
        layer_num: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(level_num, 0);
        assert_ne!(layer_num, 0);

        let required_image_num = layer_num as usize * level_num as usize;
        assert_eq!(required_image_num, images.len());

        self.level_num = level_num;
        self.width = width;
        self.doc_type = DocumentTypeDescription::Array1D { layer_num, images };
        self
    }

    pub fn image_2d(
        &mut self,
        width: u32,
        height: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(level_num, 0);
        assert_eq!(level_num as usize, images.len());
        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.doc_type = DocumentTypeDescription::Image2D { images };
        self
    }

    pub fn image_2d_array(
        &mut self,
        width: u32,
        height: u32,
        layer_num: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(level_num, 0);
        assert_ne!(layer_num, 0);

        let required_image_num = layer_num as usize * level_num as usize;
        assert_eq!(required_image_num, images.len());

        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.doc_type = DocumentTypeDescription::Array2D { layer_num, images };
        self
    }

    pub fn image_3d(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(depth, 0);
        assert_ne!(level_num, 0);
        assert_eq!(level_num as usize, images.len());
        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.depth = depth;
        self.doc_type = DocumentTypeDescription::Image3D { images };
        self
    }

    pub fn image_3d_array(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        layer_num: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(depth, 0);
        assert_ne!(level_num, 0);
        assert_ne!(layer_num, 0);

        let required_image_num = layer_num as usize * level_num as usize;
        assert_eq!(required_image_num, images.len());

        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.depth = depth;
        self.doc_type = DocumentTypeDescription::Array3D { layer_num, images };
        self
    }

    pub fn cube(
        &mut self,
        width: u32,
        height: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(level_num, 0);

        let required_image_num = 6 * level_num as usize;
        assert_eq!(required_image_num, images.len());

        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.doc_type = DocumentTypeDescription::Cube { images };
        self
    }

    pub fn cube_array(
        &mut self,
        width: u32,
        height: u32,
        layer_num: u32,
        level_num: u32,
        images: ImageSet<'a>,
    ) -> &mut Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);
        assert_ne!(level_num, 0);
        assert_ne!(layer_num, 0);

        let required_image_num = layer_num as usize * 6 * level_num as usize;
        assert_eq!(required_image_num, images.len());

        self.level_num = level_num;
        self.width = width;
        self.height = height;
        self.doc_type = DocumentTypeDescription::CubeArray { layer_num, images };
        self
    }

    pub fn write(&self, dst: &mut impl Write) -> std::io::Result<()> {
        self.validate();

        let type_size = format_type_size(self.format)
            .expect("Can't write a format we don't have a known type-size for");
        let texel_block_size = self
            .format
            .texel_block_size_ktx()
            .expect("Can't write a format we don't have a known texel block size for");
        let mip_padding = lcm(texel_block_size, 4) as u64;

        let dfd = vk2dfd(self.format.0)
            .expect("We must always have a DFD for a VkFormat we're encoding for");
        let dfd = bytemuck::cast_slice::<_, u8>(dfd);

        // The DFD offset is easy to calculate as it's simply the size of the header + size of the
        // level index. The level index is simply 3 u64 values per level stored so we can calculate
        // it all up front.
        let level_num = self.level_num as usize;
        let level_index_size = 3 * size_of::<u64>() * level_num;
        let dfd_offset = HEADER_SIZE + level_index_size;
        let dfd_offset: u32 = dfd_offset.try_into().expect("DFD offset overflows u32!");
        let dfd_bytes: u32 = dfd.len().try_into().expect("DFD length overflows u32!");

        let (kvd_offset, kvd_length) = if self.kvd.entries.is_empty() {
            (0, 0)
        } else {
            // KVD offset is immediately after dfd block so it's simply the dfd_offset + dfd_bytes.
            let kvd_offset = dfd_offset
                .checked_add(dfd_bytes)
                .expect("KVD offset overflows u32!");
            let mut kvd_length = 0;
            for entry in self.kvd.entries {
                // +4 to include 'keyAndValueByteLength'
                kvd_length += entry.total_len() + 4;
            }

            let kvd_length: u32 = kvd_length.try_into().expect("KVD length overflows u32!");
            (kvd_offset, kvd_length)
        };

        dst.write_all(&FILE_IDENTIFIER)?;
        dst.write_u32::<LittleEndian>(self.format.0)?;
        dst.write_u32::<LittleEndian>(type_size)?;
        dst.write_u32::<LittleEndian>(self.width)?;
        dst.write_u32::<LittleEndian>(self.height)?;
        dst.write_u32::<LittleEndian>(self.depth)?;
        dst.write_u32::<LittleEndian>(self.doc_type.encoded_layer_count())?;
        dst.write_u32::<LittleEndian>(self.doc_type.encoded_face_count())?;
        dst.write_u32::<LittleEndian>(self.level_num as u32)?;
        dst.write_u32::<LittleEndian>(SuperCompressionScheme::NONE.0)?;
        dst.write_u32::<LittleEndian>(dfd_offset)?;
        dst.write_u32::<LittleEndian>(dfd_bytes)?;
        dst.write_u32::<LittleEndian>(kvd_offset)?;
        dst.write_u32::<LittleEndian>(kvd_length)?;
        dst.write_u64::<LittleEndian>(0)?; // sgdByteOffset (should be zero, we dont support it)
        dst.write_u64::<LittleEndian>(0)?; // sgdByteLength (should be zero, we dont support it)

        // Texture data starts after kvd section, we don't support super compression so we don't
        // need to leave space for it
        let data_base_offset = kvd_offset + kvd_length;

        let level_index =
            self.doc_type
                .resolve_level_index(level_num, data_base_offset as usize, mip_padding);

        // Write the calculated level index out
        for level_i in 0..level_num {
            let level = level_index[level_i];
            dst.write_u64::<LittleEndian>(level.0)?;
            dst.write_u64::<LittleEndian>(level.1)?;
            dst.write_u64::<LittleEndian>(level.1)?;
        }

        dst.write_all(dfd)?;

        for entry in self.kvd.entries {
            // Can't overflow as it's checked earlier
            dst.write_u32::<LittleEndian>(entry.len() as u32)?;
            dst.write_all(entry.key.as_bytes())?;
            dst.write_u8(0)?;
            dst.write_all(entry.value)?;
            for _ in 0..entry.padding_bytes() {
                dst.write_u8(0)?;
            }
        }

        let mut accum = data_base_offset as u64;
        match self.doc_type {
            DocumentTypeDescription::Image1D { images }
            | DocumentTypeDescription::Image2D { images }
            | DocumentTypeDescription::Image3D { images } => {
                for level_i in (0..level_num).into_iter().rev() {
                    let level = &images[level_i];

                    // Forward align the level to the next mip_padding boundary and write mip
                    // padding bytes
                    let padding = accum.next_multiple_of(mip_padding) - accum;
                    for _ in 0..padding {
                        dst.write_u8(0)?;
                    }
                    accum = accum + padding;

                    // Write all the level data
                    dst.write_all(level)?;

                    // And push the offset along over the written data
                    accum = accum + level.len() as u64;
                }
            }
            DocumentTypeDescription::Cube { images } => {
                for level_i in (0..level_num).into_iter().rev() {
                    // Forward align the level to the next mip_padding boundary and write mip
                    // padding bytes
                    let padding = accum.next_multiple_of(mip_padding) - accum;
                    for _ in 0..padding {
                        dst.write_u8(0)?;
                    }
                    accum = accum + padding;

                    for face_i in 0..6 {
                        let i = calculate_set_index(6, level_num, face_i, level_i);
                        let level = images[i];

                        // Write all the level data
                        dst.write_all(level)?;

                        // And push the offset along over the written data
                        accum = accum + level.len() as u64;
                    }
                }
            }
            DocumentTypeDescription::Array1D { layer_num, images }
            | DocumentTypeDescription::Array2D { layer_num, images }
            | DocumentTypeDescription::Array3D { layer_num, images } => {
                for level_i in (0..level_num).into_iter().rev() {
                    // Forward align the level to the next mip_padding boundary and write mip
                    // padding bytes
                    let padding = accum.next_multiple_of(mip_padding) - accum;
                    for _ in 0..padding {
                        dst.write_u8(0)?;
                    }
                    accum = accum + padding;

                    for layer_i in 0..layer_num as usize {
                        let i =
                            calculate_set_index(layer_num as usize, level_num, layer_i, level_i);
                        let level = images[i];

                        // Write all the level data
                        dst.write_all(level)?;

                        // And push the offset along over the written data
                        accum = accum + level.len() as u64;
                    }
                }
            }
            DocumentTypeDescription::CubeArray { layer_num, images } => {
                let layer_num_real = layer_num as usize * 6;
                for level_i in (0..level_num).into_iter().rev() {
                    // Forward align the level to the next mip_padding boundary and write mip
                    // padding bytes
                    let padding = accum.next_multiple_of(mip_padding) - accum;
                    for _ in 0..padding {
                        dst.write_u8(0)?;
                    }
                    accum = accum + padding;

                    for layer_i in 0..layer_num_real {
                        let i = calculate_set_index(layer_num_real, level_num, layer_i, level_i);
                        let level = images[i];

                        // Write all the level data
                        dst.write_all(level)?;

                        // And push the offset along over the written data
                        accum = accum + level.len() as u64;
                    }
                }
            }
        }

        Ok(())
    }

    fn validate(&self) {
        assert_ne!(self.format, VkFormat::UNDEFINED);
        assert!(self.format.is_known());
        assert!(
            !(self.format.is_block_compressed() && self.generate_mips),
            "It is illegal to request mip generation for block compressed formats"
        );

        let texel_block_size = self
            .format
            .texel_block_size_ktx()
            .expect("Format has no known texel_block_size");

        let (bw, bh, bd) = self.format.block_dimensions();
        if bw > 1 {
            assert_ne!(
                self.width, 0,
                "Format '{:?}' must be at least 1 pixel width",
                self.format
            );
        }
        if bh > 1 {
            assert_ne!(
                self.height, 0,
                "Format '{:?}' must be at least 1 pixel height",
                self.format
            );
        }
        if bd > 1 {
            assert_ne!(
                self.depth, 0,
                "Format '{:?}' must be at least 1 pixel depth",
                self.format
            );
        }

        assert_ne!(self.level_num, 0);
        if self.generate_mips {
            assert_eq!(self.level_num, 1);
        }

        #[allow(unstable_name_collisions)]
        match self.doc_type {
            DocumentTypeDescription::Image1D { images }
            | DocumentTypeDescription::Image2D { images }
            | DocumentTypeDescription::Image3D { images } => {
                assert_ne!(images.len(), 0);
                assert_eq!(images.len(), self.level_num as usize);
                self.validate_levels(images, (bw, bh, bd), texel_block_size);
            }
            DocumentTypeDescription::Cube { images } => {
                assert_ne!(images.len(), 0);
                assert!(images.len().is_multiple_of(&6));
                assert!(images.len().is_multiple_of(&(self.level_num as usize)));
                for layer_levels in images.chunks_exact(self.level_num as usize) {
                    self.validate_levels(layer_levels, (bw, bh, bd), texel_block_size);
                }
            }
            DocumentTypeDescription::Array1D { layer_num, images }
            | DocumentTypeDescription::Array2D { layer_num, images }
            | DocumentTypeDescription::Array3D { layer_num, images } => {
                assert_ne!(layer_num, 0);
                assert_ne!(images.len(), 0);
                assert!(images.len().is_multiple_of(&(self.level_num as usize)));
                for layer_levels in images.chunks_exact(self.level_num as usize) {
                    self.validate_levels(layer_levels, (bw, bh, bd), texel_block_size);
                }
            }
            DocumentTypeDescription::CubeArray { layer_num, images } => {
                assert_ne!(layer_num, 0);
                assert_ne!(images.len(), 0);
                assert!(images.len().is_multiple_of(&6));
                assert!(images.len().is_multiple_of(&(self.level_num as usize)));
                for layer_levels in images.chunks_exact(self.level_num as usize) {
                    self.validate_levels(layer_levels, (bw, bh, bd), texel_block_size);
                }
            }
        }
    }

    fn validate_levels(
        &self,
        images: &[&[u8]],
        (bw, bh, bd): (u32, u32, u32),
        texel_block_size: usize,
    ) {
        assert_ne!(self.level_num, 0);

        let store_num = self.level_num as usize;
        let got_num = images.len();
        assert_eq!(
            store_num, got_num,
            "Must provide exactly '{}' mip levels to encode the image. Got '{}'",
            store_num, got_num
        );

        for (i, level) in images.iter().enumerate() {
            let storage_width = (self.width.max(1) >> i).max(1);
            let storage_height = (self.height.max(1) >> i).max(1);
            let storage_depth = (self.depth.max(1) >> i).max(1);
            let storage_bw = storage_width.div_ceil(bw) as usize;
            let storage_bh = storage_height.div_ceil(bh) as usize;
            let storage_bd = storage_depth.div_ceil(bd) as usize;
            let expected_mip_size = storage_bw * storage_bh * storage_bd * texel_block_size;
            assert_eq!(level.len(), expected_mip_size);
        }

        for window in images.windows(2) {
            let bigger = window[0];
            let smaller = window[1];
            assert!(
                bigger.len() >= smaller.len(),
                "Larger mip level must come first!"
            );
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LevelNum {
    /// Only store a single mip level in the texture and declare that readers should generate their
    /// own mip chain
    Generate,

    /// Store exactly the given number of mips
    Explicit(NonZero<u32>),
}

impl LevelNum {
    /// Creates a new [`LevelNum`] instance based on the KTX format rules for the level_num
    /// encoding.
    ///
    /// The rules state that:
    /// - if 'v' = 0, the file should store a single mip level and requests the reader to generate
    ///   a full mip chain.
    /// - if 'v' != 0, the file stores exactly the number of mip levels the texture is supposed to
    ///   have even if it doesn't form a full mip chain.
    pub fn from_num(v: u32) -> LevelNum {
        match NonZero::new(v) {
            Some(v) => Self::Explicit(v),
            None => Self::Generate,
        }
    }

    /// Returns the number of mip levels that should be stored in the file irrespective of the
    /// 'generate' flag.
    pub fn storage_level_num(&self) -> u32 {
        match self {
            LevelNum::Generate => 1,
            LevelNum::Explicit(non_zero) => non_zero.get(),
        }
    }

    /// Returns whether the file will declare readers to generate their own mip levels.
    pub fn is_generate(&self) -> bool {
        matches!(self, Self::Generate)
    }

    /// Returns a u32 that represents this [`LevelNum`] as would be stored in a KTX file.
    pub fn encoded_level_num(&self) -> u32 {
        match self {
            LevelNum::Generate => 0,
            LevelNum::Explicit(v) => v.get(),
        }
    }
}

/// A type alias for an array of buffer references. An [`ImageSet`] has a well defined order for how
/// the input images should be ordered in an image set.
///
/// # Singular 1D/2D/3D Image
///
/// A single image set is super simple. It's just a slice with length 1 to the image data.
///
/// # Mipmapped 1D/2D/3D Image
///
/// A mip mapped image is the next simplest case. Logically we're still encoding a single image,
/// just including the mip chain. This case should be ordered from such that set[0] = mip0,
/// set[1] = mip[1], etc...
///
/// # Array 1D/2D/3D Image
///
/// This is very similar to the mip-mapped non-array case. The same ordering is expected:
/// set[0] = arr[0], set[1] = arr[1], etc...
///
/// # Mipmapped Array 1D/2D/3D Image
///
/// This is the most complicated case. The set of images has multiple levels that need to be encoded
/// correctly for the image to come out correctly. We use a different order to how KTX actually
/// encodes the data into the file that is easier to work with compared to easy to serialize.
///
/// `set.len()` must be equal to `image_count * mip_levels` such that there is a buffer reference
/// for each image in the array, and the associated mips.
///
/// The data is ordered like a topological sort of the hierarchy of `array layer -> mip level`. We
/// order the array such that, given an array of 4 images with 2 mip levels:
///
/// - set[0] = arr[0].mip[0]
/// - set[0] = arr[0].mip[1]
/// - set[0] = arr[1].mip[0]
/// - set[0] = arr[1].mip[1]
/// - set[0] = arr[2].mip[0]
/// - set[0] = arr[2].mip[1]
/// - set[0] = arr[3].mip[0]
/// - set[0] = arr[3].mip[1]
///
/// # Cube Maps
///
/// Cube maps are just a special case of array texture, mipmapped or otherwise, where the array
/// length is required to be a multiple of 6. A single cube-map is just an array of 6 images with
/// a well-known ordering. A cube-map array is just an array image where the array length is always
/// a multiple of 6, and the cube array entries are stored one after the other.
pub type ImageSet<'a> = &'a [&'a [u8]];

/// Calculates the index into an [`ImageSet`], assuming the given number of layers and levels in
/// the set, for the given `layer` and `level`.
pub fn calculate_set_index(
    layer_num: usize,
    level_num: usize,
    layer: usize,
    level: usize,
) -> usize {
    assert!(layer < layer_num);
    assert!(level < level_num);

    let i = layer * level_num;
    let i = i + level;
    i
}

enum DocumentTypeDescription<'a> {
    Image1D {
        images: ImageSet<'a>,
    },
    Image2D {
        images: ImageSet<'a>,
    },
    Image3D {
        images: ImageSet<'a>,
    },
    Cube {
        images: ImageSet<'a>,
    },
    Array1D {
        layer_num: u32,
        images: ImageSet<'a>,
    },
    Array2D {
        layer_num: u32,
        images: ImageSet<'a>,
    },
    Array3D {
        layer_num: u32,
        images: ImageSet<'a>,
    },
    CubeArray {
        layer_num: u32,
        images: ImageSet<'a>,
    },
}

impl<'a> DocumentTypeDescription<'a> {
    const fn encoded_layer_count(&self) -> u32 {
        match self {
            DocumentTypeDescription::Image1D { .. } => 0,
            DocumentTypeDescription::Image2D { .. } => 0,
            DocumentTypeDescription::Image3D { .. } => 0,
            DocumentTypeDescription::Cube { .. } => 0,
            DocumentTypeDescription::Array1D { layer_num, .. } => *layer_num,
            DocumentTypeDescription::Array2D { layer_num, .. } => *layer_num,
            DocumentTypeDescription::Array3D { layer_num, .. } => *layer_num,
            DocumentTypeDescription::CubeArray { layer_num, .. } => *layer_num,
        }
    }

    const fn encoded_face_count(&self) -> u32 {
        match self {
            DocumentTypeDescription::Image1D { .. } => 1,
            DocumentTypeDescription::Image2D { .. } => 1,
            DocumentTypeDescription::Image3D { .. } => 1,
            DocumentTypeDescription::Cube { .. } => 6,
            DocumentTypeDescription::Array1D { .. } => 1,
            DocumentTypeDescription::Array2D { .. } => 1,
            DocumentTypeDescription::Array3D { .. } => 1,
            DocumentTypeDescription::CubeArray { .. } => 6,
        }
    }

    fn resolve_level_index(
        &self,
        level_num: usize,
        data_base_offset: usize,
        mip_padding: u64,
    ) -> [(u64, u64); 32] {
        assert!(level_num <= 32);

        // Implement a two pass level indexing algorithm.
        //
        // Pass 1 fills only the sizes of each layer in the order we will write them in
        // (reverse of input order). level_index should then look something like:
        // - (0, 64),
        // - (0, 16),
        // - (0, 4),
        //
        // The offsets can be calculated as (mostly) the prefix sum of the lengths in
        // reverse order. Pass 2 performs this prefix sum + some alignment padding that's
        // not included in the lengths. level_index should look something like:
        // - (20, 64),
        // - (4, 16),
        // - (0, 4),
        //
        // This example doesn't take into account padding, but the meaning should be clear.
        // Padding will simply move the offsets a bit further along

        let mut level_index = [(0u64, 0u64); 32];
        match self {
            DocumentTypeDescription::Image1D { images }
            | DocumentTypeDescription::Image2D { images }
            | DocumentTypeDescription::Image3D { images } => {
                // Pass 1, fill out the length for each level in order from biggest to smallest
                for level_i in 0..level_num {
                    level_index[level_i].1 = images[level_i].len() as u64;
                }
            }
            DocumentTypeDescription::Cube { images } => {
                for level_i in 0..level_num {
                    let mut size = 0u64;
                    for face_i in 0..6 {
                        let i = calculate_set_index(6, level_num, face_i, level_i);
                        let level = images[i];
                        size += level.len() as u64;
                    }
                    level_index[level_i].1 = size;
                }
            }
            DocumentTypeDescription::Array1D { layer_num, images }
            | DocumentTypeDescription::Array2D { layer_num, images }
            | DocumentTypeDescription::Array3D { layer_num, images } => {
                for level_i in 0..level_num {
                    let mut size = 0u64;
                    for layer_i in 0..*layer_num as usize {
                        let i =
                            calculate_set_index(*layer_num as usize, level_num, layer_i, level_i);
                        let level = images[i];
                        size += level.len() as u64;
                    }
                    level_index[level_i].1 = size;
                }
            }
            DocumentTypeDescription::CubeArray { layer_num, images } => {
                let layer_num_real = *layer_num as usize * 6;
                for level_i in 0..level_num {
                    let mut size = 0u64;
                    for layer_i in 0..layer_num_real {
                        let i = calculate_set_index(layer_num_real, level_num, layer_i, level_i);
                        let level = images[i];
                        size += level.len() as u64;
                    }
                    level_index[level_i].1 = size;
                }
            }
        }

        // Pass 2, prefix sum lengths in reverse order to get our final offsets
        let mut accum = data_base_offset as u64;
        for level_i in (0..level_num).into_iter().rev() {
            let level = &mut level_index[level_i];

            // Forward align the level to the next mip_padding boundary
            accum = accum.next_multiple_of(mip_padding);

            // Assign the offset
            level.0 = accum;

            // Add the length of the current level to the offset
            accum += level.1;
        }

        level_index
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KeyValueEntries<'a> {
    entries: &'a [KeyValueEntry<'a>],
}

impl<'a> KeyValueEntries<'a> {
    /// Takes the given list of KVD entries and produces a correctly sorted and validated list of
    /// KVD pairs that can be given to a KTX writer to be included in the output.
    pub fn sort(entries: &'a mut [KeyValueEntry<'a>]) -> Option<Self> {
        Some(Self { entries: &*entries })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct KeyValueEntry<'a> {
    key: &'a str,
    value: &'a [u8],
}

impl<'a> KeyValueEntry<'a> {
    /// Short hand for defining a [`KeyValueEntry`] for the 'KTXwriter' key prefilled with this
    /// crate's standard writer ID.
    pub const fn writer() -> Self {
        Self {
            key: "KTXwriter",
            value: ENCODER_NAME_CSTR.as_bytes(),
        }
    }

    /// The number of bytes needed to write exactly the key name, null terminator and value data.
    pub const fn len(&self) -> usize {
        // Length of the key + null terminator + value data
        self.key.len() + 1 + self.value.len()
    }

    /// [`Self::len`] + up to 3 additional padding bytes to ensure we keep each kvd pair on a 4 byte
    /// boundary.
    pub const fn total_len(&self) -> usize {
        let len = self.len();
        len.next_multiple_of(4)
    }

    /// The number of padding bytes needed to round [`Self::len`] up to [`Self::total_len`].
    pub const fn padding_bytes(&self) -> usize {
        let len = self.len();
        let padded = len.next_multiple_of(4);
        padded - len
    }
}

const IDENT_SIZE: usize = 12;
const HEADER_SIZE: usize = IDENT_SIZE + 68;
const ENCODER_NAME_CSTR: &str =
    concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"), "\0");
