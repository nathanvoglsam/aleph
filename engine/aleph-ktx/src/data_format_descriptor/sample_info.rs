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

use std::io::{Cursor, Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::data_format_descriptor::SampleFlags;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SampleInfo {
    pub bit_offset: u16,
    pub bit_length: u8,
    pub channel_type: u8,
    pub sample_flags: SampleFlags,
    pub sample_positions: [u8; 4],
    pub sample_lower: u32,
    pub sample_upper: u32,
}

impl SampleInfo {
    ///
    /// Unpacks the sample info from 4 words (should have been read from a file)
    ///
    #[allow(clippy::identity_op)]
    pub fn unpack_from(words: &[u32; 4]) -> Self {
        let bit_offset = ((words[0] >> 0) & 0xFFFF) as u16;
        let bit_length = ((words[0] >> 16) & 0xFF) as u8;
        let channel_type = ((words[0] >> 24) & 0xF) as u8;
        let sample_flags = ((words[0] >> 28) & 0xF) as u8;
        let sample_flags = SampleFlags::from_bits_retain(sample_flags);

        let sample_pos_0 = ((words[1] >> 0) & 0xFF) as u8;
        let sample_pos_1 = ((words[1] >> 8) & 0xFF) as u8;
        let sample_pos_2 = ((words[1] >> 16) & 0xFF) as u8;
        let sample_pos_3 = ((words[1] >> 24) & 0xFF) as u8;
        let sample_positions = [sample_pos_0, sample_pos_1, sample_pos_2, sample_pos_3];

        let sample_lower = words[2];
        let sample_upper = words[3];

        Self {
            bit_offset,
            bit_length,
            channel_type,
            sample_flags,
            sample_positions,
            sample_upper,
            sample_lower,
        }
    }
}

///
/// Iterates over the sample information blocks in the DFD block
///
pub struct SampleInfoIterator<R: Read + Seek> {
    reader: R,
    index: u16,
    count: u16,
}

impl<R: Read + Seek> SampleInfoIterator<R> {
    ///
    /// Will create an iterator that will yield `count` samples from the reader
    ///
    /// # Info
    ///
    /// There can't be more than `u16::max_value` sample infos because of how the number is stored
    /// in the file so we use a u16 for `count` to make this explicit (also makes the struct small)
    ///
    pub fn from_reader_count(reader: R, count: u16) -> Self {
        Self {
            reader,
            index: 0,
            count,
        }
    }
}

impl<'a> SampleInfoIterator<Cursor<&'a [u8]>> {
    pub fn from_words(words: &'a [u32]) -> Self {
        assert_eq!(words.len() % 4, 0);
        let count = words.len() / 4;
        let bytes = bytemuck::cast_slice::<_, u8>(words);
        let cursor = Cursor::new(bytes);
        Self::from_reader_count(cursor, count as u16)
    }
}

impl SampleInfo {
    pub fn compatible_with(&self, other: &Self) -> bool {
        let compat = self.bit_offset == other.bit_offset
            && self.bit_length == other.bit_length
            && self.channel_type == other.channel_type
            && self.sample_positions == other.sample_positions
            && self.sample_lower == other.sample_lower
            && self.sample_upper == other.sample_upper;
        let flags_compat = self.sample_flags.compatible_with(other.sample_flags);
        compat && flags_compat
    }
}

impl<R: Read + Seek> Iterator for SampleInfoIterator<R> {
    type Item = (usize, SampleInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            None
        } else {
            let mut words = [0; 4];
            self.reader.read_u32_into::<LittleEndian>(&mut words).ok()?;

            // Build output and iterate the index
            let out = Some((self.index as usize, SampleInfo::unpack_from(&words)));
            self.index += 1;
            out
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}
